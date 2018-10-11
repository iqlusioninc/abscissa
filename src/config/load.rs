use std::path::PathBuf;

use config::GlobalConfig;
use error::{FrameworkError, FrameworkErrorKind::ConfigError};
use util::{CanonicalPath, CanonicalPathBuf};

/// Support for loading configuration from a file.
/// Does not modify the global configuration. Only handles parsing and
/// deserializing it from files.
pub trait LoadConfig<C: GlobalConfig> {
    /// Path to the command's configuration file. Returns an error by default.
    fn config_path(&self) -> Option<PathBuf> {
        None
    }

    /// Load the configuration from `self.config_path()` if present
    fn load_global_config(&self) -> Result<(), FrameworkError> {
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

            C::set_global(config);
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
