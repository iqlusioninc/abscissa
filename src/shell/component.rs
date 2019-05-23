use super::ColorConfig;
use crate::{component, Application, Component, FrameworkError, Version};

/// Abscissa component for initializing the shell subsystem
#[derive(Default, Debug)]
pub struct ShellComponent(ColorConfig);

impl ShellComponent {
    /// Create a new `ShellComponent` with the given `ColorConfig`
    pub fn new(config: ColorConfig) -> ShellComponent {
        ShellComponent(config)
    }
}

impl<A> Component<A> for ShellComponent
where
    A: Application,
{
    /// Name of this component
    fn name(&self) -> component::Name {
        component::Name("shell")
    }

    /// Version of this component
    fn version(&self) -> Version {
        Version::new(0, 0, 0)
    }

    /// Initialize this component at the time the framework boots
    fn after_config(&mut self, _app: Option<&A::Cfg>) -> Result<(), FrameworkError> {
        super::config(self.0);
        Ok(())
    }
}
