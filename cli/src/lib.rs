#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iqlusioninc/abscissa/main/img/abscissa-sq.svg"
)]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, unused_lifetimes, unused_qualifications)]

pub mod application;
pub mod commands;
pub mod config;
pub mod error;
pub mod prelude;
pub mod properties;
pub mod template;
