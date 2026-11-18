// SPDX-License-Identifier: Apache-2.0 OR MIT
//! # jspassgen
//!
//! The beginning of a native Rust port of the core of
//! [jspassgen](https://github.com/sebastienrousseau/jspassgen), the
//! password and passphrase generator. The port runs in your process on
//! the operating system's CSPRNG: it makes no network call and has no
//! telemetry.
//!
//! Only the random source exists so far: the [`RandomSource`] trait, the
//! Rust form of jspassgen's `RandomGeneratorPort`, and [`OsRandom`], its
//! implementation on the operating system's random source. The password
//! generators follow, checked against jspassgen's conformance kit
//! (shipped in jspassgen 0.0.21), which every port must pass draw for
//! draw.
//!
//! ```
//! use jspassgen::{OsRandom, RandomSource};
//!
//! let mut rng = OsRandom;
//! let roll = rng.random_int(6)?; // 0 to 5, every value equally likely
//! assert!(roll < 6);
//!
//! let mut salt = [0u8; 16];
//! rng.fill_bytes(&mut salt)?;
//! # Ok::<(), jspassgen::Error>(())
//! ```

#![forbid(unsafe_code)]

mod error;
mod random;

/// Runs the Rust blocks of `README.md` as doctests.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

pub use error::Error;
pub use random::{OsRandom, RandomSource};
