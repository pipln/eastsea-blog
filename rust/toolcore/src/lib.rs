//! toolcore — single Rust core for the Eastsea tool calculators (P1 pilot).
//!
//! Public ABI (wasm-bindgen):
//! `invoke(tool_id: &str, input_json: &str) -> String`
//!
//! * Input: a (possibly partial) JSON object; missing keys are filled from the
//!   legacy `DEFAULTS` where the legacy module defines them (stripe). Tools
//!   without defaults (bond) reproduce the legacy missing-key validation
//!   error exactly. An optional reserved `"lang"` key (`"en"` | `"ko"`,
//!   default `"en"`) selects the error-message/summary locale and is removed
//!   before the tool sees the input.
//! * Output:
//!   `{"ok":true,"tool":"<id>","result":{...},"error":""}` on success, or
//!   `{"ok":false,"error":"<reason>"}` on failure (unknown tool, invalid JSON
//!   or the tool's own validation error / thrown message).

mod jsnum;
mod tools;

/// serde_json 1.0.15x + preserve_order: Map is generic (IndexMap-backed).
pub type JsonMap = serde_json::Map<String, Value>;

use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

/// Plain-Rust dispatcher (also the native test entry point).
pub fn invoke_impl(tool_id: &str, input_json: &str) -> String {
    let (input, lang) = match parse_input(input_json) {
        Ok(x) => x,
        Err(msg) => return failure(&msg),
    };

    let outcome: Result<Value, String> = match tool_id {
        tools::STRIPE_ID => tools::stripe::run(&input, &lang),
        tools::BOND_ID => tools::bond::run(&input),
        tools::UPWORK_ID => tools::upwork::run_forward(&input, &lang),
        tools::UPWORK_REVERSE_ID => tools::upwork::run_reverse(&input, &lang),
        _ => Err(format!("unknown tool: {}", tool_id)),
    };

    match outcome {
        Ok(result) => json!({
            "ok": true,
            "tool": tool_id,
            "result": result,
            "error": "",
        })
        .to_string(),
        Err(msg) => failure(&msg),
    }
}

fn failure(msg: &str) -> String {
    json!({ "ok": false, "error": msg }).to_string()
}

/// Parse the input JSON, extract the reserved `"lang"` key, and normalize
/// non-object payloads to an empty object (legacy JS reads missing
/// properties as undefined -> defaults).
fn parse_input(input_json: &str) -> Result<(JsonMap, String), String> {
    let v: Value =
        serde_json::from_str(input_json).map_err(|_| "invalid input JSON".to_string())?;
    let map = match v {
        Value::Object(m) => m,
        _ => JsonMap::new(),
    };
    let lang = match map.get("lang") {
        Some(Value::String(s)) => s.clone(),
        _ => "en".to_string(),
    };
    let mut map = map;
    map.remove("lang");
    Ok((map, lang))
}

/// Public wasm ABI.
#[wasm_bindgen]
pub fn invoke(tool_id: &str, input_json: &str) -> String {
    invoke_impl(tool_id, input_json)
}

#[cfg(test)]
mod tests {
    use super::invoke_impl;

    fn ok_of(out: &str) -> serde_json::Value {
        let v: serde_json::Value = serde_json::from_str(out).unwrap();
        assert_eq!(
            v["ok"],
            serde_json::json!(true),
            "expected ok, got: {}",
            out
        );
        v
    }

