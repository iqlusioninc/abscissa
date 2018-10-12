//! Miscellaneous utilities

extern crate canonical_path;
#[cfg(feature = "chrono")]
pub extern crate chrono as time;
#[cfg(feature = "application")]
extern crate semver;
extern crate zeroize;

pub use self::canonical_path::{current_exe, CanonicalPath, CanonicalPathBuf};
#[cfg(feature = "application")]
pub use self::semver::Version;
pub use self::zeroize::Zeroize;

#[cfg(feature = "toml")]
pub extern crate toml;
