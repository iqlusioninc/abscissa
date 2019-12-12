# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.5.0 (UNRELEASED)
### Changed
#### Replaced `lazy_static` with `once_cell` ([#167], [#168])

Abscissa primarily used `lazy_static` in two places:

- To define the `APPLICATION` constant in `application.rs`
- To provide a shared `RUNNER` object for invoking CLI tests in
  `tests/acceptance.rs`

Both usages have now been replaced with [`once_cell`], which is a better
fit for Abscissa's internal usages, and seems to stand a reasonable
chance of being incorporated into the Rust standard library:

https://github.com/rust-lang/rfcs/pull/2788

Existing applications will need to replace the `lazy_static!` directive
in `application.rs`:

```rust
lazy_static! {
    /// Application state
    pub static ref APPLICATION: application::Lock<MyApplication> = application::Lock::default();
}
```

with one based on the new [`AppCell`] type:

```rust
use abscissa_core::application::AppCell;

/// Application state
pub static APPLICATION: AppCell<MyApplication> = AppCell::new();
```

This change should otherwise be largely a drop-in replacement, and also
means you can remove `lazy_static` from `[dependencies]` in `Cargo.toml`.

For replacing `RUNNER` in `tests/acceptance.rs`, we suggest you edit
`Cargo.toml`, adding the following:

```toml
[dev-dependencies]
once_cell = "1.2"
```

And then changing the following in `tests/acceptance.rs`:

```rust
lazy_static! {
    pub static ref RUNNER: CmdRunner = CmdRunner::default();
}
```

to:

```rust
use once_cell::sync::Lazy;

pub static RUNNER: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
```

[`once_cell`]: https://docs.rs/once_cell
[`AppCell`]: https://docs.rs/abscissa_core/latest/abscissa_core/application/cell/type.AppCell.html

#### Replaced `log` with `tracing` ([#154])

[`tracing`] is a newer, more powerful application-level tracing library
which also subsumes the functionality of a logging framework, and
replaces Abscissa's previous `logging` component.

Existing apps will need to update their `application.rs`. Change the
following at the bottom of the trait impl:

```rust
impl Application {
    // ...

    /// Get logging configuration from command-line options
    fn logging_config(&self, command: &MyCommand) -> logging::Config {
        if command.verbose() {
            logging::Config::verbose()
        } else {
            logging::Config::default()
        }
    }
}
```

to:

```rust
use abscissa_core::trace;

impl Application {
    // ...

    /// Get tracing configuration from command-line options
    fn tracing_config(&self, command: &EntryPoint<MyCommand>) -> trace::Config {
        if command.verbose {
            trace::Config::verbose()
        } else {
            trace::Config::default()
        }
    }
}
```

[`tracing`]: https://github.com/tokio-rs/tracing

### Removed
#### Replaced `Config` custom derive with blanket impl ([#152])

The previous custom derive for `Config` was replaced with a blanket
impl instead. This only requires that you remove the `derive` for
`Config` in existing applications' `config.rs`. Change:

```rust
use abscissa_core::{Config, EntryPoint};

/// MyApp Configuration
#[derive(Clone, Config, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MyConfig { 
    // ...
}
```

to:

```rust
/// MyApp Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MyConfig { 
    // ...
}
```

(i.e. remove `Config` from the `#[derive(...)]` section)

#### Removed `failure` ([#151])

Since the time `failure` was written, `std::error::Error` has gained
many of the features it was originally useful for. Many libraries
now rely on it for bounds, especially in conjunction with `Box` for
type erasure.

