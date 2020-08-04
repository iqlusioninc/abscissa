//! Abscissa CLI Subcommands

pub mod gen;
pub mod new;
pub mod version;

use self::{gen::GenCommand, new::NewCommand, version::VersionCommand};
use super::config::CliConfig;
use abscissa_core::{Command, Configurable, Help, Runnable};
use clap::Clap;
use std::path::PathBuf;

/// Abscissa CLI Subcommands
#[derive(Command, Debug, Clap, Runnable)]
pub enum CliCommand {
    #[clap(help = "generate a new module in an existing app")]
    Gen(GenCommand),

    #[clap(help = "show help for a command")]
    Help(Help<Self>),

    #[clap(help = "create a new Abscissa application from a template")]
    New(NewCommand),

    #[clap(help = "display version information")]
    Version(VersionCommand),
}

impl Configurable<CliConfig> for CliCommand {
    fn config_path(&self) -> Option<PathBuf> {
        None
    }
}
