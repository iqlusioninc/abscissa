//! Abscissa CLI utility and application generator.
//!
//! For framework-level documentation, please see the `abscissa_core` docs:
//!
//! <https://docs.rs/abscissa_core>

#![deny(warnings, unsafe_code, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa-sq.svg",
    html_root_url = "https://docs.rs/abscissa_core/0.2.0"
)]

pub mod application;
pub mod commands;
pub mod config;
pub mod prelude;
pub mod properties;
pub mod template;
