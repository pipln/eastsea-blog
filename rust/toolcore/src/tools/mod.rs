//! pilot tool registry
pub mod bond;
pub mod stripe;
pub mod upwork;

pub const STRIPE_ID: &str = "stripe-fee-profit-calculator";
pub const BOND_ID: &str = "bond-yield-calculator";
pub const UPWORK_ID: &str = "upwork-fee-calculator";
/// upwork exposes two legacy entry points (`calculateForward` /
/// `calculateReverse`); the reverse one is registered as a sub-tool id.
pub const UPWORK_REVERSE_ID: &str = "upwork-fee-calculator:reverse";