    #[test]
    fn abi_envelope_shapes() {
        let out = invoke_impl("no-such-tool", "{}");
        assert_eq!(out, r#"{"ok":false,"error":"unknown tool: no-such-tool"}"#);
        let out = invoke_impl("stripe-fee-profit-calculator", "not json");
        assert_eq!(out, r#"{"ok":false,"error":"invalid input JSON"}"#);
        let out = invoke_impl("bond-yield-calculator", "{}");
        assert_eq!(
            out,
            r#"{"ok":false,"error":"faceValue must be a positive number"}"#
        );
        let v = ok_of(&invoke_impl(
            "bond-yield-calculator",
            r#"{"faceValue":1000,"couponRate":5,"currentPrice":950,"yearsToMaturity":10,"purchasePrice":950,"taxRate":15,"quantity":10}"#,
        ));
        assert_eq!(v["tool"], "bond-yield-calculator");
        assert_eq!(v["error"], "");
        assert!(v["result"].is_object());
    }

    #[test]
    fn partial_input_uses_legacy_defaults() {
        // stripe: {} -> all defaults (TC-F001)
        let v = ok_of(&invoke_impl("stripe-fee-profit-calculator", "{}"));
        assert_eq!(v["result"]["revenueExTax"], serde_json::json!(100));
        assert_eq!(v["result"]["netProfit"], serde_json::json!(66.8));
        // single-key partial input: only salePrice overrides
        let v = ok_of(&invoke_impl(
            "stripe-fee-profit-calculator",
            r#"{"salePrice": 200}"#,
        ));
        assert_eq!(v["result"]["revenueExTax"], serde_json::json!(200));
        assert_eq!(v["result"]["sellerCostTotal"], serde_json::json!(30));
        // null value behaves like a missing key (?? semantics)
        let v = ok_of(&invoke_impl(
            "stripe-fee-profit-calculator",
            r#"{"salePrice": null}"#,
        ));
        assert_eq!(v["result"]["revenueExTax"], serde_json::json!(100));
    }

    #[test]
    fn lang_reserved_key_is_stripped_and_used() {
        let out = invoke_impl(
            "stripe-fee-profit-calculator",
            r#"{"lang":"ko","salePrice":-1}"#,
        );
        assert_eq!(
            out,
            r#"{"ok":false,"error":"금액 입력값은 모두 0 이상이어야 합니다."}"#
        );
        // default lang is en
        let out = invoke_impl("stripe-fee-profit-calculator", r#"{"salePrice":-1}"#);
        assert_eq!(
            out,
            r#"{"ok":false,"error":"All money fields must be zero or above."}"#
        );
    }

    #[test]
    fn bond_golden_base_case() {
        // calculator.test.js BASE case (TC-01..TC-06, TC-11)
        let v = ok_of(&invoke_impl(
            "bond-yield-calculator",
            r#"{"faceValue":1000,"couponRate":5,"currentPrice":950,"yearsToMaturity":10,"purchasePrice":950,"taxRate":15,"quantity":10}"#,
        ));
        let r = &v["result"];
        assert_eq!(r["currentYield"], serde_json::json!(5.26));
        assert_eq!(r["annualCouponPerBond"], serde_json::json!(50));
        assert_eq!(r["annualCouponIncome"], serde_json::json!(500));
        assert_eq!(r["ytmApprox"], serde_json::json!(5.64));
        assert_eq!(r["afterTaxAnnualIncome"], serde_json::json!(425));
        assert_eq!(r["afterTaxYield"], serde_json::json!(4.47));
        assert_eq!(r["capitalGainLoss"], serde_json::json!(500));
        assert_eq!(r["totalInvestment"], serde_json::json!(9500));
        assert_eq!(r["totalCouponIncome"], serde_json::json!(5000));
        assert_eq!(r["totalReturn"], serde_json::json!(5500));
        assert_eq!(r["annualizedReturn"], serde_json::json!(5.79));
        assert_eq!(r["bondStatus"], serde_json::json!("discount"));
        assert_eq!(r["projections"][0]["year"], serde_json::json!(1));
        assert_eq!(r["projections"][0]["cumulative"], serde_json::json!(500));
        assert_eq!(
            r["projections"][0]["afterTaxIncome"],
            serde_json::json!(425)
        );
        assert_eq!(r["projections"][4]["cumulative"], serde_json::json!(2500));
    }

    #[test]
    fn bond_validation_messages_match_legacy_throws() {
        let cases = [
            (
                r#"{"faceValue":-1,"couponRate":5,"currentPrice":950,"yearsToMaturity":10,"purchasePrice":950,"taxRate":15,"quantity":10}"#,
                "faceValue must be a positive number",
            ),
            (
                r#"{"faceValue":1000,"couponRate":5,"currentPrice":950,"yearsToMaturity":0,"purchasePrice":950,"taxRate":15,"quantity":10}"#,
                "yearsToMaturity must be an integer between 1 and 50",
            ),
            (
                r#"{"faceValue":1000,"couponRate":5,"currentPrice":950,"yearsToMaturity":10,"purchasePrice":950,"taxRate":60,"quantity":10}"#,
                "taxRate must be between 0 and 50",
            ),
            (
                r#"{"faceValue":1000,"couponRate":5,"currentPrice":950,"yearsToMaturity":10,"purchasePrice":950,"taxRate":15,"quantity":0}"#,
                "quantity must be an integer between 1 and 10000",
            ),
            // missing key -> same message as typeof undefined !== 'number'
            (r#"{}"#, "faceValue must be a positive number"),
            // non-integer years
            (
                r#"{"faceValue":1000,"couponRate":5,"currentPrice":950,"yearsToMaturity":10.5,"purchasePrice":950,"taxRate":15,"quantity":10}"#,
                "yearsToMaturity must be an integer between 1 and 50",
            ),
        ];
        for (input, expected) in cases {
            let out = invoke_impl("bond-yield-calculator", input);
            let v: serde_json::Value = serde_json::from_str(&out).unwrap();
            assert_eq!(v["ok"], serde_json::json!(false), "input: {}", input);
            assert_eq!(v["error"], serde_json::json!(expected), "input: {}", input);
        }
    }

    #[test]
    fn upwork_forward_golden() {
        // A1: $300, prior $0, ACH
        let v = ok_of(&invoke_impl(
            "upwork-fee-calculator",
            r#"{"gross":300,"prior":0,"method":"ach"}"#,
        ));
        let r = &v["result"];
        assert_eq!(r["totalServiceFee"], serde_json::json!(60));
        assert_eq!(r["net"], serde_json::json!(240));
        assert_eq!(r["withdrawalFee"], serde_json::json!(0));
        assert_eq!(r["effectiveRate"].as_f64().unwrap(), 20.0);
        assert_eq!(
            r["serviceFeeBreakdown"][0]["inTier"].as_f64().unwrap(),
            300.0
        );
        assert_eq!(
            r["serviceFeeBreakdown"][0]["tierFee"].as_f64().unwrap(),
            60.0
        );
        assert_eq!(r["tierProgress"]["pctThroughTier"].as_f64().unwrap(), 60.0);

        // A4: Tier 2 -> Tier 3 transition
        let v = ok_of(&invoke_impl(
            "upwork-fee-calculator",
            r#"{"gross":1000,"prior":9600,"method":"ach"}"#,
        ));
        let r = &v["result"];
        assert_eq!(r["totalServiceFee"], serde_json::json!(70));
        assert_eq!(r["net"], serde_json::json!(930));
        assert_eq!(
            r["serviceFeeBreakdown"][0]["inTier"],
            serde_json::json!(400)
        );
        assert_eq!(
            r["serviceFeeBreakdown"][1]["inTier"],
            serde_json::json!(600)
        );
    }

    #[test]
    fn upwork_tier_progress_max_tier_json_nulls() {
        // prior $15000 -> fully in Tier 3: tierEnd/tierSize/nextRate are
        // Infinity/Infinity/null in JS -> JSON null.
        let v = ok_of(&invoke_impl(
            "upwork-fee-calculator",
            r#"{"gross":500,"prior":15000,"method":"ach"}"#,
        ));
        let tp = &v["result"]["tierProgress"];
        assert_eq!(tp["currentTierIndex"], serde_json::json!(2));
        assert_eq!(tp["isMaxTier"], serde_json::json!(true));
        assert_eq!(tp["tierEnd"], serde_json::Value::Null);
        assert_eq!(tp["tierSize"], serde_json::Value::Null);
        assert_eq!(tp["nextRate"], serde_json::Value::Null);
        assert_eq!(tp["endTotal"], serde_json::json!(15500));
    }

    #[test]
    fn upwork_reverse_golden() {
        // C1: target $240, prior $0, ACH -> gross $300
        let v = ok_of(&invoke_impl(
            "upwork-fee-calculator:reverse",
            r#"{"netTarget":240,"prior":0,"method":"ach"}"#,
        ));
        assert_eq!(v["result"]["requiredGross"], serde_json::json!(300));
        // C4: target $1000, prior $10000, wire -> $1084.21
        let v = ok_of(&invoke_impl(
            "upwork-fee-calculator:reverse",
            r#"{"netTarget":1000,"prior":10000,"method":"wire"}"#,
        ));
        let rg = v["result"]["requiredGross"].as_f64().unwrap();
        assert!((rg - 1084.21).abs() <= 0.02, "requiredGross = {}", rg);
    }

    #[test]
    fn upwork_validation_and_default_method() {
        // missing method -> 'ach'; missing gross -> invalidGross (en)
        let out = invoke_impl("upwork-fee-calculator", r#"{}"#);
        assert_eq!(
            out,
            r#"{"ok":false,"error":"Invoice amount must be a number between $0 and $999,999."}"#
        );
        let out = invoke_impl("upwork-fee-calculator", r#"{"gross":"bitcoin"}"#);
        assert_eq!(
            out,
            r#"{"ok":false,"error":"Invoice amount must be a number between $0 and $999,999."}"#
        );
        let out = invoke_impl(
            "upwork-fee-calculator",
            r#"{"gross":100,"prior":0,"method":"bitcoin"}"#,
        );
        assert_eq!(out, r#"{"ok":false,"error":"Invalid withdrawal method."}"#);
        let out = invoke_impl(
            "upwork-fee-calculator",
            r#"{"lang":"ko","gross":-1,"prior":0,"method":"ach"}"#,
        );
        assert_eq!(
            out,
            r#"{"ok":false,"error":"청구 금액은 0~999,999 사이의 숫자여야 합니다."}"#
        );
    }

    #[test]
    fn stripe_summary_and_break_even() {
        let v = ok_of(&invoke_impl("stripe-fee-profit-calculator", "{}"));
        let r = &v["result"];
        let summary = r["summary"].as_str().unwrap();
        assert!(summary.contains("[Stripe Fee Profit Summary]"));
        assert!(summary.contains("Sale price: $100.00"));
        assert!(summary.contains("Net profit: $66.80"));
        // break-even is a number below 100 for defaults
        let be = r["breakEvenSalePrice"].as_f64().unwrap();
        assert!(be < 100.0 && be > 0.0, "breakEven = {}", be);
        // TC-F006: unreachable break-even -> null
        let v = ok_of(&invoke_impl(
            "stripe-fee-profit-calculator",
            r#"{"salePrice":100,"productCost":999999,"stripeFeeRate":99}"#,
        ));
        assert_eq!(v["result"]["breakEvenSalePrice"], serde_json::Value::Null);
        assert!(v["result"]["summary"]
            .as_str()
            .unwrap()
            .contains("Break-even price: N/A"));
    }
}
