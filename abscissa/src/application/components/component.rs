use std::{fmt::Debug, slice::Iter};

use error::FrameworkError;
use util::Version;

/// Framework subcomponents. Handles framework initialization (but could
/// be used for a lot more).
///
/// Components must implement the `PartialOrd` trait, which will be used to
/// determine initialization order.
pub trait Component: Debug + Send + Sync {
    /// Name of this component
    fn name(&self) -> &'static str;

    /// Version of this component
    fn version(&self) -> Version;

    /// Names of the components this components depends on
    fn dependencies(&self) -> Iter<&'static str> {
        [].iter()
    }

    /// Initialize this component at the time the framework boots
    fn init(&mut self) -> Result<(), FrameworkError> {
        Ok(())
    }

    /// Shut down this component when the app shuts down
    fn shutdown(&self) -> Result<(), FrameworkError> {
        Ok(())
    }
}
