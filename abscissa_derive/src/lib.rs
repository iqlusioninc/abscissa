//! Custom derive for the `abscissa` microframework.
//!
//! This crate provides macros for `derive(Options)`.
//!
//! For more information, see:
//!
//! * `options` module: support for deriving command-line parsers

#![crate_name = "abscissa_derive"]
#![crate_type = "rlib"]
#![allow(unknown_lints, intra_doc_link_resolution_failure, unused_attributes)]
#![deny(warnings, unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]
#![recursion_limit = "1024"]
#![doc(
    html_logo_url = "https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa-sq.svg",
    html_root_url = "https://docs.rs/abscissa_derive/0.0.2"
)]

extern crate proc_macro;
extern crate proc_macro2;
#[cfg(feature = "options")]
#[allow(unused_imports, unknown_lints, clippy::useless_attribute)]
#[macro_use]
extern crate quote;
extern crate syn;

#[cfg(feature = "options")]
use proc_macro::TokenStream;
#[cfg(feature = "options")]
use syn::{Data, DataStruct, DeriveInput, Fields};

#[cfg(feature = "options")]
mod options;

/// Derive the `Options` trait
#[cfg(feature = "options")]
#[proc_macro_derive(Options, attributes(options))]
pub fn derive_options(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Enum(ref data) => options::derive_options_enum(&ast, data),
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => panic!("cannot derive Options for unit struct types"),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(..),
            ..
        }) => panic!("cannot derive Options for tuple struct types"),
        Data::Struct(DataStruct { ref fields, .. }) => options::derive_options_struct(&ast, fields),
        Data::Union(_) => panic!("cannot derive Options for union types"),
    }
}
