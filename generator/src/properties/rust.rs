//! Application properties related to the Rust programming language

use serde::{Deserialize, Serialize};

/// Rust edition used by the application (2018+)
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Edition {
    /// Rust 2018 edition (minimum supported by Abscissa)
    #[serde(rename = "2018")]
    Rust2018,
}
