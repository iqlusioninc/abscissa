//! Properties-related to Abscissa as an application framework
//!
//! These are presently not very configurable, but could be made to be.

pub use semver::Version;
use serde::{Deserialize, Serialize};

/// Abscissa framework-related properties
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Properties {
    /// Abscissa version
    pub version: Version,
}

impl Properties {
    /// Initialize Abscissa framework properties
    pub fn new(version: &str) -> Self {
        Self {
            version: version.parse().unwrap(),
        }
    }
}
