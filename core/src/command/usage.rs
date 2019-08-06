//! Usage information presenter

use super::Command;
use crate::{terminal::stream::STDOUT, Version};
use std::{
    io::{self, Write},
    process,
};
use termcolor::{Color, ColorSpec, WriteColor};

/// Presenter for usage information for a particular `Command`
#[derive(Debug)]
pub struct Usage {
    /// Package name
    pub package_name: String,

    /// Package version
    pub package_version: Version,

    /// Package authors
    pub package_authors: Vec<String>,

    /// Package description
    pub package_description: Option<String>,

    /// Command-line options
    pub args: Vec<Argument>,

    /// Subcommands
    pub subcommands: Vec<Subcommand>,
}

impl Usage {
    /// Build usage information for a particular command
    pub fn for_command<C>() -> Self
    where
        C: Command,
    {
        let package_name = C::name().to_owned();
        let package_version = Version::parse(C::version()).expect("invalid version");
        let package_authors = C::authors().split(':').map(String::from).collect();

        let package_description = C::description()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

        let args = C::usage()
            .split('\n')
            .filter_map(Argument::parse_usage)
            .collect();

        let subcommands = C::command_list()
            .map(|command_list| {
                command_list
                    .split('\n')
                    .map(|usage| Subcommand::parse_usage::<C>(usage))
                    .collect()
            })
            .unwrap_or_else(|| vec![]);

        Self {
            package_name,
            package_version,
            package_authors,
            package_description: if package_description.is_empty() {
                None
            } else {
                Some(package_description)
            },
            args,
            subcommands,
        }
    }

    /// Print usage for a particular subcommand
    pub fn print_subcommand(&self, args: &[String]) -> Result<(), io::Error> {
        let mut stdout = STDOUT.lock();
        stdout.reset()?;

        let mut recognized = vec![];
        let mut command = self;
        let mut description = None;

        for arg in args {
            match command.subcommands.iter().find(|cmd| &cmd.name == arg) {
                Some(subcommand) => {
                    recognized.push(arg.clone());
                    description = Some(&subcommand.description);
                    command = &subcommand.usage;
                }
                None => {
                    command.print_unrecognized_command(arg, &recognized)?;

                    // Force error status exit code
                    return Err(io::ErrorKind::NotFound.into());
                }
            }
        }

        command.print_info()?;

        let mut bold = ColorSpec::new();
        bold.set_bold(true);

        stdout.set_color(&bold)?;
        writeln!(stdout, "USAGE:")?;
        stdout.reset()?;

        let mut usage_items = vec![self.package_name.clone()];
        usage_items.extend(recognized.iter().map(|s| s.to_string()));
        let usage_string = usage_items.join(" ");

        if command.subcommands.is_empty() {
            writeln!(stdout, "    {} <OPTIONS>", &usage_string)?;
        } else {
            writeln!(stdout, "    {} <SUBCOMMAND>", &usage_string)?;
        }

        writeln!(stdout)?;

        if let Some(desc) = description {
            stdout.set_color(&bold)?;
            writeln!(stdout, "DESCRIPTION:")?;
            stdout.reset()?;

            writeln!(stdout, "    {}", desc)?;
            writeln!(stdout)?;
        }

        command.print_usage()
    }

    /// Print usage for a particular subcommand and exit
    pub fn print_subcommand_and_exit(&self, args: &[String]) -> ! {
        let exit_code = match self.print_subcommand(args) {
            Ok(_) => 0,
            Err(_) => 1,
        };

        process::exit(exit_code);
    }

