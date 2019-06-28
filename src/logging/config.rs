//! Logging configuration

use log::LevelFilter;

/// Logging configuration
// TODO: more configurability
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub(super) level_filter: LevelFilter,
}

impl Config {
    /// Create a new LoggingConfig object with verbose logging
    pub fn verbose() -> Self {
        LevelFilter::Debug.into()
    }
}

impl Default for Config {
    fn default() -> Self {
        LevelFilter::Info.into()
    }
}

impl From<LevelFilter> for Config {
    fn from(level_filter: LevelFilter) -> Config {
        Self { level_filter }
    }
}

impl From<Config> for LevelFilter {
    fn from(logging_config: Config) -> LevelFilter {
        logging_config.level_filter
    }
}
