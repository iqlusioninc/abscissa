//! Abscissa CLI Application

use super::{
    commands::CliCommand,
    config::{CliConfig, APP_CONFIG},
};
use abscissa::{self, Application, EntryPoint, LoggingConfig};

/// Abscissa CLI Application
#[derive(Debug)]
pub struct CliApplication;

impl Application for CliApplication {
    type Cmd = EntryPoint<CliCommand>;
    type Config = CliConfig;

    /// Get a read lock on the application's global configuration
    fn config(&self) -> abscissa::config::Reader<Self::Config> {
        APP_CONFIG.get()
    }

    fn logging_config(&self, command: &EntryPoint<CliCommand>) -> LoggingConfig {
        if command.verbose {
            LoggingConfig::verbose()
        } else {
            LoggingConfig::default()
        }
    }
}
