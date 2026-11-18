<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Policies

## Toolchain

The minimum supported Rust is 1.85 (`rust-version`). CI builds and tests
on 1.85.0 and on stable. Raising the floor is a release-noted change
made only when a dependency needs it or a language feature pays for
itself, and never more than to a release at least six months old.

## Versions

`0.0.x`, one `0.0.1` step per release, in lockstep with the jspassgen
suite. Breaking changes are allowed between steps while pre-1.0 and are
listed in the changelog.

## Dependencies

One runtime dependency, `getrandom` ([ADR 1](adr/0001-os-csprng-through-getrandom.md)).
`cargo deny` enforces advisories, the licence allow-list and the
crates.io-only source policy. Dependabot opens weekly updates.

## Core compatibility

Once the conformance kit exists, each release names the jspassgen commit
it is tested against (`ci/core.ref`) and vendors that commit's kit.
