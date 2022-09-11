//! Program counter

use crate::asm;

/// Reads the CPU register
#[inline(always)]
pub fn read() -> u16 {
    let r;
    unsafe {
        asm!("mov R0, {0}", out(reg) r, options(nomem, nostack, preserves_flags));
    }
    r
}
