//! Access to standard output and standard error

use super::{ColorConfig, Shell};
use lazy_static::lazy_static;
use std::{
    cell::RefCell,
    io::{self, Write},
    sync::{Mutex, MutexGuard},
};

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
    pub(crate) fn lock_shell(&self) -> MutexGuard<'_, RefCell<Shell>> {
        match self {
            // TODO: better handle `PoisonError`?
            Stream::Stdout => STDOUT.lock().unwrap(),
            Stream::Stderr => STDERR.lock().unwrap(),
        }
    }

    /// Get a boxed writer for this stream
    pub(crate) fn writer(self) -> Box<dyn Write + Send> {
        match self {
            Stream::Stdout => Box::new(io::stdout()),
            Stream::Stderr => Box::new(io::stderr()),
        }
    }

    /// Is this stream a TTY?
    pub fn is_tty(self) -> bool {
        atty::is(match self {
            Stream::Stdout => atty::Stream::Stdout,
            Stream::Stderr => atty::Stream::Stderr,
        })
    }
}
