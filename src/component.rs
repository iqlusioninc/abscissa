//! Application components.

#![allow(unused_variables)]

mod name;
mod registry;

pub use self::{name::Name, registry::Registry};
use crate::{application::Application, error::FrameworkError, shutdown::Shutdown, Version};
use std::{cmp::Ordering, fmt::Debug, slice::Iter};

/// Application components.
///
/// Components are Abscissa's primary extension mechanism, and are aware of
/// the application lifecycle. They are owned by the application as boxed trait
/// objects in a runtime type registry which is aware of a dependency ordering
/// and can (potentially in the future) support runtime reinitialization.
///
/// During application initialization, callbacks are sent to all components
/// upon events like application configuration being loaded.
///
/// Additionally, they receive a callback prior to application shutdown.
// TODO(tarcieri): downcast support for accessing components as concrete types?
pub trait Component<A>: Debug + Send + Sync
where
    A: Application,
{
    /// Name of this component
    fn name(&self) -> Name;

    /// Version of this component
    fn version(&self) -> Version;

    /// Names of the components this components depends on
    fn dependencies(&self) -> Iter<'_, Name> {
        [].iter()
    }

    /// Lifecycle event called when application configuration should be loaded
    /// if it were possible.
    fn after_config(&mut self, app: Option<&A::Cfg>) -> Result<(), FrameworkError> {
        Ok(())
    }

    /// Perform any tasks which should occur before the app exits
    fn before_shutdown(&self, kind: Shutdown) -> Result<(), FrameworkError> {
        Ok(())
    }
}

impl<A> PartialEq for Box<dyn Component<A>>
where
    A: Application,
{
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl<A> PartialOrd for Box<dyn Component<A>>
where
    A: Application,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if other.dependencies().any(|dep| *dep == self.name()) {
            if self.dependencies().any(|dep| *dep == other.name()) {
                None
            } else {
                Some(Ordering::Greater)
            }
        } else if self.dependencies().any(|dep| *dep == other.name()) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}
