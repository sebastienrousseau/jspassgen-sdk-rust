<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# AGENTS.md

Invariants for AI-assisted contributors. A human contributor reads
[CONTRIBUTING.md](CONTRIBUTING.md) and [DEVELOPMENT.md](DEVELOPMENT.md);
an agent reads those too, plus this list of things that must never
drift. The workspace-wide rules in `~/Code/AGENTS.md` apply as well;
where this file is more specific, it wins.

## What this repository is

`jspassgen-sdk-rust` is the Rust member of the jspassgen suite: a native
port of the core of [jspassgen](https://github.com/sebastienrousseau/jspassgen)
(`packages/core`), published to crates.io as `jspassgen`. It generates
passwords in the caller's process from the operating system's CSPRNG.
It is not a network client. The suite's design is jspassgen's
[ADR 0001](https://github.com/sebastienrousseau/jspassgen/blob/master/docs/adr/0001-ecosystem-lockstep.md).

Today it holds only the random source (`RandomSource`, `OsRandom`). The
generators are ported against jspassgen's conformance kit, which
jspassgen has shipped since 0.0.21.

## Versioning

- The version is `0.0.x` and moves strictly one step at a time
  (`+0.0.1`), in lockstep with the jspassgen suite. Never propose a
  `0.1.0` or `1.0` jump.
- `Cargo.toml` is the single source of truth. `CITATION.cff`, the
  `CHANGELOG.md` heading, `SECURITY.md`, `SUPPORT.md`, the bug report
  form and `Cargo.lock` restate it; `scripts/verify-release-versions`
  checks them.
- Work for the next iteration happens on `feat/v<next-version>`; one
  release pull request into `main` at a time.

## The port contract

- Randomness comes only from a `RandomSource`; outside tests that is
  `OsRandom` (the `getrandom` crate). Never `rand::thread_rng`, a seeded
  PRNG, the clock or anything else in library code.
- `random_int(max)` is an unbiased draw by rejection sampling, never a
  modulo of raw bytes. Its tests include a deterministic rejection test
  and a statistical one that must fail for a modulo sampler.
- No network call, no telemetry, no file access in library code.
- Generators, when ported, make the same draws in the same order as
  `packages/core` and must pass the conformance kit byte for byte.
- The kit, once it exists, is vendored into `conformance/` unchanged,
  with the jspassgen commit it came from in `ci/core.ref`. Never edit it
  by hand.

## Secrets

- Never log, print, `Display` or `Debug` a generated password or the
  bytes it came from. Errors carry no generated values.
- Never commit a real password or key; tests use synthetic values.

## Gates

- `make check` passes: format, clippy (pedantic, `-D warnings`), build,
  tests, 100% line and region coverage of `src/`, rustdoc with warnings
  denied, `cargo deny`, versions, package contents and complexity
  (cyclomatic 10, cognitive 15, Halstead 30, 60 lines per function, 500
  per file). `make docs-lint` passes for Markdown, spelling and REUSE.
- `#![forbid(unsafe_code)]` stays.
- No blanket lint suppressions; a specific `#[allow]` carries a
  `reason`.
- Every behaviour change lands with its test in the same commit.

## Commits

- Conventional commits, subject at most 50 characters, body wrapped at
  72; `git commit -s` (DCO) and signed. Never rewrite published history,
  never merge into `main`, tag or publish without the maintainer.

## What to leave alone

- `LICENSE-APACHE`, `LICENSE-MIT`, `LICENSES/`, `DCO.txt` and `KEYS.asc`
  unless the task is about them.
- `Cargo.lock` changes only with a dependency change, and the commit
  says what moved and why.
- `scripts/complexity-baseline.json` is regenerated only to record an
  improvement.
