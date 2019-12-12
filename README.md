# ![Abscissa][logo]

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][rustc-image]
[![Safety Dance][safety-image]][safety-link]
[![Build Status][build-image]][build-link]
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
- **configuration**: Simple parsing of TOML configurations to `serde`-parsed
  configuration types which can be dynamically updated at runtime.
- **error handling**: unified error-handling subsystem with generic error type.
- **logging**: based on the `log` to provide application-level logging.
- **secrets management**: the (optional) `secrets` module includes a `Secret`
  type which derives serde's `Deserialize` and can be used to represent secret
  values parsed from configuration files or elsewhere (e.g. credentials loaded
  from the environment or network requests)
- **terminal interactions**: support for colored terminal output (with color
  support autodetection). Useful for Cargo-like status messages with
  easy-to-use macros.

## Projects Using Abscissa

- [Canister]: deployment utility for "distroless" containers/microVMs
- [cargo-audit]: audit Cargo projects for security vulnerabilities
- [cargo-rpm]: build RPMs out of Cargo projects
- [OpenLibra]: open platform for financial inclusion. Not run by Facebook.
- [Sagan]: observability tool for Tendermint applications
- [Synchronicity]: distributed build system providing BFT proofs-of-reproducibility
- [Tendermint KMS]: key management system for Tendermint applications
- [Zebra]: Rust implementation of a Zcash node

## Crate Structure

Abscissa presently consists of three crates:

- [abscissa]: CLI app and application generator - `cargo install abscissa`
- [abscissa_core]: main framework library
- [abscissa_derive]: custom derive support - implementation detail of `abscissa_core`

## Requirements

- Rust **1.36+**

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
| 1  | [abscissa_core]        | [iqlusion]      | Apache-2.0     | Abscissa framework      |
| 2  | [arc-swap]             | [@vorner]       | Apache-2.0/MIT | Atomic swap for `Arc`   |
| 3  | [atty]                 | [@softprops]    | MIT            | Detect TTY presence     |
| 4  | [autocfg]              | [@cuviper]      | Apache-2.0/MIT | Rust compiler configs   |
| 5  | [backtrace]            | [@alexcrichton] | Apache-2.0/MIT | Capture stack traces    |
| 6  | [backtrace-sys]        | [@alexcrichton] | Apache-2.0/MIT | Capture stack traces    |
| 7  | [canonical-path]       | [iqlusion]      | Apache-2.0     | Get canonical fs paths  |
| 8  | [chrono]               | [chronotope]    | Apache-2.0/MIT | Time/date library       |
| 9  | [color-backtrace]      | [@athre0z]      | Apache-2.0/MIT | Rich colored backtraces |
| 10 | [generational-arena]   | [@fitzgen]      | MPL-2.0        | Component allocator     |
| 11 | [gumdrop]              | [@Murarth]      | Apache-2.0/MIT | Command-line options    |
| 12 | [lazy_static]          | [rust-lang]     | Apache-2.0/MIT | Heap-allocated statics  |
| 13 | [libc]                 | [rust-lang]     | Apache-2.0/MIT | C library wrapper       |
| 14 | [log]                  | [rust-lang]     | Apache-2.0/MIT | Logging facade library  |
| 15 | [num-integer]          | [rust-num]      | Apache-2.0/MIT | `Integer` trait         |
| 16 | [num-traits]           | [rust-num]      | Apache-2.0/MIT | Numeric traits          |
| 17 | [redox_syscall]        | [redox-os]      | MIT            | Redox OS syscall API    |
| 18 | [rustc-demangle]       | [@alexcrichton] | Apache-2.0/MIT | Symbol demangling       |
| 19 | [secrecy]              | [iqlusion]      | Apache-2.0     | Secret-keeping types    |
| 20 | [semver]               | [@steveklabnik] | Apache-2.0/MIT | Semantic versioning     |
| 21 | [semver-parser]        | [@steveklabnik] | Apache-2.0/MIT | Parser for semver spec  |
| 22 | [serde]                | [serde-rs]      | Apache-2.0/MIT | Serialization framework |
| 23 | [signal-hook]          | [@vorner]       | Apache-2.0/MIT | Unix signal handling    |
| 24 | [signal-hook-registry] | [@vorner]       | Apache-2.0/MIT | Unix signal registry    |
| 25 | [termcolor]            | [@BurntSushi]   | MIT/Unlicense  | Terminal color support  |
| 26 | [time]                 | [rust-lang]     | Apache-2.0/MIT | Time/date library       |
| 27 | [toml]                 | [@alexcrichton] | Apache-2.0/MIT | TOML parser library     |
| 28 | [winapi]§              | [@retep998]     | Apache-2.0/MIT | Windows FFI bindings    |
| 29 | [winapi-util]          | [@BurntSushi]   | MIT/Unlicense  | Safe winapi wrappers    |
| 30 | [wincolor]             | [@BurntSushi]   | MIT/Unlicense  | Windows console color   |
| 31 | [zeroize]              | [iqlusion]      | Apache-2.0/MIT | Zero out sensitive data |

