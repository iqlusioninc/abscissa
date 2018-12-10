#[cfg(feature = "serde_derive")]
use serde::Deserialize;
use std::fmt::{self, Display};
use std::str::FromStr;

use crate::error::{FrameworkError, FrameworkErrorKind::ParseError};

/// Color configuration
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_derive", derive(Deserialize))]
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
        }
        .fmt(f)
    }
}

impl FromStr for ColorConfig {
    type Err = FrameworkError;

    fn from_str(s: &str) -> Result<Self, FrameworkError> {
        Ok(match s {
            "always" => ColorConfig::Always,
            "auto" => ColorConfig::Auto,
            "never" => ColorConfig::Never,
            other => fail!(ParseError, "bad color config option: {}", other),
        })
    }
}

impl ColorConfig {
    /// Initialize the shell using this color configuration
    pub fn init(self) {
        super::config(self)
    }
}
