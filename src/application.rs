//! Trait for representing an Abscissa application and it's lifecycle

pub(crate) mod exit;
pub mod lock;
mod name;
mod state;

pub use self::{exit::fatal_error, lock::Lock, name::Name, state::State};

#[cfg(all(feature = "signals", unix))]
use crate::signal::Signal;
use crate::{
    command::Command,
    component::Component,
    config::{Config, Configurable},
    error::{FrameworkError, FrameworkErrorKind::*},
    logging::{self, LoggingComponent},
    path::{AbsPathBuf, ExePath, RootPath},
    runnable::Runnable,
    shutdown::Shutdown,
    terminal::{component::TerminalComponent, ColorChoice},
    Version,
};
use std::{env, path::Path, process, vec};

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
    type Paths: Default + ExePath + RootPath;

    /// Run application with the given command-line arguments and running the
    /// appropriate `Command` type.
    fn run<I>(app_lock: &'static Lock<Self>, args: I)
    where
        I: IntoIterator<Item = String>,
    {
        // Parse command line options
        let command = Self::Cmd::from_args(args);

        // Initialize application
        let mut app = Self::default();
        app.init(&command).unwrap_or_else(|e| fatal_error!(&app, e));
        app_lock.set(app);

        // Run the command
        command.run();

        // Exit gracefully
        let mut app = app_lock.write();
        app.shutdown(Shutdown::Graceful);
    }

    /// Accessor for application configuration.
    fn config(&self) -> Option<&Self::Cfg>;

    /// Borrow the application state immutably.
    fn state(&self) -> &State<Self>;

    /// Borrow the application state mutably.
    fn state_mut(&mut self) -> &mut State<Self>;

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
            .map(|path| command.process_config(self.load_config(&path)?))
            .transpose()?;

        // Fire callback regardless of whether any config was loaded to
        // in order to signal state in the application lifecycle
        self.after_config(config)?;

        Ok(())
    }

    /// Initialize the framework's default set of components, potentially
    /// sourcing terminal and logging options from command line arguments.
    fn framework_components(
        &mut self,
        command: &Self::Cmd,
    ) -> Result<Vec<Box<dyn Component<Self>>>, FrameworkError> {
        let terminal = TerminalComponent::new(self.term_colors(command));
        let logging = LoggingComponent::new(self.logging_config(command))
            .expect("logging subsystem failed to initialize");

        Ok(vec![Box::new(terminal), Box::new(logging)])
    }

    /// Load configuration from the given path.
    ///
    /// Returns an error if the configuration could not be loaded.
    fn load_config(&mut self, path: &Path) -> Result<Self::Cfg, FrameworkError> {
        let canonical_path = AbsPathBuf::canonicalize(path)
            .map_err(|e| err!(ConfigError, "invalid path '{}': {}", path.display(), e))?;

        Self::Cfg::load_toml_file(&canonical_path).map_err(|e| {
            err!(
                ConfigError,
                "error loading config from '{}': {}",
                canonical_path.display(),
                e
            )
        })
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
    fn term_colors(&self, command: &Self::Cmd) -> ColorChoice {
        ColorChoice::Auto
    }

    /// Get the logging configuration for this application.
    fn logging_config(&self, command: &Self::Cmd) -> logging::Config {
        logging::Config::default()
    }

    /// Handle a Unix signal received by this application
    #[cfg(all(feature = "signals", unix))]
    fn handle_signal(&mut self, signal: Signal) -> Result<(), FrameworkError> {
        info!("received signal: {} - shutting down", signal.number());
        self.shutdown(Shutdown::Graceful)
    }

    /// Shut down this application gracefully, exiting with success.
    fn shutdown(&mut self, shutdown: Shutdown) -> ! {
        match self.state().components.shutdown(self, shutdown) {
            Ok(()) => process::exit(0),
            Err(e) => fatal_error(self, &e.into()),
        }
    }
}

/// Boot the given application, parsing subcommand and options from
/// command-line arguments, and terminating when complete.
pub fn boot<A: Application>(app_lock: &'static Lock<A>) -> ! {
    let mut args = env::args();
    assert!(args.next().is_some(), "expected one argument but got zero");
    A::run(app_lock, args);
    process::exit(0);
}
