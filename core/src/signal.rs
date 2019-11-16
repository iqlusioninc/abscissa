//! Unix signal handling

use crate::{
    application::{self, Application},
    thread, FrameworkError,
    FrameworkErrorKind::{SignalError, ThreadError},
};
use libc::c_int;
use signal_hook::iterator::Signals;
use std::{convert::TryFrom, fmt};

/// Unix signal types.
///
/// This includes signals which are useful for Abscissa applications to handle
/// and is not intended to be an exhaustive list.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Signal {
    /// Hangup from controlling terminal/process
    Hup = 1,

    /// Keyboard interrupt
    Int = 2,

    /// Broken pipe
    Pipe = 13,

    /// Timer alarm
    Alrm = 14,

    /// Termination signal
    Term = 15,

    /// Child process terminated
    Chld = 20,

    /// User-defined signal 1
    Usr1 = 30,

    /// User-defined signal 2
    Usr2 = 31,
}

impl Signal {
    /// Get numerical signal code
    pub fn number(self) -> u32 {
        self as u32
    }

    /// Get the signal name
    pub fn name(self) -> &'static str {
        match self {
            Signal::Hup => "SIGHUP",
            Signal::Int => "SIGINT",
            Signal::Pipe => "SIGPIPE",
            Signal::Alrm => "SIGALRM",
            Signal::Term => "SIGTERM",
            Signal::Chld => "SIGCHLD",
            Signal::Usr1 => "SIGUSR1",
            Signal::Usr2 => "SIGUSR2",
        }
    }
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl TryFrom<u32> for Signal {
    type Error = FrameworkError;

    fn try_from(num: u32) -> Result<Signal, FrameworkError> {
        Ok(match num {
            1 => Signal::Hup,
            2 => Signal::Int,
            13 => Signal::Pipe,
            14 => Signal::Alrm,
            15 => Signal::Term,
            20 => Signal::Chld,
            30 => Signal::Usr1,
            31 => Signal::Usr2,
            other => fail!(SignalError, "unregistered signal number: {}", other),
        })
    }
}

/// Launch the signal handler thread, listening for the given set of signals
pub fn register_handler<A, I>(
    app_lock: &'static application::Lock<A>,
    signals: I,
) -> Result<(), FrameworkError>
where
    A: Application + Send + Sync,
    I: IntoIterator<Item = Signal>,
{
    let mut app = app_lock.write();
    let thread_name = thread::Name::new("abscissa::signal");
    let signals = Signals::new(signals.into_iter().map(|s| s.number() as c_int))
        .map_err(|e| format_err!(ThreadError, "{}", e))?;

    app.state_mut()
        .threads
        .spawn(&thread_name, move || handler_thread(app_lock, signals))
}

/// Signal handler thread
fn handler_thread<A>(app_lock: &'static application::Lock<A>, signals: Signals)
where
    A: Application,
{
    while !thread::should_terminate() {
        for sig_num in &signals {
            let sig = Signal::try_from(sig_num as u32).unwrap();
            debug!("received signal: {}", sig);

            let mut app = app_lock.write();
            app.handle_signal(sig).unwrap_or_else(|e| {
                // TODO(tarcieri): terminate process?
                status_err!("error handling signal: {}", e)
            });
        }
    }
}
