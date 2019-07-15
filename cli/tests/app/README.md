# App Tests

Any `*.rs` files in the `tests/app` directory will be copied to the `tests/`
directory of the generated application prior to `cargo test` being run.

This allows for programmatically testing properties and/or behavior within the
generated application.
