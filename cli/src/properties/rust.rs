//! Application properties related to the Rust programming language

use serde::{Deserialize, Serialize};

/// Rust edition used by the application (2021+)
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Edition {
    /// Rust 2021 edition (minimum supported by Abscissa)
    #[serde(rename = "2021")]
    Rust2021,
}
