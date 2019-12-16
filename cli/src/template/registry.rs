//! Helper for creating a Handlebars registry
//!
//! This is a workaround for `handlebars::registry::Registry` not being
//! exported as part of the public API.
//!
//! So, instead, we use a macro.

#[macro_export]
macro_rules! handlebars_registry {
    () => {{
        let mut hbs = handlebars::Handlebars::new();
        hbs.set_strict_mode(true);
        hbs.register_escape_fn(handlebars::no_escape);
        hbs
    }};
}
