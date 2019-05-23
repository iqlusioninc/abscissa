//! Main entry point for the Abscissa CLI application

mod application;
mod commands;
mod config;

use self::application::APPLICATION;

/// Boot Abscissa CLI
fn main() {
    abscissa::boot(&APPLICATION);
}
