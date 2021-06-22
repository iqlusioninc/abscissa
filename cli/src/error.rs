//! Error types

use abscissa_core::error::{BoxError, Context};
use std::{
    fmt::{self, Display},
    io,
    ops::Deref,
};

/// Kinds of errors
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// Error in configuration file
    Config,

    /// Cargo-related errors
    Cargo,

    /// Git-related errors
    Git,

    /// Input/output error
    Io,

    /// Path-related errors
    Path,

    /// Template-related errors
    Template,
}

impl ErrorKind {
    /// Create an error context from this error
    pub fn context(self, source: impl Into<BoxError>) -> Context<ErrorKind> {
        Context::new(self, Some(source.into()))
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            ErrorKind::Config => "config error",
            ErrorKind::Cargo => "cargo error",
            ErrorKind::Git => "git error",
            ErrorKind::Io => "I/O error",
            ErrorKind::Path => "bad path",
            ErrorKind::Template => "template error",
        };

        f.write_str(description)
    }
}

impl std::error::Error for ErrorKind {}

/// Error type
#[derive(Debug)]
pub struct Error(Box<Context<ErrorKind>>);

impl Deref for Error {
    type Target = Context<ErrorKind>;

    fn deref(&self) -> &Context<ErrorKind> {
        &self.0
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(other: Context<ErrorKind>) -> Self {
        Error(Box::new(other))
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(err: handlebars::RenderError) -> Self {
        ErrorKind::Template.context(err).into()
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(err: handlebars::TemplateError) -> Self {
        ErrorKind::Template.context(err).into()
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        ErrorKind::Io.context(err).into()
    }
}
