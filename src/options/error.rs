use super::Opt;
use crate::error;

/// Errors encountered during argument parsing
pub type Error = error::Error<ErrorKind>;

impl Error {
    /// Returns an error for a failed attempt at parsing an option value.
    pub fn failed_parse(opt: Opt<'_>, err: String) -> Error {
        ErrorKind::FailedParse {
            opt: opt.to_string(),
            err,
        }
        .into()
    }

    /// Returns an error for a failed attempt at parsing an option's default value.
    pub fn failed_parse_default(option: &'static str, value: &'static str, err: String) -> Error {
        ErrorKind::FailedParseDefault { option, value, err }.into()
    }

    /// Returns an error for an option expecting two or more arguments not
    /// receiving the expected number of arguments.
    pub fn insufficient_arguments(opt: Opt<'_>, expected: usize, found: usize) -> Error {
        ErrorKind::InsufficientArguments {
            option: opt.to_string(),
            expected,
            found,
        }
        .into()
    }

    /// Returns an error for an option receiving an unexpected argument value,
    /// e.g. `--option=value`.
    pub fn unexpected_argument(opt: Opt<'_>) -> Error {
        ErrorKind::UnexpectedArgument {
            opt: opt.to_string(),
        }
        .into()
    }

    /// Returns an error for an option expecting two or more argument values
    /// receiving only one in the long form, e.g. `--option=value`.
    ///
    /// These options must be passed as, e.g. `--option value second-value [...]`.
    pub fn unexpected_single_argument(opt: Opt<'_>, n: usize) -> Error {
        ErrorKind::UnexpectedSingleArgument {
            opt: opt.to_string(),
            n,
        }
        .into()
    }

    /// Returns an error for a missing required argument.
    pub fn missing_argument(opt: Opt<'_>) -> Error {
        ErrorKind::MissingArgument {
            opt: opt.to_string(),
        }
        .into()
    }

    /// Returns an error for a missing command name.
    pub fn missing_command() -> Error {
        ErrorKind::MissingCommand.into()
    }

    /// Returns an error for a missing required option.
    pub fn missing_required(opt: &str) -> Error {
        ErrorKind::MissingRequired {
            opt: opt.to_owned(),
        }
        .into()
    }

    /// Returns an error for a missing required command.
    pub fn missing_required_command() -> Error {
        ErrorKind::MissingRequiredCommand.into()
    }

    /// Returns an error for a missing required free argument.
    pub fn missing_required_free() -> Error {
        ErrorKind::MissingRequiredFree.into()
    }

    /// Returns an error when a free argument was encountered, but the options
    /// type does not support free arguments.
    pub fn unexpected_free(arg: &str) -> Error {
        ErrorKind::UnexpectedFree {
            arg: arg.to_owned(),
        }
        .into()
    }

    /// Returns an error for an unrecognized command.
    pub fn unrecognized_command(name: &str) -> Error {
        ErrorKind::UnrecognizedCommand {
            name: name.to_owned(),
        }
        .into()
    }

    /// Returns an error for an unrecognized option.
    pub fn unrecognized_option(opt: Opt<'_>) -> Error {
        match opt {
            Opt::Short(short) => Error::unrecognized_short(short),
            Opt::Long(long) | Opt::LongWithArg(long, _) => Error::unrecognized_long(long),
            Opt::Free(_) => panic!("`Error::unrecognized_option` called with `Opt::Free` value"),
        }
    }

    /// Returns an error for an unrecognized long option, e.g. `--option`.
    pub fn unrecognized_long(opt: &str) -> Error {
        ErrorKind::UnrecognizedLongOption {
            opt: opt.to_owned(),
        }
        .into()
    }

    /// Returns an error for an unrecognized short option, e.g. `-o`.
    pub fn unrecognized_short(opt: char) -> Error {
        ErrorKind::UnrecognizedShortOption { opt }.into()
    }
}

#[derive(Clone, Debug, Eq, Fail, PartialEq)]
pub enum ErrorKind {
    #[fail(display = "invalid argument to option `{}`: {}", opt, err)]
    FailedParse { opt: String, err: String },

    #[fail(
        display = "invalid default value for `{}` ({:?}): {}",
        option, value, err
    )]
    FailedParseDefault {
        option: &'static str,
        value: &'static str,
        err: String,
    },

    #[fail(
        display = "insufficient arguments to option `{}`: expected {}; found {}",
        option, expected, found
    )]
    InsufficientArguments {
        option: String,
        expected: usize,
        found: usize,
    },

    #[fail(display = "missing argument to option `{}`", opt)]
    MissingArgument { opt: String },

    #[fail(display = "missing command name")]
    MissingCommand,

    #[fail(display = "missing required option `{}`", opt)]
    MissingRequired { opt: String },

    #[fail(display = "missing required command")]
    MissingRequiredCommand,

    #[fail(display = "missing required free argument")]
    MissingRequiredFree,

    #[fail(display = "option `{}` does not accept an argument", opt)]
    UnexpectedArgument { opt: String },

    #[fail(display = "option `{}` expects {} arguments; found 1", opt, n)]
    UnexpectedSingleArgument { opt: String, n: usize },

    #[fail(display = "unexpected free argument `{}`", arg)]
    UnexpectedFree { arg: String },

    #[fail(display = "unrecognized command `{}`", name)]
    UnrecognizedCommand { name: String },

    #[fail(display = "unrecognized option `--{}`", opt)]
    UnrecognizedLongOption { opt: String },

    #[fail(display = "unrecognized option `-{}`", opt)]
    UnrecognizedShortOption { opt: char },
}
