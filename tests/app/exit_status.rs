//! Tests for different exit status codes for different usage patterns

use abscissa::testing::CmdRunner;

#[test]
fn no_args() {
    CmdRunner::default()
        .capture_stdout()
        .status()
        .unwrap()
        .expect_success();
}

#[test]
fn invalid_args() {
    CmdRunner::default()
        .arg("foobar") // invalid arg
        .capture_stdout()
        .status()
        .unwrap()
        .expect_code(1);
}
