//! Abscissa CLI Application

use super::{commands::CliCommand, config::CliConfig};
use abscissa_core::{
    application::{AppCell, State},
    trace, Application, EntryPoint, FrameworkError, StandardPaths,
};

/// Application state
pub static APPLICATION: AppCell<CliApplication> = AppCell::new();

/// Abscissa CLI Application
#[derive(Debug)]
pub struct CliApplication {
    /// Application configuration.
    config: Option<CliConfig>,

    /// Application state.
    state: State<Self>,
}

impl Default for CliApplication {
    fn default() -> Self {
        Self {
            config: None,
            state: Default::default(),
        }
    }
}

impl Application for CliApplication {
    type Cmd = EntryPoint<CliCommand>;
    type Cfg = CliConfig;
    type Paths = StandardPaths;

    fn config(&self) -> &CliConfig {
        self.config.as_ref().expect("config not loaded")
    }

    fn state(&self) -> &State<Self> {
        &self.state
    }

    fn state_mut(&mut self) -> &mut State<Self> {
        &mut self.state
    }

    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        let components = self.framework_components(command)?;
        self.state.components.register(components)
    }

    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError> {
        self.state.components.after_config(&config)?;
        self.config = Some(config);
        Ok(())
    }

    fn tracing_config(&self, command: &EntryPoint<CliCommand>) -> trace::Config {
        if command.verbose {
            trace::Config::verbose()
        } else {
            trace::Config::default()
        }
    }
}
