//! Abscissa CLI Config

use abscissa::config::{Config, Guard};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    /// Application configuration
    pub static ref APP_CONFIG: Guard<CliConfig> = Guard::default();
}

/// Abscissa CLI Config
#[derive(Config, Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CliConfig {
    /// An example configuration section
    pub example_section: ExampleSection,
}

/// Example configuration section
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExampleSection {
    /// Example configuration value
    pub example_value: String,
}
