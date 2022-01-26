//! Miscellaneous assembly instructions

use crate::asm;

/// A no-operation. Useful to prevent delay loops from being optimized away.
#[inline(always)]
pub fn nop() {
    unsafe {
        asm!("nop");
    }
}

/// A compiler fence, prevents instruction reordering.
#[inline(always)]
pub fn barrier() {
    unsafe {
        asm!("");
    }
}
