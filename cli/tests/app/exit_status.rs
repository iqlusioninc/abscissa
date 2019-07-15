//! Tests for different exit status codes for different usage patterns

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa_core::testing::CmdRunner;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RUNNER: CmdRunner = CmdRunner::default();
}

#[test]
fn no_args() {
    let mut runner = RUNNER.clone();
    runner.capture_stdout().status().expect_success();
}

#[test]
fn invalid_args() {
    let mut runner = RUNNER.clone();
    runner
        .arg("foobar") // invalid arg
        .capture_stdout()
        .status()
        .expect_code(1);
}
