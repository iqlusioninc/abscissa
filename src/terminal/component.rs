use super::color::ColorConfig;
use crate::{component, Application, Component, FrameworkError, Version};

/// Abscissa terminal subsystem component
#[derive(Default, Debug)]
pub struct TerminalComponent(ColorConfig);

impl TerminalComponent {
    /// Create a new `TerminalComponent` with the given `ColorConfig`
    pub fn new(config: ColorConfig) -> TerminalComponent {
        TerminalComponent(config)
    }
}

impl<A> Component<A> for TerminalComponent
where
    A: Application,
{
    /// Name of this component
    fn name(&self) -> component::Name {
        component::Name("abscissa::terminal")
    }

    /// Version of this component
    fn version(&self) -> Version {
        Version::new(0, 0, 0)
    }

    /// Initialize this component at the time the framework boots
    fn after_config(&mut self, _app: Option<&A::Cfg>) -> Result<(), FrameworkError> {
        self.0.init();
        Ok(())
    }
}
