//! Abscissa CLI utility and application generator.
//!
//! For framework-level documentation, please see the `abscissa_core` docs:
//!
//! <https://docs.rs/abscissa_core>

#![doc(
    html_logo_url = "https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa-sq.svg",
    html_root_url = "https://docs.rs/abscissa_core/0.5.0"
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
