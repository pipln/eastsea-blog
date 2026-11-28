//! upwork-fee-calculator — 1:1 port of
//! tools/upwork-fee-calculator/calculator.js
//!
//! Legacy module exposes `calculateForward` and `calculateReverse`; the wasm
//! ABI registers both as tool ids (`upwork-fee-calculator` and
//! `upwork-fee-calculator:reverse`). Rounding uses the Number.EPSILON
//! variants; tier ladder, withdrawal fee functions, tier progress shape
//! (including Infinity -> JSON null) and the 60-iteration reverse binary
//! search are replicated exactly.

use crate::jsnum::{js_number, num, round2_eps, round4_eps};
use crate::JsonMap;
use serde_json::{json, Value};

const INPUT_MIN: f64 = 0.0;
const INPUT_MAX: f64 = 999_999.0;
const BINARY_SEARCH_ITERATIONS: u32 = 60;

const TIER_MAXES: [f64; 3] = [500.0, 10_000.0, f64::INFINITY];
const TIER_RATES: [f64; 3] = [0.20, 0.10, 0.05];
const TIER_STARTS: [f64; 3] = [0.0, 500.0, 10_000.0];
const TIER_ENDS: [f64; 3] = [500.0, 10_000.0, f64::INFINITY];
const METHOD_KEYS: [&str; 4] = ["ach", "paypal", "wire", "instant"];

// (Legacy `negativeInput` is unreachable in JS — the range checks fire first
// — so it is intentionally absent here.)

struct Text {
    invalid_gross: &'static str,
    invalid_prior: &'static str,
    invalid_net: &'static str,
    invalid_method: &'static str,
}

fn text_for(lang: &str) -> Text {
    if lang == "ko" {
        Text {
            invalid_gross: "청구 금액은 0~999,999 사이의 숫자여야 합니다.",
            invalid_prior: "누적 청구액은 0~999,999 사이의 숫자여야 합니다.",
            invalid_net: "목표 순수익은 1~999,999 사이의 숫자여야 합니다.",
            invalid_method: "유효하지 않은 출금 방법입니다.",
        }
    } else {
        Text {
            invalid_gross: "Invoice amount must be a number between $0 and $999,999.",
            invalid_prior: "Prior lifetime billings must be a number between $0 and $999,999.",
            invalid_net: "Target net must be a number between $1 and $999,999.",
            invalid_method: "Invalid withdrawal method.",
        }
    }
}

/// WITHDRAWAL_METHODS[key].fee(amt)
fn withdrawal_fee_fn(key: &str, amt: f64) -> f64 {
    match key {
        "ach" => 0.0,
        "paypal" => 0.99f64.max(amt * 0.01),
        "wire" => 30.0,
        "instant" => 15.0f64.min(0.25f64.max(amt * 0.01)),
        // Legacy: `if (!m) return 0;` — unknown methods fall back to 0
        // (validation rejects them earlier on the public paths).
        _ => 0.0,
    }
}

fn method_label(key: &str) -> (&'static str, &'static str) {
    match key {
        "ach" => ("ACH (US Bank)", "ACH (미국 은행)"),
        "paypal" => ("PayPal", "PayPal"),
        "wire" => ("Wire Transfer", "국제 전신송금"),
        "instant" => ("Instant Pay (Visa)", "Instant Pay (Visa)"),
        _ => ("", ""),
    }
}

struct ServiceFee {
    total: f64,
    breakdown: Vec<Value>,
}

/// calcServiceFee(gross, prior)
fn calc_service_fee(gross: f64, prior: f64) -> ServiceFee {
    let mut fee = 0.0f64;
    let mut breakdown: Vec<Value> = Vec::new();
    let mut remaining = gross;
    let mut cursor = prior;

    for (i, &tier_max) in TIER_MAXES.iter().enumerate() {
        if remaining <= 0.0 {
            break;
        }
        if cursor >= tier_max {
            continue;
        }
        let in_tier = remaining.min(tier_max - cursor);
        let tier_fee = in_tier * TIER_RATES[i];
        breakdown.push(json!({
            "inTier": num(round2_eps(in_tier)),
            "rate": num(TIER_RATES[i]),
            "tierFee": num(round2_eps(tier_fee)),
            "tierIndex": Value::from(i as i64),
        }));
        fee += tier_fee;
        remaining -= in_tier;
        cursor += in_tier;
    }

    ServiceFee {
        total: round2_eps(fee),
        breakdown,
    }
}

