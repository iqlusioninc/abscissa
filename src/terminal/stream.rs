//! Terminal streams (STDOUT and STDIN)

use super::{color::ColorConfig, COLOR_CONFIG};
use termcolor::StandardStream;

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
    pub(super) fn open_with_config(self, color_config: ColorConfig) -> StandardStream {
        let color_choice = color_config.into();

        match self {
            Stream::Stdout => StandardStream::stdout(color_choice),
            Stream::Stderr => StandardStream::stderr(color_choice),
        }
    }

    /// Open using the global color config
    // TODO(tarcieri): have `terminal` component manage/own all streams
    pub(crate) fn open(self) -> StandardStream {
        let color_config = COLOR_CONFIG.read().unwrap();
        self.open_with_config(*color_config)
    }
}
