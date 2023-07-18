use bincode::config::Bounded;
use bincode::config::FixintEncoding;
use bincode::config::WithOtherIntEncoding;
use bincode::config::WithOtherLimit;
use bincode::DefaultOptions;
use bincode::Options;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct PrimeAndDivisor {
    pub prime: u128,
    pub divisor: u128,
}

pub static SERIALIZER: Lazy<
    WithOtherIntEncoding<WithOtherLimit<DefaultOptions, Bounded>, FixintEncoding>,
> = Lazy::new(|| {
    return DefaultOptions::new().with_limit(32).with_fixint_encoding();
});

pub fn compute_fast_divisor(x: u128) -> u128 {
    return u128::MAX.wrapping_div(x) + 1;
}
