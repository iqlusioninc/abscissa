//! Terminal handling code
//!
//! Some portions borrowed from the Cargo project: https://github.com/rust-lang/cargo
//!
//! These portions are redistributed under the same license as Cargo

use std::{
    fmt::Display,
    io::{self, Write},
};

pub use term::color::{self, Color};
use term::Attr;

mod color_config;
#[cfg(feature = "application")]
mod component;
pub mod extras;
mod stream;
mod terminal;

#[cfg(feature = "application")]
pub use self::component::ShellComponent;
use self::terminal::Terminal;
pub use self::{color_config::ColorConfig, stream::Stream};
use error::FrameworkError;

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

/// Reconfigure the shell (invoke this via `ColorConfig`)
fn config(color_config: ColorConfig) {
    for stream in &[Stream::Stdout, Stream::Stderr] {
        let shell = stream.lock_shell();
        shell.replace(Shell::new(*stream, color_config));
    }
}

/// Shell we output to (either STDOUT or STDERR)
pub(crate) struct Shell(Terminal);

impl Shell {
    /// Create a new shell for the given stream
    pub(crate) fn new(stream: Stream, color_config: ColorConfig) -> Self {
        let terminal = Terminal::new(stream.writer(), color_config, stream.is_tty())
            .unwrap_or_else(|_| Terminal::from(stream.writer()));

        Shell(terminal)
    }

    /// Say a status message with the given color
    pub(crate) fn status<T, U>(
        &mut self,
        clr: Color,
        status: T,
        message: U,
        justified: bool,
    ) -> Result<(), FrameworkError>
    where
        T: Display,
        U: Display,
    {
        self.reset()?;

        if clr != color::BLACK {
            self.fg(clr)?;
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

    fn fg(&mut self, clr: Color) -> Result<bool, FrameworkError> {
        if let Terminal::Colored(ref mut c) = self.0 {
            c.fg(clr)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn attr(&mut self, attr: Attr) -> Result<bool, FrameworkError> {
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

    fn reset(&mut self) -> Result<(), FrameworkError> {
        if let Terminal::Colored(ref mut c) = self.0 {
            c.reset()?;
        }

        Ok(())
    }
}

impl Write for Shell {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
