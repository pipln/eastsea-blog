//! stripe-fee-profit-calculator — 1:1 port of
//! tools/stripe-fee-profit-calculator/calculator.js
//!
//! Formula order, rounding (round2/round4 with Number.EPSILON), validation
//! order/messages, break-even binary search and the summary text are
//! replicated exactly. No behavior changes allowed (parity gate).

use crate::jsnum::{js_number, num, round2_eps, round4_eps, to_fixed2};
use crate::JsonMap;
use serde_json::{json, Value};

const BREAK_EVEN_MAX: f64 = 1_000_000.0;
const BREAK_EVEN_ITERATIONS: u32 = 60;
const BREAK_EVEN_TOL: f64 = 0.001;

// Legacy DEFAULTS (used when a key is missing or null in the input JSON).
const D_SALE_PRICE: f64 = 100.0;
const D_SHIPPING: f64 = 0.0;
const D_TAX: f64 = 0.0;
const D_PRODUCT: f64 = 25.0;
const D_FULFILLMENT: f64 = 5.0;
const D_OPS: f64 = 0.0;
const D_FEE_RATE: f64 = 2.9;
const D_FLAT_FEE: f64 = 0.30;
const D_INTL_RATE: f64 = 0.0;
const D_FX_RATE: f64 = 0.0;

struct Text {
    money: &'static str,
    rate: &'static str,
    revenue: &'static str,
    summary_title: &'static str,
    na: &'static str,
}

fn text_for(lang: &str) -> Text {
    if lang == "ko" {
        Text {
            money: "금액 입력값은 모두 0 이상이어야 합니다.",
            rate: "비율(%) 입력값은 0에서 100 사이여야 합니다.",
            revenue: "판매가 + 배송비는 0보다 커야 합니다.",
            summary_title: "[Stripe 수수료·순이익 요약]",
            na: "N/A",
        }
    } else {
        Text {
            money: "All money fields must be zero or above.",
            rate: "Percentage inputs must be between 0 and 100.",
            revenue: "Sale price + shipping must be greater than zero.",
            summary_title: "[Stripe Fee Profit Summary]",
            na: "N/A",
        }
    }
}

struct Input {
    sale_price: f64,
    shipping_charged: f64,
    tax_collected: f64,
    product_cost: f64,
    fulfillment_cost: f64,
    ops_cost: f64,
    stripe_fee_rate: f64,
    stripe_flat_fee: f64,
    international_card_rate: f64,
    currency_conversion_rate: f64,
}

/// `Number(s.<key> ?? DEFAULTS.<key>)` — missing or null -> default, other
/// JSON values go through JS Number() coercion.
fn field(map: &JsonMap, key: &str, default: f64) -> f64 {
    match map.get(key) {
        None | Some(Value::Null) => default,
        Some(v) => js_number(v),
    }
}

fn normalize(map: &JsonMap) -> Input {
    Input {
        sale_price: field(map, "salePrice", D_SALE_PRICE),
        shipping_charged: field(map, "shippingCharged", D_SHIPPING),
        tax_collected: field(map, "taxCollected", D_TAX),
        product_cost: field(map, "productCost", D_PRODUCT),
        fulfillment_cost: field(map, "fulfillmentCost", D_FULFILLMENT),
        ops_cost: field(map, "opsCost", D_OPS),
        stripe_fee_rate: field(map, "stripeFeeRate", D_FEE_RATE),
        stripe_flat_fee: field(map, "stripeFlatFee", D_FLAT_FEE),
        international_card_rate: field(map, "internationalCardRate", D_INTL_RATE),
        currency_conversion_rate: field(map, "currencyConversionRate", D_FX_RATE),
    }
}

