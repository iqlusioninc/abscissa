//! Miscellaneous utilities

extern crate canonical_path;
extern crate clear_on_drop;
#[cfg(feature = "application")]
extern crate semver;

pub use self::canonical_path::{current_exe, CanonicalPath, CanonicalPathBuf};
pub use self::clear_on_drop::{clear::Clear, ClearOnDrop};
#[cfg(feature = "application")]
pub use self::semver::Version;

#[cfg(feature = "toml")]
pub extern crate toml;
