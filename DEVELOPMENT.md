<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Development

## Toolchain

- Rust from `mise.toml` (1.98.1 locally); the floor is `rust-version`
  1.85 and CI builds and tests on 1.85.0 and stable.
  `.cargo/config.toml` makes the resolver pick dependency versions 1.85
  can build.
- Gate tools: cargo-llvm-cov 0.9.1 (`llvm-tools-preview`), cargo-deny
  0.20.2, rust-code-analysis-cli 0.0.25, Python 3 for the complexity
  script, and for the docs markdownlint-cli2 (through `npx`), codespell
  and reuse.

## Gates

| Target | What it checks | CI job |
| :--- | :--- | :--- |
| `make fmt-check` | rustfmt | Format and lint |
| `make lint` | clippy pedantic, `-D warnings`, cognitive 15 and 60 lines | Format and lint |
| `make doc` | rustdoc with warnings denied | Format and lint |
| `make build`, `make test` | every target; unit, statistical, README and doc tests | Build and tests (Linux stable and 1.85.0, macOS, Windows) |
| `make coverage` | 100% lines and regions of `src/` | Coverage |
| `make deny` | RustSec advisories, licences, bans, sources | Dependency policy |
| `make complexity` | cyclomatic 10, cognitive 15, Halstead difficulty 30, 60 lines per function, 500 lines per file, every tracked `.rs` | Complexity ceilings |
| `make versions`, `make package-check` | version restatements, package allow-list | Versions and package contents |
| `make docs-lint` | markdownlint, codespell, reuse | Docs lint |

`make check` runs every gate but `docs-lint`, in order.

## Tests

- `src/random/tests.rs` drives `random_int` with scripted 32-bit words,
  so the rejection path is tested exactly: which words are rejected,
  which are kept, and how many are read.
- `tests/uniformity.rs` runs chi-square tests on `OsRandom` with a
  critical value a correct sampler exceeds with probability below 1e-6,
  and one test that requires a modulo sampler to fail the same check.
- The Rust blocks in `README.md` run as doctests.

## Not here yet

The suite standard (jspassgen ADR 0001, section 5) also asks for these;
each arrives when it has something to work on:

| Item | Arrives with |
| :--- | :--- |
| `conformance/` and `ci/core.ref` | jspassgen's conformance kit |
| `examples/`, `benches/` (criterion) | the generators |
| `fuzz/` (cargo-fuzz) | the first input parser (configuration, templates) |
| `.github/demo.tape` and `.github/demo.gif` | an example with output worth showing |
| Rendered manual and its deploy workflow | the generators' user documentation |
| cargo-semver-checks | a published version to compare against (after 0.0.22) |
| SBOM | not scheduled yet |

## Release model

`.github/workflows/release.yml` releases a pushed `v<version>` tag.
Versions advance by `0.0.1`, in lockstep with the jspassgen suite; the
first release is the suite's first lockstep version, 0.0.22. Nothing has
been published yet.

1. On `feat/v<version>`: move the version in `Cargo.toml` and its
   restatements (`make versions`), and date the `CHANGELOG.md` heading.
   Merge the release pull request into `main` once CI is green.
2. Tag `main` with a signed annotated tag, check it with
   `git verify-tag v<version>` (`KEYS.asc`) and
   `scripts/verify-release-tag v<version>`, and push it:
   `git tag -s v<version> -m "jspassgen-sdk-rust v<version>"`.
3. The `check` job runs `scripts/verify-release-tag`, which refuses a
   tag that is not annotated, not signed, not on the checked-out commit,
   or whose message or version does not match `Cargo.toml`. It runs
   `make check` (`cargo deny` covers the RustSec advisories), builds the
   `.crate` with `cargo package` and writes `SHA256SUMS`. CI cannot
   verify the signature itself; step 2 does.
4. The `publish-crate` job waits for a maintainer to approve the
   `crates-io` environment, gets a short-lived token through crates.io
   trusted publishing (`rust-lang/crates-io-auth-action`), and runs
   `cargo publish`; no crates.io token is stored. A version already on
   crates.io is skipped. The job then checks that crates.io serves the
   `.crate` whose checksum is in `SHA256SUMS`.
5. The `github-release` job drafts the GitHub release
   `jspassgen-sdk-rust <version>` with the `.crate` and `SHA256SUMS`. The
   maintainer writes the Highlights and publishes the draft.

crates.io accepts trusted publishing only for a crate that already
exists, so the first version is published by hand: from the tagged
commit, `cargo publish --locked` with an API token, then on crates.io
add the trusted publisher (this repository, `release.yml`, environment
`crates-io`), then push the tag. The workflow skips the publish, still
compares checksums (a mismatch from a different local cargo is only a
warning), and drafts the release. From the next version on, the
workflow publishes.
