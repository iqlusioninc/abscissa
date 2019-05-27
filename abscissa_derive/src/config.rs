use quote::quote;

/// Custom derive for `abscissa::config::Config`
pub fn derive_config(s: synstructure::Structure) -> proc_macro2::TokenStream {
    s.gen_impl(quote! {
        gen impl Config for @Self {}
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use synstructure::test_derive;

    #[test]
    fn derive_config_on_struct() {
        test_derive! {
            derive_config {
                struct MyConfig {
                    attr: String,
                }
            }
            expands to {
                #[allow(non_upper_case_globals)]
                const _DERIVE_Config_FOR_MyConfig: () = {
                    impl Config for MyConfig {}
                };
            }
            no_build // tests the code compiles are in the `abscissa` crate
        }
    }
}
