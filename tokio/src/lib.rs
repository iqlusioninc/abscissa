//! Support for launching [Tokio] runtimes within Abscissa applications.
//!
//! # About
//!
//! Where normally you'd use something like the [`tokio::main`]
//! macro to launch the Tokio runtime, in Abscissa the framework is launched by
//! calling [`abscissa_core::boot`] from your application's `main()`.
//!
//! This means Abscissa applications need a slightly different convention for
//! starting the Tokio runtime, and ideally one which allows all application
//! subcomponents to register themselves before the runtime is started.
//!
//! This crate handles instantiating the Tokio runtime as an Abscissa [`Component`],
//! allowing other application components to express they have a Tokio dependency
//! so Abscissa can inject the Tokio component as a dependency.
//!
//! # Requirements
//!
//! - Rust 1.39+
//! - Abscissa 0.5
//! - Tokio 0.2
//!
//! # Usage
//!
//! ## Defining Abscissa components that depends on Tokio
//!
//! To register an Abscissa component with the Tokio runtime, add
//! [`TokioComponent`] as a dependency to be injected when the runtime
//! is available:
//!
//! ```
//! use abscissa_core::{Component, FrameworkError};
//! use abscissa_tokio::TokioComponent;
//!
//! #[derive(Component, Debug)]
//! #[component(inject = "init_tokio(abscissa_tokio::TokioComponent)")]
//! pub struct MyComponent {}
//!
//! impl MyComponent {
//!     pub fn new() -> Result<Self, FrameworkError> {
//!         Ok(Self {})
//!     }
//!
//!     /// Called automatically after `TokioComponent` is initialized
//!     pub fn init_tokio(&mut self, tokio_cmp: &TokioComponent) -> Result<(), FrameworkError> {
//!         // Register with the Tokio runtime here, e.g.:
//!         // `tokio_cmp.runtime()?.spawn(async { ... });`
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ## Add `TokioComponent` to your Abscissa application
//!
//! Inside of your app's `src/application.rs`, find the [`register_components`]
//! method and add [`TokioComponent`]:
//!
//! ```ignore
//! use abscissa_tokio::TokioComponent;
//!
//! fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
//!     let mut components = self.framework_components(command)?;
//!
//!     // Create `TokioComponent` and add it to your app's components here:
//!     components.push(Box::new(TokioComponent::new()?));
//!
//!     self.state.components.register(components)
//! }
//! ```
//!
//! Inside of the [`Runnable`] for one of your application's subcommands, call
//! [`abscissa_tokio::run`] with a provided [`Future`] to launch the Tokio runtime:
//!
//! ```ignore
//! use crate::application::APPLICATION;
//!
//! impl Runnable for StartCmd {
//!    fn run(&self) {
//!        abscissa_tokio::run(&APPLICATION, async {
//!            println!("now running inside the Tokio runtime");
//!        });
//!    }
//! }
//! ```
//!
//! [Tokio]: https://tokio.rs
//! [`tokio::main`]: https://docs.rs/tokio/latest/tokio/attr.main.html
//! [`abscissa_core::boot`]: https://docs.rs/abscissa_core/latest/abscissa_core/application/fn.boot.html
//! [`Component`]: https://docs.rs/abscissa_core/latest/abscissa_core/component/trait.Component.html
//! [`TokioComponent`]: https://docs.rs/abscissa_tokio/latest/abscissa_tokio/struct.TokioComponent.html
//! [`register_components`]: https://docs.rs/abscissa_core/latest/abscissa_core/application/trait.Application.html#tymethod.register_components
//! [`Runnable`]: https://docs.rs/abscissa_core/latest/abscissa_core/trait.Runnable.html
//! [`abscissa_tokio::run`]: https://docs.rs/abscissa_tokio/latest/abscissa_tokio/application/fn.run.html

#![doc(
    html_logo_url = "https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa-sq.svg",
    html_root_url = "https://docs.rs/abscissa_tokio/0.5.0"
)]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, unused_lifetimes, unused_qualifications)]

pub use tokio;

use abscissa_core::{
    application::{AppCell, Application},
    format_err, Component, FrameworkError, FrameworkErrorKind,
};
use std::future::Future;
use tokio::runtime::Runtime;

/// Run a [`Future`] on the [`Runtime`] for the provided [`Application`].
///
/// This requires that [`TokioComponent`] has been registered with the given
/// application, and can only be called once after the application has fully
/// booted.
pub fn run<A, F>(app: &'static AppCell<A>, future: F) -> Result<F::Output, FrameworkError>
where
    A: Application,
    F: Future,
{
    take_runtime(app).map(|mut runtime| runtime.block_on(future))
}

/// Extract the Tokio [`Runtime`] from [`TokioComponent`].
fn take_runtime<A>(app: &'static AppCell<A>) -> Result<Runtime, FrameworkError>
where
    A: Application,
{
    app.write()
        .state_mut()
        .components
        .get_downcast_mut::<TokioComponent>()
        .ok_or_else(|| {
            FrameworkError::from(format_err!(
                FrameworkErrorKind::ComponentError,
                "TokioComponent not registered"
            ))
        })?
        .runtime
        .take()
        .ok_or_else(|| {
            format_err!(
                FrameworkErrorKind::ComponentError,
                "TokioComponent's runtime is already taken"
            )
            .into()
        })
}

/// Component which manages initialization of a Tokio runtime within the
/// Abscissa application lifecycle.
///
/// See this crate's [toplevel documentation](index.html) for detailed usage notes.
#[derive(Component, Debug)]
pub struct TokioComponent {
    runtime: Option<Runtime>,
}

impl TokioComponent {
    /// Create a new Tokio runtime component with the default options
    pub fn new() -> Result<Self, FrameworkError> {
        Runtime::new().map(From::from).map_err(|e| {
            format_err!(
                FrameworkErrorKind::ComponentError,
                "couldn't start Tokio runtime: {}",
                e
            )
            .into()
        })
    }

    /// Borrow the runtime, to e.g. `::spawn` a future on it.
    ///
    /// Returns an error if the runtime has already been taken.
    pub fn runtime(&self) -> Result<&Runtime, FrameworkError> {
        self.runtime.as_ref().ok_or_else(|| {
            format_err!(
                FrameworkErrorKind::ComponentError,
                "Tokio runtime has already been taken!"
            )
            .into()
        })
    }

    /// Borrow the runtime mutably (e.g. to `block_on` it during startup).
    ///
    /// NOTE: If you are trying to transfer control of your application to the
    /// Tokio runtime, use the [`abscissa_tokio::run`] function instead.
    ///
    /// Returns an error if the runtime has already been taken.
    ///
    /// [`abscissa_tokio::run`]: https://docs.rs/abscissa_tokio/latest/abscissa_tokio/application/fn.run.html
    pub fn runtime_mut(&mut self) -> Result<&mut Runtime, FrameworkError> {
        self.runtime.as_mut().ok_or_else(|| {
            format_err!(
                FrameworkErrorKind::ComponentError,
                "Tokio runtime has already been taken!"
            )
            .into()
        })
    }
}

impl From<Runtime> for TokioComponent {
    fn from(runtime: Runtime) -> Self {
        Self {
            runtime: Some(runtime),
        }
    }
}