### Build / Development / Testing Dependencies

| #  | Crate Name        | Origin           | License        | Description             |
|----|-------------------|------------------|----------------|-------------------------|
| 1  | [abscissa_derive] | [iqlusion]       | Apache-2.0     | Abscissa custom derive  |
| 2  | [aho-corasick]    | [@BurntSushi]    | MIT/Unlicense  | Pattern-matching alg    |
| 3  | [cc]              | [@alexcrichton]  | Apache-2.0/MIT | C/C++ compiler wrapper  |
| 4  | [cfg-if]          | [@alexcrichton]  | Apache-2.0/MIT | If-like `#[cfg]` macros |
| 5  | [darling]         | [@TedDriggs]     | MIT            | Nifty attribute parser  |
| 6  | [darling_core]    | [@TedDriggs]     | MIT            | Attribute parser core   |
| 7  | [darling_macro]   | [@TedDriggs]     | MIT            | Attribute parser macros |
| 8  | [fnv]             | [@alexcrichton]  | Apache-2.0/MIT | Fast hash function      |
| 9  | [gumdrop_derive]  | [@Murarth]       | Apache-2.0/MIT | Command-line options    |
| 10 | [ident_case]      | [@TedDriggs]     | Apache-2.0/MIT | Case conversion utils   |
| 11 | [memchr]          | [@BurntSushi]    | MIT/Unlicense  | Optimized byte search   |
| 12 | [proc-macro2]     | [@alexcrichton]  | Apache-2.0/MIT | Shim for Macros 2.0 API |
| 13 | [quote]           | [@dtolnay]       | Apache-2.0/MIT | Rust AST to token macro |
| 14 | [regex]           | [rust-lang]      | Apache-2.0/MIT | Regular expressions     |
| 15 | [regex-syntax]    | [rust-lang]      | Apache-2.0/MIT | Regex syntax impl       |
| 16 | [serde_derive]    | [serde-rs]       | Apache-2.0/MIT | `serde` custom derive   |
| 17 | [strsim]          | [@dguo]          | MIT            | String similarity utils |
| 18 | [syn]             | [@dtolnay]       | Apache-2.0/MIT | Rust source code parser |
| 19 | [synstructure]    | [@mystor]        | Apache-2.0/MIT | `syn` structure macros  |
| 20 | [thread_local]    | [@Amanieu]       | Apache-2.0/MIT | Per-object thread local |
| 21 | [ucd-util]        | [@BurntSushi]    | Apache-2.0/MIT | Unicode utilities       |
| 22 | [unicode-xid]     | [unicode-rs]     | Apache-2.0/MIT | Identify valid Unicode  |
| 23 | [utf8-ranges]     | [@BurntSushi]    | MIT/Unlicense  | UTF-8 codepoint ranges  |
| 24 | [wait-timeout]    | [@alexcrichton]  | Apache-2.0/MIT | Timeouts for waitpid    |

### Dependency Relationships

The table below should help answer questions as to why a particular crate is
an Abscissa dependency and whether or not it is optional. Abscissa uses
[cargo features] to allow parts of it you aren't using to be easily disabled,
so you only compile the parts you need.

