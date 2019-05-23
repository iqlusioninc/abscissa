//! Support for sourcing/overriding configuration values from arguments
//! given on the command-line.

use crate::options::Options;

/// Merge the given options into this configuration. This allows setting of
/// global configuration values using command-line options, and also unifies
/// the global config as the one way to get application settings.
#[allow(unused_variables)]
pub trait MergeOptions<O: Options>: Sized {
    /// Process the given command line options, overriding settings from
    /// a configuration file using explicit flags taken from command-line
    /// arguments.
    ///
    /// This provides a canonical way to interpret global configuration
    /// settings when dealing with both a config file and options passed
    /// on the command line, and a unified way of accessing this information
    /// from components or in the application: from the global config.
    fn merge(self, options: &O) -> Self {
        self
    }
}
