//! Iterators over components in the registry

use crate::{
    application::Application,
    component::{Component, Handle},
};

/// Iterator over the components in the arena
pub struct Iter<'a, A: Application> {
    iter: generational_arena::Iter<'a, Box<dyn Component<A>>>,
}

impl<'a, A> Iter<'a, A>
where
    A: Application,
{
    /// Create a new iterator
    pub(super) fn new(iter: generational_arena::Iter<'a, Box<dyn Component<A>>>) -> Self {
        Self { iter }
    }
}

impl<'a, A> Iterator for Iter<'a, A>
where
    A: Application,
{
    type Item = (Handle, &'a Box<dyn Component<A>>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, component)| {
            let handle = Handle::new(component.id(), index);
            (handle, component)
        })
    }
}

/// Mutable iterator over the components in the area
pub struct IterMut<'a, A: Application> {
    iter_mut: generational_arena::IterMut<'a, Box<dyn Component<A>>>,
}

impl<'a, A> IterMut<'a, A>
where
    A: Application,
{
    /// Create a new mutable iterator
    pub(super) fn new(iter_mut: generational_arena::IterMut<'a, Box<dyn Component<A>>>) -> Self {
        Self { iter_mut }
    }
}

impl<'a, A> Iterator for IterMut<'a, A>
where
    A: Application,
{
    type Item = (Handle, &'a mut Box<dyn Component<A>>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter_mut.next().map(|(index, component)| {
            let handle = Handle::new(component.id(), index);
            (handle, component)
        })
    }
}
