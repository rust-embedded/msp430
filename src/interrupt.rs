//! Interrupts

use crate::asm;

pub use critical_section::{CriticalSection, Mutex};

/// Disables all interrupts
#[inline(always)]
pub fn disable() {
    match () {
        #[cfg(target_arch = "msp430")]
        () => unsafe {
            // Do not use `nomem` and `readonly` because prevent subsequent memory accesses from being reordered before interrupts are disabled.
            // Do not use `preserves_flags` because DINT modifies the GIE (global interrupt enable) bit of the status register.
            asm!("dint {{ nop", options(nostack));
        },
        #[cfg(not(target_arch = "msp430"))]
        () => {}
    }
}

/// Enables all the interrupts
///
/// # Safety
///
/// - In any function `f()` that calls `enable`, `CriticalSection` or `&CriticalSection` tokens cannot be used in `f()`'s body after the
///   call to `enable`. If `f()` owns `CriticalSection` tokens, it is recommended to [`drop`](https://doc.rust-lang.org/nightly/core/mem/fn.drop.html)
///   these tokens before calling `enable`.
#[inline(always)]
pub unsafe fn enable() {
    match () {
        #[cfg(target_arch = "msp430")]
        () => {
            // Do not use `nomem` and `readonly` because prevent preceding memory accesses from being reordered after interrupts are enabled.
            // Do not use `preserves_flags` because EINT modifies the GIE (global interrupt enable) bit of the status register.
            asm!("nop {{ eint {{ nop", options(nostack));
        }
        #[cfg(not(target_arch = "msp430"))]
        () => {}
    }
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
#[deprecated(since = "0.4.0", note = "critical_section::with() allows alternate implementations; interrupt:free() is a hardcoded implementation of critical_section::with() with a different type signature.")]
pub fn free<F, R>(f: F) -> R
where
    F: for<'a> FnOnce(&'a CriticalSection<'a>) -> R,
{
    // disable interrupts
    let status = unsafe { acquire() };

    let cs = unsafe { CriticalSection::new() };
    let r = f(&cs);

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    unsafe { release(status); }

    r
}

// Not strictly necessary, but these two functions exist done to keep parity
// with critical_section::with() and to test size optimizations easily.
#[cfg_attr(any(feature = "outline-cs", feature = "outline-cs-acq"), inline(never))]
#[cfg_attr(all(not(feature = "outline-cs"), not(feature="outline-cs-acq")), inline)]
unsafe fn acquire() -> crate::register::sr::Sr {
    let status = crate::register::sr::read();
    disable();
    status
}

#[cfg_attr(any(feature = "outline-cs", feature = "outline-cs-rel"), inline(never))]
#[cfg_attr(all(not(feature = "outline-cs"), not(feature="outline-cs-rel")), inline)]
unsafe fn release(sr: crate::register::sr::Sr) {
    if sr.gie() {
        enable()
    }
}
