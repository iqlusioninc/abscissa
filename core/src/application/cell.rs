//! Application Cell: holder of application state.

use super::{
    lock::{Lock, Reader, Writer},
    Application,
};
use crate::config;
use once_cell::sync::OnceCell;

/// Newtype wrapper for the cell type we use.
///
/// This allows us to define methods on the type, which we do below on the
/// `AppCell` alias (trait bounds on `const fn` types aren't yet stable).
pub struct Cell<T>(OnceCell<T>);

impl<T> Cell<T> {
    /// Create a new application cell.
    pub const fn new() -> Cell<T> {
        Self(OnceCell::new())
    }
}

/// Application cells.
///
/// These are defined as a type alias as it's not yet stable to have trait
/// bounds on types with `const fn` yet.
pub type AppCell<A> = Cell<Lock<A>>;

impl<A: Application> AppCell<A> {
    /// Set the application state to the given value.
    ///
    /// This can only be performed once without causing a crash.
    pub(crate) fn set_once(&self, app: A) {
        self.0.set(Lock::new(app)).unwrap_or_else(|_| {
            panic!("Abscissa applications can't be rebooted (yet)!");
        })
    }

    /// Get the application state, acquiring a shared, read-only lock
    /// around it which permits concurrent access by multiple readers.
    pub fn read(&'static self) -> Reader<A> {
        self.0.get().unwrap_or_else(|| not_loaded()).read()
    }

    /// Obtain an exclusive lock on the application state, allowing it to be
    /// accessed mutably.
    pub fn write(&'static self) -> Writer<A> {
        self.0.get().unwrap_or_else(|| not_loaded()).write()
    }

    /// Obtain a read-only (multi-reader) lock on the application configuration.
    ///
    /// Panics if the application configuration has not been loaded.
    pub fn config(&'static self) -> config::Reader<A> {
        config::Reader::new(self)
    }
}

/// Error handler called if `get()` is invoked before the global
/// application state has been initialized.
///
/// This indicates a bug in the program accessing this type.
fn not_loaded() -> ! {
    panic!("Abscissa application state accessed before it has been initialized!")
}
