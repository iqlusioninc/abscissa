//! Application-local prelude: conveniently import types/functions/macros
//! which are generally useful and should be available everywhere.

/// Commonly used Abscissa traits
pub use abscissa_core::{Application, Command, Runnable};

/// Error macros
pub use abscissa_core::{ensure, fail, fatal, format_err};

/// Logging macros
pub use abscissa_core::log::{debug, error, info, log, log_enabled, trace, warn};

/// Status macros
pub use abscissa_core::{status_err, status_info, status_ok, status_warn};
