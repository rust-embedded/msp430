# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.0] - 2019-12-26

Updated bare_metal to 0.2.x. The crate's API is compatible with 0.1.0, but
using bare_metal v0.2.x causes incompatible type errors with device crates
(PACs) using bare_metal v0.1.x. This requires a major version bump to fix.

## v0.1.0 - 2017-07-22

Initial release.

[Unreleased]: https://github.com/pftbest/msp430/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/pftbest/msp430/compare/v0.1.0...v0.2.0
