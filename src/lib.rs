//! Low level access to MSP430 microcontrollers
//!
//! This crate is based on [cortex-m](https://docs.rs/cortex-m)
//! crate by Jorge Aparicio (@japaric).
//!
//! It provides:
//!
//! - Access to core registers like SR and SP.
//! - Interrupt manipulation mechanisms
//! - Data structures like the vector table
//! - Safe wrappers around assembly instructions like `nop`

#![feature(abi_msp430_interrupt)]
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(naked_functions)]
#![no_std]

extern crate aligned;
extern crate volatile_register;

#[macro_use]
pub mod asm;
pub mod ctxt;
pub mod exception;
pub mod interrupt;
pub mod peripheral;
pub mod register;

/// A reserved spot in the vector table
#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Reserved {
    /// Reserved
    Vector = 0,
}
