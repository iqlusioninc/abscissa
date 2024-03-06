//! Tracing subsystem

#[cfg(all(feature = "default", feature = "application"))]
pub mod component;
mod config;

#[cfg(all(feature = "default", feature = "application"))]
pub use self::component::Tracing;
pub use self::config::Config;
