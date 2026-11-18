<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Contributing

Thank you for helping. This crate is a port: the generators, their rules
and their error codes belong to
[jspassgen](https://github.com/sebastienrousseau/jspassgen), so a new
generator or a change of behaviour starts there, in `packages/core` and
the conformance kit, and reaches this crate afterwards.

## Before you open a pull request

1. Open an issue for anything larger than a fix, so the change can be
   agreed first.
2. Branch from the current `feat/v<version>` release branch.
3. Run `make check` and `make docs-lint` (see
   [DEVELOPMENT.md](https://github.com/sebastienrousseau/jspassgen-sdk-rust/blob/main/DEVELOPMENT.md)).
4. Add or change the tests in the same commit as the behaviour.
5. Update `CHANGELOG.md` under the current version.

## Commits

- Conventional commits (`feat:`, `fix:`, `docs:`, `chore:` ...), subject
  in the imperative, at most 50 characters; body wrapped at 72, saying
  what and why.
- Sign off every commit (`git commit -s`): you certify the
  [Developer Certificate of Origin](https://github.com/sebastienrousseau/jspassgen-sdk-rust/blob/main/DCO.txt). The DCO workflow checks it.

## Rules that do not bend

- No `unsafe`, no randomness from anything but a `RandomSource`, no
  network call, no telemetry, no logging or printing of generated
  passwords.
- `random_int` stays unbiased: no modulo of raw bytes.
- No new runtime dependency without a reason in the pull request (and an
  ADR when it shapes the crate).
- Synthetic test values only.

By contributing you agree that your work is licensed under Apache-2.0 OR
MIT, as the rest of the repository.
