//! Miscellaneous assembly instructions

use crate::asm;

/// A no-operation. Useful to prevent delay loops from being optimized away.
///
/// Unlike [barrier], this does not prevent reordering of memory access.
#[inline(always)]
pub fn nop() {
    unsafe {
        // Do not use pure because prevent nop from being removed.
        asm!("nop", options(nomem, nostack, preserves_flags));
    }
}

/// A compiler fence, prevents instruction reordering.
///
/// Unlike [nop], this does not emit machine code.
#[inline(always)]
pub fn barrier() {
    unsafe {
        // Do not use `nomem` and `readonly` because prevent preceding and subsequent memory accesses from being reordered.
        asm!("", options(nostack, preserves_flags));
    }
}
