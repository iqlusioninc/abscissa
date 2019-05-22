//! Abscissa CLI Application

use super::{commands::CliCommand, config::CliConfig};
use abscissa::{
    self, application, component, Application, EntryPoint, FrameworkError, LoggingConfig,
    StandardPaths,
};
use lazy_static::lazy_static;

lazy_static! {
    /// Application state
    pub static ref APPLICATION: application::State<CliApplication> = application::State::default();
}

/// Abscissa CLI Application
#[derive(Debug)]
pub struct CliApplication {
    /// Application configuration
    config: Option<CliConfig>,

    /// Application components
    components: component::Registry<Self>,

    /// Application paths
    paths: StandardPaths,
}

impl Default for CliApplication {
    fn default() -> Self {
        Self {
            config: None,
            components: component::Registry::default(),
            paths: StandardPaths::default(),
        }
    }
}

impl Application for CliApplication {
    type Cmd = EntryPoint<CliCommand>;
    type Cfg = CliConfig;
    type Paths = StandardPaths;

    fn config(&self) -> Option<&CliConfig> {
        self.config.as_ref()
    }

    fn components(&self) -> &component::Registry<Self> {
        &self.components
    }

    fn paths(&self) -> &StandardPaths {
        &self.paths
    }

    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        let components = self.framework_components(command)?;
        self.components.register(components)
    }

    fn after_config(&mut self, config: Option<Self::Cfg>) -> Result<(), FrameworkError> {
        for component in self.components.iter_mut() {
            component.after_config(config.as_ref())?;
        }

        self.config = config;
        Ok(())
    }

    fn logging_config(&self, command: &EntryPoint<CliCommand>) -> LoggingConfig {
        if command.verbose {
            LoggingConfig::verbose()
        } else {
            LoggingConfig::default()
        }
    }
}
