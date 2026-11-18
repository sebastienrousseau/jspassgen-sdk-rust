// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Statistical checks of `OsRandom::random_int`.
//!
//! Each check is a chi-square goodness-of-fit test with a critical value
//! a correct sampler exceeds with probability below 1e-6, so a failure
//! means a biased sampler, not bad luck. The last test runs the same
//! check on a naive modulo sampler and requires it to fail, which shows
//! the check can see the bias rejection sampling removes.

use jspassgen::{OsRandom, RandomSource};

/// Chi-square critical value for 9 degrees of freedom, upper tail 1.1e-7.
const CRITICAL_DF9: f64 = 50.0;
/// Chi-square critical value for 2 degrees of freedom, upper tail 2.1e-9.
const CRITICAL_DF2: f64 = 40.0;

/// `max = 3 * 2^30`: 2^32 mod max is 2^30, so a modulo sampler gives the
/// lowest third of the range twice the weight of the other two.
const SKEWED_MAX: u32 = 3 << 30;

fn chi_square(counts: &[u64]) -> f64 {
    let total: u64 = counts.iter().sum();
    #[allow(
        clippy::cast_precision_loss,
        reason = "counts stay far below 2^52, exact in f64"
    )]
    let expected = total as f64 / counts.len() as f64;
    counts
        .iter()
        .map(|&observed| {
            #[allow(
                clippy::cast_precision_loss,
                reason = "counts stay far below 2^52, exact in f64"
            )]
            let delta = observed as f64 - expected;
            delta * delta / expected
        })
        .sum()
}

/// Counts which third of `[0, SKEWED_MAX)` each of `n` draws lands in.
fn thirds(n: usize, mut draw: impl FnMut() -> u32) -> [u64; 3] {
    let mut counts = [0u64; 3];
    for _ in 0..n {
        let value = draw();
        assert!(value < SKEWED_MAX);
        counts[(value >> 30) as usize] += 1;
    }
    counts
}

#[test]
fn small_range_is_uniform() {
    let mut rng = OsRandom;
    let mut counts = [0u64; 10];
    for _ in 0..200_000 {
        counts[rng.random_int(10).expect("the OS source answers") as usize] += 1;
    }
    let statistic = chi_square(&counts);
    assert!(
        statistic < CRITICAL_DF9,
        "chi-square {statistic} for {counts:?}"
    );
}

#[test]
fn skewed_range_is_uniform() {
    let mut rng = OsRandom;
    let counts = thirds(30_000, || {
        rng.random_int(SKEWED_MAX).expect("the OS source answers")
    });
    let statistic = chi_square(&counts);
    assert!(
        statistic < CRITICAL_DF2,
        "chi-square {statistic} for {counts:?}"
    );
}

#[test]
fn the_check_detects_modulo_bias() {
    let mut rng = OsRandom;
    let counts = thirds(30_000, || {
        let mut word = [0u8; 4];
        rng.fill_bytes(&mut word).expect("the OS source answers");
        u32::from_le_bytes(word) % SKEWED_MAX
    });
    // Expected proportions 1/2, 1/4, 1/4: the statistic is near n / 8.
    let statistic = chi_square(&counts);
    assert!(
        statistic > CRITICAL_DF2,
        "modulo bias went unseen: chi-square {statistic} for {counts:?}"
    );
}
