use std::ops::Rem;
use std::time::Instant;

const FIVE_DIVIDER: u128 = u128::MAX / 5 + 1;

/// A program for testing different ways of filtering primes.
fn main() {
    let start: Instant = Instant::now();
    let test_end: u128 = 2 << 28;
    for n in 0..=test_end as u128 {
        let _ = is_divisible(n);
    }
    let end: Instant = Instant::now();
    println!(
        "It took {} to use the new code.",
        end.duration_since(start).as_micros()
    );
    let start: Instant = Instant::now();
    for n in 0..=test_end as u128 {
        let _ = n.rem(5);
    }
    let end: Instant = Instant::now();
    println!(
        "It took {} to use the modulus operator.",
        end.duration_since(start).as_micros()
    );
}

fn is_divisible(n: u128) -> bool {
    return n.wrapping_mul(FIVE_DIVIDER) <= FIVE_DIVIDER - 1;
}
