//! Cargo-related application properties

use serde::{Deserialize, Serialize};
use std::{fmt, str};

/// Cargo features
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Feature(String);

impl AsRef<str> for Feature {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for Feature {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Feature(s.to_owned()))
    }
}
