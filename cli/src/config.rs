//! Abscissa CLI Config

use abscissa_core::config::Config;
use serde::{Deserialize, Serialize};

/// Abscissa CLI Config
#[derive(Config, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CliConfig {
    // TODO(tarcieri): configuration file?
}
