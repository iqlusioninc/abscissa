![Abscissa](https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa.svg)

# abscissa_tokio: Tokio component for Abscissa

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
[![Build Status][build-image]][build-link]

Support for launching [Tokio] runtimes within [Abscissa] applications.

[Documentation][docs-link]

## About

Where normally you'd use something like the [`tokio::main`] macro to launch
the Tokio runtime, in Abscissa the framework is launched by calling
[`abscissa_core::boot`] from your application's `main()`.

This means Abscissa applications need a slightly different convention for
starting the Tokio runtime, and ideally one which allows all application
subcomponents to register themselves before the runtime is started.

This crate handles instantiating the Tokio runtime as an Abscissa [Component],
allowing other application components to express they have a Tokio dependency
so Abscissa can inject the Tokio component as a dependency.

Once the application has booted and all subcomponents have been registered with
the Tokio runtime, it allows (any of) your application's `Runnable` types to
start the runtime without having to hold a lock on application state.

See documentation for usage instructions.

## License

The **abscissa_tokio** crate is distributed under the terms of the
Apache License (Version 2.0).

Copyright © 2020 iqlusion

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/abscissa_tokio.svg
[crate-link]: https://crates.io/crates/abscissa_tokio
[docs-image]: https://docs.rs/abscissa_tokio/badge.svg
[docs-link]: https://docs.rs/abscissa_tokio/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/abscissa/blob/main/LICENSE
[build-image]: https://github.com/iqlusioninc/abscissa/workflows/tokio/badge.svg?branch=main&event=push
[build-link]: https://github.com/iqlusioninc/abscissa/actions?query=workflow:tokio

[//]: # (general links)

[Tokio]: https://tokio.rs/
[Abscissa]: https://github.com/iqlusioninc/abscissa
[`tokio::main`]: https://docs.rs/tokio/latest/tokio/attr.main.html
[`abscissa_core::boot`]: https://docs.rs/abscissa_core/latest/abscissa_core/application/fn.boot.html
[Component]: https://docs.rs/abscissa_core/latest/abscissa_core/component/trait.Component.html
