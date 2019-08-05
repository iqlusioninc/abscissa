# ![Abscissa](https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa.svg)

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust 1.35+][rustc-image]
[![forbid(unsafe_code)][unsafe-image]][unsafe-link]
[![Build Status][build-image]][build-link]
[![Appveyor Status][appveyor-image]][appveyor-link]
[![Gitter Chat][gitter-image]][gitter-link]

Abscissa is a microframework for building Rust applications (either CLI tools
or network/web services), aiming to provide a large number of features with a
*minimal number of dependencies*, and with a *strong focus on security*.

[Documentation][docs-link]

## Features

- **command-line option parsing**: simple declarative option parser based on
  [gumdrop]. The option parser in Abcissa contains numerous improvements which
  provide better UX and tighter integration with the other parts of the
  framework (e.g. overriding configuration settings using command-line options).
- **components**: Abscissa uses a component architecture (similar to an ECS)
  for extensibility/composability, with a minimalist implementation that still
  provides such features such as calculating dependency ordering and providing
  hooks into the application lifecycle. Newly generated apps use two components
  by default: `terminal` and `logging`.
- **configuration**: Simple parsing of TOML configurations to serde-parsed
  configuration types which can be dynamically updated at runtime.
- **error handling**: generic `Error` type based on the `failure` crate, and a
  unified error-handling subsystem.
- **logging**: based on the `log` to provide application-level logging.
- **secrets management**: the (optional) `secrets` module includes a `Secret`
  type which derives serde's `Deserialize` and can be used to represent secret
  values parsed from configuration files or elsewhere (e.g. credentials loaded
  from the environment or network requests)
- **terminal interactions**: support for colored terminal output (with color
  support autodetection). Useful for Cargo-like status messages with
  easy-to-use macros.

## Requirements

- Rust 1.35+

## Installation

To generate a new Abscissa application, install the `abscissa` CLI utility:

```
$ cargo install abscissa
```

## Usage

After installing the `abscissa` CLI utility using the method above, run
`abscissa new <my_app>` to generate a new application:

<img src="https://raw.githubusercontent.com/iqlusioninc/abscissa/develop/img/abscissa-new-screenshot.png" width="400px">

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

Here are all of Abscissa's transitive dependencies when configured with the
default set of features in the application:

| #  | Crate Name             | Origin          | License        | Description             |
|----|------------------------|-----------------|----------------|-------------------------|
| 1  | [abscissa]             | [iqlusion]      | Apache-2.0     | App microframework      |
| 2  | [arc-swap]             | [@vorner]       | Apache-2.0/MIT | Atomic swap for `Arc`   |
| 3  | [autocfg]              | [@cuviper]      | Apache-2.0/MIT | Rust compiler configs   |
| 4  | [backtrace]            | [@alexcrichton] | Apache-2.0/MIT | Capture stack traces    |
| 5  | [backtrace-sys]        | [@alexcrichton] | Apache-2.0/MIT | Capture stack traces    |
| 6  | [canonical-path]       | [iqlusion]      | Apache-2.0     | Get canonical fs paths  |
| 7  | [chrono]               | [chronotope]    | Apache-2.0/MIT | Time/date library       |
| 8  | [failure]              | [@withoutboats] | Apache-2.0/MIT | Error handling          |
| 9  | [generational-arena]   | [@fitzgen]      | MPL-2.0        | Component allocator     |
| 10 | [gumdrop]              | [@Murarth]      | Apache-2.0/MIT | Command-line options    |
| 11 | [lazy_static]          | [rust-lang]     | Apache-2.0/MIT | Heap-allocated statics  |
| 12 | [libc]                 | [rust-lang]     | Apache-2.0/MIT | C library wrapper       |
| 13 | [log]                  | [rust-lang]     | Apache-2.0/MIT | Logging facade library  |
| 14 | [num-integer]          | [rust-num]      | Apache-2.0/MIT | `Integer` trait         |
| 15 | [num-traits]           | [rust-num]      | Apache-2.0/MIT | Numeric traits          |
| 16 | [redox_syscall]        | [redox-os]      | MIT            | Redox OS syscall API    |
| 17 | [rustc-demangle]       | [@alexcrichton] | Apache-2.0/MIT | Symbol demangling       |
| 18 | [secrecy]              | [iqlusion]      | Apache-2.0     | Secret-keeping types    |
| 19 | [semver]               | [@steveklabnik] | Apache-2.0/MIT | Semantic versioning     |
| 20 | [semver-parser]        | [@steveklabnik] | Apache-2.0/MIT | Parser for semver spec  |
| 21 | [serde]                | [serde-rs]      | Apache-2.0/MIT | Serialization framework |
| 22 | [signal-hook]          | [@vorner]       | Apache-2.0/MIT | Unix signal handling    |
| 23 | [signal-hook-registry] | [@vorner]       | Apache-2.0/MIT | Unix signal registry    |
| 24 | [termcolor]            | [@BurntSushi]   | MIT/Unlicense  | Terminal color support  |
| 25 | [time]                 | [rust-lang]     | Apache-2.0/MIT | Time/date library       |
| 26 | [toml]                 | [@alexcrichton] | Apache-2.0/MIT | TOML parser library     |
| 27 | [winapi]§              | [@retep998]     | Apache-2.0/MIT | Windows FFI bindings    |
| 28 | [winapi-util]          | [@BurntSushi]   | MIT/Unlicense  | Safe winapi wrappers    |
| 29 | [wincolor]             | [@BurntSushi]   | MIT/Unlicense  | Windows console color   |
| 30 | [zeroize]              | [iqlusion]      | Apache-2.0/MIT | Zero out sensitive data |

