//! Support for managing global configuration, as well as loading it from TOML

mod global;
#[cfg(feature = "options")]
mod options;
mod reader;

pub use self::global::GlobalConfig;
#[cfg(feature = "options")]
pub use self::options::MergeOptions;
pub use self::reader::ConfigReader;
