# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

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

[Unreleased]: https://github.com/pftbest/msp430/compare/v0.2.1...HEAD
[v0.2.1]: https://github.com/pftbest/msp430/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/pftbest/msp430/compare/v0.1.0...v0.2.0
