//! Application properties

pub mod framework;
pub mod name;
pub mod rust;

pub use semver::Version;
use serde::{Deserialize, Serialize};

/// Application properties: configurable and computed values for names and
/// other configurable values consumed from the templates.
#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    /// Abscissa-related properties
    pub abscissa: framework::Properties,

    /// Application name (i.e. crate name)
    pub name: name::App,

    /// Application title (longer, often capitalized name)
    pub title: String,

    /// Application description (i.e. crate description)
    pub description: String,

    /// Application authors,
    pub authors: Vec<name::Author>,

    /// Application version
    pub version: Version,

    /// Rust edition to use
    pub edition: rust::Edition,

    /// Apply patches to crates.io
    pub patch_crates_io: Option<String>,

    /// Application type name
    pub application_type: name::Type,

    /// Entrypoint command name
    pub command_type: name::Type,

    /// Configuration type name
    pub config_type: name::Type,
}
