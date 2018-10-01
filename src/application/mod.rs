//! Trait for representing an Abscissa application and it's lifecycle

use std::str::FromStr;

mod components;
pub mod exit;

pub use self::components::{Component, Components};
use command::Command;
use config::{ConfigReader, GlobalConfig, LoadConfig};
use error::FrameworkError;
use logging::{LoggingComponent, LoggingConfig};
use shell::{ColorConfig, ShellComponent};
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
    type Cmd: Command + LoadConfig<Self::Config>;

    /// Configuration type used by this application
    type Config: GlobalConfig;

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
    fn version(&self) -> Version {
        Version::from_str(Self::Cmd::version()).unwrap_or_else(|e| fatal_error!(self, e))
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

    /// Color configuration for this application
    fn color_config(&self, command: &Self::Cmd) -> ColorConfig {
        ColorConfig::default()
    }

    /// Load this application's configuration and initialize its components
    fn init(&self, command: &Self::Cmd) -> Result<Components, FrameworkError> {
        // Load the global configuration via the command's `LoadConfig` trait.
        // We do this first to ensure that the configuration is loaded before
        // the rest of the framework is initialized.
        command.load_global_config()?;

        // Create the application's components
        let mut components = self.components(command)?;

        // Initialize the components
        components.init(self)?;

        // Return the components
        Ok(components)
    }

    /// Get this application's components
    fn components(&self, command: &Self::Cmd) -> Result<Components, FrameworkError> {
        Ok(Components::new(vec![
            Box::new(ShellComponent::new(self.color_config(command))),
            Box::new(LoggingComponent::new(self.logging_config(command))),
        ]))
    }

    /// Get the logging configuration for this application
    fn logging_config(&self, command: &Self::Cmd) -> LoggingConfig {
        LoggingConfig::default()
    }

    /// Get a path associated with this application
    fn path(&self, path_type: ApplicationPath) -> Result<CanonicalPathBuf, FrameworkError> {
        Ok(match path_type {
            ApplicationPath::AppRoot => self.bin().parent()?,
            ApplicationPath::Bin => self.bin(),
            ApplicationPath::Secrets => self.bin().parent()?.join("secrets")?,
        })
    }

    /// Register a componen\t with this application. By default do nothing.
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
}

/// Various types of paths within an application
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum ApplicationPath {
    /// Root directory for this application
    AppRoot,

    /// Path to the application's compiled binary
    Bin,

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
    let components = app.init(&command).unwrap_or_else(|e| fatal_error!(&app, e));

    // Run the command
    command.run(&app);

    // Exit gracefully
    app.shutdown(components)
}
