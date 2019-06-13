//! Terminal handling (TTY interactions, colors, etc)

mod color;
#[cfg(feature = "application")]
mod component;
mod stream;

#[cfg(feature = "application")]
pub use self::component::TerminalComponent;
pub use self::{color::ColorConfig, stream::Stream};
use crate::error::FrameworkError;
use lazy_static::lazy_static;
use std::{fmt::Display, io::Write, sync::RwLock};
pub use termcolor::Color;
use termcolor::{ColorSpec, WriteColor};

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

// TODO(tarcieri): open terminal streams persistently
fn config(color_config: ColorConfig) {
    let mut global_config = COLOR_CONFIG.write().unwrap();
    *global_config = color_config;
}
