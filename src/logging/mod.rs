//! Logging subsystem

#[cfg(feature = "application")]
mod component;
mod simplelog;

#[cfg(feature = "application")]
pub use self::component::LoggingComponent;
// `simplelog` is the only logger we presently support
pub use self::simplelog::{init, LoggingConfig};
