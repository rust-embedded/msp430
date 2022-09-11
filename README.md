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

## Features

The `critical-section-single-core` feature provides a [critical section](https://github.com/rust-embedded/critical-section)
implementation based upon disabling interrupts.

Critical sections by disabling interrupts are a valuable way to access I/O and
shared data safely in msp430. However, rustc/LLVM does not always optimize
critical sections well in terms of space. For example, sometimes rustc/LLVM
will create two copies of interrupt enable assembly code when exiting a
critical section that contains a branch- one copy each for
branch-taken/branch-not-taken.

This crate provides three features for giving hints to rustc/LLVM for how to
optimize critical sections for size. Both `critical_section::with` and
the `interrupt::free` critical sections are supported, with and without the
`critical-section` feature above:

* `outline-cs-acq`: Hint to rustc/LLVM that each critical section entry should
   be a call to a single copy of an `acquire` function (disable interrupts).
* `outline-cs-rel`: Hint to rustc/LLVM that each critical section exit should
   be a call to a single copy of a `release` function (enable interrupts if not
   in a nested critical section).
* `outline-cs`: Convenience feature for enabling both of the above at the same
   time.

If saving space is a concern in your application, you should experiment with
which features provide the best size savings and balance this against the
execution overhead of the extra function calls due to outlining. The execution
overhead of enabling each hint is ~5 + 2 clock cycles for each
critical section- at least a `CALL` and `RET` instruction.

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
