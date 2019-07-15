//! Thread manager.

use super::{Name, Thread};
use crate::error::{FrameworkError, FrameworkErrorKind::ThreadError};
use std::collections::HashMap;

/// Thread manager that tracks threads spawned by the application and handles
/// shutting them down.
#[derive(Debug, Default)]
pub struct Manager {
    threads: HashMap<Name, Thread>,
}

impl Manager {
    /// Spawn a thread within the thread manager.
    pub fn spawn<F>(&mut self, name: &Name, f: F) -> Result<(), FrameworkError>
    where
        F: FnOnce() -> (),
        F: Send + 'static,
    {
        if self.threads.contains_key(name) {
            fail!(ThreadError, "duplicate name: {}", name);
        }

        let thread = Thread::spawn(name.clone(), f)?;
        self.threads.insert(name.clone(), thread).unwrap();
        Ok(())
    }

    /// Signal all running threads to terminate and then join them
    pub fn join(&mut self) -> Result<(), FrameworkError> {
        // Send termination request in advance prior to joining
        for thread in self.threads.values() {
            thread.request_termination();
        }

        for (_, thread) in self.threads.drain() {
            thread.join()?;
        }

        Ok(())
    }
}
