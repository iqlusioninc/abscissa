//! Originally from the `gumdrop` crate. See:
//! <https://github.com/murarth/gumdrop/blob/master/examples/commands.rs>

#[macro_use]
extern crate abscissa;
#[macro_use]
extern crate abscissa_derive;

use abscissa::{Callable, Command};

/// Define options for the program.
#[derive(Debug, Options)]
struct MyOptions {
    /// Options here can be accepted with any command (or none at all),
    /// but they must come before the command name.
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "be verbose")]
    verbose: bool,

    /// The `command` option will delegate option parsing to the command type,
    /// starting at the first free argument.
    #[options(command)]
    command: Option<MyCommand>,
}

// Implement abscissa's methods for a command-line option
// TODO: proc macro for this i.e. `derive(Command)`
impl_command!(MyOptions);

/// Anything we run `impl_command!` on must also impl `abscissa::Callable`
impl Callable for MyOptions {
    fn call(&self) {
        if let Some(ref command) = self.command {
            command.call()
        } else {
            Self::print_usage(&[]);
        }
    }
}

/// The set of commands and the options each one accepts.
///
/// Each variant of a command enum should be a unary tuple variant with only
/// one field. This field must implement `Options` and is used to parse arguments
/// that are given after the command name.
#[derive(Debug, Options)]
enum MyCommand {
    /// Command names are generated from variant names.
    /// By default, a CamelCase name will be converted into a lowercase,
    /// hyphen-separated name; e.g. `FooBar` becomes `foo-bar`.
    ///
    /// Names can be explicitly specified using `#[options(name = "...")]`
    #[options(help = "make stuff")]
    Make(MakeCommand),

    #[options(help = "install stuff")]
    Install(InstallCommand),
}

// TODO: `derive(Callable)` which can derive this for enums
impl Callable for MyCommand {
    fn call(&self) {
        match self {
            MyCommand::Make(make) => make.call(),
            MyCommand::Install(install) => install.call(),
        }
    }
}

/// Options accepted for the `make` command
#[derive(Debug, Options)]
struct MakeCommand {
    #[options(help = "print help message")]
    help: bool,

    #[options(free)]
    free: Vec<String>,

    #[options(help = "number of jobs", meta = "N")]
    jobs: Option<u32>,
}

impl Callable for MakeCommand {
    fn call(&self) {
        println!("*** performing make: {:?}", self.free);
    }
}

/// Options accepted for the `install` command
#[derive(Debug, Options)]
struct InstallCommand {
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "target directory")]
    dir: Option<String>,
}

impl Callable for InstallCommand {
    fn call(&self) {
        println!(
            "*** performing install: {:?}",
            self.dir.as_ref().ok_or("(none)")
        );
    }
}

fn main() {
    // Parse options from the environment.
    // If there's an error or the user requests help,
    // the process will exit after giving the appropriate response.
    let opts = MyOptions::from_env_args();
    println!("{:#?}", opts);
}
