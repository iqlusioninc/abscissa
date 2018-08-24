//! Terminal handling code
//!
//! Some portions borrowed from the Cargo project: https://github.com/rust-lang/cargo
//!
//! These portions are redistributed under the same license as Cargo

use std::{
    cell::RefCell,
    fmt::Display,
    io::{self, prelude::*},
    sync::{Mutex, MutexGuard},
};
use term::{self, terminfo::TermInfo, Attr, Terminal as TermTerminal, TerminfoTerminal};

use super::Color;
use error::CliError;

pub mod color_config;
mod isatty;

pub use self::color_config::ColorConfig;

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

/// Reconfigure the shell
pub fn config(color_config: ColorConfig) {
    STDOUT
        .lock()
        .unwrap()
        .replace(Shell::new(Stream::Stdout, color_config));

    STDERR
        .lock()
        .unwrap()
        .replace(Shell::new(Stream::Stderr, color_config));
}

/// Say a status message with the given color
pub fn status<T, U>(stream: Stream, color: Color, status: T, message: U, justified: bool)
where
    T: Display,
    U: Display,
{
    let shell = stream.lock_shell();

    shell
        .borrow_mut()
        .status(color, status, message, justified)
        .unwrap();
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
            Stream::Stdout => STDOUT.lock().unwrap(),
            Stream::Stderr => STDERR.lock().unwrap(),
        }
    }

    /// Get a boxed writer for this stream
    fn writer(self) -> Box<Write + Send> {
        match self {
            Stream::Stdout => Box::new(io::stdout()),
            Stream::Stderr => Box::new(io::stderr()),
        }
    }

    /// Is this stream a TTY?
    fn is_tty(self) -> bool {
        self::isatty::isatty(self)
    }
}

/// Terminal I/O object
enum Terminal {
    NoColor(Box<Write + Send>),
    Colored(Box<term::Terminal<Output = Box<Write + Send>> + Send>),
}

/// Shell we output to (either STDOUT or STDERR)
pub(crate) struct Shell(Terminal);

impl Shell {
    /// Create a new shell for the given stream
    pub fn new(stream: Stream, color_config: ColorConfig) -> Self {
        let terminal = TermInfo::from_env()
            .map(|ti| {
                let term = TerminfoTerminal::new_with_terminfo(stream.writer(), ti);

                match color_config {
                    ColorConfig::Always => Terminal::Colored(Box::new(term)),
                    ColorConfig::Auto => if stream.is_tty() && term.supports_color() {
                        Terminal::Colored(Box::new(term))
                    } else {
                        Terminal::NoColor(term.into_inner())
                    },
                    ColorConfig::Never => Terminal::NoColor(term.into_inner()),
                }
            })
            .unwrap_or_else(|_| Terminal::NoColor(stream.writer()));

        Shell(terminal)
    }

    /// Say a status message with the given color
    pub fn status<T, U>(
        &mut self,
        color: Color,
        status: T,
        message: U,
        justified: bool,
    ) -> Result<(), CliError>
    where
        T: Display,
        U: Display,
    {
        self.reset()?;

        if color != super::color::BLACK {
            self.fg(color)?;
        }

        if self.supports_attr(Attr::Bold) {
            self.attr(Attr::Bold)?;
        }

        if justified {
            write!(self, "{:>12}", status.to_string())?;
        } else {
            write!(self, "{}", status)?;
        }

        self.reset()?;
        writeln!(self, " {}", message)?;
        self.flush()?;

        Ok(())
    }

    fn fg(&mut self, color: Color) -> Result<bool, CliError> {
        if let Terminal::Colored(ref mut c) = self.0 {
            c.fg(color)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn attr(&mut self, attr: Attr) -> Result<bool, CliError> {
        if let Terminal::Colored(ref mut c) = self.0 {
            c.attr(attr)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn supports_attr(&self, attr: Attr) -> bool {
        if let Terminal::Colored(ref c) = self.0 {
            c.supports_attr(attr)
        } else {
            false
        }
    }

    fn reset(&mut self) -> Result<(), CliError> {
        if let Terminal::Colored(ref mut c) = self.0 {
            c.reset()?;
        }

        Ok(())
    }
}

impl Write for Shell {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.0 {
            Terminal::Colored(ref mut c) => c.write(buf),
            Terminal::NoColor(ref mut n) => n.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.0 {
            Terminal::Colored(ref mut c) => c.flush(),
            Terminal::NoColor(ref mut n) => n.flush(),
        }
    }
}
