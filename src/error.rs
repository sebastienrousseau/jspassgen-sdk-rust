// SPDX-License-Identifier: Apache-2.0 OR MIT
//! The crate's one error type.

use core::fmt;

/// Why a random draw failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// [`RandomSource::random_int`](crate::RandomSource::random_int) was
    /// called with `max` 0: the range `[0, max)` must hold at least one
    /// value.
    InvalidMax,
    /// The operating system's random source could not supply bytes.
    Unavailable(getrandom::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMax => f.write_str("max must be at least 1"),
            Self::Unavailable(_) => f.write_str("the operating system random source failed"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidMax => None,
            Self::Unavailable(cause) => Some(cause),
        }
    }
}

#[cfg(test)]
mod tests;
