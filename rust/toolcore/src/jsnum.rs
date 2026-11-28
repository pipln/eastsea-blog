//! JavaScript numeric semantics helpers.
//!
//! The legacy calculators are plain JS; the migration gate is *behavioral
//! identity*, so this module reproduces the exact ECMAScript semantics of the
//! numeric operations the calculators rely on:
//!
//! - `Math.round(v)`   — nearest integer, ties toward +infinity
//!                       (`Math.round(-2.5) === -2`, `Math.round(2.5) === 3`).
//! - `Number.EPSILON`  — 2^-52, identical to Rust's `f64::EPSILON`.
//! - `x.toFixed(2)`    — spec-exact: pick integer `n` minimizing `|n/100 - x|`
//!                       over the *real* value of the double, ties pick the
//!                       larger `n`; sign handled first.
//! - `JSON.stringify`  — NaN/Infinity serialize as `null`; integral floats
//!                       (< 2^53) print without a fractional part.
//!
//! Plain Rust `{:.2}` / `f64::round()` differ (half-to-even / ties-away-from-
//! zero), so they must NOT be used on values that reach tool output.

use serde_json::Value;

/// `Math.round(v)` with exact ECMAScript semantics.
pub fn js_round(v: f64) -> f64 {
    if v.is_nan() || v.is_infinite() || v == 0.0 {
        return v;
    }
    // |v| >= 2^52 => every double there is already an integer.
    if v.abs() >= 4503599627370496.0 {
        return v;
    }
    // For 0 < |v| < 2^52 the subtraction below is exact (Sterbenz-style),
    // so the fractional-part comparison is bit-exact. Ties (frac == 0.5)
    // round toward +infinity, matching Math.round for both signs.
    let f = v.floor();
    if v - f >= 0.5 {
        f + 1.0
    } else {
        f
    }
}

/// `round2` as in stripe/upwork calculators:
/// `Math.round((v + Number.EPSILON) * 100) / 100`
pub fn round2_eps(v: f64) -> f64 {
    js_round((v + f64::EPSILON) * 100.0) / 100.0
}

/// `round4` as in stripe/upwork calculators:
/// `Math.round((v + Number.EPSILON) * 10000) / 10000`
pub fn round4_eps(v: f64) -> f64 {
    js_round((v + f64::EPSILON) * 10000.0) / 10000.0
}

/// `round2` as in bond-yield-calculator: `Math.round(n * 100) / 100`
/// (no Number.EPSILON guard).
pub fn round2_plain(v: f64) -> f64 {
    js_round(v * 100.0) / 100.0
}

