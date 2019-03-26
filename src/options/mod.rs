//! Option parser with custom derive support
//!
//! # Examples
//!
//! ```
//! extern crate abscissa;
//! #[macro_use] extern crate abscissa_derive;
//!
//! use abscissa::Options;
//!
//! // Defines options that can be parsed from the command line.
//! //
//! // `derive(Options)` will generate an implementation of the trait `Options`.
//! // Each field must either have a `Default` implementation or an inline
//! // default value provided.
//! //
//! // (`Debug` is only derived here for demonstration purposes.)
//! #[derive(Debug, Options)]
//! struct MyOptions {
//!     // Contains "free" arguments -- those that are not options.
//!     // If no `free` field is declared, free arguments will result in an error.
//!     #[options(free)]
//!     free: Vec<String>,
//!
//!     // Boolean options are treated as flags, taking no additional values.
//!     // The optional `help` attribute is displayed in `usage` text.
//!     #[options(help = "print help message")]
//!     help: bool,
//!
//!     // Non-boolean fields will take a value from the command line.
//!     // Wrapping the type in an `Option` is not necessary, but provides clarity.
//!     #[options(help = "give a string argument")]
//!     string: Option<String>,
//!
//!     // A field can be any type that implements `FromStr`.
//!     // The optional `meta` attribute is displayed in `usage` text.
//!     #[options(help = "give a number as an argument", meta = "N")]
//!     number: Option<i32>,
//!
//!     // A `Vec` field will accumulate all values received from the command line.
//!     #[options(help = "give a list of string items")]
//!     item: Vec<String>,
//!
//!     // The `count` flag will treat the option as a counter.
//!     // Each time the option is encountered, the field is incremented.
//!     #[options(count, help = "increase a counting value")]
//!     count: u32,
//!
//!     // Option names are automatically generated from field names, but these
//!     // can be overriden. The attributes `short = "?"`, `long = "..."`,
//!     // `no_short`, and `no_long` are used to control option names.
//!     #[options(no_short, help = "this option has no short form")]
//!     long_option_only: bool,
//! }
//!
//! fn main() {
//!     let opts = MyOptions::parse_args_default_or_exit();
//!
//!     println!("{:#?}", opts);
//! }
//! ```
//!
//! `derive(Options)` can also be used on `enum`s to produce a subcommand
//! option parser.
//!
//! ```
//! extern crate abscissa;
//! #[macro_use] extern crate abscissa_derive;
//!
//! use abscissa::Options;
//!
//! // Define options for the program.
//! #[derive(Debug, Options)]
//! struct MyOptions {
//!     // Options here can be accepted with any command (or none at all),
//!     // but they must come before the command name.
//!     #[options(help = "print help message")]
//!     help: bool,
//!     #[options(help = "be verbose")]
//!     verbose: bool,
//!
//!     // The `command` option will delegate option parsing to the command type,
//!     // starting at the first free argument.
//!     #[options(command)]
//!     command: Option<Command>,
//! }
//!
//! // The set of commands and the options each one accepts.
//! //
//! // Each variant of a command enum should be a unary tuple variant with only
//! // one field. This field must implement `Options` and is used to parse arguments
//! // that are given after the command name.
//! #[derive(Debug, Options)]
//! enum Command {
//!     // Command names are generated from variant names.
//!     // By default, a CamelCase name will be converted into a lowercase,
//!     // hyphen-separated name; e.g. `FooBar` becomes `foo-bar`.
//!     //
//!     // Names can be explicitly specified using `#[options(name = "...")]`
//!     #[options(help = "show help for a command")]
//!     Help(HelpOpts),
//!     #[options(help = "make stuff")]
//!     Make(MakeOpts),
//!     #[options(help = "install stuff")]
//!     Install(InstallOpts),
//! }
//!
//! // Options accepted for the `help` command
//! #[derive(Debug, Options)]
//! struct HelpOpts {
//!     #[options(free)]
//!     free: Vec<String>,
//! }
//!
//! // Options accepted for the `make` command
//! #[derive(Debug, Options)]
//! struct MakeOpts {
//!     #[options(free)]
//!     free: Vec<String>,
//!     #[options(help = "number of jobs", meta = "N")]
//!     jobs: Option<u32>,
//! }
//!
//! // Options accepted for the `install` command
//! #[derive(Debug, Options)]
//! struct InstallOpts {
//!     #[options(help = "target directory")]
//!     dir: Option<String>,
//! }
//!
//! fn main() {
//!     let opts = MyOptions::parse_args_default_or_exit();
//!
//!     println!("{:#?}", opts);
//! }
//! ```
//!
//! A custom parsing function can be supplied for each option field.
//!
//! ```
//! extern crate abscissa;
//! #[macro_use] extern crate abscissa_derive;
//!
//! use abscissa::Options;
//!
//! #[derive(Debug, Options)]
//! struct MyOptions {
//!     // `try_from_str = "..."` supplies a conversion function that may fail
//!     #[options(help = "a hexadecimal value", parse(try_from_str = "parse_hex"))]
//!     hex: u32,
//!     // `from_str = "..."` supplies a conversion function that always succeeds
//!     #[options(help = "a string that becomes uppercase", parse(from_str = "to_upper"))]
//!     upper: String,
//! }
//!
//! fn parse_hex(s: &str) -> Result<u32, std::num::ParseIntError> {
//!     u32::from_str_radix(s, 16)
//! }
//!
//! fn to_upper(s: &str) -> String {
//!     s.to_uppercase()
//! }
//!
//! fn main() {
//!     let opts = MyOptions::parse_args_default_or_exit();
//!
//!     println!("{:#?}", opts);
//! }
//! ```
//!
//! # Notice
//!
//! This portion of `abscissa` functionality is a fork of the `gumdrop` crate:
//!
//! <https://github.com/murarth/gumdrop>
//!
//! Author: Murarth <murarth@gmail.com>

