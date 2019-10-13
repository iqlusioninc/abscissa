//! Custom derive support for the `abscissa` microframework.

#![crate_type = "proc-macro"]
#![doc(
    html_logo_url = "https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa-sq.svg",
    html_root_url = "https://docs.rs/abscissa_derive/0.4.0"
)]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, unused_lifetimes, unused_qualifications)]

mod command;
mod component;
mod config;
mod runnable;

use synstructure::decl_derive;

decl_derive!([Command] => command::derive_command);
decl_derive!([Component, attributes(component)] => component::derive_component);
decl_derive!([Config] => config::derive_config);
decl_derive!([Runnable] => runnable::derive_runnable);
