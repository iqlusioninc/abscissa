//! Application (sub)command(s), i.e. app entry points

#[doc(hidden)]
pub use abscissa_derive::Command;

use crate::{runnable::Runnable, terminal};
use clap::Parser;
use std::{env, fmt::Debug};
use termcolor::ColorChoice;

/// Subcommand of an application: derives or otherwise implements the `Options`
/// trait, but also has a `run()` method which can be used to invoke the given
/// (sub)command.
pub trait Command: Debug + Parser + Runnable {
    /// Name of this program as a string
    fn name() -> &'static str;

    /// Description of this program
    fn description() -> &'static str;

    /// Authors of this program
    fn authors() -> &'static str;

    /// Parse command-line arguments from a string iterator
    fn parse_args<A: IntoIterator<Item = String>>(into_args: A) -> Self {
        let args: Vec<_> = into_args.into_iter().collect();

        Parser::try_parse_from(args.as_slice()).unwrap_or_else(|err| {
            terminal::init(ColorChoice::Auto);
            err.exit()
        })
    }

    /// Parse command-line arguments from the environment
    fn parse_env_args() -> Self {
        Self::parse_args(env::args())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Command, Runnable};
    use clap::Parser;

    #[derive(Command, Debug, Parser)]
    pub struct DummyCommand {}

    impl Runnable for DummyCommand {
        fn run(&self) {
            panic!("unimplemented");
        }
    }

    #[test]
    fn derived_command_test() {
        assert_eq!(DummyCommand::name(), "abscissa_core");
    }
}
