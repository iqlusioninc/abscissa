//! Support for loading a global configuration (from files) and reading it in
//! a thread-safe manner (while also potentially supporting dynamic updates).

use serde::de::DeserializeOwned;
use std::{fs::File, io::Read, path::Path};
use toml;

pub use super::reader::ConfigReader;
use error::{CliError, CliErrorKind};

/// Common functions for loading and reading application configuration from
/// TOML files (providing a global lock which allows many readers, and can be
/// automatically implemented using the `impl_global_config!` macro.
// TODO: `derive(GlobalConfig)` using a proc macro.
pub trait GlobalConfig: 'static + Clone + DeserializeOwned {
    /// Get the global configuration, acquiring a lock around it. If the
    /// configuration hasn't been loaded, calls `Self::not_loaded()`.
    fn get() -> ConfigReader<Self>;

    /// Set the global configuration to the given value
    fn set(config: Self);

    /// Load the configuration from the given TOML string
    fn load_toml<T: AsRef<str>>(toml_string: T) -> Result<ConfigReader<Self>, CliError> {
        let config =
            toml::from_str(toml_string.as_ref()).map_err(|e| err!(CliErrorKind::Config, &e))?;

        Self::set(config);
        Ok(Self::get())
    }

    /// Load the global configuration from the TOML file at the given path.
    /// If an error occurs reading or parsing the file, print it out and exit.
    fn load_toml_file<P: AsRef<Path>>(path: P) -> Result<ConfigReader<Self>, CliError> {
        let mut file = File::open(path)?;
        let mut toml_string = String::new();
        file.read_to_string(&mut toml_string)?;
        Self::load_toml(toml_string)
    }

    /// Load the given TOML configuration file, printing an error message and
    /// exiting if it's invalid
    fn load_toml_file_or_exit<P: AsRef<Path>>(path: P) -> ConfigReader<Self> {
        Self::load_toml_file(path.as_ref()).unwrap_or_else(|e| {
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