#[doc(hidden)]
pub use abscissa_derive::*;

mod error;
mod opt;
mod parser;

pub use self::error::Error;
pub use self::opt::Opt;
pub use self::parser::{Parser, ParsingStyle};

/// Parses arguments from the command line.
///
/// The first argument (the program name) should be omitted.
pub fn parse_args<T: Options>(args: &[String], style: ParsingStyle) -> Result<T, Error> {
    T::parse_args(args, style)
}

/// Parses arguments from the command line using the default parsing style.
///
/// The first argument (the program name) should be omitted.
pub fn parse_args_default<T: Options>(args: &[String]) -> Result<T, Error> {
    T::parse_args_default(args)
}

/// Parses arguments from the environment.
///
/// If an error is encountered, the error is printed to `stderr` and the
/// process will exit with status code `2`.
///
/// If the user supplies a help option, option usage will be printed to
/// `stdout` and the process will exit with status code `0`.
///
/// Otherwise, the parsed options are returned.
pub fn parse_args_or_exit<T: Options>(style: ParsingStyle) -> T {
    T::parse_args_or_exit(style)
}

/// Parses arguments from the environment, using the default parsing style.
///
/// If an error is encountered, the error is printed to `stderr` and the
/// process will exit with status code `2`.
///
/// If the user supplies a help option, option usage will be printed to
/// `stdout` and the process will exit with status code `0`.
///
/// Otherwise, the parsed options are returned.
pub fn parse_args_default_or_exit<T: Options>() -> T {
    T::parse_args_default_or_exit()
}

/// Implements a set of options parsed from command line arguments.
///
/// An implementation of this trait can be generated with `#[derive(Options)]`.
pub trait Options: Sized {
    /// Parses arguments until the given parser is exhausted or until
    /// an error is encountered.
    fn parse<S: AsRef<str>>(parser: &mut Parser<'_, S>) -> Result<Self, Error>;

    /// Returns the name of a parsed command, if present.
    ///
    /// This is implemented by `derive(Options)` in one of two ways:
    ///
    /// * For `struct` types, if the type contains a field marked
    ///   `#[options(command)]`, this method is called on that value.
    ///   Otherwise, `None` is returned.
    /// * For `enum` types, the name corresponding to the variant is returned.
    fn command_name(&self) -> Option<&'static str> {
        None
    }

    /// Returns whether the user supplied a "help" option to request
    /// usage information about the program or any contained subcommands.
    ///
    /// The default implementation returns `false`.
    fn help_requested(&self) -> bool {
        false
    }

    /// Parses arguments received from the command line.
    ///
    /// The first argument (the program name) should be omitted.
    fn parse_args<S: AsRef<str>>(args: &[S], style: ParsingStyle) -> Result<Self, Error> {
        Self::parse(&mut Parser::new(args, style))
    }

    /// Parses arguments from the environment.
    ///
    /// If an error is encountered, the error is printed to `stderr` and the
    /// process will exit with status code `2`.
    ///
    /// If the user supplies a help option, option usage will be printed to
    /// `stdout` and the process will exit with status code `0`.
    ///
    /// Otherwise, the parsed options are returned.
    fn parse_args_or_exit(style: ParsingStyle) -> Self {
        use std::env::args;
        use std::process::exit;

        let args = args().collect::<Vec<_>>();

        let opts = Self::parse_args(&args[1..], style).unwrap_or_else(|e| {
            eprintln!("{}: {}", args[0], e);
            exit(2);
        });

        if opts.help_requested() {
            match opts.command_name() {
                None => {
                    println!("Usage: {} [OPTIONS]", args[0]);
                    println!();
                    println!("{}", Self::usage());

                    if let Some(cmds) = Self::command_list() {
                        println!();
                        println!("Available commands:");
                        println!();
                        println!("{}", cmds);
                    }
                }
                Some(cmd) => {
                    let help = Self::command_usage(cmd).unwrap_or_default();

                    println!("Usage: {} {} [OPTIONS]", args[0], cmd);
                    println!();
                    println!("{}", help);
                }
            }
            exit(0);
        }

        opts
    }

