//! Main Stack Pointer

/// Reads the CPU register
#[inline(always)]
pub fn read() -> u16 {
    let r;
    unsafe {
        asm!("mov R1,$0"
             : "=r"(r)
             :
             :
             : "volatile");
    }
    r
}

/// Writes `bits` to the CPU register
#[inline(always)]
pub unsafe fn write(bits: u16) {
    asm!("mov $0,R1"
         :
         : "r"(bits)
         :
         : "volatile");
}