/// `x.toFixed(2)` with spec-exact semantics (ECMA-262 22.1.3.3).
///
/// NaN -> "NaN", +/-Infinity -> "Infinity"/"-Infinity". Negative numbers
/// extract the sign first (so -0.001 gives "-0.00" but -0 gives "0.00").
/// The rounding operates on the *exact rational* value of `a * 100`, not on
/// the floating-point product — this is where naive `format!("{:.2}", x)`
/// implementations diverge from JS (half-to-even vs. spec ties-larger-n).
pub fn to_fixed2(x: f64) -> String {
    if x.is_nan() {
        return "NaN".to_string();
    }
    if x.is_infinite() {
        return if x > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    let a = x.abs();
    if a >= 1e21 {
        // Spec: |x| >= 10^21 -> ToString(x) (exponential form).
        return js_to_string_exponential(x);
    }
    let neg = x < 0.0; // -0 < 0 is false, so -0 formats unsigned like JS.
    let n = round_half_up_100(a);
    let whole = n / 100;
    let frac = n % 100;
    if neg {
        format!("-{}.{:02}", whole, frac)
    } else {
        format!("{}.{:02}", whole, frac)
    }
}

/// Exact `round_half_up(a * 100)` for finite `a >= 0`, `a < 1e21`, computed on
/// the exact rational value of `a * 100` (a dyadic rational), ties round up.
fn round_half_up_100(a: f64) -> u128 {
    let bits = a.to_bits();
    let raw_exp = ((bits >> 52) & 0x7ff) as i64;
    let frac = bits & 0x000f_ffff_ffff_ffff;
    let (mantissa, shift): (u64, i64) = if raw_exp == 0 {
        (frac, -1074) // subnormal
    } else {
        (frac | (1u64 << 52), raw_exp - 1075)
    };
    // a * 100 == (25 * mantissa) * 2^(shift + 2), exactly.
    let m25 = (mantissa as u128) * 25u128;
    let s = shift + 2;
    if s >= 0 {
        // a < 1e21 => a*100 < 1e23 < 2^77; m25 < 2^58 — fits u128.
        m25 << s
    } else {
        let k = (-s) as u32; // >= 1
        if k >= 128 {
            // a*100 < 2^-68 — nowhere near the 0.5 tie.
            0
        } else {
            let d = 1u128 << k;
            (m25 + (d >> 1)) / d
        }
    }
}

/// JS `ToString` for |x| >= 10^21: shortest round-trip with "e+NN" form.
fn js_to_string_exponential(x: f64) -> String {
    let s = format!("{:e}", x);
    if let Some(pos) = s.find('e') {
        let (mant, exp) = s.split_at(pos);
        let exp = &exp[1..]; // strip 'e'
        if !exp.starts_with('-') {
            return format!("{}e+{}", mant, exp);
        }
    }
    s
}

/// Serialize an f64 like `JSON.stringify` of a JS number:
/// non-finite -> `null`; integral values with |v| < 2^53 -> JSON integer
/// (JS prints `500`, not `500.0`); otherwise shortest round-trip float.
///
/// Domain note: tool outputs are bounded well below 2^53 and above 1e-4 in
/// magnitude, where JS and ryu agree on the shortest decimal form.
pub fn num(v: f64) -> Value {
    if v.is_finite() {
        if v == v.trunc() && v.abs() < 9007199254740992.0 {
            Value::from(v as i64)
        } else {
            Value::from(v)
        }
    } else {
        Value::Null
    }
}

/// `Number(v)` coercion for JSON values (the subset reachable from tool
/// inputs). Missing values are handled by callers (undefined -> NaN vs.
/// `?? default` -> legacy default).
pub fn js_number(v: &Value) -> f64 {
    match v {
        Value::Number(n) => n.as_f64().unwrap_or(f64::NAN),
        Value::Bool(b) => {
            if *b {
                1.0
            } else {
                0.0
            }
        }
        Value::Null => 0.0,
        Value::String(s) => js_parse_number(s),
        Value::Array(a) => {
            if a.is_empty() {
                0.0
            } else if a.len() == 1 {
                js_number(&a[0])
            } else {
                f64::NAN
            }
        }
        Value::Object(_) => f64::NAN,
    }
}

/// `Number(str)` for the common JS grammar: trimmed; "" -> 0; the Infinity
/// forms; decimal notation. Hex / octal literals and other exotic forms
/// return NaN (documented divergence — never exercised by tool inputs).
fn js_parse_number(s: &str) -> f64 {
    let t = s.trim();
    if t.is_empty() {
        return 0.0;
    }
    match t {
        "Infinity" | "+Infinity" => return f64::INFINITY,
        "-Infinity" => return f64::NEG_INFINITY,
        _ => {}
    }
    let js_numeric_charset = t
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-');
    if !js_numeric_charset {
        return f64::NAN;
    }
    t.parse::<f64>().unwrap_or(f64::NAN)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected values below were produced by running the actual JS semantics
    // in node (see rust/README.md — parity methodology).

    #[test]
    fn math_round_ties_and_signs() {
        assert_eq!(js_round(2.5), 3.0); // Math.round(2.5) === 3
        assert_eq!(js_round(-2.5), -2.0); // Math.round(-2.5) === -2
        assert_eq!(js_round(-0.5), 0.0); // Math.round(-0.5) === -0
        assert_eq!(js_round(2.4), 2.0);
        assert_eq!(js_round(-2.6), -3.0);
        assert_eq!(js_round(-0.49999999999999994), 0.0); // classic FP edge
        assert_eq!(js_round(0.49999999999999994), 0.0);
        assert_eq!(js_round(4503599627370496.5), 4503599627370496.5); // >= 2^52 integral
        assert!(js_round(f64::NAN).is_nan());
        assert_eq!(js_round(f64::INFINITY), f64::INFINITY);
        assert_eq!(js_round(0.0), 0.0);
    }

    #[test]
    fn round2_eps_matches_js() {
        // node -e 'Math.round((0.005 + Number.EPSILON)*100)/100' -> 0.01
        assert_eq!(round2_eps(0.005), 0.01);
        // node: Math.round((0.0049999999 + Number.EPSILON)*100)/100 -> 0
        assert_eq!(round2_eps(0.0049999999), 0.0);
        // node: Math.round((-0.125 + Number.EPSILON)*100)/100 -> -0.12
        assert_eq!(round2_eps(-0.125), -0.12);
        // node: Math.round((66.79999999999997 + Number.EPSILON)*100)/100 -> 66.8
        assert_eq!(round2_eps(66.79999999999997), 66.8);
    }

    #[test]
    fn round2_plain_matches_js() {
        // node: Math.round(-0.125 * 100)/100 -> -0.12 (half toward +inf)
        assert_eq!(round2_plain(-0.125), -0.12);
        // node: Math.round(2.675 * 100)/100 -> 2.68 (the FP product 2.675*100
        // lands above 267.5; note this differs from toFixed(2) -> "2.67"
        // which operates on the exact rational value).
        assert_eq!(round2_plain(2.675), 2.68);
    }

    #[test]
    fn to_fixed2_matches_js() {
        // Values verified in node against the real toFixed:
        assert_eq!(to_fixed2(66.8), "66.80");
        assert_eq!(to_fixed2(0.125), "0.13"); // tie -> larger n
        assert_eq!(to_fixed2(-0.125), "-0.13");
        assert_eq!(to_fixed2(-0.001), "-0.00"); // JS keeps the sign
        assert_eq!(to_fixed2(-0.0), "0.00"); // (-0).toFixed(2) === "0.00"
        assert_eq!(to_fixed2(0.0), "0.00");
        assert_eq!(to_fixed2(96.8), "96.80");
        assert_eq!(to_fixed2(3.2), "3.20");
        assert_eq!(to_fixed2(f64::NAN), "NaN");
        assert_eq!(to_fixed2(f64::INFINITY), "Infinity");
        assert_eq!(to_fixed2(f64::NEG_INFINITY), "-Infinity");
        // node: (1.005).toFixed(2) === "1.00" (double is below 1.005)
        assert_eq!(to_fixed2(1.005), "1.00");
    }

    #[test]
    fn num_json_stringify_semantics() {
        assert_eq!(num(500.0).to_string(), "500");
        assert_eq!(num(0.1).to_string(), "0.1");
        assert_eq!(num(96.8).to_string(), "96.8");
        assert_eq!(num(-0.0).to_string(), "0");
        assert_eq!(num(f64::INFINITY).to_string(), "null");
        assert_eq!(num(f64::NAN).to_string(), "null");
        assert_eq!(num(-30.25).to_string(), "-30.25");
    }

    #[test]
    fn js_number_coercions() {
        assert_eq!(js_number(&serde_json::json!(50.5)), 50.5);
        assert_eq!(js_number(&serde_json::json!("50.5")), 50.5);
        assert_eq!(js_number(&serde_json::json!("")), 0.0);
        assert_eq!(js_number(&serde_json::json!("  ")), 0.0);
        assert!(js_number(&serde_json::json!("abc")).is_nan());
        assert_eq!(js_number(&serde_json::Value::Null), 0.0);
        assert_eq!(js_number(&serde_json::json!(true)), 1.0);
        assert!(js_number(&serde_json::json!({"a":1})).is_nan());
        assert_eq!(js_number(&serde_json::json!([])), 0.0);
    }
}
