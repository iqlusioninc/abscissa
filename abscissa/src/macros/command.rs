//! Macros for supporting command types

/// Implement the `from_args` and `from_env_args` methods for a command
// TODO: less hax way of doing this (move into `derive(Options)`?)
#[macro_export]
macro_rules! impl_command {
    ($command:tt) => {
        impl $crate::Command for $command {
            /// Name of this program as a string
            fn name() -> &'static str {
                env!("CARGO_PKG_NAME")
            }

            /// Description of this program
            fn description() -> &'static str {
                env!("CARGO_PKG_DESCRIPTION")
            }

            /// Version of this program
            fn version() -> &'static str {
                env!("CARGO_PKG_VERSION")
            }

            /// Authors of this program
            fn authors() -> &'static str {
                env!("CARGO_PKG_AUTHORS")
            }
        }
    };
}
