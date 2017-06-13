//! Core peripherals
//!

use core::marker::PhantomData;

use interrupt::{CriticalSection};

/// A peripheral
#[derive(Debug)]
pub struct Peripheral<T>
where
    T: 'static,
{
    address: usize,
    _marker: PhantomData<&'static mut T>,
}

impl<T> Peripheral<T> {
    /// Creates a new peripheral
    ///
    /// `address` is the base address of the register block
    pub const unsafe fn new(address: usize) -> Self {
        Peripheral {
            address: address,
            _marker: PhantomData,
        }
    }

    /// Borrows the peripheral for the duration of a critical section
    pub fn borrow<'cs>(&self, _ctxt: &'cs CriticalSection) -> &'cs T {
        unsafe { &*self.get() }
    }

    /// Returns a pointer to the register block
    pub fn get(&self) -> *mut T {
        self.address as *mut T
    }
}
