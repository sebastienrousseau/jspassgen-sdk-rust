// SPDX-License-Identifier: Apache-2.0 OR MIT
//! The contract of `RandomSource::random_int`, checked on arbitrary
//! input against a reference written from the definition.
//!
//! The input is `max` (four bytes, little-endian) followed by the byte
//! stream the source replays; a source that runs dry fails the read.
//! `random_int(max)` must then return `InvalidMax` for 0 and read
//! nothing; otherwise skip every word below `2^32 mod max`, return the
//! first word at or above it reduced modulo `max` (so the result is in
//! `[0, max)` and unbiased), read exactly the words up to that one, and
//! pass on the error of a source that runs dry before it.
//!
//! The cargo-fuzz target in `fuzz/` and `tests/random_int_contract.rs`
//! both run [`check`].

use jspassgen::{Error, RandomSource};

/// The error a dry [`Script`] returns.
const DRY: Error = Error::Unavailable(getrandom::Error::UNSUPPORTED);

/// Replays a fixed byte stream, then fails, and counts the bytes read.
struct Script<'a> {
    stream: &'a [u8],
    read: usize,
}

impl RandomSource for Script<'_> {
    fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        let end = self.read + dest.len();
        let bytes = self.stream.get(self.read..end).ok_or(DRY)?;
        dest.copy_from_slice(bytes);
        self.read = end;
        Ok(())
    }
}

/// What `random_int(max)` must return on `stream`, and how many bytes
/// it must read, computed in `u64` from the definition.
fn reference(max: u32, stream: &[u8]) -> (Result<u32, Error>, usize) {
    if max == 0 {
        return (Err(Error::InvalidMax), 0);
    }
    let modulus = u64::from(max);
    let threshold = (1u64 << 32) % modulus;
    for (index, word) in stream.chunks_exact(4).enumerate() {
        let value = u64::from(u32::from_le_bytes([word[0], word[1], word[2], word[3]]));
        if value >= threshold {
            let draw = u32::try_from(value % modulus).expect("below max, a u32");
            return (Ok(draw), 4 * (index + 1));
        }
    }
    (Err(DRY), stream.len() / 4 * 4)
}

/// Runs `random_int` on one input and asserts the contract. An input
/// shorter than four bytes carries no `max` and is skipped.
pub fn check(data: &[u8]) {
    let Some((head, stream)) = data.split_first_chunk::<4>() else {
        return;
    };
    let max = u32::from_le_bytes(*head);
    let mut source = Script { stream, read: 0 };
    let drawn = source.random_int(max);
    if let Ok(value) = drawn {
        assert!(value < max, "random_int({max}) returned {value}");
    }
    let (expected, read) = reference(max, stream);
    assert_eq!(drawn, expected, "random_int({max}) on {stream:02x?}");
    assert_eq!(source.read, read, "bytes read by random_int({max})");
}
