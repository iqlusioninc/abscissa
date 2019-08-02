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
//! - **error handling**: generic `Error` type based on the `failure` crate, and a
//!   unified error-handling subsystem.
//! - **logging**: uses the `log` crate to provide application-level logging.
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
//! ```norun
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
//! [lazy_static]: https://docs.rs/lazy_static

#![deny(warnings, missing_docs, unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa-sq.svg",
    html_root_url = "https://docs.rs/abscissa_core/0.3.0-rc.0"
)]

/// Abscissa version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "logging")]
#[allow(unused_imports)]
#[macro_use]
pub extern crate log;

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
#[cfg(feature = "logging")]
pub mod logging;
pub mod path;
mod runnable;
#[cfg(feature = "application")]
mod shutdown;
#[cfg(all(feature = "signals", unix))]
pub mod signal;
#[cfg(feature = "testing")]
pub mod testing;
pub mod thread;

// Proc macros

#[doc(hidden)]
pub use abscissa_derive::{Command, Runnable};
#[cfg(feature = "options")]
pub use gumdrop::Options;

// Re-exports

#[cfg(feature = "config")]
pub use crate::config::{Config, Configurable};
pub use crate::error::{Error, Fail, FrameworkError, FrameworkErrorKind};
pub use crate::runnable::Runnable;
#[cfg(feature = "application")]
pub use crate::{
    application::{boot, Application},
    component::Component,
    shutdown::Shutdown,
};
#[cfg(feature = "options")]
pub use crate::{
    command::{Command, EntryPoint, Help},
    path::StandardPaths,
};

// Re-exported modules/types from third-party crates

#[cfg(feature = "time")]
pub use chrono as time;
#[cfg(feature = "inflector")]
pub use heck as inflector;
#[cfg(feature = "secrets")]
pub use secrecy as secret;
#[cfg(feature = "secrets")]
pub use secrecy::Secret;
#[cfg(feature = "application")]
pub use semver::Version;
