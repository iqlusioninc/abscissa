//! Tests for Abscissa's component functionality

mod example_app;

use self::example_app::ExampleApp;
use abscissa_core::{component, Component};

/// ID for `FooComponent`
const FOO_COMPONENT_ID: component::Id = component::Id::new("component::FooComponent");

/// Example component
#[derive(Component, Debug, Default)]
pub struct FooComponent {
    /// Component state
    pub state: Option<String>,
}

impl FooComponent {
    /// Set the state string to a particular value
    pub fn set_state(&mut self, state_str: &str) {
        self.state = Some(state_str.to_owned());
    }
}

#[test]
fn component_registration() {
    let mut registry = component::Registry::default();
    assert!(registry.is_empty());

    let component = Box::new(FooComponent::default()) as Box<dyn Component<ExampleApp>>;
    assert_eq!(component.id(), FOO_COMPONENT_ID);

    registry.register(vec![component]).unwrap();
    assert!(!registry.is_empty());

    let foo = registry.get_by_id(FOO_COMPONENT_ID).unwrap();
    assert_eq!(foo.id(), FOO_COMPONENT_ID);
}

#[test]
fn get_downcast_ref() {
    let mut registry = component::Registry::default();
    let component = Box::new(FooComponent::default()) as Box<dyn Component<ExampleApp>>;
    registry.register(vec![component]).unwrap();

    {
        let foo_mut = registry.get_downcast_mut::<FooComponent>().unwrap();
        foo_mut.set_state("mutated!");
    }

    {
        let foo = registry.get_downcast_ref::<FooComponent>().unwrap();
        assert_eq!(foo.state.as_ref().unwrap(), "mutated!");
    }
}
