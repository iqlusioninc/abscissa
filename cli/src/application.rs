//! Abscissa CLI Application

use super::{commands::CliCommand, config::CliConfig};
use abscissa_core::{
    Application, FrameworkError, StandardPaths,
    application::{AppCell, State},
    config::{self, CfgCell},
    trace,
};

/// Application state
pub static APP: AppCell<CliApplication> = AppCell::new();

/// Abscissa CLI Application
#[derive(Debug, Default)]
pub struct CliApplication {
    /// Application configuration.
    config: CfgCell<CliConfig>,

    /// Application state.
    state: State<Self>,
}

impl Application for CliApplication {
    type Cmd = CliCommand;
    type Cfg = CliConfig;
    type Paths = StandardPaths;

    fn config(&self) -> config::Reader<CliConfig> {
        self.config.read()
    }

    fn state(&self) -> &State<Self> {
        &self.state
    }

    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        let framework_components = self.framework_components(command)?;
        let mut app_components = self.state.components_mut();
        app_components.register(framework_components)
    }

    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError> {
        let mut components = self.state.components_mut();
        components.after_config(&config)?;
        self.config.set_once(config);
        Ok(())
    }

    fn tracing_config(&self, command: &CliCommand) -> trace::Config {
        if command.verbose {
            trace::Config::verbose()
        } else {
            trace::Config::default()
        }
    }
}
