//! Application (sub)command(s), i.e. app entry points

use std::process::exit;

use options::Options;

/// Something which can be called
pub trait Callable {
    /// Call this callable (i.e. command), running its behavior
    fn call(&self);
}

/// Subcommand of an application: derives or otherwise implements the `Options`
/// trait, but also has a `call()` method which can be used to invoke the given
/// (sub)command.
// TODO: custom derive support, i.e. `derive(Command)`
pub trait Command: Callable + Options {
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

        if args.len() == 1 {
            // TODO: hax! This can be replaced with `#[options(command)]`
            match args[0].as_ref() {
                "-h" | "--help" => {
                    Self::print_usage(&[]);
                }
                "-V" | "--version" => {
                    Self::print_package_info();
                    exit(0);
                }
                _ => (),
            }
        }

        Self::parse_args_default(args.as_slice()).unwrap_or_else(|e| {
            match e.to_string().as_ref() {
                // Show usage if no command name is given or if "help" is given
                // TODO: don't gate on a string, handle the error properly
                "missing command name" => Self::print_usage(&[]),
                string => eprintln!("{}: {}", args[0], string),
            }

            exit(2);
        })
    }

    /// Parse command-line arguments from the environment
    fn from_env_args() -> Self {
        let mut args = ::std::env::args();
        assert!(args.next().is_some(), "expected one argument but got zero");
        Self::from_args(args)
    }

    //
    // TODO: the methods below should probably be factored into `Option`
    //

    /// Print package name and version
    fn print_package_info() {
        println!("{} {}", Self::name(), Self::version());
    }

    /// Print the authors of the current package
    fn print_package_authors() {
        println!(
            "{}",
            Self::authors().split(':').collect::<Vec<_>>().join(", ")
        );
    }

    /// Print usage information for the given command to STDOUT and then exit with
    /// a usage status code (i.e. `2`).
    fn print_usage(args: &[String]) {
        Self::print_package_info();
        Self::print_package_authors();

        if args.len() == 1 {
            Self::print_subcommand_usage(&args[0]);
        }

        println!("{}", Self::description());
        println!();
        println!("USAGE:");
        println!("  {} <SUBCOMMAND>", Self::name());
        println!();
        println!("FLAGS:");
        println!("  -h, --help     Prints help information");
        println!("  -V, --version  Prints version information");
        println!();
        println!("SUBCOMMANDS:");
        println!("{}", Self::command_list().unwrap());

        exit(2);
    }

    /// Print subcommand usage
    // TODO: less hax way of doing this
    fn print_subcommand_usage(subcommand: &str) {
        Self::print_subcommand_description(subcommand);
        println!();
        println!("USAGE:");
        println!("  {} {} [OPTIONS]", Self::name(), subcommand);
        println!();
        Self::print_subcommand_flags(subcommand);

        exit(2);
    }

    /// Print a description for a subcommand
    // TODO: less hax way of doing this
    fn print_subcommand_description(subcommand: &str) {
        for command_info in Self::command_list().unwrap().split('\n') {
            let mut command_info_parts = command_info.split_whitespace();

            if subcommand != command_info_parts.next().unwrap() {
                continue;
            }

            let command_description: Vec<_> = command_info_parts.collect();
            println!("{}", command_description.join(" "));
        }
    }

    /// Print flags for a subcommand
    // TODO: less hax way of doing this
    fn print_subcommand_flags(subcommand: &str) {
        let usage = Self::command_usage(subcommand)
            .unwrap()
            .replace("Optional arguments:", "OPTIONS:");

        // TODO: descriptions for each command
        println!("{}", usage);
    }
}
