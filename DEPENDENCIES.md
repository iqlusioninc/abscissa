### NOTE: this information is out-of-date!

See [#649](https://github.com/iqlusioninc/abscissa/issues/649).

# Depencencies

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

| #  | Crate Name             | Origin           | License        | Description               |
|----|------------------------|------------------|----------------|---------------------------|
| 1  | [abscissa_core]        | [iqlusion]       | Apache-2.0     | Abscissa framework        |
| 2  | [aho-corasick]         | [@BurntSushi]    | MIT/Unlicense  | Pattern-matching alg      |
| 3  | [ansi_term]            | [@ogham]         | MIT            | Terminal color libray     |
| 4  | [arc-swap]             | [@vorner]        | Apache-2.0/MIT | Atomic swap for `Arc`     |
| 5  | [atty]                 | [@softprops]     | MIT            | Detect TTY presence       |
| 6  | [autocfg]              | [@cuviper]       | Apache-2.0/MIT | Rust compiler configs     |
| 7  | [backtrace]            | [@alexcrichton]  | Apache-2.0/MIT | Capture stack traces      |
| 8  | [backtrace-sys]        | [@alexcrichton]  | Apache-2.0/MIT | Capture stack traces      |
| 9  | [byteorder]            | [@BurntSushi]    | MIT/Unlicense  | Byte order conversions    |
| 10 | [canonical-path]       | [iqlusion]       | Apache-2.0     | Get canonical fs paths    |
| 11 | [chrono]               | [chronotope]     | Apache-2.0/MIT | Time/date library         |
| 12 | [clap]                 | [@Kevin K]       | Apache-2.0/MIT | Command-line interface    |
| 13 | [color-backtrace]      | [@athre0z]       | Apache-2.0/MIT | Rich colored backtraces   |
| 14 | [fs-err]               | [@andrewhickman] | Apache-2.0/MIT | Better filesystem errors  |
| 15 | [generational-arena]   | [@fitzgen]       | MPL-2.0        | Component allocator       |
| 16 | [lazy_static]          | [rust-lang]      | Apache-2.0/MIT | Heap-allocated statics    |
| 17 | [libc]                 | [rust-lang]      | Apache-2.0/MIT | C library wrapper         |
| 18 | [log]                  | [rust-lang]      | Apache-2.0/MIT | Logging facade library    |
| 19 | [matchers]             | [@hawkw]         | MIT            | Stream regex matching     |
| 20 | [maybe-uninit]         | [@est31]         | Apache-2.0/MIT | MaybeUninit compat        |
| 21 | [memchr]               | [@BurntSushi]    | MIT/Unlicense  | Optimized byte search     |
| 22 | [num-integer]          | [rust-num]       | Apache-2.0/MIT | `Integer` trait           |
| 23 | [num-traits]           | [rust-num]       | Apache-2.0/MIT | Numeric traits            |
| 24 | [once_cell]            | [@matklad]       | Apache-2.0/MIT | Single assignment cells   |
| 25 | [redox_syscall]        | [redox-os]       | MIT            | Redox OS syscall API      |
| 26 | [regex]                | [rust-lang]      | Apache-2.0/MIT | Regular expressions       |
| 27 | [regex-automata]       | [@BurntSushi]    | MIT/Unlicense  | Low-level regex DFAs      |
| 28 | [regex-syntax]         | [rust-lang]      | Apache-2.0/MIT | Regex syntax impl         |
| 29 | [rustc-demangle]       | [@alexcrichton]  | Apache-2.0/MIT | Symbol demangling         |
| 30 | [secrecy]              | [iqlusion]       | Apache-2.0     | Secret-keeping types      |
| 31 | [semver]               | [@steveklabnik]  | Apache-2.0/MIT | Semantic versioning       |
| 32 | [semver-parser]        | [@steveklabnik]  | Apache-2.0/MIT | Parser for semver spec    |
| 33 | [serde]                | [serde-rs]       | Apache-2.0/MIT | Serialization framework   |
| 34 | [sharded-slab]         | [@hawkw]         | MIT            | Concurrent slab allocator |
| 35 | [signal-hook]          | [@vorner]        | Apache-2.0/MIT | Unix signal handling      |
| 36 | [signal-hook-registry] | [@vorner]        | Apache-2.0/MIT | Unix signal registry      |
| 37 | [smallvec]             | [servo]          | Apache-2.0/MIT | Optimized small vectors   |
| 38 | [termcolor]            | [@BurntSushi]    | MIT/Unlicense  | Terminal color support    |
| 39 | [thread_local]         | [@Amanieu]       | Apache-2.0/MIT | Per-object thread local   |
| 40 | [time]                 | [rust-lang]      | Apache-2.0/MIT | Time/date library         |
| 41 | [toml]                 | [@alexcrichton]  | Apache-2.0/MIT | TOML parser library       |
| 42 | [tracing]              | [tokio-rs]       | MIT            | App tracing / logging     |
| 43 | [tracing-core]         | [tokio-rs]       | MIT            | App tracing / logging     |
| 44 | [tracing-log]          | [tokio-rs]       | MIT            | `log` compatibility       |
| 45 | [tracing-subscriber]   | [tokio-rs]       | MIT            | Tracing subscribers       |
| 46 | [utf8-ranges]          | [@BurntSushi]    | MIT/Unlicense  | UTF-8 codepoint ranges    |
| 47 | [winapi]§              | [@retep998]      | Apache-2.0/MIT | Windows FFI bindings      |
| 48 | [winapi-util]          | [@BurntSushi]    | MIT/Unlicense  | Safe winapi wrappers      |
| 49 | [wincolor]             | [@BurntSushi]    | MIT/Unlicense  | Windows console color     |
| 50 | [zeroize]              | [iqlusion]       | Apache-2.0/MIT | Zero out sensitive data   |

### Build / Development / Testing Dependencies

| #  | Crate Name           | Origin           | License        | Description             |
|----|----------------------|------------------|----------------|-------------------------|
| 1  | [abscissa_derive]    | [iqlusion]       | Apache-2.0     | Abscissa custom derive  |
| 2  | [cc]                 | [@alexcrichton]  | Apache-2.0/MIT | C/C++ compiler wrapper  |
| 3  | [cfg-if]             | [@alexcrichton]  | Apache-2.0/MIT | If-like `#[cfg]` macros |
| 4  | [clap_derive]        | [@Kevin K]       | Apache-2.0/MIT | Command-line interface  |
| 5  | [darling]            | [@TedDriggs]     | MIT            | Nifty attribute parser  |
| 6  | [darling_core]       | [@TedDriggs]     | MIT            | Attribute parser core   |
| 7  | [darling_macro]      | [@TedDriggs]     | MIT            | Attribute parser macros |
| 8  | [fnv]                | [@alexcrichton]  | Apache-2.0/MIT | Fast hash function      |
| 9  | [ident_case]         | [@TedDriggs]     | Apache-2.0/MIT | Case conversion utils   |
| 10 | [proc-macro2]        | [@alexcrichton]  | Apache-2.0/MIT | Shim for Macros 2.0 API |
| 11 | [quote]              | [@dtolnay]       | Apache-2.0/MIT | Rust AST to token macro |
| 12 | [serde_derive]       | [serde-rs]       | Apache-2.0/MIT | `serde` custom derive   |
| 13 | [strsim]             | [@dguo]          | MIT            | String similarity utils |
| 14 | [syn]                | [@dtolnay]       | Apache-2.0/MIT | Rust source code parser |
| 15 | [synstructure]       | [@mystor]        | Apache-2.0/MIT | `syn` structure macros  |
| 16 | [thiserror]          | [@dtolnay]       | Apache-2.0/MIT | `Error` custom derive   |
| 17 | [tracing-attributes] | [tokio-rs]       | MIT            | App tracing / logging   |
| 18 | [unicode-xid]        | [unicode-rs]     | Apache-2.0/MIT | Identify valid Unicode  |
| 19 | [wait-timeout]       | [@alexcrichton]  | Apache-2.0/MIT | Timeouts for waitpid    |

### Dependency Relationships

The table below should help answer questions as to why a particular crate is
an Abscissa dependency and whether or not it is optional. Abscissa uses
[cargo features] to allow parts of it you aren't using to be easily disabled,
so you only compile the parts you need.

| Crate Name             | [Cargo Features]   | Required By                |
|------------------------|--------------------|----------------------------|
| [abscissa_core]        | -                  | ⊤                          |
| [abscissa_derive]      | -                  | [abscissa_core]            |
| [aho-corasick]         | `trace`, `testing` | [regex]                    |
| [ansi_term]            | `trace`            | [tracing-subscriber]       |
| [arc-swap]             | `signals`          | [signal-hook-registry]     |
| [atty]                 | `terminal`         | [color-backtrace]          |
| [autocfg]              | `time`             | [num-integer]              |
| [backtrace]            | -                  | [abscissa_core]            |
| [backtrace-sys]        | -                  | [backtrace]                |
| [byteorder]            | `trace`            | [regex-automata]           |
| [canonical-path]       | -                  | [abscissa_core]            |
| [cc]                   | -                  | [backtrace-sys]            |
| [cfg-if]               | -                  | [backtrace], [log]         |
| [color-backtrace]      | `terminal`         | [abscissa_core]            |
| [chrono]               | `time`             | [abscissa_core]            |
| [clap]                 | `option`           | [abscissa_core]            |
| [clap_derive]          | `option`           | [clap]                     |
| [darling]              | -                  | [abscissa_derive]          |
| [darling_core]         | -                  | [darling], [darling_macro] |
| [darling_macro]        | -                  | [darling]                  |
| [fs-err]               | -                  | [abscissa_core]            |
| [fnv]                  | -                  | [darling_core]             |
| [generational-arena]   | `application`      | [abscissa_core]            |
| [ident_case]           | -                  | [abscissa_derive], [darling_core] |
| [lazy_static]          | `testing`, `trace` | [thread_local], [tracing-core], [tracing-log], [tracing-subscriber] |
| [libc]                 | `signals`          | [abscissa_core]            |
| [log]                  | `logging`          | [abscissa_core]            |
| [matchers]             | `trace`            | [tracing-subscriber]       |
| [memchr]               | `trace`, `testing` | [aho-corasick]             |
| [maybe-uninit]         | `trace`            | [smallvec]                 |
| [num-integer]          | `time`             | [chrono]                   |
| [num-traits]           | `time`             | [chrono], [num-integer]    |
| [once_cell]            | -                  | [abscissa_core]            |
| [proc-macro2]          | -                  | [abscissa_derive], [darling], [quote], [serde_derive], [syn] |
| [quote]                | -                  | [abscissa_derive], [darling], [serde_derive] |
| [redox_syscall]        | `time`             | [time]                     |
| [regex]                | `trace`, `testing` | [abscissa_core]            |
| [regex-automata]       | `trace`            | [matchers]                 |
| [regex-syntax]         | `trace`, `testing` | [abscissa_core]            |
| [rustc-demangle]       | -                  | [backtrace]                |
| [secrecy]              | `secrets`          | [abscissa_core]            |
| [semver]               | `application`      | [abscissa_core]            |
| [semver-parser]        | `application`      | [abscissa_core]            |
| [serde]                | `config`           | [abscissa_core]            |
| [serde_derive]         | `config`           | [serde]                    |
| [signal-hook]          | `signals`          | [abscissa_core]            |
| [sharded-slab]         | `trace`            | [tracing-subscriber]       |
| [signal-hook-registry] | `signals`          | [signal-hook]              |
| [smallvec]             | `trace`            | [tracing-subscriber]       |
| [strsim]               | -                  | [darling_core]             |
| [syn]                  | -                  | [abscissa_derive], [darling], [serde_derive] |
| [termcolor]            | `terminal`         | [abscissa_core]            |
| [thiserror]            | -                  | Abscissa boilerplate       |
| [thread_local]         | `trace`, `testing` | [regex]                    |
| [time]                 | `logging`          | [chrono]                   |
| [tracing]              | `trace`            | [abscissa_core]            |
| [tracing-attributes]   | `trace`            | [tracing]                  |
| [tracing-core]         | `trace`            | [tracing]                  |
| [tracing-log]          | `trace`            | [abscissa_core]            |
| [tracing-subscriber]   | `trace`            | [abscissa_core]            |
| [unicode-xid]          | -                  | [proc-macro2], [syn]       |
| [utf8-ranges]          | `trace`, `testing` | [regex]                    |
| [wait-timeout]         | `testing`          | [abscissa_core]            |
| [winapi]§              | -                  | [termcolor], [time], [winapi-util] |
| [winapi-util]          | -                  | [termcolor]                |
| [wincolor]             | `terminal`         | [termcolor]                |
| [zeroize]              | -                  | [abscissa_core]            |

* § `winapi` is a facade for either [winapi-i686-pc-windows-gnu] or
[winapi-x86_64-pc-windows-gnu] which aren't explicitly listed for brevity
and are only required on Windows platforms.

[//]: # (crate links)

[abscissa]: https://crates.io/crates/abscissa
[abscissa_core]: https://crates.io/crates/abscissa_core
[abscissa_derive]: https://crates.io/crates/abscissa_derive
[abscissa_tokio]: https://crates.io/crates/abscissa_tokio
[aho-corasick]: https://crates.io/crates/aho-corasick
[ansi_term]: https://crates.io/crates/ansi-term
[arc-swap]: https://crates.io/crates/arc-swap
[atty]: https://crates.io/crates/atty
[autocfg]: https://crates.io/crates/autocfg
[backtrace]: https://crates.io/crates/backtrace
[backtrace-sys]: https://crates.io/crates/backtrace-sys
[byteorder]: https://crates.io/crates/byteorder
[canonical-path]: https://crates.io/crates/canonical-path
[color-backtrace]: https://github.com/athre0z/color-backtrace
[cc]: https://crates.io/crates/cc
[cfg-if]: https://crates.io/crates/cfg-if
[chrono]: https://crates.io/crates/chrono
[clap]: https://crates.io/crates/clap
[clap_derive]: https://crates.io/crates/clap_derive
[darling]: https://crates.io/crates/darling
[darling_core]: https://crates.io/crates/darling_core
[darling_macro]: https://crates.io/crates/darling_macro
[fs-err]: https://crates.io/crates/fs-err
[fnv]: https://crates.io/crates/fnv
[generational-arena]: https://github.com/fitzgen/generational-arena
[ident_case]: https://crates.io/crates/ident_case
[lazy_static]: https://crates.io/crates/lazy_static
[libc]: https://crates.io/crates/libc
[log]: https://crates.io/crates/log
[matchers]: https://crates.io/crates/matchers
[maybe-uninit]: https://crates.io/crates/maybe-uninit
[memchr]: https://crates.io/crates/memchr
[num-integer]: https://crates.io/crates/num-integer
[num-traits]: https://crates.io/crates/num-traits
[once_cell]: https://crates.io/crates/once_cell
[proc-macro2]: https://crates.io/crates/proc-macro2
[quote]: https://crates.io/crates/quote
[redox_syscall]: https://crates.io/crates/redox_syscall
[regex]: https://crates.io/crates/regex
[regex-automata]: https://crates.io/crates/regex-automata
[regex-syntax]: https://crates.io/crates/regex-syntax
[rustc-demangle]: https://crates.io/crates/rustc_demangle
[secrecy]: https://crates.io/crates/secrecy
[semver]: https://crates.io/crates/semver
[semver-parser]: https://crates.io/crates/semver-parser
[serde]: https://crates.io/crates/serde
[serde_derive]: https://crates.io/crates/serde_derive
[sharded-slab]: https://crates.io/crates/sharded-slab
[signal-hook]: https://crates.io/crates/signal-hook
[signal-hook-registry]: https://crates.io/crates/signal-hook
[smallvec]: https://crates.io/crates/smallvec
[strsim]: https://crates.io/crates/strsim
[syn]: https://crates.io/crates/syn
[synstructure]: https://crates.io/crates/synstructure
[termcolor]: https://crates.io/crates/termcolor
[thiserror]: https://github.com/dtolnay/thiserror
[thread_local]: https://crates.io/crates/thread_local
[time]: https://crates.io/crates/time
[toml]: https://crates.io/crates/toml
[tracing]: https://crates.io/crates/tracing
[tracing-attributes]: https://crates.io/crates/tracing-attributes
[tracing-core]: https://crates.io/crates/tracing-core
[tracing-log]: https://crates.io/crates/tracing-log
[tracing-subscriber]: https://crates.io/crates/tracing-subscriber
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
[@andrewhickman]: https://github.com/andrewhickman
[@athre0z]: https://github.com/athre0z
[@BurntSushi]: https://github.com/BurntSushi
[@cuviper]: https://github.com/cuviper
[@dguo]: https://github.com/dguo
[@dtolnay]: https://github.com/dtolnay
[@est31]: https://github.com/est31
[@fitzgen]: https://github.com/fitzgen
[@hawkw]: https://github.com/hawkw
[@Kevin K]: https://github.com/kbknapp
[@Kimundi]: https://github.com/Kimundi
[@matklad]: https://github.com/matklad
[@mvdnes]: https://github.com/mvdnes
[@mystor]: https://github.com/mystor
[@ogham]: https://github.com/ogham
[@retep998]: https://github.com/retep998
[@SergioBenitez]: https://github.com/SergioBenitez
[@steveklabnik]: https://github.com/steveklabnik
[@Storyyeller]: https://github.com/storyyeller
[@softprops]: https://github.com/softprops
[@TedDriggs]: https://github.com/TedDriggs
[@vorner]: https://github.com/vorner
[@withoutboats]: https://github.com/withoutboats
[chronotope]: https://github.com/chronotope/
[iqlusion]: https://www.iqlusion.io
[redox-os]: https://github.com/redox-os
[rust-lang]: https://github.com/rust-lang/
[rust-num]: https://github.com/rust-num/
[serde-rs]: https://github.com/serde-rs/
[servo]: https://github.com/servo
[tokio-rs]: https://github.com/tokio-rs
[unicode-rs]: https://github.com/unicode-rs/
