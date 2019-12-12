//! Mutex guard for immutably accessing global application state

use super::Application;
use std::{ops::Deref, sync::RwLockReadGuard};

/// Generic `RwLockWriteGuard` for a `'static` lifetime.
pub(crate) type Guard<T> = RwLockReadGuard<'static, T>;

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
        self.0.deref()
    }
}