/// calcWithdrawalFee(grossAfterServiceFee, method)
fn calc_withdrawal_fee(amt: f64, method: &str) -> f64 {
    // Legacy returns 0 un-rounded for unknown methods; round2(0) === 0.
    round2_eps(withdrawal_fee_fn(method, amt))
}

/// calcTierProgress(gross, prior) — Infinity fields become JSON null
/// (JSON.stringify semantics), matching the legacy wire format.
fn calc_tier_progress(gross: f64, prior: f64) -> Value {
    let end_total = prior + gross;
    let mut current_tier_index: usize = 0;
    for i in (0..TIER_MAXES.len()).rev() {
        let bound = if i == 0 { 0.0 } else { TIER_STARTS[i] };
        if end_total > bound {
            current_tier_index = i;
            break;
        }
    }

    let tier_start = TIER_STARTS[current_tier_index];
    let tier_end = TIER_ENDS[current_tier_index];
    let tier_rate = TIER_RATES[current_tier_index];
    let is_max_tier = current_tier_index == TIER_MAXES.len() - 1;

    let amount_in_tier = if is_max_tier {
        end_total - tier_start
    } else {
        end_total.min(tier_end) - tier_start
    };
    let tier_size: Option<f64> = if is_max_tier {
        None
    } else {
        Some(tier_end - tier_start)
    };
    let pct_through_tier = if is_max_tier {
        100.0
    } else {
        round4_eps((amount_in_tier / tier_size.unwrap()) * 100.0)
    };
    let remaining = if is_max_tier {
        0.0
    } else {
        round2_eps(tier_end - end_total.min(tier_end))
    };
    let next_rate: Option<f64> = if is_max_tier {
        None
    } else {
        Some(TIER_RATES[current_tier_index + 1])
    };

    let tier_size_json = match tier_size {
        Some(v) => num(v),
        None => Value::Null,
    };
    let next_rate_json = match next_rate {
        Some(v) => num(v),
        None => Value::Null,
    };

    json!({
        "currentTierIndex": Value::from(current_tier_index as i64),
        "currentRate": num(tier_rate),
        "tierStart": num(tier_start),
        "tierEnd": num(tier_end),
        "amountInTier": num(round2_eps(amount_in_tier)),
        "tierSize": tier_size_json,
        "pctThroughTier": num(pct_through_tier),
        "remaining": num(remaining),
        "nextRate": next_rate_json,
        "isMaxTier": Value::Bool(is_max_tier),
        "endTotal": num(round2_eps(end_total)),
    })
}

