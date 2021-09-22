//! Logging configuration

/// Tracing configuration
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub(super) filter: String,
}

impl Config {
    /// Create a config for verbose output.
    pub fn verbose() -> Self {
        "debug".to_owned().into()
    }
}

impl Default for Config {
    fn default() -> Self {
        match std::env::var("RUST_LOG") {
            Ok(val) => {
                if val.is_empty() {
                    val.into()
                } else {
                    "info".to_owned().into()
                }
            }
            Err(_) => "info".to_owned().into(),
        }
    }
}

impl From<String> for Config {
    fn from(filter: String) -> Self {
        Self { filter }
    }
}
