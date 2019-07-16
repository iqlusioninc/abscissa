//! Templating engine for generating new Abscissa applications

#![deny(warnings, unsafe_code, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa-sq.svg",
    html_root_url = "https://docs.rs/abscissa_generator/0.2.0-rc.0"
)]

pub mod application;
pub mod commands;
pub mod config;
pub mod prelude;
pub mod properties;
pub mod template;
