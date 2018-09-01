use super::LoggingConfig;
use {Component, FrameworkError, Version};

/// Abscissa component for initializing the logging subsystem
#[derive(Debug, Default)]
pub struct LoggingComponent(LoggingConfig);

impl LoggingComponent {
    /// Create a new `LoggingComponent` with the given configuration
    pub fn new(config: LoggingConfig) -> Self {
        LoggingComponent(config)
    }
}

impl Component for LoggingComponent {
    /// Name of this component
    fn name(&self) -> &'static str {
        "logging"
    }

    /// Version of this component
    fn version(&self) -> Version {
        Version::new(0, 0, 0)
    }

    /// Initialize this component at the time the framework boots
    fn init(&mut self) -> Result<(), FrameworkError> {
        // TODO: shutdown handler?
        super::init(self.0)
    }
}
