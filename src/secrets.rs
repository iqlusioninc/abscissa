//! Marker newtype for more carefully handling secret values
/// (e.g. passwords, cryptographic keys, access tokens or other credentials)
use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use std::fmt::{self, Debug};

use util::Clear;

/// Marker newtype for serde-serializable values that contain secrets
/// (e.g. passwords, cryptographic keys, access tokens or other credentials)
#[derive(Clone)]
pub struct Secret<T>(T)
where
    T: Clear + Clone + DebugSecret + DeserializeOwned + Sized;

impl<T> BorrowSecret<T> for Secret<T>
where
    T: Clear + Clone + DebugSecret + DeserializeOwned + Sized,
{
    fn borrow_secret(&self) -> &T {
        &self.0
    }
}

impl<T> Debug for Secret<T>
where
    T: Clear + Clone + DebugSecret + DeserializeOwned + Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Secret({})", self.0.debug_secret())
    }
}

impl<'de, T> Deserialize<'de> for Secret<T>
where
    T: Clear + Clone + DebugSecret + DeserializeOwned + Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Secret)
    }
}

impl<T> Drop for Secret<T>
where
    T: Clear + Clone + DebugSecret + DeserializeOwned + Sized,
{
    fn drop(&mut self) {
        // Zero the secret out from memory
        self.0.clear();
    }
}

/// Trait for obtaining references to secrets
pub trait BorrowSecret<BorrowedSecret> {
    /// Obtain a borrowed reference to a secret
    fn borrow_secret(&self) -> &BorrowedSecret;
}

/// Debugging trait which is specialized for handling secret values
pub trait DebugSecret {
    /// Information about what the secret contains. Static so as to discourage
    /// covert channels.
    fn debug_secret(&self) -> &'static str;
}
