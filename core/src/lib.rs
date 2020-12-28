//! ![Abscissa](https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa.svg)
//!
//! Abscissa is a microframework for building Rust applications (either CLI tools
//! or network services), aiming to provide a large number of features with a
//! *minimal number of dependencies*, and with a *strong focus on security*.
//!
//! ## Features
//!
//! - **command-line option parsing**: simple declarative option parser built on
//!   top of the [gumdrop] crate.
//! - **configuration**: TOML configuration file parsing on application-defined
//!   configuration structures which can be dynamically updated at runtime.
//! - **error handling**: unified error-handling subsystem with generic errors.
//! - **tracing**: uses the `tracing` crate to provide async-aware application-
//!   level tracing.
//! - **secrets management**: the (optional) `secrets` module includes a `Secret`
//!  type which derives serde's `Deserialize` and can be used to represent secret
//!  values parsed from configuration files or elsewhere (e.g. credentials loaded
//!  from the environment or network requests)
//! - **terminal interactions**: support for colored terminal output (with color
//!   support autodetection). Useful for Cargo-like status messages with
//!   easy-to-use macros.
//!
//! # Creating a new Abscissa application
//!
//! The following commands will generate an Abscissa application skeleton:
//!
//! ```text
//! $ cargo install abscissa
//! $ abscissa new my_cool_app
//! ```
//!
//! The resulting app is a Cargo project. The following files are particularly
//! noteworthy:
//!
//! - `src/application.rs`: Abscissa application type for your app
//! - `src/commands*`: application entrypoint and subcommands. Make sure to
//!   check out the `hello.rs` example of how to make a subcommand.
//! - `src/config.rs`: application configuration
//! - `src/error.rs`: error types
//!
//! Abscissa applications are implemented as Rust libraries, but have a
//! `src/bin` subdirectory where the binary entrypoint lives. This means you
//! can run the following within your newly generated application:
//!
//! ```text
//! $ cargo run -- hello world
//! ```
//!
//! This will invoke the `hello` subcommand of your application (you'll
//! probably want to rename that in a real app) which will print the following:
//!
//! ```text
//! Hello, world!
//! ```
//!
//! You can also run the following to print basic help information:
//!
//! ```text
//! $ cargo run -- --help
//! ```
//!
//! # Option Parser
//!
//! Command-line options are parsed using the [gumdrop] crate.
//!
//! Please see the documentation for the `options` module.
//!
//! # Status Macros
//!
//! ```ignore
//! // Print a Cargo-like justified status to STDOUT
//! status_ok!("Loaded", "app loaded successfully");
//!
//! // Print an error message
//! status_err!("something bad happened");
//!
//! // Print an indented attribute to STDOUT
//! status_attr_ok!("good", "yep");
//!
//! // Print an error attribute to STDERR
//! status_attr_err!("error", "yep");
//! ```
//!
//! [gumdrop]: https://github.com/murarth/gumdrop
//! [RwLock]: https://doc.rust-lang.org/std/sync/struct.RwLock.html

#![doc(
    html_logo_url = "https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa-sq.svg",
    html_root_url = "https://docs.rs/abscissa_core/0.5.2"
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

#[cfg(feature = "application")]
pub mod application;
#[cfg(feature = "options")]
pub mod command;
#[cfg(feature = "application")]
pub mod component;
#[cfg(feature = "config")]
pub mod config;
pub mod path;
#[cfg(feature = "application")]
pub mod prelude;
mod runnable;
#[cfg(feature = "application")]
mod shutdown;
#[cfg(feature = "testing")]
pub mod testing;
pub mod thread;
#[cfg(feature = "trace")]
pub mod trace;

// Proc macros

#[cfg(feature = "options")]
pub use gumdrop::Options;

// Re-exports

pub use crate::{
    error::framework::{FrameworkError, FrameworkErrorKind},
    runnable::Runnable,
};
pub use std::collections::{btree_map as map, btree_set as set, BTreeMap as Map};

#[cfg(feature = "application")]
pub use crate::{
    application::{boot, Application},
    component::Component,
    shutdown::Shutdown,
};

#[cfg(feature = "config")]
pub use crate::config::{Config, Configurable};

#[cfg(feature = "options")]
pub use crate::{
    command::{Command, EntryPoint, Help},
    path::StandardPaths,
};

// Re-exported modules/types from third-party crates

#[cfg(feature = "time")]
pub use chrono as time;
pub use fs_err as fs;
#[cfg(feature = "secrets")]
pub use secrecy as secret;
#[cfg(feature = "secrets")]
pub use secrecy::Secret;
#[cfg(feature = "application")]
pub use semver::Version;
