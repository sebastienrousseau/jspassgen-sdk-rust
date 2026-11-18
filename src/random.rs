// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Where every random draw comes from.

use crate::Error;

/// A source of cryptographically secure random bytes, and the unbiased
/// integer draw built on it.
///
/// This is the Rust form of jspassgen's `RandomGeneratorPort`:
/// [`fill_bytes`](Self::fill_bytes) stands for `generateRandomBytes`
/// and [`random_int`](Self::random_int) for `generateRandomInt`. Every
/// generator in the crate will draw through this trait, so a test can
/// replace the operating system with a recorded sequence of draws.
///
/// An implementation used outside tests must be a CSPRNG; [`OsRandom`]
/// is the one this crate provides.
pub trait RandomSource {
    /// Fills `dest` with random bytes.
    ///
    /// # Errors
    ///
    /// Returns an error when the source cannot supply the bytes; `dest`
    /// is then left in an unspecified state and must not be used.
    fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error>;

    /// Returns an integer drawn uniformly from `[0, max)`.
    ///
    /// The default implementation reads four bytes at a time from
    /// [`fill_bytes`](Self::fill_bytes) as a little-endian `u32` and
    /// rejects the `2^32 mod max` smallest values, so the values it
    /// keeps are a whole number of copies of `[0, max)` and reducing
    /// them modulo `max` adds no bias. Fewer than one draw in two is
    /// rejected for any `max`.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidMax`] when `max` is 0, and any error from
    /// [`fill_bytes`](Self::fill_bytes).
    fn random_int(&mut self, max: u32) -> Result<u32, Error> {
        uniform_below(max, &mut |word| self.fill_bytes(word))
    }
}

/// The rejection sampler behind [`RandomSource::random_int`], kept out
/// of the trait so there is one copy of it, not one per source.
fn uniform_below(
    max: u32,
    fill: &mut dyn FnMut(&mut [u8]) -> Result<(), Error>,
) -> Result<u32, Error> {
    if max == 0 {
        return Err(Error::InvalidMax);
    }
    // 2^32 mod max, computed without leaving u32.
    let threshold = max.wrapping_neg() % max;
    loop {
        let mut word = [0u8; 4];
        fill(&mut word)?;
        let value = u32::from_le_bytes(word);
        if value >= threshold {
            return Ok(value % max);
        }
    }
}

/// The operating system's CSPRNG, through the
/// [`getrandom`](https://docs.rs/getrandom) crate: `getrandom(2)` on
/// Linux, `getentropy` on macOS, `ProcessPrng` on Windows and the
/// equivalent elsewhere.
///
/// It holds no state and no buffer; every call asks the operating
/// system.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OsRandom;

impl RandomSource for OsRandom {
    fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        getrandom::fill(dest).map_err(Error::Unavailable)
    }
}

#[cfg(test)]
mod tests;
