//! Miscellaneous assembly instructions

/// A no-operation. Useful to prevent delay loops from being optimized away.
#[inline(always)]
pub fn nop() {
    unsafe {
        asm!("nop"
             :
             :
             :
             : "volatile");
    }
}
