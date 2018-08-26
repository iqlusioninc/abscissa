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

[gumdrop]: https://github.com/murarth/gumdrop

## Frequently Asked Questions (FAQ)

### Q1: Why is it called "abscissa"?

**A1:** An abscissa represents the elevation of a point above the y-axis.
In that regard, "Abscissa" can be thought of as a pun about getting off
the ground, or elevating your project.

The word "abscissa" is also the key to the Kryptos K2 panel.

## License

The **abscissa** crate is distributed under the terms of the
Apache License (Version 2.0).

Parts of this code were taken from the following projects, which have agreed
to license their code under the Apache License (Version 2.0):

* [Cargo](https://github.com/rust-lang/cargo)
* [failure](https://github.com/withoutboats/failure)
* [gumdrop]
* [isatty](https://github.com/dtolnay/isatty)

See [LICENSE] file in the toplevel directory for more information.

[LICENSE]: https://github.com/iqlusioninc/abscissa/blob/master/LICENSE