    /// Print information about a usage error
    pub(super) fn print_error_and_exit(&self, err: gumdrop::Error, args: &[String]) -> ! {
        // TODO(tarcieri): better personalize errors based on args
        if args.is_empty() {
            self.print_info().unwrap();
            self.print_usage().unwrap();
            process::exit(0);
        }

        let mut command = self;
        let mut description = None;

        for arg in args {
            if let Some(sub) = command.subcommands.iter().find(|cmd| &cmd.name == arg) {
                command = &sub.usage;
                description = Some(&sub.description);
            } else {
                break;
            }
        }

        let mut stdout = STDOUT.lock();
        stdout.reset().unwrap();

        let mut red = ColorSpec::new();
        red.set_fg(Some(Color::Red));
        red.set_bold(true);

        stdout.set_color(&red).unwrap();
        write!(stdout, "error: ").unwrap();
        stdout.reset().unwrap();

        writeln!(stdout, "{}", err).unwrap();
        writeln!(stdout).unwrap();

        command.print_info().unwrap();

        if let Some(desc) = description {
            let mut bold = ColorSpec::new();
            bold.set_bold(true);

            stdout.set_color(&bold).unwrap();
            writeln!(stdout, "DESCRIPTION:").unwrap();
            stdout.reset().unwrap();

            writeln!(stdout, "    {}", desc).unwrap();
            writeln!(stdout).unwrap();
        }

        command.print_usage().unwrap();
        process::exit(1);
    }

    /// Print information about an unrecognized command
    fn print_unrecognized_command(
        &self,
        unrecognized: &str,
        recognized: &[String],
    ) -> Result<(), io::Error> {
        let mut stdout = STDOUT.lock();
        stdout.reset().unwrap();

        let mut unrecognized_items = recognized.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        unrecognized_items.push(unrecognized.to_owned());

        let unrecognized_string = unrecognized_items.join(" ");

        let mut red = ColorSpec::new();
        red.set_fg(Some(Color::Red));
        red.set_bold(true);

        let mut yellow = ColorSpec::new();
        yellow.set_fg(Some(Color::Yellow));
        yellow.set_bold(true);

        let mut green = ColorSpec::new();
        green.set_fg(Some(Color::Green));
        green.set_bold(true);

        stdout.set_color(&red)?;
        write!(stdout, "error: ")?;
        stdout.reset()?;

        write!(stdout, "The subcommand ")?;

        stdout.set_color(&yellow)?;
        write!(stdout, "{:?} ", &unrecognized_string)?;
        stdout.reset()?;

        writeln!(stdout, "wasn't recognized.")?;
        writeln!(stdout)?;

        let mut bold = ColorSpec::new();
        bold.set_bold(true);

        stdout.set_color(&yellow)?;
        writeln!(stdout, "USAGE:")?;
        stdout.reset()?;

        if self.subcommands.is_empty() {
            writeln!(stdout, "    {} <OPTIONS>", recognized.join(" "))?;
        } else {
            writeln!(stdout, "    {} <SUBCOMMAND>", recognized.join(" "))?;
        }

        writeln!(stdout)?;
        self.print_usage()
    }

    /// Print program and usage information
    pub fn print_info(&self) -> Result<(), io::Error> {
        let mut stdout = STDOUT.lock();
        stdout.reset()?;

        let mut bold = ColorSpec::new();
        bold.set_bold(true);

        stdout.set_color(&bold)?;
        writeln!(stdout, "{} {}", &self.package_name, &self.package_version)?;
        stdout.reset()?;

        if !self.package_authors.is_empty() {
            writeln!(stdout, "{}", self.package_authors.join(", "))?;
        }

        if let Some(ref description) = self.package_description {
            writeln!(stdout, "{}", description)?;
        }

        writeln!(stdout)?;
        Ok(())
    }

    /// Print usage information only
    pub fn print_usage(&self) -> Result<(), io::Error> {
        let mut stdout = STDOUT.lock();

        let mut bold = ColorSpec::new();
        bold.set_bold(true);

        if !self.args.is_empty() {
            stdout.set_color(&bold)?;
            writeln!(stdout, "FLAGS:")?;
            stdout.reset()?;

            for arg in &self.args {
                arg.print(&mut stdout)?;
            }

            writeln!(stdout)?;
        }

        if !self.subcommands.is_empty() {
            stdout.set_color(&bold)?;
            writeln!(stdout, "SUBCOMMANDS:")?;
            stdout.reset()?;

            for command in &self.subcommands {
                command.print_brief(&mut stdout)?;
            }
        }

        Ok(())
    }
}

