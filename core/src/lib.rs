#![doc = include_str!("../../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iqlusioninc/abscissa/main/img/abscissa-sq.svg"
)]
#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

/// Abscissa version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "trace")]
#[allow(unused_imports)]
#[macro_use]
pub extern crate tracing;

// Modules with macro exports

#[macro_use]
pub mod error;
#[cfg(feature = "terminal")]
#[macro_use]
pub mod terminal;

// Other modules

#[cfg(all(feature = "default", feature = "application"))]
pub mod application;
#[cfg(all(feature = "default", feature = "options"))]
pub mod command;
#[cfg(all(feature = "default", feature = "application"))]
pub mod component;
#[cfg(all(feature = "default", feature = "config"))]
pub mod config;
pub mod path;
#[cfg(all(feature = "default", feature = "application"))]
pub mod prelude;
mod runnable;
#[cfg(all(feature = "default", feature = "application"))]
mod shutdown;
#[cfg(all(feature = "default", feature = "testing"))]
pub mod testing;
pub mod thread;
#[cfg(feature = "trace")]
pub mod trace;

// Re-exports

pub use crate::{
    error::framework::{FrameworkError, FrameworkErrorKind},
    runnable::Runnable,
};
pub use std::collections::{btree_map as map, btree_set as set, BTreeMap as Map};

#[cfg(all(feature = "default", feature = "application"))]
pub use crate::{
    application::{boot, Application},
    component::Component,
    shutdown::Shutdown,
};

#[cfg(all(feature = "default", feature = "config"))]
pub use crate::config::{Config, Configurable};

#[cfg(all(feature = "default", feature = "options"))]
pub use crate::{command::Command, path::StandardPaths};

// Re-exported modules/types from third-party crates

#[cfg(feature = "options")]
pub use clap;
pub use fs_err as fs;
#[cfg(feature = "secrets")]
pub use secrecy as secret;
#[cfg(feature = "secrets")]
pub use secrecy::Secret;
#[cfg(all(feature = "default", feature = "application"))]
pub use semver::Version;
