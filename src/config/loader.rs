//! Configuration loader

use super::{guard::Guard, Config};
use crate::{
    error::{FrameworkError, FrameworkErrorKind::ConfigError},
    util::{CanonicalPath, CanonicalPathBuf},
};
use std::path::PathBuf;

/// Support for loading configuration from a file.
/// Does not modify the global configuration. Only handles parsing and
/// deserializing it from files.
pub trait Loader<C: Config> {
    /// Path to the command's configuration file. Returns an error by default.
    fn config_path(&self) -> Option<PathBuf> {
        None
    }

    /// Load the configuration from `self.config_path()` if present
    fn load_config(&self, config_guard: &Guard<C>) -> Result<(), FrameworkError> {
        // Only attempt to load configuration if `config_path` returned the
        // path to a configuration file.
        if let Some(ref path) = self.config_path() {
            let canonical_path = CanonicalPathBuf::canonicalize(path)
                .map_err(|e| err!(ConfigError, "invalid path '{}': {}", path.display(), e))?;

            let config = self.load_config_file(&canonical_path).map_err(|e| {
                err!(
                    ConfigError,
                    "error loading config from '{}': {}",
                    canonical_path.display(),
                    e
                )
            })?;

            config_guard.set(config);
        }

        Ok(())
    }

    /// Load the configuration for this command
    fn load_config_file<P: AsRef<CanonicalPath>>(&self, path: &P) -> Result<C, FrameworkError> {
        self.preprocess_config(C::load_toml_file(path)?)
    }

    /// Process the configuration after it has been loaded, potentially
    /// modifying it or returning an error if options are incompatible
    #[allow(unused_mut)]
    fn preprocess_config(&self, mut config: C) -> Result<C, FrameworkError> {
        Ok(config)
    }
}
