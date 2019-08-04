//! Terminal component

use super::stream;
use crate::Component;
use std::fmt;
use termcolor::ColorChoice;

/// Abscissa terminal subsystem component
#[derive(Component)]
#[component(core)]
pub struct TerminalComponent {}

impl TerminalComponent {
    /// Create a new `TerminalComponent` with the given `ColorChoice`
    pub fn new(color_choice: ColorChoice) -> TerminalComponent {
        // TODO(tarcieri): handle terminal reinit (without panicing)
        stream::set_color_choice(color_choice);
        Self {}
    }
}

impl fmt::Debug for TerminalComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TerminalComponent {{ stdout, stderr }}")
    }
}
