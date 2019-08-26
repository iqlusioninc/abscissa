//! Abscissa logger which displays through the terminal subsystem
// TODO(tarcieri): logfile support?

use super::config::Config;
use crate::terminal::stream::{STDERR, STDOUT};
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::io::{Error, Write};
use termcolor::{Color, ColorSpec, StandardStreamLock, WriteColor};

/// Initialize the global logger.
///
/// Panics if called more than once.
pub(super) fn init(config: &Config) {
    log::set_boxed_logger(Box::new(Logger::new(config))).expect("error configuring global logger");
    log::set_max_level(config.level_filter);
}

/// Abscissa component for initializing the logging subsystem
#[derive(Debug)]
pub struct Logger {
    level: LevelFilter,
}

impl Logger {
    /// Create a new logger at the given log level
    pub fn new(config: &Config) -> Self {
        Logger {
            level: config.level_filter,
        }
    }

    /// Attempt to log
    pub fn try_log(&self, record: &Record<'_>) -> Result<(), Error> {
        let mut stream = self.level_stream(record);
        let now = chrono::Utc::now();
        write!(&mut stream, "{} ", now.format("%H:%M:%S"))?;

        stream.set_color(&self.level_color(record.level()))?;
        write!(stream, "[{}] ", record.level().to_string().to_lowercase())?;
        stream.reset()?;

        writeln!(&mut stream, "{}", record.args())?;
        Ok(())
    }

    /// Get the stream to which a particular loglevel should be displayed
    fn level_stream(&self, record: &Record<'_>) -> StandardStreamLock<'_> {
        match record.level() {
            Level::Error => &*STDERR,
            _ => &*STDOUT,
        }
        .lock()
    }

    /// Get the color used to display a particular level
    fn level_color(&self, level: Level) -> ColorSpec {
        let color = match level {
            Level::Error => Color::Red,
            Level::Warn => Color::Yellow,
            Level::Info => Color::Blue,
            Level::Debug => Color::Cyan,
            Level::Trace => Color::White,
        };

        let mut cs = ColorSpec::new();
        cs.set_fg(Some(color));
        cs.set_bold(true);
        cs
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        let result = self.try_log(record);
        debug_assert!(result.is_ok(), "logging error: {}", result.err().unwrap());
    }

    fn flush(&self) {
        let mut stdout = STDOUT.lock();
        let stdout_result = stdout.flush();

        debug_assert!(
            stdout_result.is_ok(),
            "error flushing stdout: {}",
            stdout_result.err().unwrap()
        );

        let mut stderr = STDERR.lock();
        let stderr_result = stderr.flush();

        debug_assert!(
            stderr_result.is_ok(),
            "error flushing stderr: {}",
            stdout_result.err().unwrap()
        );
    }
}
