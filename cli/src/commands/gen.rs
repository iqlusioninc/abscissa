//! `gen` subcommand: code generator functionality

mod cmd;

pub use cmd::Cmd;

use abscissa_core::{Clap, Command, Runnable};

#[derive(Debug, Clap, Runnable)]
enum SubCommands {
    Cmd(Cmd),
}

/// `gen` subcommand: code generator functionality
#[derive(Command, Debug, Clap, Runnable)]
pub struct GenCommand {
    /// Generate a new subcommand
    #[clap(subcommand)]
    cmd: SubCommands,
}
