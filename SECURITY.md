<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Security Policy

## Supported versions

| Version | Status | Security fixes |
| :--- | :--- | :--- |
| `0.0.23` | Latest release | Yes |

The latest release receives security fixes; an older release is fixed
by upgrading, since each step is `0.0.1`.

## Reporting a vulnerability

**Do not open a public issue for security reports.**

Report privately through GitHub Private Vulnerability Reporting:
<https://github.com/sebastienrousseau/jspassgen-sdk-rust/security/advisories/new>.
The maintainer is notified automatically.

Please include a description and the potential impact, a minimal
reproducer (preferably a failing test), the version or commit affected,
and any proposed remediation.

These are goals, not guarantees: the project is maintained in a personal
capacity, with no SLA and no out-of-hours cover. Acknowledgement is aimed
at within 5 working days, followed by a severity assessment and a
remediation plan.

A vulnerability in the jspassgen core or its conformance kit belongs to
[jspassgen](https://github.com/sebastienrousseau/jspassgen/security/advisories/new).

## Security posture

- No `unsafe` code: the crate root has `#![forbid(unsafe_code)]`.
- Randomness comes from the operating system's CSPRNG through the
  `getrandom` crate (`OsRandom`); there is no user-space PRNG and no
  seed.
- `random_int` draws without modulo bias, by rejection sampling; its
  tests include a statistical check shown to fail for a modulo sampler.
- No network call, no telemetry, no file access.
- `cargo deny check` runs in CI and `make check`, with no ignored
  advisories; CodeQL and OpenSSF Scorecard run on `main`.
