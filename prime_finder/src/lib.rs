use bincode::config::Bounded;
use bincode::config::FixintEncoding;
use bincode::config::WithOtherIntEncoding;
use bincode::config::WithOtherLimit;
use bincode::DefaultOptions;
use bincode::Options;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Result;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct PrimeAndDivisor {
    pub prime: u128,
    pub divisor: u128,
}

impl PrimeAndDivisor {
    /// How many bits does this prime need.
    pub fn bits(&self) -> u32 {
        return 128 - self.prime.leading_zeros();
    }

    pub fn new(prime: u128) -> PrimeAndDivisor {
        return PrimeAndDivisor {
            prime: prime,
            divisor: compute_fast_divisor(prime),
        };
    }
}

pub static SERIALIZER: Lazy<
    WithOtherIntEncoding<WithOtherLimit<DefaultOptions, Bounded>, FixintEncoding>,
> = Lazy::new(|| {
    return DefaultOptions::new().with_limit(32).with_fixint_encoding();
});

pub fn compute_fast_divisor(x: u128) -> u128 {
    return u128::MAX.wrapping_div(x) + 1;
}

pub fn is_divisible(other: u128, prime: u128) -> bool {
    return other.wrapping_mul(prime) <= prime.wrapping_sub(1);
}

pub fn open_file(file_name: &String) -> Result<File> {
    match Path::new(file_name).canonicalize() {
        Err(err) => {
            eprintln!("{file_name} does not exist or cannot be accessed.");
            return Err(err);
        }
        Ok(path) => match File::open(path) {
            Ok(file) => {
                return Ok(file);
            }
            Err(err) => {
                eprintln!("{file_name} exists but cannot be opened.");
                return Err(err);
            }
        },
    }
}
