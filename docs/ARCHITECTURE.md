<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Architecture

## Today

```text
src/
├── lib.rs           crate docs, #![forbid(unsafe_code)], re-exports
├── error.rs         Error: InvalidMax, Unavailable
└── random.rs        RandomSource (trait), OsRandom, the rejection sampler
tests/uniformity.rs  chi-square tests against OsRandom
```

`RandomSource` is the port every generator will draw through, the Rust
form of `RandomGeneratorPort` in jspassgen's `packages/core`:

| jspassgen (JavaScript) | This crate |
| :--- | :--- |
| `generateRandomBytes(byteLength)` | `fill_bytes(&mut [u8])` |
| `generateRandomInt(max)` | `random_int(max: u32)` |
| Node.js `crypto`, Web Crypto | `OsRandom` (`getrandom`) |

`random_int` has a default implementation, so a source only supplies
bytes. It reads a little-endian `u32` and keeps it when it is at least
`2^32 mod max`; the kept values cover `[0, max)` an equal number of
times, so `value % max` is uniform. The sampler is one non-generic
function shared by every source.

## Next

The generators (`strong`, `base64`, `memorable`, `quantum-resistant`,
`diceware`, `honeyword`, `pronounceable`, `custom` in jspassgen), their
validation and entropy calculation are ported once jspassgen's
conformance kit exists. The kit's draw tapes record each `random_int`
call core makes; a test `RandomSource` replays them, so a port has to
make the same draws in the same order and produce the same password.
See jspassgen's
[ADR 0001](https://github.com/sebastienrousseau/jspassgen/blob/master/docs/adr/0001-ecosystem-lockstep.md),
section 4.
