# ![Abscissa](https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa.svg)

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust 1.35+][rustc-image]
[![forbid(unsafe_code)][unsafe-image]][unsafe-link]
[![Build Status][build-image]][build-link]
[![Appveyor Status][appveyor-image]][appveyor-link]

Abscissa is a microframework for building Rust applications (either CLI tools
or network/web services), aiming to provide a large number of features with a
*minimal number of dependencies*, and with a *strong focus on security*.

[Documentation][docs-link]

## Features

- **command-line option parsing**: simple declarative option parser based on
  (i.e. forked from) [gumdrop]. The option parser in Abcissa contains numerous
  improvements which provide better UX and tighter integration with the other
  parts of the framework (e.g. overriding configuration settings using
  command-line options).
- **components**: Abscissa uses a component architecture (similar to an ECS)
  for extensibility/composability, with a minimalist implementation that still
  provides such features such as calculating dependency ordering and providing
  hooks into the application lifecycle. Newly generated apps use two components
  by default: `shell` and `logging`.
- **configuration**: Simple parsing of TOML configurations to serde-parsed
  configuration types which can be dynamically updated at runtime.
- **error handling**: generic `Error` type based on the `failure` crate, and a
  unified error-handling subsystem.
- **logging**: uses the `log` and `simplelog` crates to automatically configure
  application-level logging, presently to standard output or files.
- **secrets management**: the (optional) `secrets` module includes a `Secret`
  type which derives serde's `Deserialize` and can be used to represent secret
  values parsed from configuration files or elsewhere (e.g. credentials loaded
  from the environment or network requests)
- **shell interactions**: support for colored terminal output (with color
  support autodetection). Useful for Cargo-like status messages with
  easy-to-use macros.

## Requirements

- Rust 1.35+

## Usage

If you already have Rust installed, the following commands will generate an
Abscissa application skeleton:

