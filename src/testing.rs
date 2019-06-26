//! Acceptance testing for Abscissa applications.

use crate::error::{
    FrameworkError,
    FrameworkErrorKind::{ProcessError, TimeoutError},
};
use std::{
    ffi::OsString,
    io::{self, Write},
    process::{Child, Command},
    time::Duration,
};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use wait_timeout::ChildExt;

/// Length of the default timeout in seconds (30 minutes)
const DEFAULT_TIMEOUT_SECS: u64 = 1800;

/// Run a command via `cargo run`
#[derive(Clone, Debug)]
pub struct CargoRunner {
    /// Command to run (cargo)
    cmd: OsString,

    /// Arguments to pass to the executable
    args: Vec<OsString>,

    /// How long to wait until a child exits
    timeout: Duration,
}

impl Default for CargoRunner {
    fn default() -> Self {
        Self {
            cmd: "cargo".into(),
            args: vec![],
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }
}

impl CargoRunner {
    /// Create a new `cargo` runner.
    pub fn new<I, S>(args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        let mut runner = Self::default();
        runner.args(args);
        runner
    }

    /// Append an argument to the set of arguments to run
    pub fn arg<S>(&mut self, arg: S) -> &mut Self
    where
        S: Into<OsString>,
    {
        self.args.push(arg.into());
        self
    }

    /// Append multiple arguments to the set of arguments to run
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.args.extend(args.into_iter().map(|a| a.into()));
        self
    }

    /// Set the timeout after which the command should complete
    pub fn timeout(&mut self, duration: Duration) -> &mut Self {
        self.timeout = duration;
        self
    }

    /// Run the given subcommand
    pub fn run(&self) -> Result<Process, FrameworkError> {
        self.print_command().unwrap();

        let child = Command::new(&self.cmd).args(&self.args).spawn()?;

        Ok(Process {
            child,
            timeout: self.timeout,
        })
    }

    /// Get the exit status for the given subcommand
    pub fn status(&self) -> Result<ExitStatus, FrameworkError> {
        self.run()?.wait()
    }

    /// Print the command we're about to run
    fn print_command(&self) -> Result<(), io::Error> {
        let stdout = BufferWriter::stdout(ColorChoice::Auto);
        let mut buffer = stdout.buffer();

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(&mut buffer, "+ ")?;

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)).set_bold(true))?;
        write!(&mut buffer, "run")?;

        buffer.reset()?;

        let cmd = self.cmd.to_string_lossy();
        let args: Vec<_> = self.args.iter().map(|arg| arg.to_string_lossy()).collect();
        writeln!(&mut buffer, ": {} {}", cmd, args.join(" "))?;

        stdout.print(&buffer)
    }
}

/// Run a command via `cargo run`
#[derive(Clone, Debug, Default)]
pub struct CmdRunner {
    /// Option to pass as the `--bin` argument to `cargo run`
    bin: Option<OsString>,

    /// Arguments to pass to the executable
    args: Vec<OsString>,

    /// Timeout after which command should complete.
    timeout: Option<Duration>,
}

impl CmdRunner {
    /// Provide a `--bin` to `cargo run`. Use `CmdRunner::default()` if you
    /// only have one binary in your project.
    pub fn new<S>(bin: S) -> Self
    where
        S: Into<OsString>,
    {
        Self {
            bin: Some(bin.into()),
            args: vec![],
            timeout: None,
        }
    }

    /// Append an argument to the set of arguments to run
    pub fn arg<S>(&mut self, arg: S) -> &mut Self
    where
        S: Into<OsString>,
    {
        self.args.push(arg.into());
        self
    }

    /// Append multiple arguments to the set of arguments to run
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.args.extend(args.into_iter().map(|a| a.into()));
        self
    }

    /// Set the timeout after which the command should complete.
    ///
    /// By default `CargoRunner` timeout will be used (30 minutes).
    pub fn timeout(&mut self, duration: Duration) -> &mut Self {
        self.timeout = Some(duration);
        self
    }

    /// Invoke `cargo run` with the given arguments
    pub fn run(&self) -> Result<Process, FrameworkError> {
        let mut runner = CargoRunner::default();

        // Invoke `cargo run`.
        runner.arg("run");

        // Add optional `--bin` argument.
        if let Some(ref bin) = self.bin {
            runner.arg("--bin");
            runner.arg(bin);
        }

        if !self.args.is_empty() {
            runner.arg("--");
            runner.args(&self.args);
        }

        runner.run()
    }

    /// Get the exit status after invoking the given command
    pub fn status(&self) -> Result<ExitStatus, FrameworkError> {
        self.run()?.wait()
    }
}

/// Subprocess spawned by a `CmdRunner`
#[derive(Debug)]
pub struct Process {
    /// Child process
    child: Child,

    /// Timeout after which process should complete
    timeout: Duration,
}

impl Process {
    /// Wait for the child to exit
    pub fn wait(mut self) -> Result<ExitStatus, FrameworkError> {
        match self.child.wait_timeout(self.timeout)? {
            Some(status) => {
                let code = status.code().ok_or_else(|| {
                    err!(ProcessError, "no exit status returned from subprocess!")
                })?;
                Ok(ExitStatus { code })
            }
            None => fail!(
                TimeoutError,
                "operation timed out after {} seconds",
                self.timeout.as_secs()
            ),
        }
    }
}

/// Information about a process's exit status
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ExitStatus {
    code: i32,
}

impl ExitStatus {
    /// Get the exit code
    pub fn code(&self) -> i32 {
        self.code
    }

    /// Did the process exit successfully?
    pub fn success(&self) -> bool {
        self.code == 0
    }

    /// Assert that the process exited successfully
    pub fn assert_success(&self) {
        assert_eq!(
            self.code, 0,
            "process exited with error status: {}",
            self.code
        );
    }
}