| Crate Name             | [Cargo Features] | Required By       |
|------------------------|------------------|-------------------|
| [abscissa_core]        | -                | ⊤                 |
| [abscissa_derive]      | -                | [abscissa_core]   |
| [aho-corasick]         | `testing`        | [regex]           |
| [arc-swap]             | `signals`        | [signal-hook-registry] |
| [atty]                 | `terminal`       | [color-backtrace] |
| [autocfg]              | `time`           | [num-integer]     |
| [backtrace]            | -                | [abscissa_core]   |
| [backtrace-sys]        | -                | [backtrace]       |
| [canonical-path]       | -                | [abscissa_core]   |
| [cc]                   | -                | [backtrace-sys]   |
| [cfg-if]               | -                | [backtrace], [log] |
| [color-backtrace]      | `terminal`       | [abscissa_core]   |
| [chrono]               | `time`           | [abscissa_core]   |
| [darling]              | -                | [abscissa_derive] |
| [darling_core]         | -                | [darling], [darling_macro] |
| [darling_macro]        | -                | [darling]         |
| [fnv]                  | -                | [darling_core]    |
| [generational-arena]   | `application`    | [abscissa_core]   |
| [gumdrop]              | `options`        | [abscissa_core]   |
| [gumdrop_derive]       | `options`        | [gumdrop]         |
| [ident_case]           | -                | [abscissa_derive], [darling_core] |
| [lazy_static]          | -                | [abscissa_core]   |
| [libc]                 | `signals`        | [abscissa_core]   |
| [log]                  | `logging`        | [abscissa_core]   |
| [memchr]               | `testing`        | [aho-corasick]    |
| [num-integer]          | `time`           | [chrono]          |
| [num-traits]           | `time`           | [chrono], [num-integer] |
| [proc-macro2]          | -                | [abscissa_derive], [darling], [quote], [serde_derive], [syn] |
| [quote]                | -                | [abscissa_derive], [darling], [gumdrop_derive], [serde_derive] |
| [redox_syscall]        | `time`           | [time]            |
| [regex]                | `testing`        | [abscissa_core]   |
| [rustc-demangle]       | -                | [backtrace]       |
| [secrecy]              | `secrets`        | [abscissa_core]   |
| [semver]               | `application`    | [abscissa_core]   |
| [semver-parser]        | `application`    | [abscissa_core]   |
| [serde]                | `config`         | [abscissa_core]   |
| [serde_derive]         | `config`         | [serde]           |
| [signal-hook]          | `signals`        | [abscissa_core]   |
| [signal-hook-registry] | `signals`        | [signal-hook]     |
| [strsim]               | -                | [darling_core]    |
| [syn]                  | -                | [abscissa_derive], [darling], [gumdrop_derive], [serde_derive] |
| [termcolor]            | `terminal`       | [abscissa_core]   |
| [thread_local]         | `testing`        | [regex]           |
| [time]                 | `logging`        | [chrono]          |
| [ucd-util]             | `testing`        | [regex-syntax]    |
| [unicode-xid]          | -                | [proc-macro2], [syn] |
| [utf8-ranges]          | `testing`        | [regex]           |
| [wait-timeout]         | `testing`        | [abscissa_core]   |
| [winapi]§              | -                | [termcolor], [time], [winapi-util] |
| [winapi-util]          | -                | [termcolor]       |
| [zeroize]              | -                | [abscissa_core]   |

* § `winapi` is a facade for either [winapi-i686-pc-windows-gnu] or
    [winapi-x86_64-pc-windows-gnu] which aren't explicitly listed for brevity
    and are only required on Windows platforms.

## Frequently Asked Questions (FAQ)

### Q1: Why is it called "abscissa"?

**A1:** The word "abscissa" is the key to the [Kryptos K2] panel.

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

