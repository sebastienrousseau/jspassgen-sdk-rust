// SPDX-License-Identifier: Apache-2.0 OR MIT
use super::Error;
use std::error::Error as _;

#[test]
fn invalid_max_explains_the_bound() {
    assert_eq!(Error::InvalidMax.to_string(), "max must be at least 1");
    assert!(Error::InvalidMax.source().is_none());
}

#[test]
fn unavailable_names_the_os_source_and_keeps_the_cause() {
    let error = Error::Unavailable(getrandom::Error::UNSUPPORTED);
    assert_eq!(
        error.to_string(),
        "the operating system random source failed"
    );
    let cause = error.source().expect("the getrandom error is the source");
    assert_eq!(cause.to_string(), getrandom::Error::UNSUPPORTED.to_string());
}
