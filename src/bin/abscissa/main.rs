//! Main entry point for the Abscissa CLI application

#![deny(warnings, unsafe_code, unused_qualifications)]

mod application;
mod commands;
mod config;

use self::application::APPLICATION;

/// Boot Abscissa CLI
fn main() {
    abscissa::boot(&APPLICATION);
}
