//! Support for loading a global configuration (from files) and reading it in
//! a thread-safe manner (while also potentially supporting dynamic updates).

use super::{reader::Reader, Config};
use crate::{error::FrameworkError, util::CanonicalPath};
use std::sync::RwLock;

/// Support for loading and reading application configuration from
/// TOML files.
///
/// Provides a global lock which allows many readers.
pub struct Guard<C: Config>(RwLock<Option<C>>);

impl<C: Config> Guard<C> {
    /// Create a new `Guard` with no loaded configuration
    pub fn new() -> Self {
        Guard(RwLock::new(None))
    }

    /// Get the global configuration, acquiring a lock around it. If the
    /// configuration hasn't been loaded, calls `not_loaded()`.
    pub fn get(&'static self) -> Reader<C> {
        // TODO: better handle `PoisonError`? (i.e. print a better error message)
        let config = self.0.read().unwrap();

        if config.is_none() {
            Self::not_loaded();
        }

        Reader::new(config)
    }

    /// Set the global configuration to the given value
    pub fn set(&self, config: C) {
        // TODO: better handle `PoisonError`?
        let mut cfg = self.0.write().unwrap();
        *cfg = Some(config);
    }

    /// Parse the given TOML file and set the global configuration to the result
    pub fn set_from_toml_file<P>(&'static self, path: &P) -> Result<Reader<C>, FrameworkError>
    where
        P: AsRef<CanonicalPath>,
    {
        self.set(C::load_toml_file(path)?);
        Ok(self.get())
    }

    /// Load the given TOML configuration file, printing an error message and
    /// exiting if it's invalid
    pub fn set_from_toml_file_or_exit<P>(&'static self, path: &P) -> Reader<C>
    where
        P: AsRef<CanonicalPath>,
    {
        self.set_from_toml_file(path).unwrap_or_else(|e| {
            status_err!("error loading {}: {}", &path.as_ref().display(), e);
            std::process::exit(1);
        })
    }

    /// Error handler called if `get()` is invoked before the global
    /// configuration has been loaded.
    ///
    /// This indicates a bug in the program accessing this type.
    fn not_loaded() -> ! {
        panic!("configuration accessed before being loaded!")
    }
}

impl<C: Config> Default for Guard<C> {
    fn default() -> Self {
        Self::new()
    }
}
