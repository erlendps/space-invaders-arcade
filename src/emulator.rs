pub enum RegPair {
    BC,
    DE,
    HL,
    SP,
}
/// Struct containing the state for the 8080 condition flags.
#[derive(Debug)]
pub struct Flags8080 {
    z: bool,
    s: bool,
    p: bool,
    ac: bool,
    cy: bool,
}

impl Flags8080 {
    pub fn new() -> Self {
        Self {
            z: false,
            s: false,
            p: false,
            ac: false,
            cy: false,
        }
    }
}

/// Struct containing state for the 8080 processor.
#[derive(Debug)]
pub struct Emulator8080 {
    // Special registers
    /// Program Counter
    pub pc: u16,
    /// Stack pointer
    pub sp: u16,

    // general purpose registers
    /// Register A
    pub ra: u8,
    /// Register B
    pub rb: u8,
    /// Register C
    pub rc: u8,
    /// Register D
    pub rd: u8,
    /// Register E
    pub re: u8,
    /// Register H
    pub rh: u8,
    /// Register L
    pub rl: u8,

    // condition flags
    pub flags: Flags8080,

    // memory
    pub memory: Vec<u8>,

    // enable
    enable: bool,
}

impl Emulator8080 {
    pub fn empty() -> Self {
        Self {
            pc: 0,
            sp: 0,
            ra: 0,
            rb: 0,
            rc: 0,
            rd: 0,
            re: 0,
            rl: 0,
            rh: 0,
            flags: Flags8080::new(),
            memory: vec![0; u16::MAX as usize],
            enable: true,
        }
    }
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
    /// Adds rhs to lhs.
    ///
    /// If `with_carry` is set, it also adds the content of the
    /// cy flag.
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

    /// Subtracts rhs from lhs.
    ///
    /// If with_borrow is specified, it also subtracts the content
    /// of the cy flag.
    fn sub(&mut self, lhs: u8, rhs: u8, with_borrow: bool) -> u8 {
        lhs.wrapping_sub(rhs)
            .wrapping_sub(if with_borrow { self.flags.cy as u8 } else { 0 })
    }

    /// Subtracts value from register a.
    ///
    /// This method affects the flags Z, S, P, CY and AC.
    pub fn sub_a(&mut self, value: u8, with_borrow: bool) {
        let result = self.sub(self.ra, value, with_borrow);

        self.set_flags(result, self.ra, true, true, true, true, true, true);
        self.ra = result;
    }

    pub fn sub_mem(&mut self, with_borrow: bool) {
        let address = self.get_address() as usize;
        self.sub_a(self.memory[address], with_borrow);
    }

    /// Increment the value of `reg`.
    ///
    /// This method affects Z, S, P and AC.
    pub fn inc_reg(&mut self, reg: u8) {
        let result = self.add(reg, 1, false);

        self.set_flags(result, reg, false, true, false, true, true, true);
    }

    /// Increment the value pointed to by RH and RL.
    ///
    /// This method affects Z, S, P and AC.
    pub fn inc_mem(&mut self) {
        let address = self.get_address() as usize;
        let value = self.memory[address];
        let result = self.add(value, 1, false);

        self.set_flags(result, value, false, true, false, true, true, true);
        self.memory[address] = result;
    }

    /// Decrement the value of `reg`.
    ///
    /// This method affects Z, S, P and AC.
    pub fn dec_reg(&mut self, reg: u8) {
        let result = self.sub(reg, 1, false);

        self.set_flags(result, reg, true, true, false, true, true, true);
    }

    /// Decrement the value pointed to by RH and RL.
    ///
    /// This method affects Z, S, P and AC.
    pub fn dec_mem(&mut self) {
        let address = self.get_address() as usize;
        let value = self.memory[address];
        let result = self.sub(value, 1, false);

        self.set_flags(result, value, true, true, false, true, true, true);
        self.memory[address] = result;
    }

    /// Increments or decrements the value of (rh rl).
    fn get_inc_or_dec_reg_pair(rh: u8, rl: u8, increment: bool) -> (u8, u8) {
        let value = (rh as u16) << 8 + rl as u16;
        if increment {
            let result = value.wrapping_add(1);
            return ((result >> 8) as u8, (result & 0xff) as u8);
        } else {
            let result = value.wrapping_sub(1);
            return ((result >> 8) as u8, (result & 0xff) as u8);
        }
    }

    /// Increments or decrements are register pair specified by `reg_pair`.
    pub fn inc_or_dec_reg_pair(&mut self, reg_pair: RegPair, increment: bool) {
        match reg_pair {
            RegPair::BC => {
                let result = Emulator8080::get_inc_or_dec_reg_pair(self.rb, self.rc, increment);
                self.rb = result.0;
                self.rc = result.1;
            }
            RegPair::DE => {
                let result = Emulator8080::get_inc_or_dec_reg_pair(self.rd, self.re, increment);
                self.rd = result.0;
                self.re = result.1;
            }
            RegPair::HL => {
                let result = Emulator8080::get_inc_or_dec_reg_pair(self.rh, self.rl, increment);
                self.rh = result.0;
                self.rl = result.1;
            }
            RegPair::SP => {
                if increment {
                    self.sp = self.sp.wrapping_add(1);
                } else {
                    self.sp = self.sp.wrapping_sub(1);
                }
            }
        }
    }
    /* Branch group */
    /* Data transfer group */
    /* Logical group */
    /* Stack, I/O and machine control group */

    pub fn emulate_instruction(&mut self) {}
}
