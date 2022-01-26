//! Main Stack Pointer

use crate::asm;

/// Reads the CPU register
#[inline(always)]
pub fn read() -> u16 {
    let r;
    unsafe {
        asm!("mov R1, {0}", out(reg) r);
    }
    r
}
