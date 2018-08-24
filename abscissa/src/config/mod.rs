//! Support for managing global configuration, as well as loading it from TOML

mod global;
mod reader;

pub use self::global::GlobalConfig;
pub use self::reader::ConfigReader;
