//! Toplevel entrypoint command.

use crate::{config, Callable, Command, Options};
use std::path::PathBuf;

/// Toplevel entrypoint command.
///
/// Handles obtaining toplevel help as well as verbosity settings.
#[derive(Debug, Options)]
pub struct EntryPoint<Cmd: Callable + Command> {
    /// Path to the configuration file
    #[options(help = "path to configuration file")]
    pub config: Option<PathBuf>,

    /// Obtain help about the current command
    #[options(help = "print help message")]
    pub help: bool,

    /// Increase verbosity setting
    #[options(help = "be verbose")]
    pub verbose: bool,

    /// Subcommand to execute.
    ///
    /// The `command` option will delegate option parsing to the command type,
    /// starting at the first free argument.
    #[options(command)]
    pub command: Option<Cmd>,
}

impl<Cmd> EntryPoint<Cmd>
where
    Cmd: Callable + Command,
{
    /// Borrow the underlying command type or print usage info and exit
    fn command(&self) -> &Cmd {
        self.command
            .as_ref()
            .unwrap_or_else(|| Cmd::print_usage(&[]))
    }
}

impl<Cmd> Callable for EntryPoint<Cmd>
where
    Cmd: Callable + Command,
{
    fn call(&self) {
        self.command().call()
    }
}

impl<Cmd> Command for EntryPoint<Cmd>
where
    Cmd: Callable + Command,
{
    /// Name of this program as a string
    fn name() -> &'static str {
        Cmd::name()
    }

    /// Description of this program
    fn description() -> &'static str {
        Cmd::description()
    }

    /// Version of this program
    fn version() -> &'static str {
        Cmd::version()
    }

    /// Authors of this program
    fn authors() -> &'static str {
        Cmd::authors()
    }
}

impl<Cfg, Cmd> config::Loader<Cfg> for EntryPoint<Cmd>
where
    Cmd: Callable + Command + config::Loader<Cfg>,
    Cfg: config::Config,
{
    /// Path to the command's configuration file. Returns an error by default.
    fn config_path(&self) -> Option<PathBuf> {
        self.command.as_ref().and_then(config::Loader::config_path)
    }
}
