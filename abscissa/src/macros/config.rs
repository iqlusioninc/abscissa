//! Macro which `impl`s the `GlobalConfig` trait for a given
//! `Clone + DeserializeOwned` type.

/// Impl the `GlobalConfig` trait for the given `Clone + DeserializeOwned`
/// type. TODO: replace with a proc macro and `derive(GlobalConfig)`
#[macro_export]
macro_rules! impl_global_config {
    ($config_type:ty, $config_static:tt) => {
        lazy_static! {
            pub(crate) static ref $config_static: ::std::sync::RwLock<Option<$config_type>> =
                ::std::sync::RwLock::new(None);
        }

        impl $crate::config::GlobalConfig for $config_type {
            fn get() -> $crate::config::ConfigReader<Self> {
                // TODO: better handle `PoisonError`? (i.e. print a better error message)
                let config = $config_static.read().unwrap();

                if (*config).is_none() {
                    Self::not_loaded();
                }

                $crate::config::ConfigReader::new(config)
            }

            fn set(config: Self) {
                // TODO: better handle `PoisonError`?
                let mut global_config = $config_static.write().unwrap();
                *global_config = Some(config);
            }
        }
    };
}
