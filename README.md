[![crates.io](https://img.shields.io/crates/d/msp430.svg)](https://crates.io/crates/msp430)
[![crates.io](https://img.shields.io/crates/v/msp430.svg)](https://crates.io/crates/msp430)

# `msp430`

> Low level access to MSP430 microcontrollers

This crate is based on [cortex-m](https://docs.rs/cortex-m) crate by Jorge Aparicio (@japaric).

**This crate requires a nightly rust due to the use of the new `asm!` (`0.3.0`
and above), `llvm_asm!` (`0.2.2`) or old `asm!` (`0.2.1` and below) macros.**
The below table contains compilers which are known to work:

|`msp430` version|`rustc` compiler    |
|----------------|--------------------|
|`0.3.0`         |`nightly-2022-01-24`|
|`0.2.2`         |`nightly-2020-04-22`|
|`0.2.1`         |`nightly-2020-01-04`|

## [Documentation](https://docs.rs/crate/msp430)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