### Build / Development / Testing Dependencies

| #  | Crate Name        | Origin           | License        | Description             |
|----|-------------------|------------------|----------------|-------------------------|
| 1  | [abscissa_derive] | [iqlusion]       | Apache-2.0     | Abscissa custom derive  |
| 2  | [aho-corasick]    | [@BurntSushi]    | MIT/Unlicense  | Pattern-matching alg    |
| 3  | [cc]              | [@alexcrichton]  | Apache-2.0/MIT | C/C++ compiler wrapper  |
| 4  | [cfg-if]          | [@alexcrichton]  | Apache-2.0/MIT | If-like `#[cfg]` macros |
| 5  | [darling]         | [@TedDriggs]     | MIT            | Nifty attribute parser  |
| 6  | [failure_derive]  | [@withoutboats]  | Apache-2.0/MIT | failure custom derive   |
| 7  | [gumdrop_derive]  | [@Murarth]       | Apache-2.0/MIT | Command-line options    |
| 8  | [heck]            | [@withoutboats]  | Apache-2.0/MIT | Case conversion utils   |
| 9  | [memchr]          | [@BurntSushi]    | MIT/Unlicense  | Optimize byte search    |
| 10 | [proc-macro2]     | [@alexcrichton]  | Apache-2.0/MIT | Shim for Macros 2.0 API |
| 11 | [quote]           | [@dtolnay]       | Apache-2.0/MIT | Rust AST to token macro |
| 12 | [regex]           | [rust-lang]      | Apache-2.0/MIT | Regular expressions     |
| 13 | [regex-syntax]    | [rust-lang]      | Apache-2.0/MIT | Regex syntax impl       |
| 14 | [serde_derive]    | [serde-rs]       | Apache-2.0/MIT | `serde` custom derive   |
| 15 | [syn]             | [@dtolnay]       | Apache-2.0/MIT | Rust source code parser |
| 16 | [synstructure]    | [@mystor]        | Apache-2.0/MIT | `syn` structure macros  |
| 17 | [thread_local]    | [@Amanieu]       | Apache-2.0/MIT | Per-object thread local |
| 18 | [ucd-util]        | [@BurntSushi]    | Apache-2.0/MIT | Unicode utilities       |
| 19 | [unicode-xid]     | [unicode-rs]     | Apache-2.0/MIT | Identify valid Unicode  |
| 20 | [utf8-ranges]     | [@BurntSushi]    | MIT/Unlicense  | UTF-8 codepoint ranges  |
| 21 | [wait-timeout]    | [@alexcrichton]  | Apache-2.0/MIT | Timeouts for waitpid    |

### Dependency Relationships

The table below should help answer questions as to why a particular crate is
an Abscissa dependency and whether or not it is optional. Abscissa uses
[cargo features] to allow parts of it you aren't using to be easily disabled,
so you only compile the parts you need.

