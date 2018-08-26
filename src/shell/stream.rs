//! Access to standard output and standard error

use std::{
    cell::RefCell,
    io::{self, Write},
    sync::{Mutex, MutexGuard},
};

use super::{ColorConfig, Shell};

lazy_static! {
    static ref STDOUT: Mutex<RefCell<Shell>> = {
        Mutex::new(RefCell::new(Shell::new(
            Stream::Stdout,
            ColorConfig::default(),
        )))
    };
    static ref STDERR: Mutex<RefCell<Shell>> = {
        Mutex::new(RefCell::new(Shell::new(
            Stream::Stderr,
            ColorConfig::default(),
        )))
    };
}

/// Terminal streams
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Stream {
    /// Standard output
    Stdout,

    /// Standard error
    Stderr,
}

impl Stream {
    /// Get a shell for this stream type
    #[allow(unknown_lints, trivially_copy_pass_by_ref)]
    pub(crate) fn lock_shell(&self) -> MutexGuard<RefCell<Shell>> {
        match self {
            // TODO: better handle `PoisonError`?
            Stream::Stdout => STDOUT.lock().unwrap(),
            Stream::Stderr => STDERR.lock().unwrap(),
        }
    }

    /// Get a boxed writer for this stream
    pub(crate) fn writer(self) -> Box<Write + Send> {
        match self {
            Stream::Stdout => Box::new(io::stdout()),
            Stream::Stderr => Box::new(io::stderr()),
        }
    }

    /// Is this stream a TTY?
    pub fn is_tty(self) -> bool {
        super::isatty::isatty(self)
    }
}