[//]: # (badges)

[logo]: https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa.svg
[crate-image]: https://img.shields.io/crates/v/abscissa_core.svg
[crate-link]: https://crates.io/crates/abscissa_core
[docs-image]: https://docs.rs/abscissa_core/badge.svg
[docs-link]: https://docs.rs/abscissa_core/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/abscissa/blob/develop/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.36+-blue.svg
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[build-image]: https://github.com/iqlusioninc/abscissa/workflows/Rust/badge.svg?branch=develop&event=push
[build-link]: https://github.com/iqlusioninc/abscissa/actions
[gitter-image]: https://badges.gitter.im/iqlusioninc/community.svg
[gitter-link]: https://gitter.im/iqlusioninc/community

[//]: # (general links)

[cargo]: https://github.com/rust-lang/cargo
[cargo features]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
[Kryptos K2]: https://en.wikipedia.org/wiki/Kryptos#Solution_of_passage_2
[cc]: https://contributor-covenant.org
[CODE_OF_CONDUCT.md]: https://github.com/iqlusioninc/abscissa/blob/develop/CODE_OF_CONDUCT.md
[CONTRIBUTING.md]: https://github.com/iqlusioninc/abscissa/blob/develop/CONTRIBUTING.md

[//]: # (projects using abscissa)

[Tendermint KMS]: https://github.com/tendermint/kms
[Canister]:  https://github.com/iqlusioninc/canister
[cargo-audit]: https://github.com/rustsec/cargo-audit
[cargo-rpm]: https://github.com/rustrpm/cargo-rpm
[OpenLibra]: https://github.com/open-libra/cli
[Sagan]: https://github.com/iqlusioninc/sagan
[Synchronicity]: https://github.com/iqlusioninc/synchronicity
[Zebra]: https://github.com/ZcashFoundation/zebra

[//]: # (crate links)

[abscissa]: https://crates.io/crates/abscissa
[abscissa_core]: https://crates.io/crates/abscissa_core
[abscissa_derive]: https://crates.io/crates/abscissa_derive
[aho-corasick]: https://crates.io/crates/aho-corasick
[arc-swap]: https://crates.io/crates/arc-swap
[atty]: https://github.com/softprops/atty
[autocfg]: https://crates.io/crates/autocfg
[backtrace]: https://crates.io/crates/backtrace
[backtrace-sys]: https://crates.io/crates/backtrace-sys
[byteorder]: https://crates.io/crates/byteorder
[canonical-path]: https://crates.io/crates/canonical-path
[color-backtrace]: https://github.com/athre0z/color-backtrace
[cc]: https://crates.io/crates/cc
[cfg-if]: https://crates.io/crates/cfg-if
[chrono]: https://crates.io/crates/chrono
[darling]: https://crates.io/crates/darling
[darling_core]: https://crates.io/crates/darling_core
[darling_macro]: https://crates.io/crates/darling_macro
[fnv]: https://crates.io/crates/fnv
[generational-arena]: https://github.com/fitzgen/generational-arena
[gumdrop]: https://crates.io/crates/gumdrop
[gumdrop_derive]: https://crates.io/crates/gumdrop_derive
[ident_case]: https://crates.io/crates/ident_case
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
[strsim]: https://crates.io/crates/strsim
[syn]: https://crates.io/crates/syn
[synstructure]: https://crates.io/crates/synstructure
[termcolor]: https://crates.io/crates/termcolor
[thread_local]: https://crates.io/crates/thread_local
[time]: https://crates.io/crates/time
[toml]: https://crates.io/crates/toml
[ucd-util]: https://crates.io/crates/ucd-util
[unicode-xid]: https://crates.io/crates/unicode-xid
[utf8-ranges]: https://crates.io/crates/utf8-ranges
[wait-timeout]: https://crates.io/crates/wait-timeout
[winapi]: https://crates.io/crates/winapi
[winapi-util]: https://crates.io/crates/winapi-util
[winapi-i686-pc-windows-gnu]: https://crates.io/crates/winapi-i686-pc-windows-gnu
[winapi-x86_64-pc-windows-gnu]: https://crates.io/crates/winapi-x86_64-pc-windows-gnu
[wincolor]: https://crates.io/crates/wincolor
[zeroize]: https://crates.io/crates/zeroize

[//]: # (author links)

[@alexcrichton]: https://github.com/alexcrichton
[@Amanieu]: https://github.com/Amanieu
[@BurntSushi]: https://github.com/BurntSushi
[@cuviper]: https://github.com/cuviper
[@dguo]: https://github.com/dguo
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