| Crate Name             | [Cargo Features] | Required By     |
|------------------------|------------------|-----------------|
| [abscissa]             | -                | ⊤               |
| [abscissa_derive]      | -                | [abscissa]      |
| [aho-corasick]         | `testing`        | [regex]         |
| [arc-swap]             | `signals`        | [signal-hook-registry] |
| [autocfg]              | `time`           | [num-integer]   |
| [backtrace]            | -                | [failure]       |
| [backtrace-sys]        | -                | [backtrace]     |
| [canonical-path]       | -                | [abscissa]      |
| [cc]                   | -                | [backtrace-sys] |
| [cfg-if]               | -                | [backtrace], [log] |
| [chrono]               | `time`           | [abscissa]      |
| [failure]              | -                | [abscissa]      |
| [failure_derive]       | -                | [failure]       |
| [generational-arena]   | `application`    | [abscissa]      |
| [gumdrop]              | `options`        | [abscissa]      |
| [gumdrop_derive]       | `options`        | [gumdrop]       |
| [heck]                 | `inflector`      | [abscissa]      |
| [lazy_static]          | -                | [abscissa]      |
| [libc]                 | `signals`        | [abscissa]      |
| [log]                  | `logging`        | [abscissa]      |
| [memchr]               | `testing`        | [aho-corasick]  |
| [num-integer]          | `time`           | [chrono]        |
| [num-traits]           | `time`           | [chrono], [num-integer] |
| [proc-macro2]          | -                | [abscissa_derive], [failure_derive], [quote], [serde_derive] |
| [redox_syscall]        | `time`           | [time]          |
| [regex]                | `testing`        | [abscissa]      |
| [rustc-demangle]       | -                | [backtrace]     |
| [secrecy]              | `secrets`        | [abscissa]      |
| [semver]               | `application`    | [abscissa]      |
| [semver-parser]        | `application`    | [abscissa]      |
| [serde]                | `config`         | [abscissa]      |
| [serde_derive]         | `config`         | [serde]         |
| [signal-hook]          | `signals`        | [abscissa]      |
| [signal-hook-registry] | `signals`        | [signal-hook] |
| [termcolor]            | `terminal`       | [abscissa]      |
| [thread_local]         | `testing`        | [regex]         |
| [time]                 | `logging`        | [chrono]        |
| [ucd-util]             | `testing`        | [regex-syntax]  |
| [unicode-xid]          | -                | [proc-macro2]   |
| [utf8-ranges]          | `testing`        | [regex]         |
| [wait-timeout]         | `testing`        | [abscissa]      |
| [winapi]§              | -                | [termcolor], [time], [winapi-util] |
| [winapi-util]          | -                | [termcolor]     |
| [zeroize]              | -                | [abscissa]      |

* § `winapi` is a facade for either [winapi-i686-pc-windows-gnu] or
    [winapi-x86_64-pc-windows-gnu] which aren't explicitly listed for brevity
    and are only required on Windows platforms.

## Frequently Asked Questions (FAQ)

### Q1: Why is it called "abscissa"?

**A1:** An abscissa represents the elevation of a point above the x-axis.
In that regard, "Abscissa" can be thought of as a pun about getting off
the ground, or elevating your project.

The word "abscissa" is also the key to the [Kryptos K2] panel.

### Q2: "Abscissa" is a hard name to remember! Got any tips?

**A2**: Imagine you're A-B testing a couple of scissors... with attitude.

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

If you are interested in contributing to this repository, please make sure to
read the [CONTRIBUTING.md] and [CODE_OF_CONDUCT.md] files first.

