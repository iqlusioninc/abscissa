//! Application properties

use super::CamelCase;
pub use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    str::FromStr,
};

/// Application properties: configurable and computed values for names and
/// other configurable values consumed from the templates.
#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    /// Abscissa-related properties
    pub abscissa: FrameworkProperties,

    /// Application name (i.e. crate name)
    pub name: AppName,

    /// Application title (longer, often capitalized name)
    pub title: String,

    /// Application description (i.e. crate description)
    pub description: String,

    /// Application authors,
    pub authors: Vec<AuthorName>,

    /// Application version
    pub version: Version,

    /// Rust edition to use
    pub edition: Edition,

    /// Apply patches to crates.io
    pub patch_crates_io: Option<String>,

    /// Application type name
    pub application_type: TypeName,

    /// Entrypoint command name
    pub command_type: TypeName,

    /// Configuration type name
    pub config_type: TypeName,

    /// Error type name
    pub error_type: TypeName,

    /// Error kind type name
    pub error_kind_type: TypeName,
}

/// Default Cargo features to enable
const DEFAULT_CARGO_FEATURES: &[&str] = &["application", "chrono"];

/// Abscissa framework-related properties
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FrameworkProperties {
    /// Abscissa version
    pub version: Version,

    /// Cargo features to enable
    pub cargo_features: Vec<CargoFeature>,
}

impl FrameworkProperties {
    /// Create Abscissa framework properties
    pub fn new(version: &str) -> Self {
        Self {
            version: Version::from_str(version).unwrap(),
            cargo_features: DEFAULT_CARGO_FEATURES
                .iter()
                .map(|feature| feature.parse::<CargoFeature>().unwrap())
                .collect(),
        }
    }
}

/// Cargo feature
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CargoFeature(String);

impl AsRef<str> for CargoFeature {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for CargoFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CargoFeature {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CargoFeature(s.to_owned()))
    }
}

/// Application name
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AppName(String);

impl AsRef<str> for AppName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for AppName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for AppName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AppName(s.to_owned()))
    }
}

/// Author name
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthorName(String);

impl AsRef<str> for AuthorName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<String> for AuthorName {
    fn from(s: String) -> AuthorName {
        AuthorName(s)
    }
}

/// Rust editions
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Edition {
    /// Rust 2015 edition
    #[serde(rename = "2015")]
    Rust2015,

    /// Rust 2018 edition
    #[serde(rename = "2018")]
    Rust2018,
}

/// Type names
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TypeName(String);

impl AsRef<str> for TypeName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl TypeName {
    /// Inflect a snake case name into a type name
    pub fn from_snake_case(s: &str) -> TypeName {
        TypeName(s.to_camel_case())
    }
}
