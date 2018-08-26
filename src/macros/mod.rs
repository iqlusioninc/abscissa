//! Macros defined by this crate

#[cfg(feature = "options")]
#[macro_use]
pub mod command;

#[cfg(feature = "config")]
#[macro_use]
pub mod config;

#[cfg(feature = "errors")]
#[macro_use]
pub mod error;

#[cfg(feature = "log")]
#[macro_use]
pub mod log;

#[cfg(feature = "status")]
#[macro_use]
pub mod status;
