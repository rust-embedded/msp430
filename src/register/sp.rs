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

/// Writes `bits` to the CPU register
#[inline(always)]
pub unsafe fn write(bits: u16) {
    asm!("mov {0}, R1", in(reg) bits);
}
