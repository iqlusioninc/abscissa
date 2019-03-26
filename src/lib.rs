//! Abscissa: an app microframework
//!
//! Abscissa is a microframework for building Rust applications (either CLI tools
//! or network services), aiming to provide a large number of features with a
//! *minimal number of dependencies*, and with a *strong focus on security*.
//!
//! ## Features
//!
//! - **command-line option parsing**: simple declarative option parser built on
//!   top of the [gumdrop] crate.
//! - **configuration**: declarative global configuration support using a [RwLock]
//!   on a [lazy_static]. Simple parsing of TOML configurations to serde-parsed
//!   global structures which can be dynamically updated at runtime.
//! - **error handling**: generic `Error` type based on the `failure` crate, and a
//!   unified error-handling subsystem.
//! - **logging**: uses the `log` and `simplelog` crates to automatically configure
//!   application-level logging, presently to standard output or files.
//! - **secrets management**: the (optional) `secrets` module includes a `Secret`
//!  type which derives serde's `Deserialize` and can be used to represent secret
//!  values parsed from configuration files or elsewhere (e.g. credentials loaded
//!  from the environment or network requests)
//! - **shell interactions**: support for colored terminal output (with color
//!   support autodetection). Useful for Cargo-like status messages with
//!   easy-to-use macros.
//!
//! [gumdrop]: https://github.com/murarth/gumdrop
//! [RwLock]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
//! [lazy_static]: https://docs.rs/lazy_static
//!
//! # Option Parser
//!
//! Please see the documentation for the `options` module.
//!
//! # Status Macros
//!
//! ```
//! # #[macro_use] extern crate abscissa;
//! # fn main() {
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
//! # }
//! ```

#![crate_name = "abscissa"]
#![crate_type = "rlib"]
#![deny(
    warnings,
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#![doc(
    html_logo_url = "https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa-sq.svg",
    html_root_url = "https://docs.rs/abscissa/0.0.6"
)]

// TODO: our own proc macros
//#[macro_use]
extern crate abscissa_derive;

pub use failure;
#[cfg(any(feature = "errors", feature = "options"))]
#[macro_use]
extern crate failure_derive;
#[cfg(feature = "shell")]
extern crate isatty;
#[cfg(feature = "shell")]
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "logging")]
pub extern crate log;

#[cfg(feature = "logging")]
extern crate simplelog;
#[cfg(feature = "shell")]
extern crate term;

// Load macros first
#[macro_use]
pub mod macros;

#[cfg(feature = "application")]
mod application;
#[cfg(feature = "options")]
mod command;
#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "errors")]
pub mod error;
#[cfg(feature = "logging")]
pub mod logging;
#[cfg(feature = "secrets")]
pub mod secrets;
#[cfg(feature = "shell")]
pub mod shell;
pub mod util;

#[cfg(feature = "application")]
pub use crate::application::{boot, Application, ApplicationPath, Component, Components};
#[cfg(feature = "options")]
pub use crate::command::{Callable, Command};
#[cfg(feature = "config")]
pub use crate::config::{ConfigReader, GlobalConfig, LoadConfig};
#[cfg(feature = "errors")]
pub use crate::error::{Error, Fail, FrameworkError, FrameworkErrorKind};
#[cfg(feature = "logging")]
pub use crate::logging::LoggingConfig;
#[cfg(feature = "secrets")]
pub use crate::secrets::Secret;
#[cfg(feature = "shell")]
pub use crate::shell::{status, ColorConfig, Stream};
#[cfg(feature = "application")]
pub use crate::util::{CanonicalPath, CanonicalPathBuf, Version};
#[cfg(feature = "options")]
pub use gumdrop::Options;
