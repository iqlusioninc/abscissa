//! Main entry point for the Abscissa CLI application

mod application;
mod commands;
mod config;

use self::application::CliApplication;
use self::config::APP_CONFIG;

/// Boot Abscissa CLI
fn main() {
    abscissa::boot(CliApplication, &APP_CONFIG);
}
