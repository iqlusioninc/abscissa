//! Logging subsystem

#[cfg(feature = "application")]
pub mod component;
mod config;
mod logger;

#[cfg(feature = "application")]
pub use self::component::LoggingComponent;
pub use self::config::LoggingConfig;
