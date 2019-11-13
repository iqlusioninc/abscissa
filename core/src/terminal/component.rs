//! Terminal component

use super::stream;
use crate::Component;
use std::fmt;
use termcolor::ColorChoice;

/// Abscissa terminal subsystem component
#[derive(Component)]
#[component(core)]
pub struct Terminal {}

impl Terminal {
    /// Create a new `TerminalComponent` with the given `ColorChoice`
    pub fn new(color_choice: ColorChoice) -> Terminal {
        // TODO(tarcieri): handle terminal reinit (without panicking)
        stream::set_color_choice(color_choice);

        if color_choice != ColorChoice::Never {
            color_backtrace::install();
        }

        Self {}
    }
}

impl fmt::Debug for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TerminalComponent {{ stdout, stderr }}")
    }
}
