//! Trait for representing an Abscissa application and it's lifecycle

pub(crate) mod exit;
mod name;
mod reader;
mod state;
mod writer;

pub use self::{exit::fatal_error, name::Name, reader::Reader, state::State, writer::Writer};

use crate::{
    callable::Callable,
    command::Command,
    component::{self, Component},
    config::{Config, Configurable},
    error::{FrameworkError, FrameworkErrorKind::*},
    logging::{LoggingComponent, LoggingConfig},
    path::{AbsPathBuf, ExePath, RootPath},
    shell::{ColorConfig, ShellComponent},
    shutdown::Shutdown,
    Version,
};
use std::{env, process, vec};

/// Application types implementing this trait own global application state,
/// including configuration and arbitrary other values stored within
/// application components.
///
/// Application lifecycle is handled by a set of components owned by types
/// implementing this trait. It also ties together the following:
///
/// - `Cmd`: application entrypoint
/// - `Config `: application configuration
/// - `Paths`: paths to various resources within the application
#[allow(unused_variables)]
pub trait Application: Default + Sized {
    /// Application (sub)command which serves as the main entry point.
    type Cmd: Command + Configurable<Self::Cfg>;

    /// Configuration type used by this application.
    type Cfg: Config;

    /// Paths to application resources,
    type Paths: ExePath + RootPath;

    /// Run application with the given command-line arguments and running the
    /// appropriate `Command` type.
    fn run<I>(app_state: &'static State<Self>, args: I)
    where
        I: IntoIterator<Item = String>,
    {
        // Parse command line options
        let command = Self::Cmd::from_args(args);

        // Initialize application
        let mut app = Self::default();
        app.init(&command).unwrap_or_else(|e| fatal_error!(&app, e));
        app_state.set(app);

        // Run the command
        command.call();

        // Exit gracefully
        let mut app = app_state.get_mut();
        app.shutdown(&Shutdown::Graceful);
    }

    /// Accessor for application configuration.
    fn config(&self) -> Option<&Self::Cfg>;

    /// Borrow the component registry for this application.
    fn components(&self) -> &component::Registry<Self>;

    /// Locations of various paths used by the application.
    fn paths(&self) -> &Self::Paths;

    /// Register all components used by this application
    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError>;

    /// Post-configuration lifecycle callback.
    ///
    /// Called regardless of whether config is loaded to indicate this is the
    /// time in app lifecycle when configuration would be loaded if
    /// possible.
    ///
    /// This method is responsible for invoking the `after_config` handlers on
    /// all components in the registry. This is presently done in the standard
    /// application template, but is not otherwise handled directly by the
    /// framework (as ownership precludes it).
    fn after_config(&mut self, config: Option<Self::Cfg>) -> Result<(), FrameworkError>;

    /// Load this application's configuration and initialize its components.
    fn init(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        // Create and register components with the application.
        // We do this first to calculate a proper dependency ordering before
        // application configuration is processed
        self.register_components(command)?;

        // Load configuration
        let config = command
            .config_path()
            .map(|_| self.load_config(command))
            .transpose()?;

        // Fire callback regardless of whether any config was loaded to
        // in order to signal state in the application lifecycle
        self.after_config(config)?;

        Ok(())
    }

    /// Initialize the framework's default set of components, potentially
    /// sourcing shell and logging options from command line arguments.
    fn framework_components(
        &mut self,
        command: &Self::Cmd,
    ) -> Result<Vec<Box<dyn Component<Self>>>, FrameworkError> {
        let shell = ShellComponent::new(self.term_colors(command));
        let logging = LoggingComponent::new(self.logging_config(command));
        Ok(vec![Box::new(shell), Box::new(logging)])
    }

    /// Load configuration from command's `config_path()`.
    ///
    /// Returns an error if the configuration could not be loaded or if the
    /// command's `config_path()` is none.
    fn load_config(&mut self, command: &Self::Cmd) -> Result<Self::Cfg, FrameworkError> {
        // Only attempt to load configuration if `config_path` returned the
        // path to a configuration file.
        if let Some(ref path) = command.config_path() {
            let canonical_path = AbsPathBuf::canonicalize(path)
                .map_err(|e| err!(ConfigError, "invalid path '{}': {}", path.display(), e))?;

            command.load_config_file(&canonical_path).map_err(|e| {
                err!(
                    ConfigError,
                    "error loading config from '{}': {}",
                    canonical_path.display(),
                    e
                )
            })
        } else {
            fail!(PathError, "no config path for command: {:?}", command);
        }
    }

    /// Name of this application as a string.
    fn name(&self) -> &'static str {
        Self::Cmd::name()
    }

    /// Description of this application.
    fn description(&self) -> &'static str {
        Self::Cmd::description()
    }

    /// Version of this application.
    fn version(&self) -> Version {
        Self::Cmd::version()
            .parse::<Version>()
            .unwrap_or_else(|e| fatal_error!(self, e))
    }

    /// Authors of this application.
    fn authors(&self) -> Vec<String> {
        Self::Cmd::authors().split(':').map(str::to_owned).collect()
    }

    /// Color configuration for this application.
    fn term_colors(&self, command: &Self::Cmd) -> ColorConfig {
        ColorConfig::default()
    }

    /// Get the logging configuration for this application.
    fn logging_config(&self, command: &Self::Cmd) -> LoggingConfig {
        LoggingConfig::default()
    }

    /// Shut down this application gracefully, exiting with success.
    fn shutdown(&mut self, shutdown: &Shutdown) -> ! {
        match self.components().shutdown(self, shutdown) {
            Ok(()) => process::exit(0),
            Err(e) => fatal_error(self, &e.into()),
        }
    }
}

/// Boot the given application, parsing subcommand and options from
/// command-line arguments, and terminating when complete.
pub fn boot<A: Application>(app_state: &'static State<A>) -> ! {
    let mut args = env::args();
    assert!(args.next().is_some(), "expected one argument but got zero");
    A::run(app_state, args);
    process::exit(0);
}
