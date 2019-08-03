//! Abscissa's component registry

mod iter;

pub use self::iter::{Iter, IterMut};

use super::{handle::Handle, id::Id, Component};
use crate::{
    application::{self, Application},
    error::{FrameworkError, FrameworkErrorKind::ComponentError},
    shutdown::Shutdown,
};
use generational_arena::{Arena, Index};
use std::{borrow::Borrow, collections::BTreeMap};

/// Index of component identifiers to their arena locations
type IndexMap = BTreeMap<Id, Index>;

/// The component registry provides a system for runtime registration of
/// application components which can interact with each other dynamically.
///
/// Components are sorted according to a dependency ordering, started
/// in-order, and at application termination time, shut down in reverse order.
#[derive(Debug)]
pub struct Registry<A: Application> {
    /// Generational arena of registered components
    components: Arena<Box<dyn Component<A>>>,

    /// Map of component identifiers to their indexes
    index_map: IndexMap,
}

impl<A> Default for Registry<A>
where
    A: Application,
{
    fn default() -> Self {
        Registry {
            components: Arena::new(),
            index_map: IndexMap::new(),
        }
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

        let mut components = components.into_iter().collect::<Vec<_>>();

        components.sort_by(|a, b| {
            a.partial_cmp(b)
                .unwrap_or_else(|| application::exit::bad_component_order(a.borrow(), b.borrow()))
        });

        for component in components {
            let id = component.id();
            let index = self.components.insert(component);

            if self.index_map.insert(id, index).is_some() {
                self.components.remove(index);
                fail!(ComponentError, "duplicate component ID: {}", id);
            }
        }

        Ok(())
    }

    /// Callback fired by application when configuration has been loaded
    pub fn after_config(&mut self, config: &A::Cfg) -> Result<(), FrameworkError> {
        let mut component_indexes: Vec<(Index, Vec<Index>)> = vec![];

        for (index, component) in self.components.iter() {
            let mut dep_indexes = vec![];

            for id in component.dependencies() {
                if let Some(index) = self.index_map.get(id) {
                    dep_indexes.push(*index);
                } else {
                    fail!(ComponentError, "unregistered dependency ID: {}", id);
                }
            }

            component_indexes.push((index, dep_indexes));
        }

        for (component_index, dep_indexes) in component_indexes {
            {
                let component = self.components.get_mut(component_index).unwrap();
                component.after_config(config)?;
            }

            for dep_index in dep_indexes {
                if let (Some(component), Some(dep)) =
                    self.components.get2_mut(component_index, dep_index)
                {
                    let dep_handle = Handle::new(dep.id(), dep_index);
                    component.register_dependency(dep_handle, dep.as_mut())?;
                } else {
                    unreachable!();
                }
            }
        }

        Ok(())
    }

    /// Get the number of currently registered components
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Is the registry empty?
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Get a component reference by its handle
    pub fn get(&self, handle: Handle) -> Option<&dyn Component<A>> {
        self.components.get(handle.index).map(AsRef::as_ref)
    }

    /// Get a mutable component reference by its handle
    pub fn get_mut(&mut self, handle: Handle) -> Option<&mut (dyn Component<A> + 'static)> {
        self.components.get_mut(handle.index).map(AsMut::as_mut)
    }

    /// Get a component's handle by its ID
    pub fn get_handle_by_id(&self, id: Id) -> Option<Handle> {
        Some(Handle::new(id, *self.index_map.get(&id)?))
    }

    /// Get a component ref by its ID
    pub fn get_by_id(&self, id: Id) -> Option<&dyn Component<A>> {
        self.get(self.get_handle_by_id(id)?)
    }

    /// Get a mutable component ref by its ID
    pub fn get_mut_by_id(&mut self, id: Id) -> Option<&mut (dyn Component<A> + 'static)> {
        self.get_mut(self.get_handle_by_id(id)?)
    }

    /// Iterate over the components.
    pub fn iter(&self) -> Iter<A> {
        Iter::new(self.components.iter())
    }

    /// Iterate over the components mutably.
    pub fn iter_mut(&mut self) -> IterMut<A> {
        IterMut::new(self.components.iter_mut())
    }

    /// Shutdown components (in the reverse order they were started)
    pub fn shutdown(&self, app: &A, shutdown: Shutdown) -> Result<(), FrameworkError> {
        for (_, component) in self.components.iter().rev() {
            component.before_shutdown(shutdown)?;
        }

        Ok(())
    }
}
