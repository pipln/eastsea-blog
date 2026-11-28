//! bond-yield-calculator — 1:1 port of
//! tools/bond-yield-calculator/calculator.js
//!
//! Legacy behavior: strict `typeof === 'number'` validation that *throws*
//! Error(...) on bad input (the only pilot tool using the throw convention —
//! mapped to the wasm `{"ok":false,"error":<message>}` envelope). round2 is
//! the plain `Math.round(n*100)/100` variant (no Number.EPSILON).

use crate::jsnum::{num, round2_plain};
use crate::JsonMap;
use serde_json::{json, Value};

const ERR_FACE: &str = "faceValue must be a positive number";
const ERR_COUPON: &str = "couponRate must be between 0 and 30";
const ERR_PRICE: &str = "currentPrice must be a positive number";
const ERR_YEARS: &str = "yearsToMaturity must be an integer between 1 and 50";
const ERR_PURCHASE: &str = "purchasePrice must be a positive number";
const ERR_TAX: &str = "taxRate must be between 0 and 50";
const ERR_QTY: &str = "quantity must be an integer between 1 and 10000";

/// `typeof map[key] === 'number'` — JSON numbers only.
fn get_number(map: &JsonMap, key: &str) -> Option<f64> {
    match map.get(key) {
        Some(Value::Number(n)) => n.as_f64(),
        _ => None,
    }
}

fn req_positive(map: &JsonMap, key: &str, err: &str) -> Result<f64, String> {
    match get_number(map, key) {
        Some(v) if v > 0.0 => Ok(v),
        _ => Err(err.to_string()),
    }
}

fn req_range(map: &JsonMap, key: &str, lo: f64, hi: f64, err: &str) -> Result<f64, String> {
    match get_number(map, key) {
        Some(v) if v >= lo && v <= hi => Ok(v),
        _ => Err(err.to_string()),
    }
}

/// `Number.isInteger(v) && lo <= v && v <= hi`
fn req_integer_range(map: &JsonMap, key: &str, lo: f64, hi: f64, err: &str) -> Result<f64, String> {
    match get_number(map, key) {
        Some(v) if v.fract() == 0.0 && v >= lo && v <= hi => Ok(v),
        _ => Err(err.to_string()),
    }
}

pub fn run(map: &JsonMap) -> Result<Value, String> {
    // Validation order is part of the observable behavior (error messages).
    let face_value = req_positive(map, "faceValue", ERR_FACE)?;
    let coupon_rate = req_range(map, "couponRate", 0.0, 30.0, ERR_COUPON)?;
    let current_price = req_positive(map, "currentPrice", ERR_PRICE)?;
    let years = req_integer_range(map, "yearsToMaturity", 1.0, 50.0, ERR_YEARS)?;
    let purchase_price = req_positive(map, "purchasePrice", ERR_PURCHASE)?;
    let tax_rate = req_range(map, "taxRate", 0.0, 50.0, ERR_TAX)?;
    let quantity = req_integer_range(map, "quantity", 1.0, 10000.0, ERR_QTY)?;

    let annual_coupon_per_bond = round2_plain(face_value * coupon_rate / 100.0);
    let annual_coupon_income = round2_plain(annual_coupon_per_bond * quantity);

    let current_yield = if current_price > 0.0 {
        round2_plain((annual_coupon_per_bond / current_price) * 100.0)
    } else {
        0.0
    };

    // YTM approximation formula — same expression tree as the JS source.
    let ytm_approx = round2_plain(
        ((annual_coupon_per_bond + (face_value - current_price) / years)
            / ((face_value + current_price) / 2.0))
            * 100.0,
    );

    let after_tax_annual_income = round2_plain(annual_coupon_income * (1.0 - tax_rate / 100.0));
    let after_tax_yield = round2_plain(current_yield * (1.0 - tax_rate / 100.0));

    let total_investment = round2_plain(purchase_price * quantity);
    let capital_gain_loss = round2_plain((face_value - purchase_price) * quantity);
    let total_coupon_income = round2_plain(annual_coupon_income * years);
    let total_return = round2_plain(total_coupon_income + capital_gain_loss);
    let annualized_return = if total_investment > 0.0 {
        round2_plain((total_return / total_investment) / years * 100.0)
    } else {
        0.0
    };

    // Bond status
    let bond_status = if current_price < face_value {
        "discount"
    } else if current_price > face_value {
        "premium"
    } else {
        "par"
    };

    // Year-by-year projection (coupon is fixed for bonds)
    let mut projections: Vec<Value> = Vec::new();
    let mut cumulative = 0.0;
    for y in 1..=(years as i64) {
        cumulative = round2_plain(cumulative + annual_coupon_income);
        let after_tax = round2_plain(annual_coupon_income * (1.0 - tax_rate / 100.0));
        projections.push(json!({
            "year": Value::from(y),
            "annualIncome": num(annual_coupon_income),
            "afterTaxIncome": num(after_tax),
            "cumulative": num(cumulative),
        }));
    }

    Ok(json!({
        "annualCouponPerBond": num(annual_coupon_per_bond),
        "annualCouponIncome": num(annual_coupon_income),
        "currentYield": num(current_yield),
        "ytmApprox": num(ytm_approx),
        "afterTaxAnnualIncome": num(after_tax_annual_income),
        "afterTaxYield": num(after_tax_yield),
        "totalInvestment": num(total_investment),
        "capitalGainLoss": num(capital_gain_loss),
        "totalCouponIncome": num(total_coupon_income),
        "totalReturn": num(total_return),
        "annualizedReturn": num(annualized_return),
        "bondStatus": bond_status,
        "projections": projections,
    }))
}
