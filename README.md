# ![Abscissa](https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa.svg)

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
[![Build Status][build-image]][build-link]
[![Appveyor Status][appveyor-image]][appveyor-link]

[crate-image]: https://img.shields.io/crates/v/abscissa.svg
[crate-link]: https://crates.io/crates/abscissa
[docs-image]: https://docs.rs/abscissa/badge.svg
[docs-link]: https://docs.rs/abscissa/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/abscissa/blob/master/LICENSE
[build-image]: https://circleci.com/gh/iqlusioninc/abscissa.svg?style=shield
[build-link]: https://circleci.com/gh/iqlusioninc/abscissa
[appveyor-image]: https://ci.appveyor.com/api/projects/status/9bgh8je3rsmbyo0y?svg=true
[appveyor-link]: https://ci.appveyor.com/project/tony-iqlusion/abscissa

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
- **configuration**: declarative global configuration support using an `RwLock`
  on a `lazy_static`. Simple parsing of TOML configurations to serde-parsed
  global structures which can be dynamically updated at runtime.
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
| 2  | [backtrace]      | [@alexcrichton] | MIT/Apache-2.0 | yes       | Capture stack traces    |
| 3  | [backtrace-sys]  | [@alexcrichton] | MIT/Apache-2.0 | yes       | Capture stack traces    |
| 4  | [byteorder]      | [@BurntSushi]   | MIT/Unlicense  | yes       | Convert endianness      |
| 5  | [canonical-path] | [iqlusion]      | Apache-2.0     | yes       | Get canonical fs paths  |
| 6  | [chrono]         | [chronotope]    | MIT/Apache-2.0 | yes       | Time/date library       |
| 7  | [clear_on_drop]  | [@cesarb]       | MIT/Apache-2.0 | yes       | Zero out sensitive data |
| 8  | [failure]        | [@withoutboats] | MIT/Apache-2.0 | yes       | Error handling          |
| 9  | [isatty]         | [@dtolnay]      | MIT/Apache-2.0 | yes       | Are stdout/stderr TTYs? |
| 10 | [lazy_static]    | [rust-lang]     | MIT/Apache-2.0 | yes       | Heap-allocated statics  |
| 11 | [libc]           | [rust-lang]     | MIT/Apache-2.0 | yes       | C library wrapper       |
| 12 | [log]            | [rust-lang]     | MIT/Apache-2.0 | yes       | Logging facade library  |
| 13 | [num-integer]    | [rust-num]      | MIT/Apache-2.0 | yes       | `Integer` trait         |
| 14 | [num-traits]     | [rust-num]      | MIT/Apache-2.0 | yes       | Numeric traits          |
| 15 | [redox_syscall]  | [redox-os]      | MIT            | yes       | Redox OS syscall API    |
| 16 | [rustc_demangle] | [@alexcrichton] | MIT/Apache-2.0 | yes       | Symbol demangling       |
| 17 | [semver]         | [@steveklabnik] | MIT/Apache-2.0 | yes       | Semantic versioning     |
| 18 | [semver-parser]  | [@steveklabnik] | MIT/Apache-2.0 | no†       | Parser for semver spec  |
| 19 | [serde]          | [serde-rs]      | MIT/Apache-2.0 | yes       | Serialization framework |
| 20 | [simplelog]      | [@drakulix]     | MIT/Apache-2.0 | yes       | Simple logging facility |
| 21 | [term]           | [@Stebalien]    | MIT/Apache-2.0 | yes‡      | Terminal color support  |
| 22 | [time]           | [rust-lang]     | MIT/Apache-2.0 | yes       | Time/date library       |
| 23 | [toml]           | [@alexcrichton] | MIT/Apache-2.0 | no        | TOML parser library     |
| 24 | [winapi]§        | [@retep998]     | MIT/Apache-2.0 | yes       | Windows API bindings    |

* † `semver-parser` has one usage of `unsafe` which is not compiled by Abscissa.
* ‡ `term` has one usage of unsafe on Windows. Other platforms do not use unsafe.
* § `winapi` also pulls in either [winapi-i686-pc-windows-gnu] or [winapi-x86_64-pc-windows-gnu]
    which are omitted for brevity.

### Build / Development Dependencies

