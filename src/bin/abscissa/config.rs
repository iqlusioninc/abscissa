//! Abscissa CLI Config

use abscissa::config::Config;
use serde::{Deserialize, Serialize};

/// Abscissa CLI Config
#[derive(Config, Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CliConfig {
    // TODO(tarcieri): configuration file?
}
