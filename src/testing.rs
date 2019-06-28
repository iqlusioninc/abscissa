//! Acceptance testing for Abscissa applications.

use crate::error::{
    FrameworkError,
    FrameworkErrorKind::{ProcessError, TimeoutError},
};
use regex::Regex;
use std::{
    ffi::OsString,
    io::{self, BufRead, BufReader, Write},
    ops::{Deref, DerefMut},
    process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio},
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

    /// Capture standard output to a pipe
    capture_stdout: bool,

    /// Capture standard error to a pipe
    capture_stderr: bool,
}

impl Default for CargoRunner {
    fn default() -> Self {
        Self {
            cmd: "cargo".into(),
            args: vec![],
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            capture_stdout: false,
            capture_stderr: false,
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

    /// Enable capturing of standard output
    pub fn capture_stdout(&mut self) -> &mut Self {
        self.capture_stdout = true;
        self
    }

    /// Enable capturing of standard error
    pub fn capture_stderr(&mut self) -> &mut Self {
        self.capture_stderr = true;
        self
    }

    /// Run the given subcommand
    pub fn run(&self) -> Result<Process, FrameworkError> {
        self.print_command().unwrap();

        let stdout = if self.capture_stdout {
            Stdio::piped()
        } else {
            Stdio::inherit()
        };

        let stderr = if self.capture_stderr {
            Stdio::piped()
        } else {
            Stdio::inherit()
        };

        let child = Command::new(&self.cmd)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(stdout)
            .stderr(stderr)
            .spawn()?;

        Ok(Process::new(child, self.timeout))
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

    /// Capture standard output to a pipe
    capture_stdout: bool,

    /// Capture standard error to a pipe
    capture_stderr: bool,
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
            capture_stdout: false,
            capture_stderr: false,
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

    /// Enable capturing of standard output
    pub fn capture_stdout(&mut self) -> &mut Self {
        self.capture_stdout = true;
        self
    }

    /// Enable capturing of standard error
    pub fn capture_stderr(&mut self) -> &mut Self {
        self.capture_stderr = true;
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

        if self.capture_stdout {
            runner.capture_stdout();
        }

        if self.capture_stderr {
            runner.capture_stderr();
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

    /// Standard output (if captured)
    stdout: Option<Stdout>,

    /// Standard error (if captured)
    stderr: Option<Stderr>,

    /// Standard input
    stdin: ChildStdin,
}

impl Process {
    /// Create a process from the given `Child`.
    ///
    /// This gets invoked from `CargoRunner::run`
    fn new(mut child: Child, timeout: Duration) -> Self {
        let stdout = child.stdout.take().map(Stdout::new);
        let stderr = child.stderr.take().map(Stderr::new);
        let stdin = child.stdin.take().unwrap();
        Self {
            child,
            timeout,
            stdout,
            stderr,
            stdin,
        }
    }

    /// Gets a handle to the child's stdout.
    ///
    /// Panics if the child's stdout isn't captured (via `capture_stdout`)
    pub fn stdout(&mut self) -> &mut Stdout {
        self.stdout
            .as_mut()
            .expect("child stdout not captured (use 'capture_stdout' method)")
    }

    /// Gets a handle to the child's stderr.
    ///
    /// Panics if the child's stderr isn't captured (via `capture_stderr`)
    pub fn stderr(&mut self) -> &mut Stderr {
        self.stderr
            .as_mut()
            .expect("child stderr not captured (use 'capture_stderr' method)")
    }

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

impl Write for Process {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdin.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdin.flush()
    }
}

/// Buffered reader for standard output
#[derive(Debug)]
pub struct Stdout(BufReader<ChildStdout>);

/// Buffered reader for standard error
#[derive(Debug)]
pub struct Stderr(BufReader<ChildStderr>);

macro_rules! impl_output_stream {
    ($name:tt, $inner:ty) => {
        impl $name {
            /// Create standard output wrapper
            fn new(stream: $inner) -> $name {
                $name(BufReader::new(stream))
            }

            /// Read a line and ensure it matches the expected value.
            ///
            /// Panics if it is not the expected value.
            pub fn expect_line(&mut self, expected_line: &str) {
                let mut actual_line = String::new();
                self.0.read_line(&mut actual_line).unwrap_or_else(|e| {
                    panic!("error reading line from {}: {}", stringify!($name), e)
                });

                assert_eq!(expected_line, actual_line.trim_end_matches('\n'));
            }

            /// Read a line and test it against the given regex.
            ///
            /// Panics if the line does not match the regex.
            pub fn expect_regex(&mut self, regex: &str) {
                let r = Regex::new(regex)
                    .unwrap_or_else(|e| panic!("error compiling regex {:?}: {}", regex, e));

                let mut line = String::new();
                self.0.read_line(&mut line).unwrap_or_else(|e| {
                    panic!("error reading line from {}: {}", stringify!($name), e)
                });

                assert!(
                    r.is_match(line.trim_end_matches('\n')),
                    "regex {:?} did not match line: {:?}",
                    regex,
                    line
                );
            }
        }

        impl Deref for $name {
            type Target = BufReader<$inner>;

            fn deref(&self) -> &BufReader<$inner> {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut BufReader<$inner> {
                &mut self.0
            }
        }
    };
}

impl_output_stream!(Stdout, ChildStdout);
impl_output_stream!(Stderr, ChildStderr);

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
    pub fn expect_success(&self) {
        assert_eq!(
            0, self.code,
            "process exited with error status: {}",
            self.code
        );
    }

    /// Assert that the process exited with the given code
    pub fn expect_code(&self, code: i32) {
        assert_eq!(
            code, self.code,
            "process exited with status code: {} (expected {})",
            self.code, code
        )
    }
}
