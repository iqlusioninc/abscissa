//! Acceptance testing for Abscissa applications.
//!
//! The recommended way to import types for testing is:
//!
//! ```
//! use abscissa::testing::prelude::*;
//! ```
//!
//! The main entrypoint for running tests is [abscissa::testing::CmdRunner].

mod config;
pub mod prelude;
pub mod process;
mod runner;

pub use self::runner::CmdRunner;
