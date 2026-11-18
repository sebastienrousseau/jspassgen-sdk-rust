<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# ADR 0001: The OS CSPRNG through `getrandom`, unbiased by rejection

## Status

Accepted (0.0.22).

## Context

jspassgen's ADR 0001 requires every port to draw only from the platform
CSPRNG, behind an injectable port matching `RandomGeneratorPort`, and to
map `generateRandomInt(max)` to an unbiased draw. It names
`getrandom` or `rand::rngs::OsRng` for Rust.

## Decision

- `RandomSource` is the port: `fill_bytes` and `random_int(max: u32)`,
  with `&mut self` so a test source can replay recorded draws.
- `OsRandom` calls `getrandom::fill` directly. `rand` is not used: it
  would add a dependency tree for one call, and its `OsRng` wraps
  `getrandom` anyway.
- `random_int` takes a `u32`. Core's largest draw is the EFF wordlist
  (7776 entries); a `u32` covers every bound core uses.
- The draw is a little-endian `u32`, rejected when below `2^32 mod max`,
  then reduced modulo `max`. At most half the draws are rejected, for
  any `max`.
- `max` 0 is `Error::InvalidMax`, the counterpart of core's
  `RangeError`; the error codes the conformance kit defines will be
  mapped when it exists.

## Consequences

- One small runtime dependency, maintained by the rust-random project,
  with an MSRV (1.85) equal to this crate's.
- The byte-level draw is this crate's own; the conformance kit pins the
  integer draws, which do not depend on it.
- `getrandom::Error` is part of the public API through
  `Error::Unavailable`; a major `getrandom` upgrade is a breaking change
  here.
