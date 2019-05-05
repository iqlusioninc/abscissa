//! Custom derive support for the `abscissa` microframework.

#![crate_type = "proc-macro"]
#![deny(warnings, unsafe_code, unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa-sq.svg",
    html_root_url = "https://docs.rs/abscissa_derive/0.0.2"
)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use synstructure::decl_derive;

/// Custom derive for `abscissa::callable::Callable`
fn derive_callable(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.each(|bi| {
        quote! { #bi.call() }
    });

    s.gen_impl(quote! {
        gen impl Callable for @Self {
            fn call(&self) {
                match *self { #body }
            }
        }
    })
}
decl_derive!([Callable] => derive_callable);

/// Custom derive for `abscissa::command::Command`
#[proc_macro_derive(Command)]
pub fn derive_command(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let impl_command = quote! {
        impl #impl_generics Command for #name #ty_generics #where_clause {
            #[doc = "Name of this program as a string"]
            fn name() -> &'static str {
                env!("CARGO_PKG_NAME")
            }

            #[doc = "Description of this program"]
            fn description() -> &'static str {
                env!("CARGO_PKG_DESCRIPTION")
            }

            #[doc = "Version of this program"]
            fn version() -> &'static str {
                env!("CARGO_PKG_VERSION")
            }

            #[doc = "Authors of this program"]
            fn authors() -> &'static str {
                env!("CARGO_PKG_AUTHORS")
            }
        }
    };

    impl_command.into()
}
