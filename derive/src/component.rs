//! Custom derive support for `abscissa_core::component::Component`.

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Lit, Meta, MetaList, MetaNameValue, NestedMeta};
use synstructure::Structure;

/// Custom derive for `abscissa_core::component::Component`
pub fn derive_component(s: Structure<'_>) -> TokenStream {
    let attrs = ComponentAttributes::from_derive_input(s.ast());
    let name = &s.ast().ident;
    let abscissa_core = attrs.abscissa_core_crate();
    let after_config = attrs.after_config();
    let before_shutdown = attrs.before_shutdown();
    let dependency_methods = attrs.dependency_methods();

    let body = quote! {
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

        #after_config

        #before_shutdown
    };

    s.gen_impl(match attrs.application() {
        Some(application) => {
            quote! {
                gen impl Component<#application> for @Self {
                    #body
                }
            }
        }
        None => quote! {
            gen impl<A> Component<A> for @Self
            where
                A: #abscissa_core::Application
            {
                #body
            }
        },
    })
}

/// Parsed `#[component(...)]` attribute fields
#[derive(Debug)]
struct ComponentAttributes {
    after_config: bool,

    application: Option<String>,

    before_shutdown: bool,

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
        let mut after_config = false;
        let mut application = None;
        let mut before_shutdown = false;
        let mut core = false;
        let mut inject = Vec::new();

        for attr in &input.attrs {
            if !attr.path.is_ident("component") {
                continue;
            }

            match attr.parse_meta().expect("error parsing meta") {
                Meta::List(MetaList { nested, .. }) => {
                    for meta in &nested {
                        match meta {
                            NestedMeta::Meta(Meta::Path(path)) => match path.get_ident() {
                                Some(id) if id == "after_config" => after_config = true,
                                Some(id) if id == "before_shutdown" => before_shutdown = true,
                                Some(id) if id == "core" => core = true,
                                _ => panic!("malformed `component` attribute: {:?}", meta),
                            },
                            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                                path,
                                lit: Lit::Str(lit_str),
                                ..
                            })) if path.is_ident("application") => {
                                application = Some(lit_str.value())
                            }
                            NestedMeta::Meta(Meta::NameValue { .. }) => {
                                inject.push(InjectAttribute::from_nested_meta(meta))
                            }
                            _ => panic!("malformed `component` attribute: {:?}", meta),
                        }
                    }
                }
                other => panic!("malformed `component` attribute: {:?}", other),
            };
        }

        Self {
            after_config,
            application,
            before_shutdown,
            core,
            inject,
        }
    }

    /// Ident for the `abscissa_core` crate.
    ///
    /// Allows `abscissa_core` itself to override this so it can consume its
    /// own traits/custom derives.
    pub fn abscissa_core_crate(&self) -> Ident {
        let crate_name = if self.core { "crate" } else { "abscissa_core" };

        Ident::new(crate_name, Span::call_site())
    }

    pub fn after_config(&self) -> TokenStream {
        let abscissa_core = self.abscissa_core_crate();

        match (self.after_config, self.application()) {
            (false, _) => quote!(),
            (true, None) => quote! {
                fn after_config(&mut self, config: &A::Cfg) -> Result<(), FrameworkError> {
                    self.after_config::<A>(config)
                }
            },
            (true, Some(application)) => quote! {
                fn after_config(&mut self, config: &<#application as #abscissa_core::Application>::Cfg) -> Result<(), FrameworkError> {
                    self.after_config(config)
                }
            },
        }
    }

    pub fn application(&self) -> Option<Ident> {
        self.application
            .as_ref()
            .map(|application| Ident::new(application.as_ref(), Span::call_site()))
    }

    pub fn before_shutdown(&self) -> TokenStream {
        if !self.before_shutdown {
            return quote!();
        }

        quote! {
            fn before_shutdown(&self, kind: Shutdown) -> Result<(), FrameworkError> {
                self.before_shutdown(kind)
            }
        }
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
    pub fn from_nested_meta(meta: &NestedMeta) -> Self {
        match meta {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(lit_str),
                ..
            })) if path.is_ident("inject") => Self(lit_str.value()),
            _ => panic!("malformed `component` attribute: {:?}", meta),
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
