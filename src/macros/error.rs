//! Error-handling macros for the `abscissa` framework
//!
//! This crate defines two error handling macros designed to produce formatted
//! error messages from error kind enums that implement the `Fail` trait:
//!
//! * `err!(kind, description)` creates a new `Error<Kind>` with the given
//!   description. If additional parameters are given, `description` is treated as
//!   a format string, e.g. `err!(kind, "something went wrong: {}", &wrongness)`.
//! * `fail!(kind, description)` creates a new `Error<kind>` and returns it.

/// Create a new error (of a given kind) with a formatted message
#[macro_export]
macro_rules! err {
    ($kind:path, $msg:expr) => {
        $crate::error::Error::new($crate::error::Context::new($kind), Some($msg.to_string()))
    };
    ($kind:path, $fmt:expr, $($arg:tt)+) => {
        err!($kind, &format!($fmt, $($arg)+))
    };
}

/// Create and return an error with a formatted message
#[macro_export]
macro_rules! fail {
    ($kind:path, $msg:expr) => {
        return Err(err!($kind, $msg).into());
    };
    ($kind:path, $fmt:expr, $($arg:tt)+) => {
        fail!($kind, &format!($fmt, $($arg)+));
    };
}

/// Implement an error conversion for the given type
#[macro_export]
macro_rules! impl_error_from {
    ($from:path, $type:ty) => {
        use abscissa::error::ToError;
        impl From<$from> for $type {
            fn from(other: io::Error) -> Self {
                CliErrorKind::Io.to_error(&other)
            }
        }
    };
}
