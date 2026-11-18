// SPDX-License-Identifier: Apache-2.0 OR MIT
use super::{OsRandom, RandomSource};
use crate::Error;

/// Replays fixed 32-bit words, little-endian, and counts the reads.
struct Words {
    words: Vec<u32>,
    reads: usize,
}

impl Words {
    fn new(words: &[u32]) -> Self {
        Self {
            words: words.to_vec(),
            reads: 0,
        }
    }
}

impl RandomSource for Words {
    fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        assert_eq!(dest.len(), 4, "random_int reads one u32 at a time");
        let word = self.words[self.reads];
        self.reads += 1;
        dest.copy_from_slice(&word.to_le_bytes());
        Ok(())
    }
}

/// Fails every read, as an operating system without a source would.
struct Broken;

impl RandomSource for Broken {
    fn fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), Error> {
        Err(Error::Unavailable(getrandom::Error::UNSUPPORTED))
    }
}

#[test]
fn max_zero_is_an_error_and_reads_nothing() {
    let mut source = Words::new(&[]);
    assert_eq!(source.random_int(0), Err(Error::InvalidMax));
    assert_eq!(source.reads, 0);
    assert_eq!(OsRandom.random_int(0), Err(Error::InvalidMax));
}

#[test]
fn a_failing_source_fails_the_draw() {
    assert_eq!(
        Broken.random_int(10),
        Err(Error::Unavailable(getrandom::Error::UNSUPPORTED))
    );
}

#[test]
fn max_one_always_draws_zero_without_rejection() {
    let mut source = Words::new(&[0, 7, u32::MAX]);
    for _ in 0..3 {
        assert_eq!(source.random_int(1), Ok(0));
    }
    assert_eq!(source.reads, 3);
}

#[test]
fn powers_of_two_never_reject() {
    // 2^32 mod 2^k is 0: every word is kept.
    let mut source = Words::new(&[0, 255, 256, u32::MAX]);
    let draws: Vec<_> = (0..4).map(|_| source.random_int(256)).collect();
    assert_eq!(draws, [Ok(0), Ok(255), Ok(0), Ok(255)]);
    assert_eq!(source.reads, 4);
}

#[test]
fn values_below_the_threshold_are_rejected_not_reduced() {
    // 2^32 mod 3 is 1, so 0 is rejected and the next word is used.
    let mut source = Words::new(&[0, 5]);
    assert_eq!(source.random_int(3), Ok(2));
    assert_eq!(source.reads, 2);
}

#[test]
fn the_threshold_itself_is_kept() {
    // 2^32 mod (2^31 + 1) is 2^31 - 1.
    let max = (1u32 << 31) + 1;
    let threshold = (1u32 << 31) - 1;
    let mut source = Words::new(&[threshold - 1, 0, threshold]);
    assert_eq!(source.random_int(max), Ok(threshold));
    assert_eq!(source.reads, 3);
}

#[test]
fn max_u32_rejects_only_zero() {
    // 2^32 mod (2^32 - 1) is 1.
    let mut source = Words::new(&[0, u32::MAX, u32::MAX - 1]);
    assert_eq!(source.random_int(u32::MAX), Ok(0));
    assert_eq!(source.random_int(u32::MAX), Ok(u32::MAX - 1));
    assert_eq!(source.reads, 3);
}

#[test]
fn os_random_fills_every_length() {
    let mut rng = OsRandom;
    rng.fill_bytes(&mut []).expect("an empty fill succeeds");
    let mut buf = [0u8; 64];
    rng.fill_bytes(&mut buf).expect("the OS source answers");
    // 64 zero bytes from a working CSPRNG has probability 2^-512.
    assert!(buf.iter().any(|&b| b != 0));
}

#[test]
fn os_random_stays_in_range() {
    let mut rng = OsRandom;
    for max in [1, 2, 3, 10, 26, 62, 7776, u32::MAX] {
        for _ in 0..200 {
            assert!(rng.random_int(max).expect("the OS source answers") < max);
        }
    }
}
