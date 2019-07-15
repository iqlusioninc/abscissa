//! Generate an Abscissa application using the `abscissa` CLI tool and run
//! tests against it (also `clippy`, `rustfmt`).

#![deny(warnings, missing_docs, unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa_core::testing::prelude::*;
use lazy_static::lazy_static;
use std::{env, fs, path::Path};

/// Name of our test application
const APP_NAME: &str = "abscissa_gen_test_app";

/// Cargo commands to run against the generated application
const TEST_COMMANDS: &[&str] = &[
    "fmt -- --check",
    "test --release",
    "run -- version",
    "clippy",
];

lazy_static! {
    pub static ref RUNNER: CmdRunner = {
        let mut runner = CmdRunner::new("cargo");
        runner.exclusive();
        runner
    };
}

/// Run tests against the generated application
#[test]
fn test_generated_app() {
    let app_path = env::temp_dir().join(APP_NAME);
    generate_app(&app_path);
    env::set_current_dir(&app_path).unwrap();

    for test_command in TEST_COMMANDS {
        let mut runner = RUNNER.clone();
        runner
            .args(test_command.split(" "))
            .status()
            .expect_success();
    }
}

/// Generate the app
fn generate_app(path: &Path) {
    let cwd = env::current_dir().unwrap();
    let core_dir = cwd.join("../core").canonicalize().unwrap();
    let abscissa_crate_patch = format!("abscissa_core = {{ path = '{}' }}", core_dir.display());

    // Run `abscissa new` to generate the app
    CmdRunner::default()
        .args(&[
            "new",
            &path.display().to_string(),
            "--force",
            "--patch-crates-io",
            &abscissa_crate_patch,
        ])
        .status()
        .expect_success();

    let app_test_dir = path.join("tests");

    // Copy supplemental application tests into the newly generated application
    fs::create_dir_all(&app_test_dir).unwrap();

    for entry in fs::read_dir("tests/app").unwrap() {
        let test_file = entry.unwrap().path();
        fs::copy(
            &test_file,
            app_test_dir.join(&test_file.file_name().unwrap()),
        )
        .unwrap();
    }
}
