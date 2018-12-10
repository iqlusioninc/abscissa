//! Default exit handlers for Abscissa applications

use failure::Error;
use std::process;

use super::{Application, Component, Components};

/// Exit gracefully (returning a status code of 0)
#[allow(unknown_lints, clippy::needless_pass_by_value)]
pub fn shutdown<A: Application>(app: &A, components: Components) -> ! {
    match components.shutdown(app) {
        Ok(()) => process::exit(0),
        Err(e) => fatal_error(app, &e.into()),
    }
}

/// Print a fatal error message and exit
pub fn fatal_error<A: Application>(app: &A, err: &Error) -> ! {
    status_err!("{} fatal error: {}", app.name(), err);
    process::exit(1)
}

/// Exit because component startup ordering could not be determined.
/// This is a barebones implementation using basic std facilities
/// because it might be called before the shell component has been
/// started, and we can't use it to log errors about itself.
pub(crate) fn bad_component_order(a: &Component, b: &Component) -> ! {
    eprintln!("*** error(abscissa): couldn't determine startup order for components:");
    eprintln!(" - {:?}", a);
    eprintln!(" - {:?}", b);
    process::exit(1)
}

/// Exit because we encountered a duplicate component name
pub(crate) fn duplicate_component_name(component: &Component) -> ! {
    eprintln!(
        "*** error(abscissa): component with duplicate name: {:?}",
        component
    );
    process::exit(1);
}
