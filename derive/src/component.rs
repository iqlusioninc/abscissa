//! Custom derive support for `abscissa_core::component::Component`.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, Ident, Meta, NestedMeta};
use synstructure::Structure;

/// Ident name for component attributes
const COMPONENT_IDENT: &str = "component";

/// Custom derive for `abscissa_core::component::Component`
pub fn derive_component(s: Structure) -> TokenStream {
    let derive_attributes = DeriveAttributes::parse(&s);
    let abscissa_core = derive_attributes.crate_name;
    let name = &s.ast().ident;

    s.gen_impl(quote! {
        gen impl<A> Component<A> for @Self
        where
            A: #abscissa_core::Application
        {
            #[doc = "Identifier for this component"]
            fn id(&self) -> #abscissa_core::component::Id {
                // TODO(tarcieri): use `core::any::type_name` here when stable
                #abscissa_core::component::Id::new(concat!(module_path!(), "::", stringify!(#name)))
            }

            #[doc = "Version of this component"]
            fn version(&self) -> #abscissa_core::Version {
                #abscissa_core::Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
            }
        }
    })
}

/// Derive attributes: parser for #[component(...)] attributes
// TODO(tarcieri): replace this with e.g. `darling`?
struct DeriveAttributes {
    /// Crate name for `abscissa_core`.
    ///
    /// Workaround for using this custom derive in `abscissa_core` itself. See:
    /// <https://github.com/rust-lang/rust/issues/54363>
    crate_name: Ident,
}

impl Default for DeriveAttributes {
    fn default() -> Self {
        Self {
            crate_name: Ident::new("abscissa_core", Span::call_site()),
        }
    }
}

impl DeriveAttributes {
    /// Parse `#[component(...)]` attributes from the incoming AST
    fn parse(s: &Structure) -> Self {
        let mut result = Self::default();

        for v in s.variants().iter() {
            result.parse_attributes(v.ast().attrs);
        }

        result
    }

    /// Parse `#[component(...)]` attributes
    fn parse_attributes(&mut self, attributes: &[Attribute]) {
        for attr in attributes {
            let meta = attr
                .parse_meta()
                .unwrap_or_else(|e| panic!("error parsing attribute: {} ({})", attr.tts, e));

            if let Meta::List(list) = meta {
                if list.ident != COMPONENT_IDENT {
                    return;
                }

                for nested_meta in &list.nested {
                    if let NestedMeta::Meta(Meta::Word(ident)) = nested_meta {
                        self.parse_ident_attribute(ident);
                    } else {
                        panic!("malformed #[component] attribute: {:?}", nested_meta);
                    }
                }
            }
        }
    }

    /// Parse a `#[component(...)]` attribute containing a single ident (e.g. `core`)
    fn parse_ident_attribute(&mut self, ident: &Ident) {
        if ident == "core" {
            self.crate_name = Ident::new("crate", Span::call_site());
        } else {
            panic!("unknown #[component] attribute type: {}", ident);
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
            derive_component {
                struct MyComponent {}
            }
            expands to {
                #[allow(non_upper_case_globals)]
                const _DERIVE_Component_A_FOR_MyComponent: () = {
                    impl<A> Component<A> for MyComponent
                    where
                        A: abscissa_core::Application
                    {
                        #[doc = "Identifier for this component" ]
                        fn id(&self) -> abscissa_core::component::Id {
                            abscissa_core::component::Id::new(
                                concat!(module_path!(), "::" , stringify!(MyComponent))
                            )
                        }

                        #[doc = "Version of this component"]
                        fn version(&self) -> abscissa_core::Version {
                            abscissa_core::Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
                        }
                    }
                };
            }
            no_build // tests the code compiles are in the `abscissa` crate
        }
    }
}
