<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Changelog

All notable changes to this crate are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions move in
`0.0.1` steps, in lockstep with the jspassgen suite.

## [0.0.22] - 2026-11-18

The repository scaffold and the random source the port will build on.

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
- A release workflow for `v<version>` tags: `scripts/verify-release-tag`
  refuses a tag that is not annotated and signed, not on the released
  commit, or whose message or version disagrees with `Cargo.toml`; then
  `make check`, `cargo package` and `SHA256SUMS`, a crates.io publish
  through trusted publishing behind the `crates-io` environment
  (skipped for a version already published), and a draft GitHub release
  with the `.crate` and its checksum.

### Changed

- `Cargo.toml` allows publishing to crates.io (`publish = ["crates-io"]`
  instead of `publish = false`), so the release workflow can publish.
