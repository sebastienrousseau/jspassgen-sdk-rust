<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://raw.githubusercontent.com/sebastienrousseau/jspassgen/main/.github/assets/logo.svg" alt="jspassgen logo" width="128" />
</p>

<h1 align="center">jspassgen-sdk-rust</h1>

<p align="center">
  A native Rust port of the jspassgen password generator core on the operating system's CSPRNG, which so far provides the random source and not yet the generators.
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/jspassgen-sdk-rust/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/jspassgen-sdk-rust/ci.yml?branch=main&style=for-the-badge&logo=github" alt="Build" height="28" /></a>
  <a href="https://crates.io/crates/jspassgen"><img src="https://img.shields.io/crates/v/jspassgen?style=for-the-badge&color=fc8d62&logo=rust" alt="crates.io version" height="28" /></a>
  <a href="https://github.com/sebastienrousseau/jspassgen-sdk-rust/blob/main/DEVELOPMENT.md"><img src="https://img.shields.io/badge/docs-development-blue?style=for-the-badge&labelColor=555555&logo=readthedocs" alt="Docs" height="28" /></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/sebastienrousseau/jspassgen-sdk-rust"><img src="https://img.shields.io/ossf-scorecard/github.com/sebastienrousseau/jspassgen-sdk-rust?style=for-the-badge&label=OpenSSF%20Scorecard&logo=openssf" alt="OpenSSF Scorecard" height="28" /></a>
  <a href="LICENSE-APACHE"><img src="https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg?style=for-the-badge" alt="License: Apache-2.0 OR MIT" height="28" /></a>
  <a href="https://github.com/sebastienrousseau/jspassgen-sdk-rust/blob/main/docs/POLICIES.md"><img src="https://img.shields.io/badge/Rust-1.85%2B-93450a.svg?style=for-the-badge&logo=rust" alt="Rust 1.85 or newer" height="28" /></a>
</p>

---

## Contents

**Getting started**

