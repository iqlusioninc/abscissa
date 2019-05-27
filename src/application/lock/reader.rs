//! Mutex guard for immutably accessing global application state

use super::Application;
use std::{ops::Deref, sync::RwLockReadGuard};

/// Generic `RwLockWriteGuard` for a `'static` lifetime.
pub(crate) type Guard<T> = RwLockReadGuard<'static, Option<T>>;

/// Wrapper around a `RwLockReadGuard` for reading global application state.
pub struct Reader<A>(Guard<A>)
where
    A: 'static + Application;

impl<A> Reader<A>
where
    A: 'static + Application,
{
    /// Create wrapper around a read-only application mutex guard
    pub(super) fn new(guard: Guard<A>) -> Self {
        Reader(guard)
    }
}

impl<A> Deref for Reader<A>
where
    A: 'static + Application,
{
    type Target = A;

    fn deref(&self) -> &A {
        self.0.deref().as_ref().unwrap_or_else(|| not_loaded())
    }
}

/// Error handler called if `get()` is invoked before the global
/// application state has been initialized.
///
/// This indicates a bug in the program accessing this type.
fn not_loaded() -> ! {
    panic!("Abscissa application state accessed before it has been initialized!")
}
