//! Custom derive support for `abscissa_core::component::Component`.

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{meta::ParseNestedMeta, DeriveInput, Lit};
use synstructure::Structure;

/// Custom derive for `abscissa_core::component::Component`
pub fn derive_component(s: Structure<'_>) -> TokenStream {
    let attrs = ComponentAttributes::from_derive_input(s.ast());
    let name = &s.ast().ident;
    let abscissa_core = attrs.abscissa_core_crate();
    let dependency_methods = attrs.dependency_methods();

    s.gen_impl(quote! {
        #[allow(unknown_lints)]
        #[allow(non_local_definitions)]
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

            #dependency_methods
        }
    })
}

/// Parsed `#[component(...)]` attribute fields
#[derive(Debug)]
struct ComponentAttributes {
    /// Special attribute used by `abscissa_core` to `derive(Component)`.
    ///
    /// Workaround for using custom derive on traits defined in the same crate:
    /// <https://github.com/rust-lang/rust/issues/54363>
    core: bool,

    /// Dependent components to inject into the current component
    inject: Vec<InjectAttribute>,
}

impl ComponentAttributes {
    /// Parse component attributes from custom derive input.
    pub fn from_derive_input(input: &DeriveInput) -> Self {
        let mut core = false;
        let mut inject = Vec::new();

        for attr in &input.attrs {
            if !attr.path().is_ident("component") {
                continue;
            }

            attr.parse_nested_meta(|nested| {
                if nested.path.is_ident("core") {
                    core = true;
                    Ok(())
                } else if nested.path.is_ident("inject") {
                    inject.push(InjectAttribute::from_nested_meta(&nested)?);
                    Ok(())
                } else {
                    Err(nested.error("malformed `component` attribute"))
                }
            })
            .expect("error parsing meta");
        }

        Self { core, inject }
    }

    /// Ident for the `abscissa_core` crate.
    ///
    /// Allows `abscissa_core` itself to override this so it can consume its
    /// own traits/custom derives.
    pub fn abscissa_core_crate(&self) -> Ident {
        let crate_name = if self.core { "crate" } else { "abscissa_core" };

        Ident::new(crate_name, Span::call_site())
    }

    /// Generate `Component::dependencies()` and `register_dependencies()`
    pub fn dependency_methods(&self) -> TokenStream {
        if self.inject.is_empty() {
            return quote!();
        }

        let abscissa_core = self.abscissa_core_crate();
        let ids = self
            .inject
            .iter()
            .map(|inject| inject.id_tokens(&abscissa_core));

        let match_arms = self.inject.iter().map(|inject| inject.match_arm());

        quote! {
            fn dependencies(&self) -> std::slice::Iter<'_, #abscissa_core::component::Id> {
                const DEPENDENCIES: &[#abscissa_core::component::Id] = &[#(#ids),*];
                DEPENDENCIES.iter()
            }

            fn register_dependency(
                &mut self,
                handle: #abscissa_core::component::Handle,
                dependency: &mut dyn Component<A>,
            ) -> Result<(), FrameworkError> {
                match dependency.id().as_ref() {
                    #(#match_arms),*
                    _ => unreachable!()
                }
            }
        }
    }
}

/// Attribute declaring a dependency which should be injected
#[derive(Debug)]
pub struct InjectAttribute(String);

impl InjectAttribute {
    /// Parse an [`InjectAttribute`] from [`NestedMeta`].
    pub fn from_nested_meta(meta: &ParseNestedMeta<'_>) -> Result<Self, syn::Error> {
        let Ok(value) = meta.value() else {
            return Err(meta.error("expected value for `inject` attribute"));
        };

        let Ok(lit) = value.parse::<Lit>() else {
            return Err(value.error("expected literal for `inject` value"));
        };

        match lit {
            Lit::Str(lit_str) => Ok(Self(lit_str.value())),
            _ => Err(value.error("expected string literal for `inject` value")),
        }
    }

    /// Parse the callback and component ID of the value of an inject attribute.
    fn parse_value(&self) -> (&str, &str) {
        assert!(
            self.0.ends_with(')'),
            "expected {} to end with ')'",
            &self.0
        );

        let mut paren_parts = self.0[..(self.0.len() - 1)].split('(');
        let callback = paren_parts.next().unwrap();
        let component_id = paren_parts.next().unwrap();
        assert_eq!(paren_parts.next(), None);

        (callback, component_id)
    }

    /// Get the callback associated with this inject attribute
    pub fn callback(&self) -> Ident {
        Ident::new(self.parse_value().0, Span::call_site())
    }

    /// Get the component ID associated with this inject attribute
    pub fn component_id(&self) -> &str {
        self.parse_value().1
    }

    /// Get the tokens representing a component ID
    pub fn id_tokens(&self, abscissa_core: &Ident) -> TokenStream {
        let component_id = self.component_id();
        quote! { #abscissa_core::component::Id::new(#component_id) }
    }

    /// Get match arm that invokes a concrete callback
    pub fn match_arm(&self) -> TokenStream {
        let id_str = self.component_id();
        let callback = self.callback();

        quote! {
            #id_str => {
                let component_ref = (*dependency).as_mut_any().downcast_mut().unwrap();
                self.#callback(component_ref)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use synstructure::test_derive;

    #[test]
    fn derive_component_struct() {
        test_derive! {
            derive_component {
                struct MyComponent {}
            }
            expands to {
                const _: () = {
                    #[allow(unknown_lints)]
                    #[allow(non_local_definitions)]
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
