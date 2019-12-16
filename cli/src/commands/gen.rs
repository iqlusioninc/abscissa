//! `gen` subcommand: code generator functionality

mod cmd;

pub use cmd::Cmd;

use abscissa_core::{Command, Options, Runnable};

/// `gen` subcommand: code generator functionality
#[derive(Command, Debug, Options, Runnable)]
pub enum GenCommand {
    /// Generate a new subcommand
    Cmd(Cmd),
}
