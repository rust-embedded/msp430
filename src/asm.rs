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

/// A compiler fence, prevents instruction reordering.
#[inline(always)]
pub fn barrier() {
    unsafe {
        asm!("" ::: "memory" : "volatile");
    }
}
