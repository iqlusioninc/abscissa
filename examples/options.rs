//! Originally from the `gumdrop` crate. See:
//! <https://github.com/murarth/gumdrop/blob/master/examples/options.rs>

extern crate abscissa;
#[macro_use]
extern crate abscissa_derive;

use abscissa::Options;

/// Defines options that can be parsed from the command line.
///
/// `derive(Options)` will generate an implementation of the trait `Options`.
/// Each field must either have a `Default` implementation or an inline
/// default value provided.
///
/// (`Debug` is only derived here for demonstration purposes.)
#[derive(Debug, Options)]
struct MyOptions {
    /// Contains "free" arguments -- those that are not options.
    /// If no `free` field is declared, free arguments will result in an error.
    #[options(free)]
    free: Vec<String>,

    /// Boolean options are treated as flags, taking no additional values.
    /// The optional `help` attribute is displayed in `usage` text.
    #[options(help = "print help message")]
    help: bool,

    /// Non-boolean fields will take a value from the command line.
    /// Wrapping the type in an `Option` is not necessary, but provides clarity.
    #[options(help = "give a string argument")]
    string: Option<String>,

    /// A field can be any type that implements `FromStr`.
    /// The optional `meta` attribute is displayed in `usage` text.
    #[options(help = "give a number as an argument", meta = "N")]
    number: Option<i32>,

    /// A `Vec` field will accumulate all values received from the command line.
    #[options(help = "give a list of string items")]
    item: Vec<String>,

    /// The `count` flag will treat the option as a counter.
    /// Each time the option is encountered, the field is incremented.
    #[options(count, help = "increase a counting value")]
    count: u32,

    /// Option names are automatically generated from field names, but these
    /// can be overriden. The attributes `short = "?"`, `long = "..."`,
    /// `no_short`, and `no_long` are used to control option names.
    #[options(no_short, help = "this option has no short form")]
    long_option_only: bool,
}

fn main() {
    // Parse options from the environment.
    // If there's an error or the user requests help,
    // the process will exit after giving the appropriate response.
    let opts = MyOptions::parse_args_default_or_exit();

    println!("{:#?}", opts);
}
