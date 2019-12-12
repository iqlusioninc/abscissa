# App Tests

Any `*.rs` files in the `tests/app` directory will be copied to the `tests/`
directory of the generated application prior to `cargo test` being run
during Abscissa's own integration testing.

This allows for programmatically testing properties and/or behavior within the
generated application without making the tests part of the default application
template.
