//! Terminal handling (TTY interactions, colors, etc)

#[cfg(feature = "application")]
pub mod component;
#[macro_use]
pub mod status;
pub(crate) mod stream;

pub use termcolor::{Color, ColorChoice};

/// Initialize the terminal subsystem manually, using automatic color
/// detection.
///
/// This is useful when Abscissa internally leverages the terminal subsystem
/// without booting a full application, such as displaying usage information.
pub(crate) fn init() {
    self::component::TerminalComponent::new(ColorChoice::Auto);
}
