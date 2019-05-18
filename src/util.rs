//! Miscellaneous utilities

pub use canonical_path::{current_exe, CanonicalPath, CanonicalPathBuf};
#[cfg(feature = "chrono")]
pub use chrono as time;
#[cfg(feature = "inflector")]
pub use heck as inflector;
#[cfg(feature = "application")]
pub use semver::Version;
#[cfg(feature = "toml")]
pub use toml;
pub use zeroize::Zeroize;
