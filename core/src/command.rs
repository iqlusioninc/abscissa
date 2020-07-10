//! Application (sub)command(s), i.e. app entry points

mod entrypoint;
mod help;
mod usage;

pub use self::{entrypoint::EntryPoint, help::Help, usage::Usage};
#[doc(hidden)]
pub use abscissa_derive::Command;

use crate::{runnable::Runnable, terminal};
use gumdrop::Options;
use std::fmt::Debug;
use termcolor::ColorChoice;

/// Subcommand of an application: derives or otherwise implements the `Options`
/// trait, but also has a `run()` method which can be used to invoke the given
/// (sub)command.
pub trait Command: Debug + Options + Runnable {
    /// Name of this program as a string
    fn name() -> &'static str;

    /// Description of this program
    fn description() -> &'static str;

    /// Version of this program
    fn version() -> &'static str;

    /// Authors of this program
    fn authors() -> &'static str;

    /// Parse command-line arguments from a string iterator
    fn from_args<A: IntoIterator<Item = String>>(into_args: A) -> Self {
        let args: Vec<_> = into_args.into_iter().collect();

        Self::parse_args_default(args.as_slice()).unwrap_or_else(|err| {
            terminal::init(ColorChoice::Auto);
            Usage::for_command::<Self>().print_error_and_exit(err, args.as_slice());
        })
    }

    /// Parse command-line arguments from the environment
    fn from_env_args() -> Self {
        let mut args = ::std::env::args();
        assert!(args.next().is_some(), "expected one argument but got zero");
        Self::from_args(args)
    }

    /// Print usage information and exit
    fn print_usage_and_exit(args: &[String]) -> ! {
        Usage::for_command::<Self>().print_subcommand_and_exit(args);
    }

    /// Get usage information for a particular subcommand (if available)
    fn subcommand_usage(_command: &str) -> Option<Usage> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{Command, Options, Runnable};

    #[derive(Command, Debug, Options)]
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