| #  | Crate Name        | Origin           | License        | `unsafe`? | Description             |
|----|-------------------|------------------|----------------|-----------|-------------------------|
| 1  | [abscissa_derive] | [iqlusion]       | Apache-2.0     | yes       | Abscissa custom derive  |
| 2  | [cc]              | [@alexcrichton]  | MIT/Apache-2.0 | yes       | C/C++ compiler wrapper  |
| 3  | [cfg-if]          | [@alexcrichton]  | MIT/Apache-2.0 | no        | If-like `#[cfg]` macros |
| 4  | [failure_derive]  | [@withoutboats]  | MIT/Apache-2.0 | yes       | failure custom derive   |
| 5  | [proc-macro2]     | [@alexcrichton]  | MIT/Apache-2.0 | yes       | Shim for Macros 2.0 API |
| 6  | [quote]           | [@dtolnay]       | MIT/Apache-2.0 | no        | Rust AST to token macro |
| 7  | [serde_derive]    | [serde-rs]       | MIT/Apache-2.0 | no        | `serde` custom derive   |
| 8  | [syn]             | [@dtolnay]       | MIT/Apache-2.0 | yes       | Rust source code parser |
| 9  | [synstructure]    | [@mystor]        | MIT/Apache-2.0 | no        | `syn` structure macros  |
| 10 | [unicode-xid]     | [unicode-rs]     | MIT/Apache-2.0 | no        | Identify valid Unicode  |
| 11 | [version_check]   | [@SergioBenitez] | MIT/Apache-2.0 | no        | rustc feature detection |

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
| [byteorder]       | `shell`          | -         | [term]      |
| [canonical-path]  | mandatory        | -         | [abscissa]  |
| [cc]              | mandatory        | -         | [backtrace-sys], [clear_on_drop] |
| [cfg-if]          | mandatory        | -         | [backtrace] |
| [chrono]          | `logging`        | -         | [simplelog] |
| [clear_on_drop]   | mandatory        | -         | [abscissa]  |
| [failure]         | mandatory        | -         | [abscissa]  |
| [failure_derive]  | mandatory        | -         | [failure]   |
| [isatty]          | `shell`          | -         | [abscissa]  |
| [lazy_static]     | mandatory        | -         | [abscissa]  |
| [libc]            | `shell`          | `unix`    | [isatty]    |
| [log]             | `logging`        | -         | [abscissa]  |
| [num-integer]     | `logging`        | -         | [chrono]    |
| [num-traits]      | `logging`        | -         | [chrono], [num-integer] |
| [proc-macro2]     | mandatory        | -         | [abscissa_derive], [failure_derive], [quote], [serde_derive] |
| [redox_syscall]   | `shell`          | `redox`   | [isatty]    |
| [rustc_demangle]  | mandatory        | -         | [backtrace] |
| [semver]          | `application`    | -         | [abscissa]  |
| [semver-parser]   | `application`    | -         | [abscissa]  |
| [serde]           | `config`         | -         | [abscissa]  |
| [serde_derive]    | `config`         | -         | [serde]     |
| [simplelog]       | `logging`        | -         | [abscissa]  |
| [term]            | `shell`          | -         | [abscissa]  |
| [time]            | `logging`        | -         | [chrono]    |
| [unicode-xid]     | mandatory        | -         | [proc-macro2] |
| [version_check]   | mandatory        | -         | [lazy_static] |
| [winapi]§         | `shell`          | `windows` | [isatty]    |

* § `winapi` also pulls in either [winapi-i686-pc-windows-gnu] or [winapi-x86_64-pc-windows-gnu]
    which are omitted for brevity.

## Frequently Asked Questions (FAQ)

### Q1: Why is it called "abscissa"?

**A1:** An abscissa represents the elevation of a point above the y-axis.
In that regard, "Abscissa" can be thought of as a pun about getting off
the ground, or elevating your project.

The word "abscissa" is also the key to the [Kryptos K2] panel.

## License

The **abscissa** crate is distributed under the terms of the
Apache License (Version 2.0).

Copyright © 2018 iqlusion

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

Parts of this code were taken from the following projects, which have agreed
to license their code under the Apache License (Version 2.0):

* [Cargo]
* [failure]
* [gumdrop]

[//]: # (general links)

[cargo]: https://github.com/rust-lang/cargo
[cargo features]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
[Kryptos K2]: https://en.wikipedia.org/wiki/Kryptos#Solution_of_passage_2
[LICENSE]: https://github.com/iqlusioninc/abscissa/blob/master/LICENSE

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
[clear_on_drop]: https://crates.io/crates/clear_on_drop
[failure]: https://crates.io/crates/failure
[failure_derive]: https://crates.io/crates/failure_derive
[gumdrop]: https://crates.io/crates/gumdrop
[isatty]: https://crates.io/crates/isatty
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
[term]: https://crates.io/crates/term
[time]: https://crates.io/crates/time
[toml]: https://crates.io/crates/toml
[unicode-xid]: https://crates.io/crates/unicode-xid
[version_check]: https://crates.io/crates/version_check
[winapi]: https://crates.io/crates/winapi
[winapi-i686-pc-windows-gnu]: https://crates.io/crates/winapi-i686-pc-windows-gnu
[winapi-x86_64-pc-windows-gnu]: https://crates.io/crates/winapi-x86_64-pc-windows-gnu

[//]: # (author links)

[@alexcrichton]: https://github.com/alexcrichton
[@BurntSushi]: https://github.com/BurntSushi
[@cesarb]: https://github.com/cesarb
[@drakulix]: https://github.com/drakulix
[@dtolnay]: https://github.com/dtolnay
[@mystor]: https://github.com/mystor
[@retep998]: https://github.com/retep998
[@SergioBenitez]: https://github.com/SergioBenitez
[@Stebalien]: https://github.com/Stebalien
[@steveklabnik]: https://github.com/steveklabnik
[@withoutboats]: https://github.com/withoutboats
[chronotope]: https://github.com/chronotope/
[iqlusion]: https://www.iqlusion.io
[redox-os]: https://github.com/redox-os
[rust-lang]: https://github.com/rust-lang/
[rust-num]: https://github.com/rust-num/
[serde-rs]: https://github.com/serde-rs/
[unicode-rs]: https://github.com/unicode-rs/

