//! Support for managing global configuration, as well as loading it from TOML

mod global;
mod load;
mod reader;

pub use self::{global::GlobalConfig, load::LoadConfig, reader::ConfigReader};
