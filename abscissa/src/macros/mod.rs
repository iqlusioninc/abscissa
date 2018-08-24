//! Macros defined by this crate

#[cfg(feature = "config")]
#[macro_use]
pub mod config;

#[cfg(feature = "errors")]
#[macro_use]
pub mod error;

#[cfg(feature = "log")]
#[macro_use]
pub mod log;

#[cfg(feature = "options")]
#[macro_use]
pub mod options;

#[cfg(feature = "status")]
#[macro_use]
pub mod status;
