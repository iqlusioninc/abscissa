//! `version` subcommand

#![allow(clippy::never_loop)]

use super::CliCommand;
use abscissa_core::{Clap, Command, Runnable};

/// `version` subcommand
#[derive(Command, Debug, Default, Clap)]
pub struct VersionCommand {}

impl Runnable for VersionCommand {
    /// Print version message
    fn run(&self) {
        println!("{} {}", CliCommand::name(), CliCommand::version());
    }
}
