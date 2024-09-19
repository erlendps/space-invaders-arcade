/// Struct containing the state for the 8080 condition flags.
#[derive(Debug)]
pub struct Flags8080 {
    z: bool,
    s: bool,
    p: bool,
    ac: bool,
    cy: bool,
}

/// Struct containing state for the 8080 processor.
#[derive(Debug)]
pub struct Emulator8080 {
    // Special registers
    /// Program Counter
    pc: u16,
    /// Stack pointer
    sp: u16,

    // general purpose registers
    /// Register A
    ra: u8,
    /// Register B
    rb: u8,
    /// Register C
    rc: u8,
    /// Register D
    rd: u8,
    /// Register E
    re: u8,
    /// Register H
    rh: u8,
    /// Register L
    rl: u8,

    // condition flags
    flags: Flags8080,

    // memory
    memory: Vec<u8>,

    // enable
    enable: bool,
}

impl Emulator8080 {
    /* Helper functions */

    /// Decides if the operation results in a auxiliary carry.
    ///
    /// TODO: implement
    pub fn aux_carry(_: u8) -> bool {
        false
    }

    /// Determines whether the value (which is a result of a arithmetic operation)
    /// has carried/borrowed into/from bit 8.
    ///
    /// Example:
    /// ```rust
    /// use emulator::Emulator8080
    /// Emulator8080::carry(0x14f) // returns true
    /// ```
    pub fn carry(value: u8, compare_to: u8, subtraction: bool) -> bool {
        if subtraction {
            return value > compare_to;
        }
        value < compare_to
    }
    /// The parity method performs a parity check of the value.
    ///
    /// The method returns true if there is an even number of 1's in `value`
    /// and false otherwise.
    ///
    /// Example:
    /// ```rust
    /// use emulator::Emulator8080
    /// Emulator8080::parity(0x02) // returns false
    /// ```
    pub fn parity(mut value: u8) -> bool {
        value ^= value >> 4;
        value ^= value >> 2;
        value ^= value >> 1;
        (!value & 0x1) != 0
    }

    /// The sign method decides the sign of the `value` byte.
    ///
    /// If the most significant but of `value` is set, return `true`,
    /// `false` otherwise.
    pub fn sign(value: u8) -> bool {
        (value & 0x80) != 0
    }

    /// The zero method decides if `value` is zero
    pub fn zero(value: u8) -> bool {
        (value & 0xff) == 0
    }

    /// Sets the flags specified by the `set_*` parameters.
    ///
    /// This method is meant to be used in arithmetic operation.
    fn set_flags(
        &mut self,
        result: u8,
        compare_to: u8,
        subtraction: bool,
        set_ac: bool,
        set_cy: bool,
        set_p: bool,
        set_s: bool,
        set_z: bool,
    ) {
        // carries, requires u16
        if set_ac {
            self.flags.ac = Emulator8080::aux_carry(result);
        }
        if set_cy {
            self.flags.cy = Emulator8080::carry(result, compare_to, subtraction);
        }
        // requires u8
        if set_p {
            self.flags.p = Emulator8080::parity(result);
        }
        if set_s {
            self.flags.s = Emulator8080::sign(result);
        }
        if set_z {
            self.flags.z = Emulator8080::zero(result);
        }
    }

    /// Fetches the address that is in H and L.
    fn get_address(&self) -> u16 {
        ((self.rh as u16) << 8) | (self.rl as u16)
    }

    /* Arithmetic group */
    fn add(&mut self, lhs: u8, rhs: u8, with_carry: bool) -> u8 {
        lhs.wrapping_add(rhs)
            .wrapping_add(if with_carry { self.flags.cy as u8 } else { 0 })
    }

    /// Adds the content of `value` to the accumulator register (`ra`).
    ///
    /// The method is a general method, in that it can add registers,
    /// immediates and everything that is u8.
    ///
    /// The function also sets flags affected. This method affects
    /// Z, S, P, CY and AC.
    pub fn add_a(&mut self, value: u8, with_carry: bool) {
        let result = self.add(self.ra, value, with_carry);

        self.set_flags(result, self.ra, false, true, true, true, true, true);
        self.ra = result;
    }

    /// Adds the content of the memory location specified by (HL).
    ///
    /// The method affects Z, S, P, CY and AC
    pub fn add_mem(&mut self, with_carry: bool) {
        let address = self.get_address() as usize;
        self.add_a(self.memory[address], with_carry);
    }
    /* Branch group */
    /* Data transfer group */
    /* Logical group */
    /* Stack, I/O and machine control group */
}
