//! Abscissa CLI Config

use serde::{Deserialize, Serialize};

/// Abscissa CLI Config
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CliConfig {
    // TODO(tarcieri): configuration file?
}
