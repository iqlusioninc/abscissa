//! Terminal component

use super::stream;
use crate::{component, Application, Component, FrameworkError, Version};
use std::fmt;
use termcolor::ColorChoice;

/// Abscissa terminal subsystem component
pub struct TerminalComponent {}

impl TerminalComponent {
    /// Create a new `TerminalComponent` with the given `ColorChoice`
    pub fn new(color_choice: ColorChoice) -> TerminalComponent {
        // TODO(tarcieri): handle terminal reinit (without panicing)
        stream::set_color_choice(color_choice);
        Self {}
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
    fn after_config(&mut self, _config: &A::Cfg) -> Result<(), FrameworkError> {
        Ok(())
    }
}

impl fmt::Debug for TerminalComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TerminalComponent {{ stdout, stderr }}")
    }
}
