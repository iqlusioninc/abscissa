//! Abscissa logger which displays through the terminal subsystem
// TODO(tarcieri): logfile support?

use crate::terminal::Stream;
use lazy_static::lazy_static;
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::io::{Error, Write};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

/// Default loglevel
const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Info;

lazy_static! {
    /// Global logger.
    ///
    /// The `log` crate only supports one global logger, and it must be
    /// initialized exactly once.
    static ref LOGGER: Logger = Logger::default();
}

/// Initialize the global logger.
///
/// Panics if called more than once.
pub(super) fn init() {
    log::set_logger(&*LOGGER).expect("error configuring global logger");
    log::set_max_level(DEFAULT_LOG_LEVEL);
}

/// Abscissa component for initializing the logging subsystem
#[derive(Debug)]
pub struct Logger {
    level: LevelFilter,
}

impl Default for Logger {
    fn default() -> Logger {
        Logger {
            level: DEFAULT_LOG_LEVEL,
        }
    }
}

impl Logger {
    /// Attempt to log
    fn try_log(&self, record: &Record) -> Result<(), Error> {
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
    fn level_stream(&self, record: &Record) -> StandardStream {
        // TODO(tarcieri): persistent streams managed by `terminal` component
        match record.level() {
            Level::Error => Stream::Stderr,
            _ => Stream::Stdout,
        }
        .open()
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
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        let result = self.try_log(record);
        debug_assert!(result.is_ok(), "logging error: {}", result.err().unwrap());
    }

    fn flush(&self) {
        // TODO(tarcieri): persistent streams managed by `terminal` component
        let mut stdout = Stream::Stdout.open();
        let stdout_result = stdout.flush();

        debug_assert!(
            stdout_result.is_ok(),
            "error flushing stdout: {}",
            stdout_result.err().unwrap()
        );

        let mut stderr = Stream::Stderr.open();
        let stderr_result = stderr.flush();

        debug_assert!(
            stderr_result.is_ok(),
            "error flushing stderr: {}",
            stdout_result.err().unwrap()
        );
    }
}