    /// Parses arguments from the environment, using the default parsing style.
    ///
    /// If an error is encountered, the error is printed to `stderr` and the
    /// process will exit with status code `2`.
    ///
    /// If the user supplies a help option, option usage will be printed to
    /// `stdout` and the process will exit with status code `0`.
    ///
    /// Otherwise, the parsed options are returned.
    fn parse_args_default_or_exit() -> Self {
        Self::parse_args_or_exit(ParsingStyle::default())
    }

    /// Parses arguments received from the command line,
    /// using the default parsing style.
    ///
    /// The first argument (the program name) should be omitted.
    fn parse_args_default<S: AsRef<str>>(args: &[S]) -> Result<Self, Error> {
        Self::parse(&mut Parser::new(args, ParsingStyle::default()))
    }

    /// Parses options for the named command.
    fn parse_command<S: AsRef<str>>(name: &str, parser: &mut Parser<'_, S>) -> Result<Self, Error>;

    /// Returns a string showing usage and help for each supported option.
    ///
    /// Option descriptions are separated by newlines. The returned string
    /// should **not** end with a newline.
    fn usage() -> &'static str;

    /// Returns a usage string for the named command.
    ///
    /// If the named command does not exist, `None` is returned.
    ///
    /// Command descriptions are separated by newlines. The returned string
    /// should **not** end with a newline.
    fn command_usage(command: &str) -> Option<&'static str>;

    /// Returns a string listing available commands and help text.
    ///
    /// Commands are separated by newlines. The string should **not** end with
    /// a newline.
    ///
    /// For `enum` types with `derive(Options)`, this is the same as `usage`.
    ///
    /// For `struct` types containing a field marked `#[options(command)]`,
    /// `usage` is called on the command type.
    fn command_list() -> Option<&'static str>;
}

#[cfg(test)]
mod test {
    use super::{Opt, Parser, ParsingStyle};

    #[test]
    fn test_parser() {
        let args = &[
            "-a",
            "b",
            "-cde",
            "arg",
            "-xfoo",
            "--long",
            "--opt=val",
            "--",
            "y",
            "-z",
        ];

        let mut p = Parser::new(args, ParsingStyle::AllOptions);

        assert_matches!(p.next_opt(), Some(Opt::Short('a')));
        assert_matches!(p.next_opt(), Some(Opt::Free("b")));
        assert_matches!(p.next_opt(), Some(Opt::Short('c')));
        assert_matches!(p.next_opt(), Some(Opt::Short('d')));
        assert_matches!(p.next_opt(), Some(Opt::Short('e')));
        assert_matches!(p.next_arg(), Some("arg"));
        assert_matches!(p.next_opt(), Some(Opt::Short('x')));
        assert_matches!(p.next_arg(), Some("foo"));
        assert_matches!(p.next_opt(), Some(Opt::Long("long")));
        assert_matches!(p.next_opt(), Some(Opt::LongWithArg("opt", "val")));
        assert_matches!(p.next_opt(), Some(Opt::Free("y")));
        assert_matches!(p.next_opt(), Some(Opt::Free("-z")));
        assert_matches!(p.next_opt(), None);
    }

    #[test]
    fn test_parsing_style() {
        let args = &["-a", "b", "-c", "--d"];

        let mut p = Parser::new(args, ParsingStyle::AllOptions);

        assert_matches!(p.next_opt(), Some(Opt::Short('a')));
        assert_matches!(p.next_opt(), Some(Opt::Free("b")));
        assert_matches!(p.next_opt(), Some(Opt::Short('c')));
        assert_matches!(p.next_opt(), Some(Opt::Long("d")));
        assert_matches!(p.next_opt(), None);

        let mut p = Parser::new(args, ParsingStyle::StopAtFirstFree);

        assert_matches!(p.next_opt(), Some(Opt::Short('a')));
        assert_matches!(p.next_opt(), Some(Opt::Free("b")));
        assert_matches!(p.next_opt(), Some(Opt::Free("-c")));
        assert_matches!(p.next_opt(), Some(Opt::Free("--d")));
        assert_matches!(p.next_opt(), None);
    }
}
