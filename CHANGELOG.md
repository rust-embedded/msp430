# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.3.0] - 2022-01-25

### Changed
- Bumped `bare-metal` to version `1.0.0`. Using bare_metal v1.x causes
incompatible type errors with device crates (PACs) using bare-metal v0.2.x.
This, _among other removed features_, requires a major version bump to fix.
- All uses of the `llvm_asm!` macro have been replaced with `asm!`, in
accordance with [Issue 92816](https://github.com/rust-lang/rust/pull/92816).

### Removed
- `enable_cs` removed due to soundness hole when interacting with `Clone` and
  `interrupt::free`.
- Remove `peripherals` module since the peripheral API is no longer provided by
  `bare-metal`.
- `register::{sp, pc}::write` have been removed; inline assembly [mandates](https://doc.rust-lang.org/nightly/reference/inline-assembly.html#rules-for-inline-assembly)
  that the stack pointer is restored before leaving an asm block. Writing
  PC is also being removed as a precaution.

## [v0.2.2] - 2020-04-23

### Changed
- All uses of the `asm!` macro have been replace with `llvm_asm!`, in
accordance with [Issue 70173](https://github.com/rust-lang/rust/issues/70173).

### Removed
- `deny(warnings)` lint, as it's too restrictive.

## [v0.2.1] - 2020-04-23

### Added
- Add `enable_cs(_cs: CriticalSection)` function, which safely enable
interrupts by consuming a `CriticalSection` token. Because the token is
consumed, it is not possible to enable interrupts using `enable_cs` while
inside an `interrupt::free` critical section.

## [v0.2.0] - 2019-12-26

### Added
- CI Support

### Changed
- Updated [bare-metal] to 0.2.x. This crate's API is source-compatible with
0.1.0, but using bare_metal v0.2.x causes incompatible type errors with device
crates (PACs) using [bare-metal] v0.1.x. This requires a major version bump to
fix.

## v0.1.0 - 2017-07-22

Initial release.

[bare-metal]: https://github.com/japaric/bare-metal

[Unreleased]: https://github.com/rust-embedded/msp430/compare/v0.3.0...HEAD
[v0.3.0]: https://github.com/rust-embedded/msp430/compare/v0.2.2...v0.3.0
[v0.2.2]: https://github.com/rust-embedded/msp430/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/rust-embedded/msp430/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/rust-embedded/msp430/compare/v0.1.0...v0.2.0
