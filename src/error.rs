//! Error types used by this crate

mod framework;

pub use self::framework::{FrameworkError, FrameworkErrorKind};
pub use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};

/// Error types used by this library, generic around `Kind`s
#[derive(Debug)]
pub struct Error<Kind>
where
    Kind: Fail + Clone + Display + Eq + PartialEq,
{
    /// Contextual information about the error
    inner: Context<Kind>,

    /// Description of the error providing additional information
    description: Option<String>,
}

impl<Kind> Error<Kind>
where
    Kind: Fail + Clone + Display + Eq + PartialEq,
{
    /// Create a new error from the given context object and description
    pub fn new<C>(into_context: C, description: Option<String>) -> Self
    where
        C: Into<Context<Kind>>,
    {
        let inner = into_context.into();
        Self { inner, description }
    }

    /// Obtain the error's `Kind`
    pub fn kind(&self) -> &Kind {
        self.inner.get_context()
    }
}

impl<Kind> Display for Error<Kind>
where
    Kind: Fail + Clone + Display + Eq + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.description {
            Some(ref desc) => write!(f, "{}: {}", self.kind(), desc),
            None => write!(f, "{}", self.kind()),
        }
    }
}

impl<Kind> Fail for Error<Kind>
where
    Kind: Fail + Clone + Display + Eq + PartialEq,
{
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl<Kind> From<Kind> for Error<Kind>
where
    Kind: Fail + Clone + Display + Eq + PartialEq,
{
    fn from(kind: Kind) -> Self {
        Error::new(kind, None)
    }
}
