//! Abscissa's component registry

pub use super::Component;
use crate::{
    application::{self, Application},
    error::{FrameworkError, FrameworkErrorKind::ComponentError},
    shutdown::Shutdown,
};
use std::{any::Any, borrow::Borrow, collections::HashSet, slice};

/// The component registry provides a system for runtime registration of
/// application components which can interact with each other dynamically.
///
/// Components are sorted according to a dependency ordering, started
/// in-order, and at application termination time, shut down in reverse order.
#[derive(Debug)]
pub struct Registry<A: Application> {
    components: Vec<Box<dyn Component<A>>>,
}

impl<A> Default for Registry<A>
where
    A: Application,
{
    fn default() -> Self {
        Registry { components: vec![] }
    }
}

impl<A> Registry<A>
where
    A: Application,
{
    /// Register components, determining their dependency order
    pub fn register<I>(&mut self, components: I) -> Result<(), FrameworkError>
    where
        I: IntoIterator<Item = Box<dyn Component<A>>>,
    {
        // TODO(tarcieri): flexible runtime registration?
        ensure!(
            self.components.is_empty(),
            ComponentError,
            "no support for registering additional components (yet)"
        );

        let mut result = Registry {
            components: components.into_iter().collect::<Vec<_>>(),
        };

        // Ensure all component names are unique
        let mut names = HashSet::new();

        for component in &result.components {
            ensure!(
                names.insert(component.name()),
                ComponentError,
                "duplicate component name: {}",
                component.name()
            );
        }

        result.sort();
        Ok(())
    }

    /// Callback fired by application when configuration has been loaded
    pub fn after_config(&mut self, config: &A::Cfg) -> Result<(), FrameworkError> {
        for component in self.iter_mut() {
            component.after_config(config)?;
        }

        Ok(())
    }

    /// Iterate over the components mutably.
    pub fn iter(&mut self) -> slice::Iter<Box<dyn Component<A>>> {
        self.components.iter()
    }

    /// Iterate over the components mutably.
    pub fn iter_mut(&mut self) -> slice::IterMut<Box<dyn Component<A>>> {
        self.components.iter_mut()
    }

    /// Shutdown components (in the reverse order they were started)
    pub fn shutdown(&self, app: &A, shutdown: Shutdown) -> Result<(), FrameworkError> {
        for component in self.components.iter().rev() {
            component.before_shutdown(shutdown)?;
        }

        Ok(())
    }

    /// Sort components by dependency ordering, loading the components that depend
    /// on others after their dependencies.
    ///
    /// Exits the application if the ordering cannot be resolved.
    fn sort(&mut self) {
        self.components.sort_by(|a, b| {
            a.partial_cmp(b)
                .unwrap_or_else(|| application::exit::bad_component_order(a.borrow(), b.borrow()))
        })
    }
}

impl<A> Registry<A>
where
    A: Application + 'static,
{
    /// Get a reference to a component of the given type, if one has been registered
    pub fn get_ref<C>(&self) -> Option<&C>
    where
        C: Component<A> + 'static,
    {
        // TODO(tarcieri): more efficient implementation
        for component in self.components.iter() {
            if let Some(result) = (component as &dyn Any).downcast_ref::<C>() {
                return Some(result);
            }
        }

        None
    }

    /// Get a mutable reference component of the given type, if one has been registered
    pub fn get_mut<C>(&mut self) -> Option<&mut C>
    where
        C: Component<A> + 'static,
    {
        // TODO(tarcieri): more efficient implementation
        for component in self.components.iter_mut() {
            if let Some(result) = (component as &mut dyn Any).downcast_mut::<C>() {
                return Some(result);
            }
        }

        None
    }
}
