//! Application state managed by the framework.

use crate::{application::Application, component, thread};

/// Framework-managed application state
#[derive(Debug, Default)]
pub struct State<A: Application + 'static> {
    /// Application components.
    pub components: component::Registry<A>,

    /// Application paths.
    pub paths: A::Paths,

    /// Thread manager.
    pub threads: thread::Manager,
}
