//! Mutex guard for mutably accessing global application state

use super::Application;
use std::{ops, sync::RwLockWriteGuard};

/// Generic `RwLockReadGuard` for a `'static` lifetime.
pub(crate) type WriterGuard<T> = RwLockWriteGuard<'static, Option<T>>;

/// Wrapper around a `RwLockReadGuard` for reading global application state.
pub struct Writer<A>(WriterGuard<A>)
where
    A: 'static + Application;

impl<A> Writer<A>
where
    A: 'static + Application,
{
    /// Create wrapper around a read-only application mutex guard
    pub(super) fn new(config_guard: WriterGuard<A>) -> Self {
        Writer(config_guard)
    }
}

impl<A> ops::Deref for Writer<A>
where
    A: 'static + Application,
{
    type Target = A;

    fn deref(&self) -> &A {
        self.0.deref().as_ref().unwrap()
    }
}

impl<A> ops::DerefMut for Writer<A>
where
    A: 'static + Application,
{
    fn deref_mut(&mut self) -> &mut A {
        self.0.deref_mut().as_mut().unwrap()
    }
}
