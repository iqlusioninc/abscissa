//! Tests to ensure generated app metadata is correct

#[test]
fn crate_name() {
    assert_eq!(env!("CARGO_PKG_NAME"), "generated_test_app");
}

#[test]
fn crate_version() {
    assert_eq!(env!("CARGO_PKG_VERSION"), "0.1.0");
}
