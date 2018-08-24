//! Abscissa: an app microframework
//!
//! Abscissa is a microframework for building Rust applications (either CLI tools
//! or network services), aiming to provide a large number of features with a
//! minimal number of dependencies, and with a strong focus on security.
//!
//! ## Features
//!
//! - **command-line option parsing**: simple declarative option parser based on
//!   (i.e. forked from) [gumdrop]. The option parser in Abcissa contains numerous
//!   improvements which provide better UX and tighter integration with the other
//!   parts of the framework (e.g. overriding configuration settings using
//!   command-line options).
//! - **configuration**: declarative global configuration support using a [RwLock]
//!   on a [lazy_static]. Simple parsing of TOML configurations to serde-parsed
//!   global structures which can be dynamically updated at runtime.
//! - **error handling**: generic `Error` type based on the `failure` crate, and a
//!   unified error-handling subsystem.
//! - **logging**: uses the `log` and `simplelog` crates to automatically configure
//!   application-level logging, presently to standard output or files.
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
#![doc(html_root_url = "https://docs.rs/abscissa/0.0.0")]

extern crate failure;
#[allow(unknown_lints, unused_imports, useless_attribute)]
#[macro_use]
extern crate abscissa_derive;
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "log")]
pub extern crate log;
#[cfg(feature = "config")]
extern crate serde;
#[cfg(feature = "simplelog")]
extern crate simplelog;
extern crate term;
#[cfg(feature = "toml")]
extern crate toml;

#[cfg(all(test, feature = "options"))]
#[macro_use]
extern crate assert_matches;

pub use term::color::{self, Color};

// Load macros first
#[macro_use]
pub mod macros;

#[cfg(feature = "config")]
pub mod config;
mod error;
mod init;
#[cfg(feature = "options")]
pub mod options;
#[cfg(feature = "secrets")]
pub mod secrets;
mod shell;
pub mod util;

pub use config::{ConfigReader, GlobalConfig};
pub use error::Error;
pub use init::{init, InitOpts};
#[cfg(feature = "options")]
pub use options::Options;
#[cfg(feature = "secrets")]
pub use secrets::Secret;
pub use shell::{status, ColorConfig, Stream};
