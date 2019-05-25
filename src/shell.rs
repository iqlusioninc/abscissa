//! Shell/terminal colors and interactions

mod color_config;
#[cfg(feature = "application")]
mod component;

pub use self::color_config::ColorConfig;
#[cfg(feature = "application")]
pub use self::component::ShellComponent;
use crate::error::FrameworkError;
use lazy_static::lazy_static;
use std::{fmt::Display, io::Write, sync::RwLock};
pub use termcolor::Color;
use termcolor::{ColorSpec, StandardStream, WriteColor};

lazy_static! {
    /// Color configuration
    static ref COLOR_CONFIG: RwLock<ColorConfig> = RwLock::new(ColorConfig::Auto);
}

/// Say a status message with the given color
pub fn status<T, U>(
    stream: Stream,
    color: Color,
    bold: bool,
    status: T,
    message: U,
    justified: bool,
) -> Result<(), FrameworkError>
where
    T: Display,
    U: Display,
{
    let mut s = stream.open();
    s.reset()?;
    s.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(bold))?;

    if justified {
        write!(s, "{:>12}", status)?;
    } else {
        write!(s, "{}", status)?;
    }

    s.reset()?;
    writeln!(s, " {}", message)?;
    s.flush()?;

    Ok(())
}

// TODO(tarcieri): hold onto shell streams in the shell component
fn config(color_config: ColorConfig) {
    let mut global_config = COLOR_CONFIG.write().unwrap();
    *global_config = color_config;
}

/// Terminal streams
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Stream {
    /// Standard output
    Stdout,

    /// Standard error
    Stderr,
}

impl Stream {
    /// Open the given stream
    fn open(self) -> StandardStream {
        let color_choice = {
            let cfg = COLOR_CONFIG.read().unwrap();
            *cfg
        }
        .into();

        match self {
            Stream::Stdout => StandardStream::stdout(color_choice),
            Stream::Stderr => StandardStream::stderr(color_choice),
        }
    }
}
