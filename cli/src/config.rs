//! Abscissa CLI Config

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    env,
    path::{Path, PathBuf},
    process::exit,
};

/// Locate the root of the target application (with optional manifest path)
pub fn target_app_root(manifest_path: Option<&Path>) -> PathBuf {
    match manifest_path {
        Some(path) => {
            if !path.exists() || path.parent().is_none() {
                status_err!("the manifest-path must be a path to a Cargo.toml file");
                exit(1);
            }

            path.parent().unwrap().to_owned()
        }
        None => {
            let cwd = env::current_dir()
                .expect("couldn't get current working directory!")
                .canonicalize()
                .expect("couldn't canonicalize current working directory!");

            let mut path = cwd.as_path();

            loop {
                if path.join("Cargo.toml").exists() {
                    return path.to_owned();
                }

                match path.parent() {
                    Some(p) => path = p,
                    None => {
                        status_err!("couldn't find application root!");
                        exit(1);
                    }
                }
            }
        }
    }
}

/// Abscissa CLI Config
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CliConfig {
    // TODO(tarcieri): configuration file?
}
