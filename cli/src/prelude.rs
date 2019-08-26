//! Application-local prelude: conveniently import types/functions/macros
//! which are generally useful and should be available everywhere.

/// Commonly used Abscissa traits
pub use abscissa_core::{Application, Command, Runnable};

/// Error macros
pub use abscissa_core::error::macros::{};

/// Logging macros
pub use abscissa_core::log::{debug, error, info, log, log_enabled, trace, warn};
