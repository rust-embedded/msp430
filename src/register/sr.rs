//! Status Register

/// Status Register
#[derive(Clone, Copy, Debug)]
pub struct Sr {
    bits: u16,
}

impl Sr {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> u16 {
        self.bits
    }

    /// Carry flag
    /// This bit is set when the result of an operation produced a carry 
    /// and cleared when no carry occurred.
    pub fn c(&self) -> bool {
        self.bits & (1 << 0) != 0
    }

    /// Zero flag
    /// Set when the result of a byte or word operation is 0 and cleared
    /// when the result is not 0.
    pub fn z(&self) -> bool {
        self.bits & (1 << 1) != 0
    }

    /// Negative flag
    /// Set when the result of a byte or word operation is negative and cleared
    /// when the result is not negative.
    pub fn n(&self) -> bool {
        self.bits & (1 << 2) != 0
    }

    /// General interrupt enable flag
    /// When this bit is set, it enables maskable interrupts.
    /// When it is reset, all maskable interrupts are disabled.
    pub fn gie(&self) -> bool {
        self.bits & (1 << 3) != 0
    }

    /// CPU off flag
    /// When set, turns off the CPU.
    pub fn cpuoff(&self) -> bool {
        self.bits & (1 << 4) != 0
    }

    /// Oscillator off flag
    /// When set, turns off the LFXT1 crystal oscillator
    pub fn oscoff(&self) -> bool {
        self.bits & (1 << 5) != 0
    }

    /// System clock generator 0 flag
    /// When set, turns off the DCO dc generator.
    pub fn scg0(&self) -> bool {
        self.bits & (1 << 6) != 0
    }

    /// System clock generator 1 flag
    /// When set, turns off the SMCLK.
    pub fn scg1(&self) -> bool {
        self.bits & (1 << 7) != 0
    }

    /// Overflow flag
    /// This bit is set when the result of an arithmetic operation
    /// overflows the signed-variable range.
    pub fn v(&self) -> bool {
        self.bits & (1 << 8) != 0
    }
}

/// Reads the CPU register
#[inline(always)]
pub fn read() -> Sr {
    let r: u16;
    unsafe {
        asm!("mov $0, R2"
             : "=r"(r)
             :
             :
             : "volatile");
    }
    Sr { bits: r }
}
