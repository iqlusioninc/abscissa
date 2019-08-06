//! Custom derive support for the `abscissa` microframework.

#![crate_type = "proc-macro"]
#![deny(warnings, unsafe_code, unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa-sq.svg",
    html_root_url = "https://docs.rs/abscissa_derive/0.3.0"
)]

extern crate proc_macro;

mod command;
mod component;
mod config;
mod runnable;

use synstructure::decl_derive;

decl_derive!([Command] => command::derive_command);
decl_derive!([Component, attributes(component)] => component::derive_component);
decl_derive!([Config] => config::derive_config);
decl_derive!([Runnable] => runnable::derive_runnable);
