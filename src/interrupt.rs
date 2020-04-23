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
/// - Do not call this function inside an `interrupt::free` critical section
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

/// Safely enables all interrupts by consuming a `CriticalSection`, which ensures that subsequent
/// code cannot borrow from a `Mutex` without creating a new critical section.
#[inline(always)]
pub fn enable_cs(_cs: CriticalSection) {
    unsafe { enable() };
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    let status = ::register::sr::read();

    // disable interrupts
    disable();

    let r = f(unsafe { &CriticalSection::new() });

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    if status.gie() {
        unsafe { enable() }
    }

    r
}
