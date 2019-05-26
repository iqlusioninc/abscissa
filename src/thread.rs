//! Thread wrapper types.
//!
//! These types provide simple wrappers for Rust's core threading primitives.

mod kill_switch;
mod name;

pub use self::name::Name;

use self::kill_switch::KillSwitch;
use crate::error::{FrameworkError, FrameworkErrorKind::ThreadError};
use std::{io, thread};

/// Threads spawned and managed by Abscissa
#[derive(Debug)]
pub struct Thread<T = ()>
where
    T: Send + 'static,
{
    /// Name of the current thread
    name: Name,

    /// Kill switch used to terminate the thread
    kill_switch: KillSwitch,

    /// Join handle to the thread
    handle: thread::JoinHandle<T>,
}

impl<T> Thread<T>
where
    T: Send + 'static,
{
    /// Check whether the currently running thread should exit, as signaled by
    /// `Thread::request_termination()`.
    ///
    /// Panics if called outside a thread spawned by `abscissa::Thread`.
    pub fn should_terminate() -> bool {
        kill_switch::is_thrown()
    }

    /// Spawn a new thread, executing the given runnable
    pub fn spawn<F>(name: Name, f: F) -> Result<Self, FrameworkError>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
    {
        let kill_switch = KillSwitch::new();
        let handle = spawn_thread(name.to_string(), kill_switch.clone(), f)?;

        Ok(Self {
            name,
            kill_switch,
            handle,
        })
    }

    /// Get the name of this thread.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Request that this thread terminate.
    ///
    /// Note this does not have immediate effect: it signals to the thread
    /// that it should exit, however the target thread needs to poll the
    /// `Thread::should_terminate()` flag in order to receive this signal
    /// (and exit accordingly when it is set).
    pub fn request_termination(&self) {
        self.kill_switch.throw();
    }

    /// Join to a running thread, waiting for it to finish
    pub fn join(self) -> Result<(), FrameworkError> {
        // Trigger the kill switch in order to signal the thread to stop.
        self.request_termination();

        // Wait for the other thread to exit
        self.handle
            .join()
            .map_err(|e| err!(ThreadError, "{:?}", e))?;

        Ok(())
    }
}

/// Spawn a thread
fn spawn_thread<F, T>(
    name: String,
    kill_switch: KillSwitch,
    f: F,
) -> Result<thread::JoinHandle<T>, io::Error>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    thread::Builder::new().name(name).spawn(move || {
        kill_switch::set(kill_switch);
        f()
    })
}