There is no prescribed replacement for `failure` at a framework-level
(yet, see [#144] for some discussion).

The new `abscissa_core::error::Context` type subsumes the functionality
of both the previous `abscissa_core::error::Error` type and its previous
usages of `failure::Context`, e.g. capturing error sources and
backtraces.

All existing usages of `abscissa_core::error::Error` will need to be
updated, per the instructions below.

Existing apps are advised to do the following:

- `Cargo.toml`: remove `failure`
- `error.rs`: remove `failure` imports and `Fail` derive. `thiserror`
  or `err-derive` are the suggested custom derive replacements.
  The `Error` type definition should be changed from:
  
```rust
/// Error type
#[derive(Debug)]
pub struct Error(abscissa_core::Error<ErrorKind>);
```

to:

```rust
use abscissa_core::error::Context;

/// Error type
#[derive(Debug)]
pub struct Error(Box<Context<ErrorKind>>);
```

- `error.rs`: add `impl ErrorKind` with `context` method:
  this is used for capturing an error source when using your own
  application's `ErrorKind` type, ala similar functionality in
  `failure`. Add the following impl to `ErrorKind`, allowing you to
  capture an error context for any error type which meets the trait
  bounds specified by the [`BoxError`] type:

```rust
use abscissa_core::error::BoxError;

impl ErrorKind {
    /// Create an error context from this error
    pub fn context(self, source: impl Into<BoxError>) -> Context<ErrorKind> {
        Context::new(self, Some(source.into()))
    }
}
```

- `error.rs`: replace `ErrorKind` impl of `From` with one for `Context`.
  After using the above `context` method, it's useful be able to convert
  it `into()` your application's wrapper type, particularly when writing
  `From` coercions from foreign error types.
  
Change:
 
```rust 
impl From<abscissa_core::Error<ErrorKind>> for Error {
    fn from(other: abscissa_core::Error<ErrorKind>) -> Self {
        Error(other)
    }
}
```

to:

```rust
use abscissa_core::error::Context;

impl From<Context<ErrorKind>> for Error {
    fn from(context: Context<ErrorKind>) -> Self {
        Error(Box::new(context))
    }
}
```

- `error.rs`: update `Error` impl of `Deref`:

```rust
impl Deref for Error {
    type Target = abscissa_core::Error<ErrorKind>;

    fn deref(&self) -> &abscissa_core::Error<ErrorKind> {
        &self.0
    }
}
```

to:

```rust
impl Deref for Error {
    type Target = Context<ErrorKind>;

    fn deref(&self) -> &Context<ErrorKind> {
        &self.0
    }
}
```

- `error.rs`: add a `std::error::Error` impl for the app `Error` type:

```rust
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}
```

[`BoxError`]: https://docs.rs/abscissa_core/latest/abscissa_core/error/type.BoxError.html

## [0.4.0] (2019-10-13)

- Update dependencies: `gumdrop` 0.7, `secrecy` 0.4 ([#141])
- template: Lint for 2018 edition idioms; export error macros ([#136])
- Improve lints and deny policy ([#135])
- derive: Update to 1.0 versions of proc-macro2/quote/syn ([#134])
- Support positional arguments in usage descriptions ([#131])
- Fix issues with the signal handler thread ([#130])
- template: Add `Deref` impl on Error newtype ([#129])

## [0.3.0] (2019-08-05)

- usage: Use bold rather than explicit color ([#126])
- Switch from `heck` to `ident_case` ([#124])
- component: Ensure registration will succeed before mutating ([#122])
- component: Add #[component(dep = "...")] attribute ([#121])
- derive: Custom derive support for `Component` ([#119])
- component: Index Registry by TypeId ([#118])
- component: Add `get_downcast_ref`/`get_downcast_mut` to `Registry` ([#117])
- component: Add get_mut methods to `Registry` ([#115])
- component: Use `generational-arena` for component storage ([#113])
- component: allow `Registry::iter()` without mutable reference([#110])

## [0.2.1] (2019-07-27)

- usage: Fix bugs when traversing subcommands ([#107])
- usage: Fix panic when printing usage on error ([#106])

## [0.2.0] (2019-07-16)

- Merge `abscissa_generator` into `abscissa` CLI crate ([#95])
- Rename `abscissa` crate to `abscissa_core` ([#94])
- generator: Remove `hashbrown` dependency ([#93])
- testing: Add Regex newtype ([#91])
- config: Mandate `Default` bound + testing support ([#84], [#90])
- generator: Add `serde(deny_unknown_fields)` ([#88])
- generator: Add config filename boilerplate to template ([#82])
- Configuration loading improvements ([#81])
- Refactor and improve `abscissa::testing` ([#78])

## [0.1.0] (2019-07-02)

- components: Add basic downcasting support ([#72])
- components: impl `PartialOrd` for `Box<dyn Component>` ([#71])
- generator: Run `git init` automatically ([#69])
- abscissa new: add `--force` option ([#68])
- testing: Support for capturing/asserting on streams ([#65])
- testing: Add initial testing subsystem ([#64])
- Upgrade to `gumdrop` v0.6 ([#63])
- command: Show full usage on error with no args ([#61])
- Command usage improvements ([#59])
- entrypoint: Use config arg ([#58])
- terminal: Persistent stdout/stderr streams ([#57])
- Integrated logging support ([#56])
- abscissa_generator: Use `Error`/`ErrorKind` names ([#55])
- Add `app_config()`/`app_reader()`/`app_writer()` and `prelude` to template ([#52])
- abscissa_derive: Test all proc macros ([#51])
- Signal handling using the signal-hook crate ([#50])
- thread: Wrappers for spawning threads ([#49])
- Impl `Runnable`/`RunnableMut` for `Box<dyn Fn/FnMut>` ([#48])
- Switch from `term` crate to `termcolor` crate ([#47])
- Rename `Callable` to `Runnable` ([#45])
- Bump minimum Rust version to 1.35.0 ([#44])
- Use `secrecy` crate for secret-keeping ([#43], [#53])
- Refactor `Application` and `Config` ([#42])
- abscissa_generator: Generate new apps with `abscissa new` ([#38], [#39], [#40], [#41])
- config: Refactor to eliminate `macro_rules!` ([#34])
- Update to Rust 2018 edition ([#30], [#36])

## [0.0.6] (2018-10-12)

- Upgrade to zeroize v0.4 ([#24])

## [0.0.5] (2018-10-11)

- Use zeroize crate for clearing secrets from memory ([#22])
- config/load.rs: Improved error messages ([#21])
- Add `fatal!` and `fatal_error!` macros ([#20])
- Remove `LOGO_ASCII_ART` ([#19])
- Make error module public ([#18])

## [0.0.4] (2018-09-01)

- secrets.rs: Fix trait bounds for `BorrowSecret<T>` ([#16])

## [0.0.3] (2018-09-01)

- Full application lifecycle implementation ([#14])

## [0.0.2] (2018-08-28)

- Use the `failure_derive` crate ([#11])
- Use the `isatty` crate ([#10]
- **util.rs**: Export the `chrono` crate under `util::time` ([#8])

## 0.0.1 (2018-08-25)

- Initial release

[#168]: https://github.com/iqlusioninc/abscissa/pull/168
[#167]: https://github.com/iqlusioninc/abscissa/pull/167
[#154]: https://github.com/iqlusioninc/abscissa/pull/154
[#152]: https://github.com/iqlusioninc/abscissa/pull/152
[#151]: https://github.com/iqlusioninc/abscissa/pull/151
[#144]: https://github.com/iqlusioninc/abscissa/issues/144
[0.4.0]: https://github.com/iqlusioninc/abscissa/pull/142
[#141]: https://github.com/iqlusioninc/abscissa/pull/141
[#136]: https://github.com/iqlusioninc/abscissa/pull/136
[#135]: https://github.com/iqlusioninc/abscissa/pull/135
[#134]: https://github.com/iqlusioninc/abscissa/pull/134
[#130]: https://github.com/iqlusioninc/abscissa/pull/130
[#131]: https://github.com/iqlusioninc/abscissa/pull/131
[#129]: https://github.com/iqlusioninc/abscissa/pull/129
[0.3.0]: https://github.com/iqlusioninc/abscissa/pull/127
[#126]: https://github.com/iqlusioninc/abscissa/pull/126
[#124]: https://github.com/iqlusioninc/abscissa/pull/124
[#122]: https://github.com/iqlusioninc/abscissa/pull/122
[#121]: https://github.com/iqlusioninc/abscissa/pull/121
[#119]: https://github.com/iqlusioninc/abscissa/pull/119
[#118]: https://github.com/iqlusioninc/abscissa/pull/118
[#117]: https://github.com/iqlusioninc/abscissa/pull/117
[#115]: https://github.com/iqlusioninc/abscissa/pull/115
[#113]: https://github.com/iqlusioninc/abscissa/pull/113
[#110]: https://github.com/iqlusioninc/abscissa/pull/110
[0.2.1]: https://github.com/iqlusioninc/abscissa/pull/108
[#107]: https://github.com/iqlusioninc/abscissa/pull/107
[#106]: https://github.com/iqlusioninc/abscissa/pull/106
[0.2.0]: https://github.com/iqlusioninc/abscissa/pull/96
[#95]: https://github.com/iqlusioninc/abscissa/pull/95
[#94]: https://github.com/iqlusioninc/abscissa/pull/94
[#93]: https://github.com/iqlusioninc/abscissa/pull/93
[#91]: https://github.com/iqlusioninc/abscissa/pull/91
[#84]: https://github.com/iqlusioninc/abscissa/pull/84
[#90]: https://github.com/iqlusioninc/abscissa/pull/90
[#88]: https://github.com/iqlusioninc/abscissa/pull/88
[#82]: https://github.com/iqlusioninc/abscissa/pull/82
[#81]: https://github.com/iqlusioninc/abscissa/pull/81
[#78]: https://github.com/iqlusioninc/abscissa/pull/78
[0.1.0]: https://github.com/iqlusioninc/abscissa/pull/77
[#72]: https://github.com/iqlusioninc/abscissa/pull/72
[#71]: https://github.com/iqlusioninc/abscissa/pull/71
[#69]: https://github.com/iqlusioninc/abscissa/pull/69
[#68]: https://github.com/iqlusioninc/abscissa/pull/68
[#65]: https://github.com/iqlusioninc/abscissa/pull/65
[#64]: https://github.com/iqlusioninc/abscissa/pull/64
[#63]: https://github.com/iqlusioninc/abscissa/pull/63
[#61]: https://github.com/iqlusioninc/abscissa/pull/61
[#59]: https://github.com/iqlusioninc/abscissa/pull/59
[#58]: https://github.com/iqlusioninc/abscissa/pull/58
[#57]: https://github.com/iqlusioninc/abscissa/pull/57
[#56]: https://github.com/iqlusioninc/abscissa/pull/56
[#55]: https://github.com/iqlusioninc/abscissa/pull/55
[#53]: https://github.com/iqlusioninc/abscissa/pull/53
[#52]: https://github.com/iqlusioninc/abscissa/pull/52
[#51]: https://github.com/iqlusioninc/abscissa/pull/51
[#50]: https://github.com/iqlusioninc/abscissa/pull/50
[#49]: https://github.com/iqlusioninc/abscissa/pull/49
[#48]: https://github.com/iqlusioninc/abscissa/pull/48
[#47]: https://github.com/iqlusioninc/abscissa/pull/47
[#45]: https://github.com/iqlusioninc/abscissa/pull/45
[#44]: https://github.com/iqlusioninc/abscissa/pull/44
[#43]: https://github.com/iqlusioninc/abscissa/pull/43
[#42]: https://github.com/iqlusioninc/abscissa/pull/42
[#41]: https://github.com/iqlusioninc/abscissa/pull/41
[#40]: https://github.com/iqlusioninc/abscissa/pull/40
[#39]: https://github.com/iqlusioninc/abscissa/pull/39
[#38]: https://github.com/iqlusioninc/abscissa/pull/38
[#36]: https://github.com/iqlusioninc/abscissa/pull/36
[#34]: https://github.com/iqlusioninc/abscissa/pull/34
[#30]: https://github.com/iqlusioninc/abscissa/pull/30
[0.0.6]: https://github.com/iqlusioninc/abscissa/pull/25
[#24]: https://github.com/iqlusioninc/abscissa/pull/24
[0.0.5]: https://github.com/iqlusioninc/abscissa/pull/23
[#22]: https://github.com/iqlusioninc/abscissa/pull/22
[#21]: https://github.com/iqlusioninc/abscissa/pull/21
[#20]: https://github.com/iqlusioninc/abscissa/pull/20
[#19]: https://github.com/iqlusioninc/abscissa/pull/19
[#18]: https://github.com/iqlusioninc/abscissa/pull/18
[0.0.4]: https://github.com/iqlusioninc/abscissa/compare/v0.0.3...v0.0.4
[#16]: https://github.com/iqlusioninc/abscissa/pull/16
[0.0.3]: https://github.com/iqlusioninc/abscissa/compare/v0.0.2...v0.0.3
[#14]: https://github.com/iqlusioninc/abscissa/pull/14
[0.0.2]: https://github.com/iqlusioninc/abscissa/compare/v0.0.1...v0.0.2
[#11]: https://github.com/iqlusioninc/abscissa/pull/11
[#10]: https://github.com/iqlusioninc/abscissa/pull/10
[#8]: https://github.com/iqlusioninc/abscissa/pull/8
