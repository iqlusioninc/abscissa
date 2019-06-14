//! Terminal streams (STDOUT and STDIN)

use lazy_static::lazy_static;
use std::sync::Mutex;
use termcolor::{ColorChoice, StandardStream};

lazy_static! {
    /// Color configuration
    static ref COLOR_CHOICE: Mutex<Option<ColorChoice>> = Mutex::new(None);

    /// Standard output
    pub static ref STDOUT: StandardStream = StandardStream::stdout(get_color_choice());

    /// Standard error
    pub static ref STDERR: StandardStream = StandardStream::stderr(get_color_choice());
}

/// Obtain the color configuration.
///
/// Panics if no configuration has been provided.
fn get_color_choice() -> ColorChoice {
    let choice = COLOR_CHOICE.lock().unwrap();
    *choice
        .as_ref()
        .expect("terminal stream accessed before initialized!")
}

/// Set the color configuration.
///
/// Panics if the terminal has already been configured.
pub(super) fn set_color_choice(color_choice: ColorChoice) {
    let mut choice = COLOR_CHOICE.lock().unwrap();
    assert!(choice.is_none(), "terminal colors already configured!");
    *choice = Some(color_choice);
}
