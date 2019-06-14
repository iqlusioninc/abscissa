//! Logging configuration

use log::LevelFilter;

/// Logging configuration
// TODO: more configurability
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LoggingConfig {
    level_filter: LevelFilter,
}

impl LoggingConfig {
    /// Create a new LoggingConfig object with verbose logging
    pub fn verbose() -> Self {
        LevelFilter::Debug.into()
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        LevelFilter::Info.into()
    }
}

impl From<LevelFilter> for LoggingConfig {
    fn from(level_filter: LevelFilter) -> LoggingConfig {
        Self { level_filter }
    }
}

impl From<LoggingConfig> for LevelFilter {
    fn from(logging_config: LoggingConfig) -> LevelFilter {
        logging_config.level_filter
    }
}
