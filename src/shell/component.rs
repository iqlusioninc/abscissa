use super::ColorConfig;
use {Component, FrameworkError, Version};

/// Abscissa component for initializing the shell subsystem
#[derive(Default, Debug)]
pub struct ShellComponent(ColorConfig);

impl ShellComponent {
    /// Create a new `ShellComponent` with the given `ColorConfig`
    pub fn new(config: ColorConfig) -> ShellComponent {
        ShellComponent(config)
    }
}

impl Component for ShellComponent {
    /// Name of this component
    fn name(&self) -> &'static str {
        "shell"
    }

    /// Version of this component
    fn version(&self) -> Version {
        Version::new(0, 0, 0)
    }

    /// Initialize this component at the time the framework boots
    fn init(&mut self) -> Result<(), FrameworkError> {
        super::config(self.0);
        Ok(())
    }
}
