//! Program counter

use crate::asm;

/// Reads the CPU register
#[inline(always)]
pub fn read() -> u16 {
    let r;
    unsafe {
        asm!("mov R0, {0}", out(reg) r);
    }
    r
}

/// Writes `bits` to the CPU register
#[inline(always)]
pub unsafe fn write(bits: u16) {
    asm!("mov {0}, R0", in(reg) bits);
}
