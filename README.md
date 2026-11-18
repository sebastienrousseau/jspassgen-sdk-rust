<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://raw.githubusercontent.com/sebastienrousseau/jspassgen/master/.github/assets/logo.svg" alt="jspassgen logo" width="128" />
</p>

<h1 align="center">jspassgen for Rust</h1>

<p align="center">
  The Rust member of the jspassgen suite: a native port of the jspassgen password generator core, on the operating system's CSPRNG. Work in progress: only the random source exists so far.
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/jspassgen-sdk-rust/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/jspassgen-sdk-rust/ci.yml?style=for-the-badge&logo=github" alt="Build" height="28" /></a>
  <a href="https://crates.io/crates/jspassgen"><img src="https://img.shields.io/crates/v/jspassgen.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Registry" height="28" /></a>
  <a href="https://docs.rs/jspassgen"><img src="https://img.shields.io/docsrs/jspassgen?style=for-the-badge&labelColor=555555&logo=docsdotrs" alt="Docs" height="28" /></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/sebastienrousseau/jspassgen-sdk-rust"><img src="https://img.shields.io/ossf-scorecard/github.com/sebastienrousseau/jspassgen-sdk-rust?style=for-the-badge&label=OpenSSF%20Scorecard&logo=openssf" alt="OpenSSF Scorecard" height="28" /></a>
  <a href="LICENSE-APACHE"><img src="https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg?style=for-the-badge" alt="License: Apache-2.0 OR MIT" height="28" /></a>
  <a href="https://github.com/sebastienrousseau/jspassgen-sdk-rust/blob/main/docs/POLICIES.md"><img src="https://img.shields.io/badge/Rust-1.85%2B-93450a.svg?style=for-the-badge&logo=rust" alt="Rust 1.85 or newer" height="28" /></a>
</p>

<p align="center">
  No demo yet: the crate has no generator to show. The demo is recorded once it does.
</p>

---

## Contents

**Getting started**

