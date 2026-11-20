// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Replays the `random_int` contract (`tests/contract`) on the inputs
//! where a sampler goes wrong: every edge of `max`, words on each side
//! of the rejection threshold and at the ends of the `u32` range, runs
//! of rejected words, and streams that end mid-word. The cargo-fuzz
//! target in `fuzz/` checks the same contract on generated input.

mod contract;

/// Values of `max` at the edges: 0, 1, small and alphabet-sized ranges,
/// powers of two and their neighbours, and the top of the `u32` range.
const MAXES: [u32; 20] = [
    0,
    1,
    2,
    3,
    5,
    6,
    10,
    26,
    62,
    94,
    255,
    256,
    1000,
    (1 << 31) - 1,
    1 << 31,
    (1 << 31) + 1,
    3 << 30,
    u32::MAX - 1,
    u32::MAX,
    0x8000_0001,
];

/// Words that straddle the rejection threshold of `max` and the ends of
/// the `u32` range.
fn edge_words(max: u32) -> Vec<u32> {
    let threshold = if max == 0 {
        0
    } else {
        max.wrapping_neg() % max
    };
    let mut words = vec![
        0,
        1,
        threshold.wrapping_sub(1),
        threshold,
        threshold.wrapping_add(1),
        max.wrapping_sub(1),
        max,
        u32::MAX - 1,
        u32::MAX,
    ];
    words.sort_unstable();
    words.dedup();
    words
}

/// `max` followed by `words`, little-endian, and the first `extra`
/// bytes of one more word, so the stream can end mid-word.
fn input(max: u32, words: &[u32], extra: usize) -> Vec<u8> {
    let mut data = max.to_le_bytes().to_vec();
    for word in words {
        data.extend_from_slice(&word.to_le_bytes());
    }
    data.extend_from_slice(&[0xa5; 3][..extra]);
    data
}

#[test]
fn inputs_too_short_to_hold_max_are_skipped() {
    for len in 0..4 {
        contract::check(&[0xff; 3][..len]);
    }
}

#[test]
fn every_edge_pair_keeps_the_contract() {
    let mut cases = 0;
    for max in MAXES {
        let words = edge_words(max);
        for &first in &words {
            for &second in &words {
                for extra in 0..4 {
                    contract::check(&input(max, &[], extra));
                    contract::check(&input(max, &[first], extra));
                    contract::check(&input(max, &[first, second], extra));
                    cases += 3;
                }
            }
        }
    }
    assert!(cases > 5_000, "only {cases} cases ran");
}

#[test]
fn long_runs_of_rejected_words_keep_the_contract() {
    // 2^32 mod max is 2^31 - 1 for max 2^31 + 1: every word below it is
    // rejected, so the sampler must keep reading until the last word.
    let max = (1 << 31) + 1;
    for run in 0..64 {
        let mut words: Vec<u32> = (0..run).collect();
        contract::check(&input(max, &words, 0));
        words.push(u32::MAX);
        contract::check(&input(max, &words, 0));
    }
}
