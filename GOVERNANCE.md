<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Governance

`jspassgen-sdk-rust` is one of the jspassgen suite repositories,
maintained by one person in a personal capacity.

## Roles

| Role | Who | Responsibilities |
| :--- | :--- | :--- |
| Maintainer | Sebastien Rousseau ([@sebastienrousseau](https://github.com/sebastienrousseau)) | Reviews and merges pull requests, opens and tags releases, publishes to crates.io, triages issues and security reports, owns the signing keys in `KEYS.asc` |
| Contributor | anyone | Opens issues and pull requests under the DCO (`git commit -s`), following [CONTRIBUTING.md](https://github.com/sebastienrousseau/jspassgen-sdk-rust/blob/main/CONTRIBUTING.md) |
| Automated contributor | AI agents, Dependabot | Prepare changes on branches under [AGENTS.md](https://github.com/sebastienrousseau/jspassgen-sdk-rust/blob/main/AGENTS.md); never merge, tag, publish or report outside the repository |

## Decisions

- Everyday changes: a pull request into `main`, merged by the maintainer
  when CI is green.
- Decisions that will be questioned later get an ADR in
  [`docs/adr/`](https://github.com/sebastienrousseau/jspassgen-sdk-rust/tree/main/docs/adr).
- Suite-wide decisions (the version, the generators, their error codes,
  the conformance kit) are made in
  [jspassgen](https://github.com/sebastienrousseau/jspassgen) and
  followed here; this crate releases in lockstep with the suite.

## Releases

The maintainer opens each release (`0.0.1` at a time), merges the release
pull request and pushes the signed tag. See
[DEVELOPMENT.md](https://github.com/sebastienrousseau/jspassgen-sdk-rust/blob/main/DEVELOPMENT.md#release-model).

## Access continuity

A single maintainer is a single point of failure. The plan:

- **Code and history** live on GitHub, and every commit and tag is
  signed with a key listed in `KEYS.asc`, so a fork can verify what it
  starts from.
- **If the maintainer is unreachable** for 90 days with open security
  reports, a contributor may fork, keep the licence and history, and
  publish under a new crate name; the dual Apache-2.0 OR MIT grant allows
  it. Security reports should then go to the fork's advisories page.
- **Succession**: the maintainer can add a second owner on GitHub and,
  once the crate is published, on crates.io; this file will name them
  when that happens.

## Code of conduct

[CODE_OF_CONDUCT.md](https://github.com/sebastienrousseau/jspassgen-sdk-rust/blob/main/CODE_OF_CONDUCT.md)
applies to every space of the project.
