//! Logging subsystem

#[cfg(feature = "application")]
pub mod component;
mod config;
mod logger;

#[cfg(feature = "application")]
pub use self::component::Logging;
pub use self::config::Config;
