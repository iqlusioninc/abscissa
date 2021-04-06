//! Trait for representing an Abscissa application and it's lifecycle

pub mod cell;
pub(crate) mod exit;
mod name;
mod state;

pub use self::{cell::AppCell, exit::fatal_error, name::Name, state::State};

use crate::{
    command::Command,
    component::Component,
    config::{self, Config, Configurable},
    path::{AbsPathBuf, ExePath, RootPath},
    runnable::Runnable,
    shutdown::Shutdown,
    terminal::{component::Terminal, ColorChoice},
    trace::{self, Tracing},
    FrameworkError,
    FrameworkErrorKind::*,
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
pub trait Application: Default + Sized + 'static {
    /// Application (sub)command which serves as the main entry point.
    type Cmd: Command + Configurable<Self::Cfg>;

    /// Configuration type used by this application.
    type Cfg: Config;

    /// Paths to application resources,
    type Paths: Default + ExePath + RootPath;

    /// Run application with the given command-line arguments and running the
    /// appropriate `Command` type.
    fn run<I>(app_cell: &'static AppCell<Self>, args: I)
    where
        I: IntoIterator<Item = String>,
    {
        // Parse command line options
        let command = Self::Cmd::from_args(args);

        // Initialize application
        let mut app = Self::default();
        app.init(&command).unwrap_or_else(|e| fatal_error(&app, &e));
        app_cell.set_once(app);

        // Run the command
        command.run();

        // Exit gracefully
        app_cell.shutdown(Shutdown::Graceful);
    }

    /// Accessor for application configuration.
    fn config(&self) -> config::Reader<Self::Cfg>;

    /// Borrow the application state.
    fn state(&self) -> &State<Self>;

    /// Register all components used by this application.
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
    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError>;

    /// Load this application's configuration and initialize its components.
    fn init(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        // Create and register components with the application.
        // We do this first to calculate a proper dependency ordering before
        // application configuration is processed
        self.register_components(command)?;

        // Load configuration
        let config = command
            .config_path()
            .map(|path| self.load_config(&path))
            .transpose()?
            .unwrap_or_default();

        // Fire callback regardless of whether any config was loaded to
        // in order to signal state in the application lifecycle
        self.after_config(command.process_config(config)?)?;

        Ok(())
    }

    /// Initialize the framework's default set of components, potentially
    /// sourcing terminal and tracing options from command line arguments.
    fn framework_components(
        &mut self,
        command: &Self::Cmd,
    ) -> Result<Vec<Box<dyn Component<Self>>>, FrameworkError> {
        let terminal = Terminal::new(self.term_colors(command));
        let tracing = Tracing::new(self.tracing_config(command), self.term_colors(command))
            .expect("tracing subsystem failed to initialize");

        Ok(vec![Box::new(terminal), Box::new(tracing)])
    }

    /// Load configuration from the given path.
    ///
    /// Returns an error if the configuration could not be loaded.
    fn load_config(&mut self, path: &Path) -> Result<Self::Cfg, FrameworkError> {
        let canonical_path = AbsPathBuf::canonicalize(path).map_err(|e| {
            let path_error = PathError {
                name: Some(path.into()),
            };
            FrameworkError::from(ConfigError.context(path_error))
        })?;
        Self::Cfg::load_toml_file(&canonical_path)
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
            .unwrap_or_else(|e| fatal_error(self, &e))
    }

    /// Authors of this application.
    fn authors(&self) -> Vec<String> {
        Self::Cmd::authors().split(':').map(str::to_owned).collect()
    }

    /// Color configuration for this application.
    fn term_colors(&self, command: &Self::Cmd) -> ColorChoice {
        ColorChoice::Auto
    }

    /// Get the tracing configuration for this application.
    fn tracing_config(&self, command: &Self::Cmd) -> trace::Config {
        trace::Config::default()
    }

    /// Shut down this application gracefully, exiting with success.
    fn shutdown(&self, shutdown: Shutdown) -> ! {
        let components = self.state().components();

        if let Err(e) = components.shutdown(self, shutdown) {
            fatal_error(self, &e)
        }

        process::exit(0);
    }
}

/// Boot the given application, parsing subcommand and options from
/// command-line arguments, and terminating when complete.
pub fn boot<A: Application>(app_cell: &'static AppCell<A>) -> ! {
    let args = env::args();
    A::run(app_cell, args);
    process::exit(0);
}
