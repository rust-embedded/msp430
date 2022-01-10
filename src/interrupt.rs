//! Interrupts

pub use bare_metal::{CriticalSection, Mutex};

/// Disables all interrupts
#[inline(always)]
pub fn disable() {
    match () {
        #[cfg(target_arch = "msp430")]
        () => unsafe {
            llvm_asm!("dint { nop"
                 :
                 :
                 : "memory"
                 : "volatile");
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
            llvm_asm!("nop { eint { nop"
                 :
                 :
                 : "memory"
                 : "volatile");
        }
        #[cfg(not(target_arch = "msp430"))]
        () => {}
    }
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
pub fn free<F, R>(f: F) -> R
where
    F: for<'a> FnOnce(&'a CriticalSection<'a>) -> R,
{
    let status = ::register::sr::read();

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