- [Install](#install): from the git repository (nothing is on crates.io yet)
- [Requirements](#requirements): toolchain floor, platforms
- [Quick Start](#quick-start): an unbiased draw from the OS CSPRNG

**The jspassgen ecosystem**

- [The jspassgen ecosystem](#the-jspassgen-ecosystem): the core, the tools, the SDKs and the site

**Library reference**

- [Capabilities at a glance](#capabilities-at-a-glance): what exists and what does not yet
- [Ecosystem comparison](#ecosystem-comparison): not written yet
- [Benchmarks](#benchmarks): none yet
- [Features](#features): module-level capability list
- [Configuration](#configuration): none
- [Examples](#examples): none yet

**Operational**

- [When not to use jspassgen](#when-not-to-use-jspassgen): limitations
- [Development](#development): make targets and CI
- [Security](#security): guarantees and their limits
- [Documentation](#documentation): all reference docs
- [Stability guarantees](#stability-guarantees): SemVer axis, output stability, toolchain floor
- [License](#license)

---

## Install

### As a Rust library

```toml
[dependencies]
jspassgen = { git = "https://github.com/sebastienrousseau/jspassgen-sdk-rust" }
```

The crate is not published to crates.io; the name `jspassgen` is
planned for it. Version 0.0.22, the planned first suite version, is in
development on the `feat/v0.0.22` branch and has not been released.

---

## Requirements

- Rust 1.85 or newer (`rust-version` in `Cargo.toml`), built and tested
  in CI on 1.85.0 and stable on Linux, and on stable on macOS and
  Windows ([toolchain policy](docs/POLICIES.md)).
- An operating system random source that `getrandom` 0.4 supports:
  `getrandom(2)` on Linux, `getentropy` on macOS, `ProcessPrng` on
  Windows 10 and later, and the platforms listed in
  [its documentation](https://docs.rs/getrandom).

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

jspassgen is one core and a suite of repositories that release at one
version, listed in jspassgen's
[`docs/ecosystem.json`](https://github.com/sebastienrousseau/jspassgen/blob/master/docs/ecosystem.json).
The SDKs are native ports, not network clients: a password is made on
the caller's machine and never leaves it. Every member but the core is
planned; none has been released yet.

| Component | Purpose | Use case |
| :--- | :--- | :--- |
| [`jspassgen`][core] | Zero-dependency JavaScript core, Node.js library, web UI and workers; source of the conformance kit | Generate passwords in JavaScript |
| [`jspassgen-cli`][cli] | Command-line generator on the core | Generate passwords in a shell |
| [`jspassgen-mcp`][mcp] | Local stdio MCP server | Generate passwords from an AI agent |
| [`jspassgen-lsp`][lsp] | Language server for weak literal secrets | Find weak secrets in configuration files |
| [`jspassgen-sdk-python`][python] | Native Python port | Generate passwords in Python |
| **`jspassgen-sdk-rust`** | Native Rust port (this crate) | Generate passwords in Rust |
| [`jspassgen-sdk-java`][java] | Native Java port | Generate passwords on the JVM |
| [`jspassgen-sdk-dotnet`][dotnet] | Native .NET port | Generate passwords in .NET |
| [`jspassgen-sdk-cpp`][cpp] | Native C++17 port | Generate passwords in C++ |
| [`jspassgen.github.io`][site] | jspassgen.com, product and SDK pages | Read about the suite |

---

## Capabilities at a glance

| Area | Capability | Status |
| :--- | :--- | :--- |
| Random source | `RandomSource` trait: `fill_bytes`, `random_int(max)` | Available |
| OS CSPRNG | `OsRandom`, through `getrandom` | Available |
| Unbiased integers | `random_int` by rejection sampling; `Error::InvalidMax` for `max` 0 | Available |
| Password generators | `strong`, `base64`, `memorable`, `quantum-resistant`, `diceware`, `honeyword`, `pronounceable`, `custom` | Not available yet: waits for the conformance kit |
| Validation and entropy | configuration checks, entropy calculation, error codes matching core | Not available yet |
| EFF diceware wordlist | vendored byte-identical from core | Not available yet |
| Conformance kit | draw tapes replayed through `RandomSource` | Not available yet: the kit does not exist |

---

## Ecosystem comparison

Not written yet. A comparison with other Rust password crates needs the
generators and their benchmarks first; until then there is nothing to
compare.

---

## Benchmarks

None yet. Criterion benchmarks arrive with the generators.

---

## Features

- **`RandomSource`** (`src/random.rs`): the port every generator will
  draw through. Implement `fill_bytes`; `random_int` comes with it.
- **`OsRandom`** (`src/random.rs`): the operating system's CSPRNG through
  `getrandom::fill`; no state, no buffer, no seed.
- **`Error`** (`src/error.rs`): `InvalidMax` for an empty range,
  `Unavailable` when the operating system cannot supply bytes, with the
  `getrandom` error as its `source()`.

The crate has no Cargo features.

---

## Configuration

None. The crate reads no environment variables and no files.

---

## Examples

None yet; the Quick Start above is the one runnable sample. Examples
arrive with the generators.

---

## When not to use jspassgen

- **You need a password generator today.** This crate does not generate
  passwords yet; use [jspassgen](https://github.com/sebastienrousseau/jspassgen)
  in JavaScript, or another crate, until the port lands.
- **You need a general-purpose RNG.** `OsRandom` asks the operating
  system on every call and offers only bytes and bounded `u32` draws;
  [`rand`](https://docs.rs/rand) covers distributions, seeding and speed.
- **You need reproducible output from a seed.** There is no seeded
  source by design.

---

## Development

```bash
make check
```

`make check` runs rustfmt, clippy (pedantic, `-D warnings`), the build,
the tests, 100% line and region coverage (cargo-llvm-cov), rustdoc with
warnings denied, `cargo deny`, the version and package-contents checks
and the complexity ceilings (cyclomatic 10, cognitive 15, Halstead
difficulty 30, 60 lines per function, 500 per file). `make docs-lint`
runs markdownlint, codespell and reuse. See
[`DEVELOPMENT.md`](DEVELOPMENT.md) for every gate, its CI job, and what
the suite standard asks for that is not here yet (fuzzing, benchmarks,
the manual, the release workflow).

---

## Security

The crate has no `unsafe` code (`#![forbid(unsafe_code)]`). Randomness
comes only from the operating system's CSPRNG; there is no user-space
PRNG and no seed. `random_int` is free of modulo bias, and its tests
include a statistical check that is shown to fail for a modulo sampler.
The crate makes no network call, has no telemetry and reads no files.
`cargo deny` checks every dependency, and CodeQL and OpenSSF Scorecard
run on `main`. Nothing is fuzzed yet: there is no parser to fuzz.

Report vulnerabilities according to [`SECURITY.md`](SECURITY.md).

---

## Documentation

API reference: run `cargo doc --open` (docs.rs follows the first
release) ·
[Developer docs](DEVELOPMENT.md) ·
[Ecosystem map](#the-jspassgen-ecosystem)

Also: [Architecture](docs/ARCHITECTURE.md) ·
[Decision records](docs/adr/README.md) ·
[Changelog](CHANGELOG.md) · [Policies](docs/POLICIES.md) ·
[Support](SUPPORT.md) · [Governance](GOVERNANCE.md) ·
[Agent rules](AGENTS.md)

No rendered manual yet; it arrives with the generators.

---

## Stability guarantees

The project is pre-1.0 and versions advance in `0.0.1` steps, in lockstep
with the jspassgen suite. The breaking axis is the public Rust API: the
`RandomSource` trait, `OsRandom` and the `Error` variants (`Error` is
`#[non_exhaustive]`). Once generators exist, their output is part of it:
for the same draws, a change to the password produced is breaking even
when no signature moves, and the conformance kit pins it. The Rust floor
moves only as described in the [toolchain policy](docs/POLICIES.md).

---

## License

This project is dual-licensed under either:

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([`LICENSE-MIT`](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option. Dependencies keep their own licences.

[core]: https://github.com/sebastienrousseau/jspassgen
[cli]: https://github.com/sebastienrousseau/jspassgen-cli
[mcp]: https://github.com/sebastienrousseau/jspassgen-mcp
[lsp]: https://github.com/sebastienrousseau/jspassgen-lsp
[python]: https://github.com/sebastienrousseau/jspassgen-sdk-python
[java]: https://github.com/sebastienrousseau/jspassgen-sdk-java
[dotnet]: https://github.com/sebastienrousseau/jspassgen-sdk-dotnet
[cpp]: https://github.com/sebastienrousseau/jspassgen-sdk-cpp
[site]: https://github.com/sebastienrousseau/jspassgen.github.io
