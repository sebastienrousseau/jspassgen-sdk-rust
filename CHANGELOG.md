<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Changelog

All notable changes to this crate are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions move in
`0.0.1` steps, in lockstep with the jspassgen suite.

## [0.0.22] - Unreleased

The repository scaffold and the random source the port will build on.
Nothing is published yet.

### Added

- `RandomSource`, the Rust form of jspassgen's `RandomGeneratorPort`:
  `fill_bytes` and `random_int(max)`, an unbiased draw from `[0, max)` by
  rejection sampling that returns `Error::InvalidMax` for `max` 0.
- `OsRandom`, the operating system's CSPRNG through `getrandom` 0.4.
- `Error` (`InvalidMax`, `Unavailable`).
- Unit tests with scripted draws, chi-square uniformity tests against the
  OS source, and a check that those tests catch a modulo sampler.
- The repository standard: `make check`, CI (format, clippy, tests on
  Linux, macOS and Windows and on Rust 1.85, coverage, cargo-deny,
  complexity, release checks), CodeQL, DCO, docs lint, OpenSSF
  Scorecard, Dependabot, and the community and policy documents.
