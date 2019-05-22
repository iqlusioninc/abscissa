use super::Error;
use failure::Fail;
use std::io;

/// Types of errors which occur internally within the framework
#[derive(Fail, Clone, Debug, Eq, PartialEq)]
pub enum FrameworkErrorKind {
    /// Errors relating to components
    #[cfg(feature = "application")]
    #[fail(display = "component error")]
    ComponentError,

    /// Error reading configuration file
    #[fail(display = "config error")]
    ConfigError,

    /// I/O operation failed
    #[fail(display = "I/O operation failed")]
    IoError,

    /// Couldn't parse the given value
    #[fail(display = "parse error")]
    ParseError,

    /// Errors associated with filesystem paths
    #[fail(display = "path error")]
    PathError,
}

impl From<io::Error> for FrameworkError {
    fn from(err: io::Error) -> Self {
        err!(FrameworkErrorKind::IoError, err)
    }
}

#[cfg(feature = "term")]
impl From<term::Error> for FrameworkError {
    fn from(err: term::Error) -> Self {
        err!(FrameworkErrorKind::IoError, err)
    }
}

#[cfg(feature = "toml")]
impl From<toml::de::Error> for FrameworkError {
    fn from(err: toml::de::Error) -> Self {
        err!(FrameworkErrorKind::ParseError, err)
    }
}

/// Abscissa-internal framework errors
pub type FrameworkError = Error<FrameworkErrorKind>;
