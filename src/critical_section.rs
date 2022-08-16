//! Single-core critical section implementation using the [`critical_section`]
//! crate. Only the [`with`] function is publicly exposed.

#[cfg(all(target_arch = "msp430", feature = "critical-section"))]
mod critical_section {
    use crate::{interrupt, register};
    use critical_section::RawRestoreState;

    struct CriticalSection;
    critical_section::set_impl!(CriticalSection);

    unsafe impl critical_section::Impl for CriticalSection {
        unsafe fn acquire() -> RawRestoreState {
            let enabled_on_entry = register::sr::read().gie();
            interrupt::disable();
            enabled_on_entry
        }

        unsafe fn release(enabled_on_entry: RawRestoreState) {
            if enabled_on_entry {
                interrupt::enable();
            }
        }
    }
}

pub use ::critical_section::with;
