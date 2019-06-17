//! Custom derive support for `abscissa::command::Command`.

use heck::KebabCase;
use proc_macro2::TokenStream;
use quote::quote;
use synstructure::Structure;

/// Custom derive for `abscissa::command::Command`
pub fn derive_command(s: Structure) -> TokenStream {
    let subcommand_usage = match &s.ast().data {
        syn::Data::Enum(data) => impl_subcommand_usage_for_enum(data),
        _ => quote!(),
    };

    s.gen_impl(quote! {
        gen impl Command for @Self {
            #[doc = "Name of this program as a string"]
            fn name() -> &'static str {
                env!("CARGO_PKG_NAME")
            }

            #[doc = "Description of this program"]
            fn description() -> &'static str {
                env!("CARGO_PKG_DESCRIPTION").trim()
            }

            #[doc = "Version of this program"]
            fn version() -> &'static str {
                env!("CARGO_PKG_VERSION")
            }

            #[doc = "Authors of this program"]
            fn authors() -> &'static str {
                env!("CARGO_PKG_AUTHORS")
            }

            #subcommand_usage
        }
    })
}

/// Impl `subcommand_usage` which walks the enum variants and returns
/// usage info for them.
fn impl_subcommand_usage_for_enum(data: &syn::DataEnum) -> TokenStream {
    let match_arms = data.variants.iter().map(|variant| {
        // TODO(tarcieri): support `#[options(name = "...")]` attribute
        let name = variant.ident.to_string().to_kebab_case();

        let subcommand = match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    Some(&fields.unnamed.first().unwrap().into_value().ty)
                } else {
                    None
                }
            }
            syn::Fields::Unit | syn::Fields::Named(_) => None,
        }
        .unwrap_or_else(|| panic!("command variants must be unary tuples"));

        quote! {
            #name => {
                Some(abscissa::command::Usage::for_command::<#subcommand>())
            }
        }
    });

    quote! {
        #[doc = "get usage information for the named subcommand"]
        fn subcommand_usage(command: &str) -> Option<abscissa::command::Usage> {
            match command {
                #(#match_arms)*
                _ => None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use synstructure::test_derive;

    #[test]
    fn derive_command_on_struct() {
        test_derive! {
            derive_command {
                struct MyCommand {}
            }
            expands to {
                #[allow(non_upper_case_globals)]
                const _DERIVE_Command_FOR_MyCommand: () = {
                    impl Command for MyCommand {
                        #[doc = "Name of this program as a string"]
                        fn name() -> & 'static str {
                            env!("CARGO_PKG_NAME")
                        }

                        #[doc = "Description of this program"]
                        fn description () -> & 'static str {
                            env!("CARGO_PKG_DESCRIPTION" ).trim()
                        }

                        #[doc = "Version of this program"]
                        fn version() -> & 'static str {
                            env!( "CARGO_PKG_VERSION")
                        }

                        #[doc = "Authors of this program"]
                        fn authors() -> & 'static str {
                            env!("CARGO_PKG_AUTHORS")
                        }
                    }
                };
            }
            no_build // tests the code compiles are in the `abscissa` crate
        }
    }

    #[test]
    fn derive_command_on_enum() {
        test_derive! {
            derive_command {
                enum MyCommand {
                    Foo(A),
                    Bar(B),
                    Baz(C),
                }
            }
            expands to {
                #[allow(non_upper_case_globals)]
                const _DERIVE_Command_FOR_MyCommand: () = {
                    impl Command for MyCommand {
                        #[doc = "Name of this program as a string"]
                        fn name() -> & 'static str {
                            env!("CARGO_PKG_NAME")
                        }

                        #[doc = "Description of this program"]
                        fn description () -> & 'static str {
                            env!("CARGO_PKG_DESCRIPTION" ).trim()
                        }

                        #[doc = "Version of this program"]
                        fn version() -> & 'static str {
                            env!( "CARGO_PKG_VERSION")
                        }

                        #[doc = "Authors of this program"]
                        fn authors() -> & 'static str {
                            env!("CARGO_PKG_AUTHORS")
                        }

                        #[doc = "get usage information for the named subcommand"]
                        fn subcommand_usage(command: &str) -> Option <abscissa::command::Usage > {
                            match command {
                                "foo" => {
                                    Some(abscissa::command::Usage::for_command::<A>())
                                }
                                "bar" => {
                                    Some(abscissa::command::Usage::for_command::<B>())
                                }
                                "baz" => {
                                    Some(abscissa::command::Usage::for_command::<C>())
                                }
                                _ => None
                            }
                        }
                    }
                };
            }
            no_build // tests the code compiles are in the `abscissa` crate
        }
    }
}
