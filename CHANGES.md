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
