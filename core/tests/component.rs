//! Tests for Abscissa's component functionality

mod example_app;

use self::example_app::{ExampleApp, ExampleConfig};
use abscissa_core::{component, Component, FrameworkError, Version};

/// ID for `FooComponent`
const FOO_COMPONENT_ID: component::Id = component::Id::new("FooComponent");

/// Example component
#[derive(Clone, Debug, Default)]
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

impl Component<ExampleApp> for FooComponent {
    fn id(&self) -> component::Id {
        FOO_COMPONENT_ID
    }

    fn version(&self) -> Version {
        Version::new(0, 0, 0)
    }

    fn after_config(&mut self, _config: &ExampleConfig) -> Result<(), FrameworkError> {
        Ok(())
    }
}

#[test]
fn component_registration() {
    let mut registry = component::Registry::default();
    assert!(registry.is_empty());

    let component = Box::new(FooComponent::default()) as Box<dyn Component<ExampleApp>>;
    registry.register(vec![component]).unwrap();
    assert!(!registry.is_empty());

    let foo = registry.get_by_id(FOO_COMPONENT_ID).unwrap();
    assert_eq!(foo.id(), FOO_COMPONENT_ID);
}
