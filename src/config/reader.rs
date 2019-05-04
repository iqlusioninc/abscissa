use std::{ops::Deref, sync::RwLockReadGuard};

use super::Config;

/// Generic RwLockReadGuard for a static lifetime
type ReaderGuard<C> = RwLockReadGuard<'static, Option<C>>;

/// Wrapper around a `RwLockReadGuard` for reading global configuration data
/// from global static values defined by the `init_config!` macro.
pub struct Reader<C: 'static + Config>(ReaderGuard<C>);

impl<C: Config> Reader<C> {
    /// Obtain a read-only handle to the inner configuration from an `RwLock`.
    /// This is intended to be used with a global static configuration defined
    /// by the `init_config!` macro.
    ///
    /// Panics if the configuration has not been loaded.
    pub fn new(config_guard: ReaderGuard<C>) -> Self {
        Reader(config_guard)
    }
}

impl<C: Config> Deref for Reader<C> {
    type Target = C;

    fn deref(&self) -> &C {
        // We assert this value `is_some` inside the `::new` method so we can
        // safely unwrap here.
        self.0.deref().as_ref().unwrap()
    }
}
