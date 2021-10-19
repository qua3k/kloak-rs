use rand::distributions::{Distribution, Uniform};
use rand::prelude::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Generate a value with a ChaCha8 CSPRNG within a uniform distribution between 0 and a max inclusive
/// Seeded by getrandom(2), panics if getrandom(2) is unable to provide entropy
fn random_at_most(max: u64) -> u64 {
    let mut rng = ChaCha8Rng::from_entropy();
    let between = Uniform::new_inclusive(0, max);
    rng.sample(between)
}

/// Generate a random integer between a minimum and a maximum
/// If the minimum is larger than the maximum, return max
pub fn random_between(min: u64, max: u64) -> u64 {
    // Default to max if the interval is not valid
    if min >= max {
        return max
    }
    min + random_at_most(max - min)
}
