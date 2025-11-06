//! Application properties related to the Rust programming language

use serde::{Deserialize, Serialize};

/// Rust edition used by the application (2021+)
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Edition {
    /// Rust 2024 edition
    #[default]
    #[serde(rename = "2024")]
    Rust2024,
}
