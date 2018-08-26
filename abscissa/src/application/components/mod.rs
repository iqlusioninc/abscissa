//! Abscissa's component system: support for subsystems like logging

use std::{borrow::Borrow, cmp::Ordering, collections::HashSet};
mod component;

pub use self::component::Component;
use application::{self, Application};
use error::FrameworkError;
use logging::LoggingComponent;
use shell::ShellComponent;

/// Collections of components, sorted by dependency ordering
pub struct Components(Vec<Box<Component>>);

impl Components {
    /// Create a new collection of components
    pub fn new(components: Vec<Box<Component>>) -> Self {
        // Ensure all component names are unique
        let mut names = HashSet::new();
        for component in &components {
            if !names.insert(component.name()) {
                application::exit::duplicate_component_name((*component).borrow());
            }
        }

        let mut result = Components(components);
        result.sort();
        result
    }

    /// Initialize components and register them with the given application
    pub fn init<A: Application>(&mut self, app: &A) -> Result<(), FrameworkError> {
        for component in &mut self.0 {
            component.init()?;
            app.register((*component).borrow())?;
        }

        Ok(())
    }

    /// Shutdown components (in the reverse order they were started)
    pub fn shutdown<A: Application>(&self, app: &A) -> Result<(), FrameworkError> {
        for component in self.0.iter().rev() {
            component.shutdown()?;
            app.unregister((*component).borrow())?;
        }

        Ok(())
    }

    /// Sort components by dependency ordering, loading the components that depend
    /// on others after their dependencies.
    ///
    /// Exits the application if the ordering cannot be resolved.
    fn sort(&mut self) {
        self.0.sort_by(|a, b| {
            if b.dependencies().any(|dep| *dep == a.name()) {
                if a.dependencies().any(|dep| *dep == b.name()) {
                    application::exit::bad_component_order(a.borrow(), b.borrow());
                } else {
                    Ordering::Greater
                }
            } else if a.dependencies().any(|dep| *dep == b.name()) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
    }
}

impl Default for Components {
    fn default() -> Self {
        Self::new(vec![
            Box::new(ShellComponent::default()),
            Box::new(LoggingComponent::default()),
        ])
    }
}
