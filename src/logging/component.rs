use super::LoggingConfig;
use crate::{component, Application, Component, FrameworkError, Version};

/// Abscissa component for initializing the logging subsystem
#[derive(Debug, Default)]
pub struct LoggingComponent(LoggingConfig);

impl LoggingComponent {
    /// Create a new `LoggingComponent` with the given configuration
    pub fn new(config: LoggingConfig) -> Self {
        LoggingComponent(config)
    }
}

// TODO: shutdown handler?
impl<A> Component<A> for LoggingComponent
where
    A: Application,
{
    /// Name of this component
    fn name(&self) -> component::Name {
        component::Name("LoggingComponent")
    }

    /// Version of this component
    fn version(&self) -> Version {
        Version::new(0, 0, 0)
    }

    /// Initialize this component at the time the framework boots
    fn after_config(&mut self, _config: Option<&A::Cfg>) -> Result<(), FrameworkError> {
        super::init(self.0)
    }
}
