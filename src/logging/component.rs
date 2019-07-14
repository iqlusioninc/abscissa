//! Abscissa logging component

// TODO(tarcieri): logfile support?

use super::{config::Config, logger};
use crate::{component, Application, Component, FrameworkError, Version};

/// Abscissa component for initializing the logging subsystem
#[derive(Debug, Default)]
pub struct LoggingComponent {
    config: Config,
}

impl LoggingComponent {
    /// Create a new logging component
    pub fn new(config: Config) -> Result<Self, FrameworkError> {
        logger::init(&config);
        Ok(Self { config })
    }
}

// TODO: shutdown handler?
impl<A> Component<A> for LoggingComponent
where
    A: Application,
{
    /// Name of this component
    fn name(&self) -> component::Name {
        component::Name("abscissa::logging")
    }

    /// Version of this component
    fn version(&self) -> Version {
        Version::new(0, 0, 0)
    }

    /// Initialize this component at the time the framework boots
    fn after_config(&mut self, _config: &A::Cfg) -> Result<(), FrameworkError> {
        // TODO(tarcieri): set logging configuration here instead of earlier?
        Ok(())
    }
}
