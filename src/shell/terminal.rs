use std::io::{self, Write};
use term::{self, terminfo::TermInfo, Terminal as TerminalTrait, TerminfoTerminal};

use super::color_config::ColorConfig;
use crate::error::FrameworkError;

/// Terminal I/O object
pub(super) enum Terminal {
    NoColor(Box<Write + Send>),
    Colored(Box<term::Terminal<Output = Box<Write + Send>> + Send>),
}

impl Terminal {
    /// Create a new shell for the given stream
    pub(super) fn new(
        writer: Box<Write + Send>,
        color_config: ColorConfig,
        is_tty: bool,
    ) -> Result<Self, FrameworkError> {
        let terminfo = TermInfo::from_env()?;
        let terminfo_terminal = TerminfoTerminal::new_with_terminfo(writer, terminfo);

        let terminal = match color_config {
            ColorConfig::Always => Terminal::Colored(Box::new(terminfo_terminal)),
            ColorConfig::Auto => if is_tty && terminfo_terminal.supports_color() {
                Terminal::Colored(Box::new(terminfo_terminal))
            } else {
                Terminal::NoColor(terminfo_terminal.into_inner())
            },
            ColorConfig::Never => Terminal::NoColor(terminfo_terminal.into_inner()),
        };

        Ok(terminal)
    }
}

impl From<Box<Write + Send>> for Terminal {
    /// Create a Terminal with no color configuration
    fn from(writer: Box<Write + Send>) -> Self {
        Terminal::NoColor(writer)
    }
}

impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Terminal::Colored(ref mut c) => c.write(buf),
            Terminal::NoColor(ref mut n) => n.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Terminal::Colored(ref mut c) => c.flush(),
            Terminal::NoColor(ref mut n) => n.flush(),
        }
    }
}
