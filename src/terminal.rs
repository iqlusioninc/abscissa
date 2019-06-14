//! Terminal handling (TTY interactions, colors, etc)

#[cfg(feature = "application")]
pub mod component;
#[macro_use]
pub mod status;
pub(crate) mod stream;

pub use termcolor::{Color, ColorChoice};