![abscissa new my_cool_app](https://raw.githubusercontent.com/iqlusioninc/abscissa/develop/img/abscissa-new.png)

This will generate a new Abscissa application in the `my_cool_app` directory.
For more information, please see the [Documentation][docs-link].

## Depencencies

*or: "Know Your Dependencies"*

[//]: # (TODO: test this information is up-to-date)

One of Abscissa's primary goals is to maximize functionality while minimizing
the number of dependencies. Abscissa is used in a number of high-security
contexts, and as such we view each additional dependency as additional attack
surface and therefore a potential liability. We have therefore been very
conscientious about the dependencies we use and will not add additional
dependencies without due consideration.

Here are all of Abscissa's transitive dependencies:

| #  | Crate Name       | Origin          | License        | `unsafe`? | Description             |
|----|------------------|-----------------|----------------|-----------|-------------------------|
| 1  | [abscissa]       | [iqlusion]      | Apache-2.0     | yes       | App microframework      |
| 2  | [atty]           | [@softprops]    | MIT            | yes       | Are stdout/stderr TTYs? |
| 3  | [backtrace]      | [@alexcrichton] | Apache-2.0/MIT | yes       | Capture stack traces    |
| 4  | [backtrace-sys]  | [@alexcrichton] | Apache-2.0/MIT | yes       | Capture stack traces    |
| 5  | [byteorder]      | [@BurntSushi]   | MIT/Unlicense  | yes       | Convert endianness      |
| 6  | [canonical-path] | [iqlusion]      | Apache-2.0     | yes       | Get canonical fs paths  |
| 7  | [chrono]         | [chronotope]    | Apache-2.0/MIT | yes       | Time/date library       |
| 8  | [failure]        | [@withoutboats] | Apache-2.0/MIT | yes       | Error handling          |
| 9  | [lazy_static]    | [rust-lang]     | Apache-2.0/MIT | yes       | Heap-allocated statics  |
| 10 | [libc]           | [rust-lang]     | Apache-2.0/MIT | yes       | C library wrapper       |
| 11 | [log]            | [rust-lang]     | Apache-2.0/MIT | yes       | Logging facade library  |
| 12 | [num-integer]    | [rust-num]      | Apache-2.0/MIT | yes       | `Integer` trait         |
| 13 | [num-traits]     | [rust-num]      | Apache-2.0/MIT | yes       | Numeric traits          |
| 14 | [redox_syscall]  | [redox-os]      | MIT            | yes       | Redox OS syscall API    |
| 15 | [rustc_demangle] | [@alexcrichton] | Apache-2.0/MIT | yes       | Symbol demangling       |
| 16 | [semver]         | [@steveklabnik] | Apache-2.0/MIT | yes       | Semantic versioning     |
| 17 | [semver-parser]  | [@steveklabnik] | Apache-2.0/MIT | no†       | Parser for semver spec  |
| 18 | [serde]          | [serde-rs]      | Apache-2.0/MIT | yes       | Serialization framework |
| 19 | [simplelog]      | [@drakulix]     | Apache-2.0/MIT | yes       | Simple logging facility |
| 20 | [termcolor]      | [@BurntSushi]   | MIT/Unlicense  | no        | Terminal color support  |
| 21 | [time]           | [rust-lang]     | Apache-2.0/MIT | yes       | Time/date library       |
| 22 | [toml]           | [@alexcrichton] | Apache-2.0/MIT | no        | TOML parser library     |
| 23 | [winapi]§        | [@retep998]     | Apache-2.0/MIT | yes       | Windows API bindings    |
| 24 | [zeroize]        | [iqlusion]      | Apache-2.0     | yes       | Zero out sensitive data |

* † `semver-parser` has one usage of `unsafe` which is not compiled by Abscissa.
* § `winapi` also pulls in either [winapi-i686-pc-windows-gnu] or [winapi-x86_64-pc-windows-gnu]
    which are omitted for brevity.

### Build / Development Dependencies

| #  | Crate Name        | Origin           | License        | `unsafe`? | Description             |
|----|-------------------|------------------|----------------|-----------|-------------------------|
| 1  | [abscissa_derive] | [iqlusion]       | Apache-2.0     | no        | Abscissa custom derive  |
| 2  | [cc]              | [@alexcrichton]  | Apache-2.0/MIT | yes       | C/C++ compiler wrapper  |
| 3  | [cfg-if]          | [@alexcrichton]  | Apache-2.0/MIT | no        | If-like `#[cfg]` macros |
| 4  | [failure_derive]  | [@withoutboats]  | Apache-2.0/MIT | yes       | failure custom derive   |
| 5  | [proc-macro2]     | [@alexcrichton]  | Apache-2.0/MIT | yes       | Shim for Macros 2.0 API |
| 6  | [quote]           | [@dtolnay]       | Apache-2.0/MIT | no        | Rust AST to token macro |
| 7  | [serde_derive]    | [serde-rs]       | Apache-2.0/MIT | no        | `serde` custom derive   |
| 8  | [syn]             | [@dtolnay]       | Apache-2.0/MIT | yes       | Rust source code parser |
| 9  | [synstructure]    | [@mystor]        | Apache-2.0/MIT | no        | `syn` structure macros  |
| 10 | [unicode-xid]     | [unicode-rs]     | Apache-2.0/MIT | no        | Identify valid Unicode  |
| 11 | [version_check]   | [@SergioBenitez] | Apache-2.0/MIT | no        | rustc feature detection |

### Dependency Relationships

The table below should help answer questions as to why a particular crate is
an Abscissa dependency and whether or not it is optional. Abscissa uses
[cargo features] to allow parts of it you aren't using to be easily disabled,
so you only compile the parts you need.

| Crate Name        | [Cargo Features] | Platform  | Required By |
|-------------------|------------------|-----------|-------------|
| [abscissa]        | mandatory        | -         | -           |
| [abscissa_derive] | mandatory        | -         | [abscissa]  |
| [backtrace]       | mandatory        | -         | [failure]   |
| [backtrace-sys]   | mandatory        | -         | [backtrace] |
| [canonical-path]  | mandatory        | -         | [abscissa]  |
| [cc]              | mandatory        | -         | [backtrace-sys], [zeroize] |
| [cfg-if]          | mandatory        | -         | [backtrace] |
| [chrono]          | `logging`        | -         | [simplelog] |
| [failure]         | mandatory        | -         | [abscissa]  |
| [failure_derive]  | mandatory        | -         | [failure]   |
| [atty]            | `shell`          | -         | [abscissa]  |
| [lazy_static]     | mandatory        | -         | [abscissa]  |
| [libc]            | `shell`          | `unix`    | [atty]      |
| [log]             | `logging`        | -         | [abscissa]  |
| [num-integer]     | `logging`        | -         | [chrono]    |
| [num-traits]      | `logging`        | -         | [chrono], [num-integer] |
| [proc-macro2]     | mandatory        | -         | [abscissa_derive], [failure_derive], [quote], [serde_derive] |
| [redox_syscall]   | `shell`          | `redox`   | [atty]    |
| [rustc_demangle]  | mandatory        | -         | [backtrace] |
| [semver]          | `application`    | -         | [abscissa]  |
| [semver-parser]   | `application`    | -         | [abscissa]  |
| [serde]           | `config`         | -         | [abscissa]  |
| [serde_derive]    | `config`         | -         | [serde]     |
| [simplelog]       | `logging`        | -         | [abscissa]  |
| [termcolor]       | `shell`          | -         | [abscissa]  |
| [time]            | `logging`        | -         | [chrono]    |
| [unicode-xid]     | mandatory        | -         | [proc-macro2] |
| [version_check]   | mandatory        | -         | [lazy_static] |
| [winapi]§         | `shell`          | `windows` | [atty]      |
| [zeroize]         | mandatory        | -         | [abscissa]  |

* § `winapi` also pulls in either [winapi-i686-pc-windows-gnu] or [winapi-x86_64-pc-windows-gnu]
    which are omitted for brevity.

## Frequently Asked Questions (FAQ)

### Q1: Why is it called "abscissa"?

**A1:** An abscissa represents the elevation of a point above the y-axis.
In that regard, "Abscissa" can be thought of as a pun about getting off
the ground, or elevating your project.

The word "abscissa" is also the key to the [Kryptos K2] panel.

## Testing Framework Changes

The main way to test framework changes is by generating an application with
Abscissa's built-in application generator and running tests against the
generated application (also rustfmt, clippy).

To generate a test application and test it automatically, you can simply do:

```
$ cargo test
```

However, when debugging test failures against a generated app, it's helpful to
know how to drive the app generation and testing process manually. Below are
instructions on how to do so. 

If you've already run:

```
$ git clone https://github.com/iqlusioninc/abscissa/
```

...and are inside the `abscissa` directory and want to test your changes,
you can generate an application by running the following command:

```
$ cargo run -- new /tmp/example_app --patch-crates-io='abscissa = { path = "$PWD" }'
```

This will generate a new Abscissa application in `/tmp/example_app` which
references your local copy of Abscissa.

After that, change directory to the newly generated app and run the tests
to ensure things are still working (the tests, along with rustfmt and clippy
are run as part of the CI process):

```
$ cd /tmp/example_app # or 'pushd /tmp/example_app' and 'popd' to return
$ cargo test
$ cargo fmt -- --check # generated app is expected to pass rustfmt
$ cargo clippy
```

## Code of Conduct

We abide by the [Contributor Covenant][cc] and ask that you do as well.

For more information, please see [CODE_OF_CONDUCT.md].

## License

The **abscissa** crate is distributed under the terms of the
Apache License (Version 2.0).

Copyright © 2018-2019 iqlusion

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be dual licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/abscissa.svg
[crate-link]: https://crates.io/crates/abscissa
[docs-image]: https://docs.rs/abscissa/badge.svg
[docs-link]: https://docs.rs/abscissa/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/abscissa/blob/develop/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.35+-blue.svg
[unsafe-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[unsafe-link]: https://internals.rust-lang.org/t/disabling-unsafe-by-default/7988
[build-image]: https://travis-ci.com/iqlusioninc/abscissa.svg?branch=develop
[build-link]: https://travis-ci.com/iqlusioninc/abscissa/
[appveyor-image]: https://ci.appveyor.com/api/projects/status/9bgh8je3rsmbyo0y?svg=true
[appveyor-link]: https://ci.appveyor.com/project/tony-iqlusion/abscissa

[//]: # (general links)

[cargo]: https://github.com/rust-lang/cargo
[cargo features]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
[Kryptos K2]: https://en.wikipedia.org/wiki/Kryptos#Solution_of_passage_2
[cc]: https://contributor-covenant.org
[CODE_OF_CONDUCT.md]: https://github.com/iqlusioninc/abscissa/blob/develop/CODE_OF_CONDUCT.md

[//]: # (crate links)

[abscissa]: https://crates.io/crates/abscissa
[abscissa_derive]: https://crates.io/crates/abscissa_derive
[backtrace]: https://crates.io/crates/backtrace
[backtrace-sys]: https://crates.io/crates/backtrace-sys
[byteorder]: https://crates.io/crates/byteorder
[canonical-path]: https://crates.io/crates/canonical-path
[cc]: https://crates.io/crates/cc
[cfg-if]: https://crates.io/crates/cfg-if
[chrono]: https://crates.io/crates/chrono/
[failure]: https://crates.io/crates/failure
[failure_derive]: https://crates.io/crates/failure_derive
[gumdrop]: https://crates.io/crates/gumdrop
[atty]: https://crates.io/crates/atty
[lazy_static]: https://crates.io/crates/lazy_static
[libc]: https://crates.io/crates/libc
[log]: https://crates.io/crates/log
[num-integer]: https://crates.io/crates/num-integer
[num-traits]: https://crates.io/crates/num-traits
[proc-macro2]: https://crates.io/crates/proc-macro2
[quote]: https://crates.io/crates/quote
[redox_syscall]: https://crates.io/crates/redox_syscall
[rustc_demangle]: https://crates.io/crates/rustc_demangle
[semver]: https://crates.io/crates/semver
[semver-parser]: https://crates.io/crates/semver-parser
[serde]: https://crates.io/crates/serde
[serde_derive]: https://crates.io/crates/serde_derive
[simplelog]: https://crates.io/crates/simplelog
[syn]: https://crates.io/crates/syn
[synstructure]: https://crates.io/crates/
[termcolor]: https://crates.io/crates/termcolor
[time]: https://crates.io/crates/time
[toml]: https://crates.io/crates/toml
[unicode-xid]: https://crates.io/crates/unicode-xid
[version_check]: https://crates.io/crates/version_check
[winapi]: https://crates.io/crates/winapi
[winapi-i686-pc-windows-gnu]: https://crates.io/crates/winapi-i686-pc-windows-gnu
[winapi-x86_64-pc-windows-gnu]: https://crates.io/crates/winapi-x86_64-pc-windows-gnu
[zeroize]: https://crates.io/crates/zeroize

[//]: # (author links)

[@alexcrichton]: https://github.com/alexcrichton
[@BurntSushi]: https://github.com/BurntSushi
[@cesarb]: https://github.com/cesarb
[@drakulix]: https://github.com/drakulix
[@dtolnay]: https://github.com/dtolnay
[@mystor]: https://github.com/mystor
[@retep998]: https://github.com/retep998
[@SergioBenitez]: https://github.com/SergioBenitez
[@softprops]: https://github.com/softprops
[@steveklabnik]: https://github.com/steveklabnik
[@withoutboats]: https://github.com/withoutboats
[chronotope]: https://github.com/chronotope/
[iqlusion]: https://www.iqlusion.io
[redox-os]: https://github.com/redox-os
[rust-lang]: https://github.com/rust-lang/
[rust-num]: https://github.com/rust-num/
[serde-rs]: https://github.com/serde-rs/
[unicode-rs]: https://github.com/unicode-rs/

