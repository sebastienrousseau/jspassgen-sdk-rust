// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Fuzzes `RandomSource::random_int` against its contract: the first
//! four bytes are `max`, the rest the stream the source replays. The
//! contract lives in `tests/contract`, shared with the stable test
//! `tests/random_int_contract.rs`.

#![no_main]

use libfuzzer_sys::fuzz_target;

#[path = "../../tests/contract/mod.rs"]
mod contract;

fuzz_target!(|data: &[u8]| contract::check(data));
