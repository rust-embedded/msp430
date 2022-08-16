//! Interrupts

use crate::asm;

pub use critical_section::{CriticalSection, Mutex};

/// Disables all interrupts
#[inline(always)]
pub fn disable() {
    match () {
        #[cfg(target_arch = "msp430")]
        () => unsafe {
            asm!("dint {{ nop");
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
            asm!("nop {{ eint {{ nop");
        }
        #[cfg(not(target_arch = "msp430"))]
        () => {}
    }
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
#[deprecated(since = "0.3.1", note = "critical_section::with() allows alternate implementations; interrupt:free() is a hardcoded implementation of critical_section::with() with a different type signature.")]
pub fn free<F, R>(f: F) -> R
where
    F: for<'a> FnOnce(&'a CriticalSection<'a>) -> R,
{
    let status = crate::register::sr::read();

    // disable interrupts
    disable();

    let cs = unsafe { CriticalSection::new() };
    let r = f(&cs);

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    if status.gie() {
        unsafe { enable() }
    }

    r
}
