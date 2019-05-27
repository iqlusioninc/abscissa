use quote::quote;

/// Custom derive for `abscissa::command::Command`
pub fn derive_command(s: synstructure::Structure) -> proc_macro2::TokenStream {
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
        }
    })
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
}
