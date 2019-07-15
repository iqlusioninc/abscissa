//! Help command

use super::Command;
use crate::runnable::Runnable;
use gumdrop::{Error, Opt, Options, Parser};
use std::marker::PhantomData;

/// Help command which prints usage information.
///
/// Generic over `Command` types.
#[derive(Debug, Default)]
pub struct Help<C: Command> {
    /// Obtain help for a particular subcommand
    pub opts: Vec<String>,

    /// Command for which to obtain usage information
    command: PhantomData<C>,
}

impl<C> Command for Help<C>
where
    C: Command,
{
    fn name() -> &'static str {
        C::name()
    }

    fn description() -> &'static str {
        C::description()
    }

    fn version() -> &'static str {
        C::version()
    }

    fn authors() -> &'static str {
        C::authors()
    }
}

impl<C> Options for Help<C>
where
    C: Command,
{
    fn parse<S: AsRef<str>>(parser: &mut Parser<S>) -> Result<Self, Error> {
        let mut opts = vec![];

        while let Some(opt) = parser.next_opt() {
            match opt {
                Opt::Free(free_opt) => opts.push(free_opt.to_owned()),
                _ => return Err(Error::unexpected_argument(opt)),
            }
        }

        Ok(Self {
            opts,
            command: PhantomData,
        })
    }

    fn parse_command<S: AsRef<str>>(_name: &str, parser: &mut Parser<S>) -> Result<Self, Error> {
        // TODO(tarcieri): is this necessary or the best approach?
        Self::parse(parser)
    }

    fn usage() -> &'static str {
        ""
    }

    fn command_usage(_command: &str) -> Option<&'static str> {
        None
    }

    fn command_list() -> Option<&'static str> {
        None
    }
}

impl<C> Runnable for Help<C>
where
    C: Command,
{
    /// Print help information for the given command
    fn run(&self) {
        C::print_usage_and_exit(&self.opts);
    }
}
