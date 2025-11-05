//! Abscissa CLI Subcommands

pub mod generate;
pub mod new;

use self::{generate::GenCommand, new::NewCommand};
use super::config::CliConfig;
use abscissa_core::{Command, Configurable, Runnable, clap::Parser};
use std::path::PathBuf;

#[derive(Debug, Parser, Runnable)]
enum SubCommands {
    /// generate a new module in an existing app
    Gen(GenCommand),

    /// Create a new Abscissa application from a template
    New(NewCommand),
}

/// Abscissa CLI Subcommands
#[derive(Command, Debug, Parser)]
#[command(author, about, version)]
pub struct CliCommand {
    #[command(subcommand)]
    subcmd: SubCommands,

    /// Enable verbose mode
    #[arg(short, long)]
    pub verbose: bool,
}

impl Runnable for CliCommand {
    fn run(&self) {
        self.subcmd.run()
    }
}

impl Configurable<CliConfig> for CliCommand {
    fn config_path(&self) -> Option<PathBuf> {
        None
    }
}
