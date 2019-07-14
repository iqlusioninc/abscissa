//! Tests to ensure generated app metadata is correct

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

#[test]
fn crate_name() {
    assert_eq!(env!("CARGO_PKG_NAME"), "abscissa_gen_test_app");
}

#[test]
fn crate_version() {
    assert_eq!(env!("CARGO_PKG_VERSION"), "0.1.0");
}
