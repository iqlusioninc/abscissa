//! Abscissa logging component

// TODO(tarcieri): logfile support?

use super::{config::Config, logger};
use crate::{Component, FrameworkError};

/// Abscissa component for initializing the logging subsystem
#[derive(Component, Debug, Default)]
#[component(core)]
pub struct Logging {
    config: Config,
}

impl Logging {
    /// Create a new logging component
    pub fn new(config: Config) -> Result<Self, FrameworkError> {
        logger::init(&config);
        Ok(Self { config })
    }
}