/// calcRequiredGross(netTarget, prior, method) — 60-iteration binary search.
fn calc_required_gross(net_target: f64, prior: f64, method: &str) -> f64 {
    let mut lo = net_target;
    let mut hi = net_target * 1.35 + 50.0; // generous upper bound

    for _ in 0..BINARY_SEARCH_ITERATIONS {
        let mid = (lo + hi) / 2.0;
        let sf = calc_service_fee(mid, prior);
        let after_fee = mid - sf.total;
        let w_fee = calc_withdrawal_fee(after_fee, method);
        let net = after_fee - w_fee;
        if net < net_target {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    round2_eps((lo + hi) / 2.0)
}

/// calcAllWithdrawals(grossAfterServiceFee) — key order: ach, paypal, wire,
/// instant (Object.keys order of the legacy WITHDRAWAL_METHODS literal).
fn calc_all_withdrawals(gross_after_service_fee: f64) -> Value {
    let mut map = JsonMap::new();
    for key in METHOD_KEYS {
        let w_fee = round2_eps(withdrawal_fee_fn(key, gross_after_service_fee));
        let (label, label_ko) = method_label(key);
        map.insert(
            key.to_string(),
            json!({
                "label": label,
                "labelKo": label_ko,
                "fee": num(w_fee),
                "net": num(round2_eps(gross_after_service_fee - w_fee)),
            }),
        );
    }
    Value::Object(map)
}

/// `Number(input.<key>)` — missing key -> NaN (Number(undefined)); null -> 0.
fn num_field(map: &JsonMap, key: &str) -> f64 {
    match map.get(key) {
        None => f64::NAN,
        Some(v) => js_number(v),
    }
}

/// `input.method || 'ach'` — empty string / null / missing -> "ach";
/// truthy non-strings coerce to object keys that never exist -> invalid.
/// Returns None for the latter.
fn method_field(map: &JsonMap) -> Option<String> {
    match map.get("method") {
        Some(Value::String(s)) => {
            if s.is_empty() {
                Some("ach".to_string())
            } else {
                Some(s.clone())
            }
        }
        Some(Value::Null) | None => Some("ach".to_string()),
        Some(_) => None,
    }
}

fn valid_method(method: &Option<String>) -> bool {
    match method {
        Some(m) => METHOD_KEYS.contains(&m.as_str()),
        None => false,
    }
}

fn validate_forward(
    gross: f64,
    prior: f64,
    method: &Option<String>,
    t: &Text,
) -> Result<(), String> {
    if gross.is_nan() || gross < INPUT_MIN || gross > INPUT_MAX {
        return Err(t.invalid_gross.to_string());
    }
    if prior.is_nan() || prior < INPUT_MIN || prior > INPUT_MAX {
        return Err(t.invalid_prior.to_string());
    }
    if !valid_method(method) {
        return Err(t.invalid_method.to_string());
    }
    Ok(())
}

fn validate_reverse(
    net_target: f64,
    prior: f64,
    method: &Option<String>,
    t: &Text,
) -> Result<(), String> {
    if net_target.is_nan() || net_target < 1.0 || net_target > INPUT_MAX {
        return Err(t.invalid_net.to_string());
    }
    if prior.is_nan() || prior < INPUT_MIN || prior > INPUT_MAX {
        return Err(t.invalid_prior.to_string());
    }
    if !valid_method(method) {
        return Err(t.invalid_method.to_string());
    }
    Ok(())
}

/// calculateForward(input, {lang})
pub fn run_forward(map: &JsonMap, lang: &str) -> Result<Value, String> {
    let t = text_for(lang);
    let gross = num_field(map, "gross");
    let prior = num_field(map, "prior");
    let method = method_field(map);

    validate_forward(gross, prior, &method, &t)?;
    let method = method.unwrap();

    let sf = calc_service_fee(gross, prior);
    let gross_after_service_fee = round2_eps(gross - sf.total);
    let w_fee = calc_withdrawal_fee(gross_after_service_fee, &method);
    let net = round2_eps(gross_after_service_fee - w_fee);
    let effective_rate = if gross > 0.0 {
        round4_eps(((sf.total + w_fee) / gross) * 100.0)
    } else {
        0.0
    };
    let tier_progress = calc_tier_progress(gross, prior);
    let all_withdrawals = calc_all_withdrawals(gross_after_service_fee);

    Ok(json!({
        "gross": num(gross),
        "prior": num(prior),
        "method": method,
        "serviceFeeBreakdown": Value::Array(sf.breakdown),
        "totalServiceFee": num(sf.total),
        "grossAfterServiceFee": num(gross_after_service_fee),
        "withdrawalFee": num(w_fee),
        "net": num(net),
        "effectiveRate": num(effective_rate),
        "tierProgress": tier_progress,
        "allWithdrawals": all_withdrawals,
    }))
}

/// calculateReverse(input, {lang})
pub fn run_reverse(map: &JsonMap, lang: &str) -> Result<Value, String> {
    let t = text_for(lang);
    let net_target = num_field(map, "netTarget");
    let prior = num_field(map, "prior");
    let method = method_field(map);

    validate_reverse(net_target, prior, &method, &t)?;
    let method = method.unwrap();

    let required_gross = calc_required_gross(net_target, prior, &method);
    let sf = calc_service_fee(required_gross, prior);
    let gross_after_service_fee = round2_eps(required_gross - sf.total);
    let w_fee = calc_withdrawal_fee(gross_after_service_fee, &method);
    let net = round2_eps(gross_after_service_fee - w_fee);
    let effective_rate = if required_gross > 0.0 {
        round4_eps(((sf.total + w_fee) / required_gross) * 100.0)
    } else {
        0.0
    };
    let tier_progress = calc_tier_progress(required_gross, prior);
    let all_withdrawals = calc_all_withdrawals(gross_after_service_fee);

    Ok(json!({
        "netTarget": num(net_target),
        "prior": num(prior),
        "method": method,
        "requiredGross": num(required_gross),
        "serviceFeeBreakdown": Value::Array(sf.breakdown),
        "totalServiceFee": num(sf.total),
        "grossAfterServiceFee": num(gross_after_service_fee),
        "withdrawalFee": num(w_fee),
        "net": num(net),
        "effectiveRate": num(effective_rate),
        "tierProgress": tier_progress,
        "allWithdrawals": all_withdrawals,
    }))
}
