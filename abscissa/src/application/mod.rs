//! Trait for representing an Abscissa application and it's lifecycle

use failure;
use std::str::FromStr;

mod components;
pub mod exit;

pub use self::components::{Component, Components};
use command::Command;
use config::{ConfigReader, GlobalConfig, MergeOptions};
use error::FrameworkError;
use util::{self, CanonicalPathBuf, Version};

/// Core Abscissa trait used for managing the application lifecycle.
///
/// The `Application` trait ties together the `GlobalConfig`, `Options`, and
/// `Error` types for a particular application.
///
/// It provides the main framework entrypoint: `Application::boot()`, which
/// will parse command line options and launch a given application.
// TODO: custom derive support, i.e. `derive(Command)`
#[allow(unused_variables)]
pub trait Application: Send + Sized + Sync {
    /// Application (sub)command which serves as the main entry point
    type Cmd: Command;

    /// Configuration type used by this application
    type Config: GlobalConfig + MergeOptions<Self::Cmd>;

    /// Get a read lock on the application's global configuration
    fn config(&self) -> ConfigReader<Self::Config> {
        Self::Config::get_global()
    }

    /// Name of this application as a string
    fn name(&self) -> &'static str {
        Self::Cmd::name()
    }

    /// Description of this application
    fn description(&self) -> &'static str {
        Self::Cmd::description()
    }

    /// Version of this application
    /// `::abscissa::util::Version::parse(env!("CARGO_PKG_VERSION")).unwrap()`
    fn version(&self) -> Version {
        Version::from_str(Self::Cmd::version()).unwrap_or_else(|e| self.fatal_error(e))
    }

    /// Authors of this application
    fn authors(&self) -> Vec<String> {
        Self::Cmd::authors().split(':').map(str::to_owned).collect()
    }

    /// Path to this application's binary
    fn bin(&self) -> CanonicalPathBuf {
        // TODO: handle errors?
        util::current_exe().unwrap()
    }

    /// Load this application's configuration and initialize its components
    fn init(&self, command: &Self::Cmd) -> Result<Components, FrameworkError> {
        // We do this first to ensure that the configuration is loaded
        // before the rest of the framework is initialized
        let config = self.load_config_file()?;

        // Set the global configuration to what we loaded from the config file
        // overridden with flags from the command line
        Self::Config::set_global(config.merge(command));

        // Create the application's components
        let mut components = self.components()?;

        // Initialize the components
        components.init(self)?;

        // Return the components
        Ok(components)
    }

    /// Load the application's global configuration from a file
    fn load_config_file(&self) -> Result<Self::Config, FrameworkError> {
        Self::Config::load_toml_file(self.path(ApplicationPath::ConfigFile)?)
    }

    /// Get this application's components
    fn components(&self) -> Result<Components, FrameworkError> {
        Ok(Components::default())
    }

    /// Get a path associated with this application
    fn path(&self, path_type: ApplicationPath) -> Result<CanonicalPathBuf, FrameworkError> {
        Ok(match path_type {
            //ApplicationPath::AppRoot => self.bin().parent()?,
            ApplicationPath::Bin => self.bin(),
            other => panic!("KABOOM! {:?}", other)
            //ApplicationPath::ConfigFile => Self::Config::default_location().ok_or_else(|| {
            //    self.fatal_error(err!(
            //        FrameworkErrorKind::Config,
            //        "no default configuration path configured for this config type"
            //    ))
            //}),
            //ApplicationPath::Secrets => self.bin().parent()?.join("secrets")?,
        })
    }

    /// Register a component with this application. By default do nothing.
    fn register(&self, component: &Component) -> Result<(), FrameworkError> {
        Ok(())
    }

    /// Register a component with this application. By default do nothing.
    fn unregister(&self, component: &Component) -> Result<(), FrameworkError> {
        Ok(())
    }

    /// Shut down this application gracefully, exiting with success
    fn shutdown(&self, components: Components) -> ! {
        exit::shutdown(self, components)
    }

    /// Handle a fatal error (by printing the error message and exiting by default)
    fn fatal_error<E: Into<failure::Error>>(&self, err: E) -> ! {
        exit::fatal_error(self, &err.into())
    }
}

/// Various types of paths within an application
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum ApplicationPath {
    /// Root directory for this application
    AppRoot,

    /// Path to the application's compiled binary
    Bin,

    /// Path to the application's configuration
    ConfigFile,

    /// Path to the application's secrets directory
    Secrets,
}

/// Boot an application of the given type, parsing command-line options from
/// the environment and running the appropriate `Command` type.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn boot<A: Application>(app: A) -> ! {
    // Parse command line options
    let command = A::Cmd::from_env_args();

    // Initialize the application
    let components = app.init(&command).unwrap_or_else(|e| app.fatal_error(e));

    // Exit gracefully
    app.shutdown(components)
}
