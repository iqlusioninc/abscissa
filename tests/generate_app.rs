//! Generate an Abscissa application using the `abscissa` CLI tool and run
//! tests against it (also `clippy`, `rustfmt`).

#![deny(warnings, missing_docs, unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa::testing::{CargoRunner, CmdRunner};
use std::{env, fs, path::Path};
use tempfile::TempDir;

/// Name of our test application
const APP_NAME: &str = "generated_test_app";

/// Cargo commands to run against the generated application
const TEST_COMMANDS: &[&str] = &[
    "fmt -- --check",
    "test --release",
    "run -- version",
    "clippy",
];

/// Run tests against the generated application
#[test]
fn test_generated_app() {
    let tmp = TempDir::new().unwrap();
    let app_path = tmp.path().join(APP_NAME);

    generate_app(&app_path);

    assert!(env::set_current_dir(&app_path).is_ok());

    for test_command in TEST_COMMANDS {
        CargoRunner::new(test_command.split(" "))
            .status()
            .unwrap()
            .assert_success();
    }
}

/// Generate the app
fn generate_app(path: &Path) {
    let cwd = env::current_dir().unwrap();
    let abscissa_crate_patch = format!("abscissa = {{ path = '{}' }}", cwd.display());

    // Run `abscissa new` to generate the app
    CmdRunner::default()
        .args(&[
            "new",
            &path.display().to_string(),
            "--patch-crates-io",
            &abscissa_crate_patch,
        ])
        .status()
        .unwrap()
        .assert_success();

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
