//! Miscellaneous utilities

#[cfg(feature = "chrono")]
pub extern crate chrono as time;
#[cfg(feature = "toml")]
pub extern crate toml;

pub use canonical_path::{current_exe, CanonicalPath, CanonicalPathBuf};
#[cfg(feature = "application")]
pub use semver::Version;
pub use zeroize::Zeroize;
