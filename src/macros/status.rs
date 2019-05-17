//! Macros for ergonomic status messages printed to stdout/stderr
//!
//! # `status_ok!`: Successful status messages
//!
//! ```
//! # #[macro_use] extern crate abscissa;
//! # fn main() {
//! // Print a Cargo-like justified status to STDOUT
//! status_ok!("Loaded", "app loaded successfully");
//! # }
//! ```
//!
//! # `status_err!`: Error messages
//!
//! ```
//! # #[macro_use] extern crate abscissa;
//! # fn main() {
//! // Print an error message
//! status_err!("something bad happened");
//! # }
//! ```
//!
//! # `status_attr_ok!`: Successful attributes
//!
//! ```
//! # #[macro_use] extern crate abscissa;
//! # fn main() {
//! // Print an indented attribute to STDOUT
//! status_attr_ok!("good", "yep");
//! # }
//! ```
//!
//! # `status_attr_error!`: Error attributes
//!
//! ```
//! # #[macro_use] extern crate abscissa;
//! # fn main() {
//! // Print an error attribute to STDERR
//! status_attr_err!("error", "yep");
//! # }
//! ```

/// Print a justified status message (in the given color if colors are enabled)
#[macro_export]
macro_rules! status {
    ($stream:expr, $color:expr, $status:expr, $msg:expr) => {
        $crate::status($stream, $color, $status, $msg, true);
    };
    ($stream:expr, $color:expr, $status:expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status!($stream, $color, $status, format!($fmt, $($arg)+));
    };
}

/// Print an unjustified status message (in the given color if colors are enabled)
#[macro_export]
macro_rules! status_nojust {
    ($stream:expr, $color:expr, $status:expr, $msg:expr) => {
        $crate::status($stream, $color, $status, $msg, false);
    };
    ($stream:expr, $color:expr, $status:expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status!($stream, $color, $status, format!($fmt, $($arg)+));
    };
}

/// Print a success status message (in green if colors are enabled)
///
/// ```
/// # #[macro_use] extern crate abscissa;
/// # fn main() {
/// // Print a Cargo-like justified status to STDOUT
/// status_ok!("Loaded", "app loaded successfully");
/// # }
/// ```
#[macro_export]
macro_rules! status_ok {
    ($status:expr, $msg:expr) => {
        $crate::status!($crate::Stream::Stdout, $crate::shell::color::GREEN, $status, $msg);
    };
    ($status:expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status_ok!($status, format!($fmt, $($arg)+));
    };
}

/// Print an informational status message (in cyan if colors are enabled)
///
/// ```
/// # #[macro_use] extern crate abscissa;
/// # fn main() {
/// // Print a Cargo-like justified status to STDOUT
/// status_info!("Info", "you may care to know about");
/// # }
/// ```
#[macro_export]
macro_rules! status_info {
    ($status:expr, $msg:expr) => {
        $crate::status!($crate::Stream::Stdout, $crate::shell::color::BRIGHT_CYAN, $status, $msg);
    };
    ($status:expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status_info!($status, format!($fmt, $($arg)+));
    };
}

/// Print a warning status message (in yellow if colors are enabled)
///
/// ```
/// # #[macro_use] extern crate abscissa;
/// # fn main() {
/// // Print a Cargo-like justified status to STDOUT
/// status_warn!("heads up, there's something you should know");
/// # }
/// ```
#[macro_export]
macro_rules! status_warn {
    ($msg:expr) => {
        $crate::status_nojust!($crate::Stream::Stdout, $crate::shell::color::YELLOW, "warning:", $msg);
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::status_warn!(format!($fmt, $($arg)+));
    };
}

/// Print an error message (in red if colors are enabled)
///
/// ```
/// # #[macro_use] extern crate abscissa;
/// # fn main() {
/// // Print an error message
/// status_err!("something bad happened");
/// # }
/// ```
#[macro_export]
macro_rules! status_err {
    ($msg:expr) => {
        $crate::status_nojust!($crate::Stream::Stderr, $crate::shell::color::RED, "error:", $msg);
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::status_err!(format!($fmt, $($arg)+));
    };
}

/// Print a tab-delimited status (with the given color if enabled)
#[macro_export]
macro_rules! status_attr {
    ($stream:expr, $color:expr, $attr:expr, $msg:expr) => {
        // TODO: this is kind of hax... use a better format string?
        let attr_delimited = if $attr.len() >= 7 {
            format!("{}:", $attr)
        } else {
            format!("{}:\t", $attr)
        };

        $crate::status_nojust!(
            $stream,
            $color,
            attr_delimited,
            $msg
        );
    };
    ($stream:expr, $color:expr, $attr: expr, $fmt:expr, $($arg:tt)+) => {
        status_attr!($stream, $attr, format!($fmt, $($arg)+));
    }
}

/// Print a tab-delimited status attribute (in green if colors are enabled)
///
/// ```
/// # #[macro_use] extern crate abscissa;
/// # fn main() {
/// // Print an indented attribute to STDOUT
/// status_attr_ok!("good", "yep");
/// # }
/// ```
#[macro_export]
macro_rules! status_attr_ok {
    ($attr:expr, $msg:expr) => {
        $crate::status_attr!($crate::Stream::Stdout, $crate::shell::color::GREEN, $attr, $msg);
    };
    ($attr: expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status_attr_ok!($attr, format!($fmt, $($arg)+));
    }
}

/// Print a tab-delimited status attribute (in red if colors are enabled)
///
/// ```
/// # #[macro_use] extern crate abscissa;
/// # fn main() {
/// // Print an error attribute to STDERR
/// status_attr_err!("error", "yep");
/// # }
/// ```
#[macro_export]
macro_rules! status_attr_err {
    ($attr:expr, $msg:expr) => {
        $crate::status_attr!($crate::Stream::Stderr, $crate::shell::color::RED, $attr, $msg);
    };
    ($attr: expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status_attr_err!($attr, format!($fmt, $($arg)+));
    }
}
