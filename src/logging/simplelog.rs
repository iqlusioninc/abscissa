//! `simplelog`-based logging subsystem

use simplelog::{self, CombinedLogger, LevelFilter, TermLogger};

use crate::error::FrameworkError;

/// Logging configuration
// TODO: make things configurable via this newtype
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LoggingConfig {
    level_filter: LevelFilter,
    simplelog_config: simplelog::Config,
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
        Self {
            level_filter,
            simplelog_config: Default::default(),
        }
    }
}

/// Initialize the logging subsystem (i.e. simplelog) using this configuration
pub fn init(config: LoggingConfig) -> Result<(), FrameworkError> {
    let LoggingConfig {
        level_filter,
        simplelog_config,
    } = config;

    if let Some(logger) = TermLogger::new(level_filter, simplelog_config) {
        CombinedLogger::init(vec![logger]).unwrap()
    } // TODO: handle the case we don't get the logger?

    Ok(())
}