/// Presenter for flags/options
#[derive(Debug)]
pub struct Argument {
    /// Short name (one char)
    pub short: Option<char>,

    /// Long name
    pub long: Option<String>,

    /// Long param
    pub long_param: Option<String>,

    /// Description
    pub description: Option<String>,
}

impl Argument {
    /// Parse flags from `gumdrop` usage string.
    // TODO(tarcieri): less hacky approach
    fn parse_usage(usage: &str) -> Option<Self> {
        let words = usage.split_whitespace().collect::<Vec<_>>();

        if words.is_empty() {
            return None;
        }

        let mut arg = Self {
            short: None,
            long: None,
            long_param: None,
            description: None,
        };

        if words[0].starts_with('-') && !words[0].starts_with("--") {
            arg.short = Some(words[0].chars().nth(1).expect("truncated short arg"));
            arg.parse_long_arg(&words[1..]);
        } else {
            arg.parse_long_arg(&words);
        }

        if arg.short.is_some() || arg.long.is_some() {
            Some(arg)
        } else {
            None
        }
    }

    /// Parse the long form argument and description
    fn parse_long_arg(&mut self, usage: &[&str]) {
        if usage.is_empty() {
            return;
        }

        let word = usage[0];

        if word.starts_with("--") {
            if usage.len() < 2 {
                return;
            }

            self.long = Some(word[2..].to_owned());

            if usage[1].chars().all(|c| c.is_uppercase() || c == '-') {
                self.long_param = Some(usage[1].to_owned());
                self.parse_description(&usage[2..]);
            } else {
                self.parse_description(&usage[1..])
            }
        } else {
            self.parse_description(usage)
        }
    }

    /// Parse description
    fn parse_description(&mut self, usage: &[&str]) {
        if !usage.is_empty() {
            self.description = Some(usage.join(" "));
        }
    }

    /// Print the argument to the given I/O stream
    fn print(&self, stream: &mut impl Write) -> Result<(), io::Error> {
        let mut arg_str = String::new();

        if let Some(short) = self.short {
            arg_str.push('-');
            arg_str.push(short);

            if self.long.is_some() {
                arg_str.push_str(", ");
            }
        }

        if let Some(ref long) = self.long {
            arg_str.push_str("--");
            arg_str.push_str(long);

            if let Some(ref param) = self.long_param {
                arg_str.push(' ');
                arg_str.push_str(param);
            }
        }

        let description = self.description.as_ref().map(String::as_str).unwrap_or("");
        writeln!(stream, "    {:<25} {}", &arg_str, description)
    }
}

/// Presenter for subcommands
#[derive(Debug)]
pub struct Subcommand {
    /// Subcommand name
    pub name: String,

    /// Subcommand description
    pub description: String,

    /// Subcommand usage
    pub usage: Usage,
}

impl Subcommand {
    /// Parse usage information for a particular subcommand
    // TODO(tarcieri): less hacky approach
    fn parse_usage<C>(usage_string: &str) -> Self
    where
        C: Command,
    {
        let words = usage_string.split_whitespace().collect::<Vec<_>>();
        let name = words[0].to_owned();
        let description = words[1..].join(" ");
        let usage = C::subcommand_usage(&name)
            .unwrap_or_else(|| panic!("error fetching usage for subcommand: {:?}", name));

        Self {
            name,
            description,
            usage,
        }
    }

    /// Print the subcommand to the given I/O stream
    fn print_brief(&self, stream: &mut impl Write) -> Result<(), io::Error> {
        writeln!(stream, "    {:<10} {}", &self.name, &self.description)
    }
}
