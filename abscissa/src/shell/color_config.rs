use std::fmt::{self, Display};
use std::str::FromStr;

use error::{CliError, CliErrorKind, ToError};

/// Color configuration
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorConfig {
    /// Pick colors automatically based on whether we're using a TTY
    Auto,

    /// Always use colors
    Always,

    /// Never use colors
    Never,
}

impl Default for ColorConfig {
    fn default() -> ColorConfig {
        ColorConfig::Auto
    }
}

impl Display for ColorConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ColorConfig::Always => "always",
            ColorConfig::Auto => "auto",
            ColorConfig::Never => "never",
        }.fmt(f)
    }
}

impl FromStr for ColorConfig {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self, CliError> {
        match s {
            "always" => Ok(ColorConfig::Always),
            "auto" => Ok(ColorConfig::Auto),
            "never" => Ok(ColorConfig::Never),
            other => {
                let msg = format!("bad color config option: {}", other);
                Err(CliErrorKind::Parse.to_error(Some(&msg)))
            }
        }
    }
}
