//! Abscissa's component registry

mod iter;

pub use self::iter::{Iter, IterMut};

use super::{handle::Handle, id::Id, Component};
use crate::{
    application::{self, Application},
    shutdown::Shutdown,
    FrameworkError,
    FrameworkErrorKind::ComponentError,
};
use generational_arena::{Arena, Index};
use std::{any::TypeId, borrow::Borrow, collections::BTreeMap};

/// Map type used by the registry
type Map<K, V> = BTreeMap<K, V>;

/// Index of component identifiers to their arena locations
type IdMap = Map<Id, Index>;

/// Index of component type IDs to their arena locations
type TypeMap = Map<TypeId, Index>;

/// The component registry provides a system for runtime registration of
/// application components which can interact with each other dynamically.
///
/// Components are sorted according to a dependency ordering, started
/// in-order, and at application termination time, shut down in reverse order.
#[derive(Debug)]
pub struct Registry<A: Application + 'static> {
    /// Generational arena of registered components
    components: Arena<Box<dyn Component<A>>>,

    /// Map of component identifiers to their indexes
    id_map: IdMap,

    /// Map of component types to their identifiers
    type_map: TypeMap,
}

impl<A> Default for Registry<A>
where
    A: Application + 'static,
{
    fn default() -> Self {
        Registry {
            components: Arena::new(),
            id_map: IdMap::new(),
            type_map: TypeMap::new(),
        }
    }
}

impl<A> Registry<A>
where
    A: Application + 'static,
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
            self.register_component(component)?;
        }

        Ok(())
    }

    /// Callback fired by application when configuration has been loaded
    pub fn after_config(&mut self, config: &A::Cfg) -> Result<(), FrameworkError> {
        let mut component_indexes: Vec<(Index, Vec<Index>)> = vec![];

        for (index, component) in &mut self.components {
            // Fire the `after_config` callback for each subcomponent.
            //
            // Note that these are fired for *all* components prior to subcomponent registration
            component.after_config(config)?;

            let mut dep_indexes = vec![];

            for id in component.dependencies() {
                if let Some(index) = self.id_map.get(id) {
                    dep_indexes.push(*index);
                } else {
                    fail!(ComponentError, "unregistered dependency ID: {}", id);
                }
            }

            component_indexes.push((index, dep_indexes));
        }

        // Fire the `register_dependency` callbacks for each component's dependencies
        for (component_index, dep_indexes) in component_indexes {
            for dep_index in dep_indexes {
                if let (Some(component), Some(dep)) =
                    self.components.get2_mut(component_index, dep_index)
                {
                    let dep_handle = Handle::new(dep.id(), dep_index);
                    component.register_dependency(dep_handle, dep.as_mut())?;
                } else {
                    // In theory we just looked all of these up and they should always be valid
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
        Some(Handle::new(id, *self.id_map.get(&id)?))
    }

    /// Get the handle for the given component, if it's registered
    pub fn get_handle(&self, component: &dyn Component<A>) -> Option<Handle> {
        self.get_handle_by_id(component.id())
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
    pub fn iter(&self) -> Iter<'_, A> {
        Iter::new(self.components.iter())
    }

    /// Iterate over the components mutably.
    pub fn iter_mut(&mut self) -> IterMut<'_, A> {
        IterMut::new(self.components.iter_mut())
    }

    /// Shutdown components (in the reverse order they were started)
    pub fn shutdown(&self, app: &A, shutdown: Shutdown) -> Result<(), FrameworkError> {
        for (_, component) in self.components.iter().rev() {
            component.before_shutdown(shutdown)?;
        }

        Ok(())
    }

    /// Get a component reference by its type
    pub fn get_downcast_ref<C>(&self) -> Option<&C>
    where
        C: Component<A>,
    {
        let index = *self.type_map.get(&TypeId::of::<C>())?;
        self.components
            .get(index)
            .and_then(|box_component| (*(*box_component)).as_any().downcast_ref())
    }

    /// Get a mutable component reference by its type
    pub fn get_downcast_mut<C>(&mut self) -> Option<&mut C>
    where
        C: Component<A>,
    {
        let index = *self.type_map.get(&TypeId::of::<C>())?;
        self.components
            .get_mut(index)
            .and_then(|box_component| (*(*box_component)).as_mut_any().downcast_mut())
    }

    /// Register an individual component.
    ///
    /// This is an internal method used by `Registry::register`.
    /// It shouldn't be exposed through the public API without careful
    /// consideration, as it's not yet designed to be used outside of
    /// that particular context.
    fn register_component(
        &mut self,
        component: Box<dyn Component<A>>,
    ) -> Result<(), FrameworkError> {
        let id = component.id();
        let version = component.version();
        let type_id = (*component).type_id();

        ensure!(
            !self.id_map.contains_key(&id) && !self.type_map.contains_key(&type_id),
            ComponentError,
            "duplicate component registration: {}",
            id
        );

        let index = self.components.insert(component);

        // Index component by ID and type
        assert!(self.id_map.insert(id, index).is_none());
        assert!(self.type_map.insert(type_id, index).is_none());

        debug!("registered component: {} (v{})", id, version);
        Ok(())
    }
}
