//! `gen` subcommand: code generator functionality

mod cmd;

pub use cmd::Cmd;

use abscissa_core::{Command, Runnable, clap::Parser};

#[derive(Debug, Parser, Runnable)]
enum SubCommands {
    Cmd(Cmd),
}

/// `gen` subcommand: code generator functionality
#[derive(Command, Debug, Parser, Runnable)]
pub struct GenCommand {
    /// Generate a new subcommand
    #[clap(subcommand)]
    cmd: SubCommands,
}
