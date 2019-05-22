//! Configuration loader

use super::Config;
use crate::{error::FrameworkError, path::AbsPath};
use std::path::PathBuf;

/// Command type with which a configuration file is associated
pub trait Configurable<C: Config> {
    /// Path to the command's configuration file. Returns an error by default.
    fn config_path(&self) -> Option<PathBuf> {
        None
    }

    /// Load the configuration for this command
    fn load_config_file<P: AsRef<AbsPath>>(&self, path: &P) -> Result<C, FrameworkError> {
        self.preprocess_config(C::load_toml_file(path)?)
    }

    /// Process the configuration after it has been loaded, potentially
    /// modifying it or returning an error if options are incompatible
    #[allow(unused_mut)]
    fn preprocess_config(&self, mut config: C) -> Result<C, FrameworkError> {
        Ok(config)
    }
}
