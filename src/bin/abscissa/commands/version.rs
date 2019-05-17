//! `version` subcommand

#![allow(clippy::never_loop)]

use abscissa::{Callable, Command, Options};

/// `version` subcommand
#[derive(Debug, Default, Options)]
pub struct VersionCommand {}

impl Callable for VersionCommand {
    /// Print version message
    fn call(&self) {
        super::CliCommand::print_package_info();
    }
}
