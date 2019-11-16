//! Example application used for testing purposes

use abscissa_core::{
    application, Application, Command, Configurable, FrameworkError, Options, Runnable,
    StandardPaths,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExampleConfig {}

#[derive(Command, Debug, Options)]
pub struct ExampleCommand {}

impl Configurable<ExampleConfig> for ExampleCommand {
    fn config_path(&self) -> Option<PathBuf> {
        None
    }
}

impl Runnable for ExampleCommand {
    fn run(&self) {
        unimplemented!();
    }
}

#[derive(Debug, Default)]
pub struct ExampleApp {
    config: Option<ExampleConfig>,
    state: application::State<Self>,
}

impl Application for ExampleApp {
    type Cmd = ExampleCommand;
    type Cfg = ExampleConfig;
    type Paths = StandardPaths;

    fn config(&self) -> &ExampleConfig {
        unimplemented!();
    }

    fn state(&self) -> &application::State<Self> {
        unimplemented!();
    }

    fn state_mut(&mut self) -> &mut application::State<Self> {
        unimplemented!();
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
}
