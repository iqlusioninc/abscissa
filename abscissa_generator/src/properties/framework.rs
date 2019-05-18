//! Properties-related to Abscissa as an application framework
//!
//! These are presently not very configurable, but could be made to be.

use super::cargo;
pub use semver::Version;
use serde::{Deserialize, Serialize};

/// Default Cargo features to enable in the `abscissa` crate
const DEFAULT_CARGO_FEATURES: &[&str] = &["application", "config", "time"];

/// Abscissa framework-related properties
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Properties {
    /// Abscissa version
    pub version: Version,

    /// Cargo features to enable
    pub cargo_features: Vec<cargo::Feature>,
}

impl Properties {
    /// Initialize Abscissa framework properties
    pub fn new(version: &str) -> Self {
        Self {
            version: version.parse().unwrap(),
            cargo_features: DEFAULT_CARGO_FEATURES
                .iter()
                .map(|feature| feature.parse::<cargo::Feature>().unwrap())
                .collect(),
        }
    }
}