fn validate(inp: &Input, t: &Text) -> Result<(), String> {
    let money_fields = [
        inp.sale_price,
        inp.shipping_charged,
        inp.tax_collected,
        inp.product_cost,
        inp.fulfillment_cost,
        inp.ops_cost,
        inp.stripe_flat_fee,
    ];
    if money_fields.iter().any(|&v| v < 0.0) {
        return Err(t.money.to_string());
    }
    let rate_fields = [
        inp.stripe_fee_rate,
        inp.international_card_rate,
        inp.currency_conversion_rate,
    ];
    if rate_fields.iter().any(|&v| v < 0.0 || v > 100.0) {
        return Err(t.rate.to_string());
    }
    if inp.sale_price + inp.shipping_charged <= 0.0 {
        return Err(t.revenue.to_string());
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct AtPrice {
    revenue_ex_tax: f64,
    gross_charge: f64,
    stripe_variable_fee: f64,
    international_fee: f64,
    currency_conv_fee: f64,
    total_stripe_fees: f64,
    payout_after_fees: f64,
    payout_before_seller_costs: f64,
    seller_cost_total: f64,
    net_profit: f64,
}

fn compute_at_sale_price(sp: f64, inp: &Input) -> AtPrice {
    let revenue_ex_tax = sp + inp.shipping_charged;
    let gross_charge = revenue_ex_tax + inp.tax_collected;
    let stripe_variable_fee = gross_charge * (inp.stripe_fee_rate / 100.0);
    let international_fee = gross_charge * (inp.international_card_rate / 100.0);
    let currency_conv_fee = gross_charge * (inp.currency_conversion_rate / 100.0);
    let total_stripe_fees =
        stripe_variable_fee + inp.stripe_flat_fee + international_fee + currency_conv_fee;
    let payout_after_fees = gross_charge - total_stripe_fees;
    let payout_before_seller_costs = revenue_ex_tax - total_stripe_fees;
    let seller_cost_total = inp.product_cost + inp.fulfillment_cost + inp.ops_cost;
    let net_profit = payout_before_seller_costs - seller_cost_total;
    AtPrice {
        revenue_ex_tax,
        gross_charge,
        stripe_variable_fee,
        international_fee,
        currency_conv_fee,
        total_stripe_fees,
        payout_after_fees,
        payout_before_seller_costs,
        seller_cost_total,
        net_profit,
    }
}

fn find_break_even(inp: &Input) -> Option<f64> {
    let at_lo = compute_at_sale_price(0.0, inp).net_profit;
    let at_hi = compute_at_sale_price(BREAK_EVEN_MAX, inp).net_profit;

    if at_hi < 0.0 {
        return None; // never profitable even at huge price
    }
    if at_lo >= 0.0 {
        return Some(0.0); // already profitable at zero price
    }

    let mut lo = 0.0f64;
    let mut hi = BREAK_EVEN_MAX;
    for _ in 0..BREAK_EVEN_ITERATIONS {
        let mid = (lo + hi) / 2.0;
        let profit = compute_at_sale_price(mid, inp).net_profit;
        if profit.abs() <= BREAK_EVEN_TOL {
            return Some(round2_eps(mid));
        }
        if profit < 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Some(round2_eps((lo + hi) / 2.0))
}

fn build_summary(
    lang: &str,
    inp: &Input,
    r_revenue_ex_tax: f64,
    r_gross_charge: f64,
    r_total_stripe_fees: f64,
    r_payout_after_fees: f64,
    r_net_profit: f64,
    r_effective_rate: f64,
    r_net_margin: f64,
    r_break_even: Option<f64>,
    t: &Text,
) -> String {
    let ko = lang == "ko";
    // Legacy: const fmt = (v) => v == null ? na : '$' + v.toFixed(2);
    // (the '$' belongs to the number branch — N/A prints without it)
    let fmt_opt = |v: Option<f64>| match v {
        None => t.na.to_string(),
        Some(x) => format!("${}", to_fixed2(x)),
    };
    let lines = [
        t.summary_title.to_string(),
        String::new(),
        format!(
            "{}: ${}",
            if ko { "판매가" } else { "Sale price" },
            to_fixed2(inp.sale_price)
        ),
        format!(
            "{}: ${}",
            if ko {
                "세전 매출"
            } else {
                "Revenue ex tax"
            },
            to_fixed2(r_revenue_ex_tax)
        ),
        format!(
            "{}: ${}",
            if ko { "총 청구액" } else { "Gross charge" },
            to_fixed2(r_gross_charge)
        ),
        format!(
            "{}: ${}",
            if ko {
                "Stripe 총 수수료"
            } else {
                "Total Stripe fees"
            },
            to_fixed2(r_total_stripe_fees)
        ),
        format!(
            "{}: ${}",
            if ko {
                "수수료 공제 후 수취액"
            } else {
                "Payout after fees"
            },
            to_fixed2(r_payout_after_fees)
        ),
        format!(
            "{}: ${}",
            if ko { "순이익" } else { "Net profit" },
            to_fixed2(r_net_profit)
        ),
        format!(
            "{}: {}%",
            if ko {
                "실효 수수료율"
            } else {
                "Effective fee rate"
            },
            to_fixed2(r_effective_rate)
        ),
        format!(
            "{}: {}%",
            if ko { "순마진율" } else { "Net margin" },
            to_fixed2(r_net_margin)
        ),
        format!(
            "{}: {}",
            if ko {
                "손익분기 판매가"
            } else {
                "Break-even price"
            },
            fmt_opt(r_break_even)
        ),
    ];
    lines.join("\n")
}

pub fn run(map: &JsonMap, lang: &str) -> Result<Value, String> {
    let t = text_for(lang);
    let inp = normalize(map);

    validate(&inp, &t)?;

    let base = compute_at_sale_price(inp.sale_price, &inp);

    let effective_stripe_fee_rate_pct = if base.revenue_ex_tax > 0.0 {
        round4_eps((base.total_stripe_fees / base.revenue_ex_tax) * 100.0)
    } else {
        0.0
    };
    let net_margin_pct = if base.revenue_ex_tax > 0.0 {
        round4_eps((base.net_profit / base.revenue_ex_tax) * 100.0)
    } else {
        0.0
    };

    let break_even = find_break_even(&inp);

    // Rounded result values (exact key order of the legacy object literal).
    let r_revenue_ex_tax = round2_eps(base.revenue_ex_tax);
    let r_gross_charge = round2_eps(base.gross_charge);
    let r_stripe_variable_fee = round2_eps(base.stripe_variable_fee);
    let r_stripe_flat_fee = round2_eps(inp.stripe_flat_fee);
    let r_international_fee = round2_eps(base.international_fee);
    let r_currency_conversion_fee = round2_eps(base.currency_conv_fee);
    let r_total_stripe_fees = round2_eps(base.total_stripe_fees);
    let r_payout_after_fees = round2_eps(base.payout_after_fees);
    let r_payout_before_seller_costs = round2_eps(base.payout_before_seller_costs);
    let r_seller_cost_total = round2_eps(base.seller_cost_total);
    let r_net_profit = round2_eps(base.net_profit);

    let summary = build_summary(
        lang,
        &inp,
        r_revenue_ex_tax,
        r_gross_charge,
        r_total_stripe_fees,
        r_payout_after_fees,
        r_net_profit,
        effective_stripe_fee_rate_pct,
        net_margin_pct,
        break_even,
        &t,
    );

    let break_even_json = match break_even {
        Some(v) => num(v),
        None => Value::Null,
    };

    Ok(json!({
        "revenueExTax": num(r_revenue_ex_tax),
        "grossCharge": num(r_gross_charge),
        "stripeVariableFee": num(r_stripe_variable_fee),
        "stripeFlatFee": num(r_stripe_flat_fee),
        "internationalFee": num(r_international_fee),
        "currencyConversionFee": num(r_currency_conversion_fee),
        "totalStripeFees": num(r_total_stripe_fees),
        "payoutAfterFees": num(r_payout_after_fees),
        "payoutBeforeSellerCosts": num(r_payout_before_seller_costs),
        "sellerCostTotal": num(r_seller_cost_total),
        "netProfit": num(r_net_profit),
        "effectiveStripeFeeRatePct": num(effective_stripe_fee_rate_pct),
        "netMarginPct": num(net_margin_pct),
        "breakEvenSalePrice": break_even_json,
        "summary": summary,
    }))
}