- [Install](#install): from crates.io with cargo
- [Requirements](#requirements): toolchain floor, platforms
- [Quick Start](#quick-start): an unbiased draw from the OS CSPRNG

**The jspassgen ecosystem**

- [The jspassgen ecosystem](#the-jspassgen-ecosystem): the core library, its tools and native SDKs, released in lockstep

**Library reference**

- [Capabilities at a glance](#capabilities-at-a-glance): what exists and what does not yet
- [Ecosystem comparison](#ecosystem-comparison): not written yet
- [Benchmarks](#benchmarks): none published yet
- [Features](#features): module-level capability list
- [Configuration](#configuration): none
- [Examples](#examples): none yet

**Operational**

- [When not to use jspassgen-sdk-rust](#when-not-to-use-jspassgen-sdk-rust): limitations
- [Development](#development): make targets and CI
- [Security](#security): guarantees and their limits
- [Documentation](#documentation): all reference docs
- [Stability guarantees](#stability-guarantees): SemVer axis, output stability, toolchain floor
- [License](#license)

---

## Install

### As a Rust library

```bash
cargo add jspassgen@0.0.23
```

The package and the library are both named `jspassgen`. This version
provides the random source; the generators come in a later version.
Releases are cut from signed tags by the release workflow
([release model](DEVELOPMENT.md#release-model)).

Build and test from source:

```bash
git clone https://github.com/sebastienrousseau/jspassgen-sdk-rust.git
cd jspassgen-sdk-rust
cargo test
```

---

## Requirements

- Rust 1.85 or newer (`rust-version` in `Cargo.toml`). CI builds and
  tests on Rust 1.85.0 and stable on Linux, and on stable on macOS and
  Windows ([toolchain policy](docs/POLICIES.md)).
- An operating system random source that `getrandom` 0.4 supports:
  `getrandom(2)` on Linux, `getentropy` on macOS, `ProcessPrng` on
  Windows 10 and later, and the other platforms listed in
  [its documentation](https://docs.rs/getrandom).
- One runtime dependency, `getrandom`.

---

## Quick Start

```rust
use jspassgen::{OsRandom, RandomSource};

fn main() -> Result<(), jspassgen::Error> {
    let mut rng = OsRandom;

    // A uniform index into a 62-character alphabet.
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let index = rng.random_int(62)?;
    assert!(usize::try_from(index).is_ok_and(|i| i < alphabet.len()));

    // Raw bytes, straight from the operating system.
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes)?;

    // An empty range is an error, not a panic.
    assert_eq!(rng.random_int(0), Err(jspassgen::Error::InvalidMax));
    Ok(())
}
```

`random_int(max)` returns a value in `[0, max)` with every value equally
likely: draws that would bias the result are rejected and drawn again.
This block runs as a doctest in CI.

---

## The jspassgen ecosystem

jspassgen is one core library and the tools, SDKs and site around it.
Every member is listed in
[`docs/ecosystem.json`](https://github.com/sebastienrousseau/jspassgen/blob/main/docs/ecosystem.json)
and, from the suite's first lockstep release, every member ships the same
version ([ADR 0001](https://github.com/sebastienrousseau/jspassgen/blob/main/docs/adr/0001-ecosystem-lockstep.md)).
The JavaScript tools depend on the core and add no generation logic of
their own; each SDK is a native port that is checked against the core's
conformance kit as its generators land.

| Component | Purpose | Use case |
| :--- | :--- | :--- |
| [`jspassgen`](https://github.com/sebastienrousseau/jspassgen) | Password and passphrase core, Node.js library, CLI, web UI and workers; source of the conformance kit | Programs that generate secrets |
| [`jspassgen-cli`](https://github.com/sebastienrousseau/jspassgen-cli) | Command-line generator built on the core | Passwords in a terminal or a script |
| [`jspassgen-mcp`](https://github.com/sebastienrousseau/jspassgen-mcp) | Local stdio MCP server | AI agents that need a generated secret |
| [`jspassgen-lsp`](https://github.com/sebastienrousseau/jspassgen-lsp) | Language server for weak literal secrets | Editors checking configuration and `.env` files |
| [`jspassgen-sdk-python`](https://github.com/sebastienrousseau/jspassgen-sdk-python) | Native port on the `secrets` module | Python programs |
| **`jspassgen-sdk-rust`** | Native port on the OS CSPRNG | Rust programs |
| [`jspassgen-sdk-java`](https://github.com/sebastienrousseau/jspassgen-sdk-java) | Native port on `SecureRandom` | JVM programs |
| [`jspassgen-sdk-dotnet`](https://github.com/sebastienrousseau/jspassgen-sdk-dotnet) | Native port on `RandomNumberGenerator` | .NET programs |
| [`jspassgen-sdk-cpp`](https://github.com/sebastienrousseau/jspassgen-sdk-cpp) | Native C++17 port on the OS CSPRNG | C++ programs |
| [`jspassgen.github.io`](https://github.com/sebastienrousseau/jspassgen.github.io) | Source of [jspassgen.com](https://jspassgen.com) | Evaluating the suite |

Every member ships at 0.0.23, in lockstep since 0.0.22: the
core, CLI, MCP and LSP servers on npm, the Python, Rust, Java and .NET
SDKs on PyPI, crates.io, Maven Central and NuGet, and the C++ SDK and
the site as GitHub releases. The table gives each member's purpose, and
each README's Capabilities at a glance says what works today.

---

## Capabilities at a glance

| Area | Capability | Status |
| :--- | :--- | :--- |
| Random source | `RandomSource` trait: `fill_bytes`, `random_int(max)` | Available |
| OS CSPRNG | `OsRandom`, through `getrandom` | Available |
| Unbiased integers | `random_int` by rejection sampling; `Error::InvalidMax` for `max` 0 | Available |
| Password generators | `strong`, `base64`, `memorable`, `quantum-resistant`, `diceware`, `honeyword`, `pronounceable`, `custom` | Not available yet |
| Validation and entropy | Configuration checks, entropy calculation, error messages matching the core | Not available yet |
| EFF diceware wordlist | Vendored byte-identical from the core | Not available yet |
| Conformance kit | jspassgen's draw tapes replayed through `RandomSource` | Not run yet: the kit ships in jspassgen 0.0.21 and is not vendored here yet |

---

## Ecosystem comparison

Not written yet. A comparison with other Rust password crates needs the
generators and their benchmarks first; until then there is nothing to
compare.

---

## Benchmarks

None published yet, and the crate has no benchmark target. Criterion
benchmarks arrive with the generators.

---

## Features

- `RandomSource` (`src/random.rs`) is the port every generator will draw
  through. Implement `fill_bytes`; `random_int` comes with it.
- `OsRandom` (`src/random.rs`) is the operating system's CSPRNG through
  `getrandom::fill`, with no state, no buffer and no seed.
- `Error` (`src/error.rs`) has `InvalidMax` for an empty range and
  `Unavailable` when the operating system cannot supply bytes, with the
  `getrandom` error as its `source()`. It is `#[non_exhaustive]`.

The crate has no Cargo features.

---

## Configuration

None. The crate reads no environment variables and no files.

---

## Examples

None yet; the Quick Start above is the one runnable sample. An
`examples/` directory arrives with the generators.

---

## When not to use jspassgen-sdk-rust

- This crate does not generate passwords yet. Use
  [jspassgen](https://github.com/sebastienrousseau/jspassgen) in
  JavaScript, or another crate, until the port lands.
- `OsRandom` is not a general-purpose RNG. It asks the operating system
  on every call and offers only bytes and bounded `u32` draws;
  [`rand`](https://docs.rs/rand) covers distributions, seeding and speed.
- There is no seeded source, by design, so it cannot give reproducible
  output from a seed.

---

## Development

```bash
make check      # every gate CI runs, in order
make docs-lint  # markdownlint, codespell and reuse
```

`make check` runs rustfmt, clippy (pedantic, `-D warnings`), the build,
the tests, 100% line and region coverage (cargo-llvm-cov), rustdoc with
warnings denied, `cargo deny`, the version and package-contents checks
and the complexity ceilings (cyclomatic 10, cognitive 15, Halstead
difficulty 30, 60 lines per function, 500 per file). See
[`DEVELOPMENT.md`](DEVELOPMENT.md) for every gate, its CI job, and what
the suite standard asks for that is not here yet (fuzzing, benchmarks,
the manual, an SBOM), and the release model.

---

## Security

The crate has no `unsafe` code (`#![forbid(unsafe_code)]`). Randomness
comes only from the operating system's CSPRNG; there is no user-space
PRNG and no seed. `random_int` is free of modulo bias, and its tests
include a chi-square check that is shown to fail for a modulo sampler.
The crate makes no network call, has no telemetry and reads no files.
`cargo deny` checks every dependency, and CodeQL and OpenSSF Scorecard
run on `main`. Nothing is fuzzed yet: there is no parser to fuzz.

Report vulnerabilities according to [`SECURITY.md`](SECURITY.md).

---

## Documentation

- [Developer documentation](DEVELOPMENT.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Decision records](docs/adr/README.md)
- [Engineering policies](docs/POLICIES.md)
- [Changelog](CHANGELOG.md)
- [Contributing](CONTRIBUTING.md)
- [Support](SUPPORT.md)
- [Governance](GOVERNANCE.md)
- [Agent rules](AGENTS.md)

The API reference is built locally with `cargo doc --open`, and by
docs.rs for each crates.io release. There is no rendered manual yet; it
arrives with the generators.

---

## Stability guarantees

- The project is pre-1.0; versions advance in `0.0.1` steps, in lockstep
  with the jspassgen suite, and each step may break.
- The breaking axis is the public Rust API: the `RandomSource` trait,
  `OsRandom` and the `Error` variants (`Error` is `#[non_exhaustive]`).
- Once generators exist, their output is part of that axis: for the same
  draws, a change to the password produced is breaking even when no
  signature moves, and the conformance kit pins it.
- The Rust floor moves only as described in the
  [toolchain policy](docs/POLICIES.md).

---

## License

Licensed under either [Apache License 2.0](LICENSE-APACHE) or
[MIT](LICENSE-MIT), at your option.