[CONTRIBUTING.md]: https://github.com/iqlusioninc/abscissa/blob/develop/CONTRIBUTING.md
[CODE_OF_CONDUCT.md]: https://github.com/iqlusioninc/abscissa/blob/develop/CODE_OF_CONDUCT.md

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/abscissa_core.svg
[crate-link]: https://crates.io/crates/abscissa_core
[docs-image]: https://docs.rs/abscissa_core/badge.svg
[docs-link]: https://docs.rs/abscissa_core/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/abscissa/blob/develop/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.35+-blue.svg
[unsafe-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[unsafe-link]: https://internals.rust-lang.org/t/disabling-unsafe-by-default/7988
[build-image]: https://travis-ci.com/iqlusioninc/abscissa.svg?branch=develop
[build-link]: https://travis-ci.com/iqlusioninc/abscissa/
[appveyor-image]: https://ci.appveyor.com/api/projects/status/9bgh8je3rsmbyo0y?svg=true
[appveyor-link]: https://ci.appveyor.com/project/tony-iqlusion/abscissa
[gitter-image]: https://badges.gitter.im/iqlusioninc/community.svg
[gitter-link]: https://gitter.im/iqlusioninc/community

[//]: # (general links)

[cargo]: https://github.com/rust-lang/cargo
[cargo features]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
[Kryptos K2]: https://en.wikipedia.org/wiki/Kryptos#Solution_of_passage_2
[cc]: https://contributor-covenant.org
[CODE_OF_CONDUCT.md]: https://github.com/iqlusioninc/abscissa/blob/develop/CODE_OF_CONDUCT.md

[//]: # (crate links)

[abscissa]: https://crates.io/crates/abscissa_core
[abscissa_derive]: https://crates.io/crates/abscissa_derive
[aho-corasick]: https://crates.io/crates/aho-corasick
[arc-swap]: https://crates.io/crates/arc-swap
[autocfg]: https://crates.io/crates/autocfg
[backtrace]: https://crates.io/crates/backtrace
[backtrace-sys]: https://crates.io/crates/backtrace-sys
[byteorder]: https://crates.io/crates/byteorder
[canonical-path]: https://crates.io/crates/canonical-path
[cc]: https://crates.io/crates/cc
[cfg-if]: https://crates.io/crates/cfg-if
[chrono]: https://crates.io/crates/chrono
[darling]: https://github.com/TedDriggs/darling
[failure]: https://crates.io/crates/failure
[failure_derive]: https://crates.io/crates/failure_derive
[generational-arena]: https://github.com/fitzgen/generational-arena
[gumdrop]: https://crates.io/crates/gumdrop
[gumdrop_derive]: https://crates.io/crates/gumdrop_derive
[heck]: https://crates.io/crates/heck
[lazy_static]: https://crates.io/crates/lazy_static
[libc]: https://crates.io/crates/libc
[log]: https://crates.io/crates/log
[memchr]: https://crates.io/crates/memchr
[num-integer]: https://crates.io/crates/num-integer
[num-traits]: https://crates.io/crates/num-traits
[proc-macro2]: https://crates.io/crates/proc-macro2
[quote]: https://crates.io/crates/quote
[redox_syscall]: https://crates.io/crates/redox_syscall
[regex]: https://crates.io/crates/regex
[regex-syntax]: https://crates.io/crates/regex-syntax
[rustc-demangle]: https://crates.io/crates/rustc_demangle
[secrecy]: https://crates.io/crates/secrecy
[semver]: https://crates.io/crates/semver
[semver-parser]: https://crates.io/crates/semver-parser
[serde]: https://crates.io/crates/serde
[serde_derive]: https://crates.io/crates/serde_derive
[signal-hook]: https://crates.io/crates/signal-hook
[signal-hook-registry]: https://crates.io/crates/signal-hook
[syn]: https://crates.io/crates/syn
[synstructure]: https://crates.io/crates/
[termcolor]: https://crates.io/crates/termcolor
[thread_local]: https://crates.io/crates/thread_local
[time]: https://crates.io/crates/time
[toml]: https://crates.io/crates/toml
[ucd-util]: https://crates.io/crates/ucd-util
[unicode-xid]: https://crates.io/crates/unicode-xid
[utf8-ranges]: https://crates.io/crates/utf8-ranges
[wait-timeout]: https://crates.io/crates/wait-timeout
[winapi]: https://crates.io/crates/winapi
[winapi-util]: https://crates.io/crates/winapi
[winapi-i686-pc-windows-gnu]: https://crates.io/crates/winapi-i686-pc-windows-gnu
[winapi-x86_64-pc-windows-gnu]: https://crates.io/crates/winapi-x86_64-pc-windows-gnu
[wincolor]: https://crates.io/crates/winapi
[zeroize]: https://crates.io/crates/zeroize

[//]: # (author links)

[@alexcrichton]: https://github.com/alexcrichton
[@Amanieu]: https://github.com/Amanieu
[@BurntSushi]: https://github.com/BurntSushi
[@cuviper]: https://github.com/cuviper
[@dtolnay]: https://github.com/dtolnay
[@fitzgen]: https://github.com/fitzgen
[@Murarth]: https://github.com/Murarth
[@mystor]: https://github.com/mystor
[@retep998]: https://github.com/retep998
[@SergioBenitez]: https://github.com/SergioBenitez
[@steveklabnik]: https://github.com/steveklabnik
[@TedDriggs]: https://github.com/TedDriggs
[@vorner]: https://github.com/vorner
[@withoutboats]: https://github.com/withoutboats
[chronotope]: https://github.com/chronotope/
[iqlusion]: https://www.iqlusion.io
[redox-os]: https://github.com/redox-os
[rust-lang]: https://github.com/rust-lang/
[rust-num]: https://github.com/rust-num/
[serde-rs]: https://github.com/serde-rs/
[unicode-rs]: https://github.com/unicode-rs/

