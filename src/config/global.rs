//! Support for loading a global configuration (from files) and reading it in
//! a thread-safe manner (while also potentially supporting dynamic updates).

use serde::de::DeserializeOwned;
use std::{fs::File, io::Read};

pub use super::reader::ConfigReader;
use crate::{
    error::{FrameworkError, FrameworkErrorKind::ConfigError},
    util::{toml, CanonicalPath},
};

/// Common functions for loading and reading application configuration from
/// TOML files (providing a global lock which allows many readers, and can be
/// automatically implemented using the `impl_global_config!` macro.
// TODO: `derive(GlobalConfig)` using a proc macro.
pub trait GlobalConfig: 'static + Clone + DeserializeOwned {
    /// Get the global configuration, acquiring a lock around it. If the
    /// configuration hasn't been loaded, calls `Self::not_loaded()`.
    fn get_global() -> ConfigReader<Self>;

    /// Set the global configuration to the given value
    fn set_global(config: Self);

    /// Load the configuration from the given TOML string
    fn load_toml<T: AsRef<str>>(toml_string: T) -> Result<Self, FrameworkError> {
        Ok(toml::from_str(toml_string.as_ref())?)
    }

    /// Load the global configuration from the TOML file at the given path.
    /// If an error occurs reading or parsing the file, print it out and exit.
    fn load_toml_file<P>(path: &P) -> Result<Self, FrameworkError>
    where
        P: AsRef<CanonicalPath>,
    {
        let mut file = File::open(path.as_ref()).map_err(|e| {
            err!(
                ConfigError,
                "couldn't open {}: {}",
                path.as_ref().display(),
                e
            )
        })?;

        let mut toml_string = String::new();
        file.read_to_string(&mut toml_string)?;
        Self::load_toml(toml_string)
    }

    /// Parse the given TOML file and set the global configuration to the result
    fn set_from_toml_file<P>(path: &P) -> Result<ConfigReader<Self>, FrameworkError>
    where
        P: AsRef<CanonicalPath>,
    {
        Self::set_global(Self::load_toml_file(path)?);
        Ok(Self::get_global())
    }

    /// Load the given TOML configuration file, printing an error message and
    /// exiting if it's invalid
    fn set_from_toml_file_or_exit<P>(path: &P) -> ConfigReader<Self>
    where
        P: AsRef<CanonicalPath>,
    {
        Self::set_from_toml_file(path).unwrap_or_else(|e| {
            status_err!("error loading {}: {}", &path.as_ref().display(), e);
            ::std::process::exit(1);
        })
    }

    /// Error handler called if `Self::get()` is invoked before the global
    /// configuration has been loaded. This indicates a bug in the program
    /// accessing this type.
    fn not_loaded() -> ! {
        panic!("configuration accessed before being loaded!")
    }
}
