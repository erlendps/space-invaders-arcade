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

    /// fetches the content of the given `reg_pair`
    fn get_reg_pair(&self, reg_pair: RegPair) -> u16 {
        match reg_pair {
            RegPair::BC => ((self.rb as u16) << 8) | (self.rc as u16),
            RegPair::DE => ((self.rd as u16) << 8) | (self.re as u16),
            RegPair::HL => ((self.rh as u16) << 8) | (self.rl as u16),
            RegPair::SP => self.sp,
        }
    }

    /// Sets `reg_pair` to `value`
    fn set_reg_pair(&mut self, value: u16, reg_pair: RegPair) {
        let value_tuple = ((value >> 8) as u8, (value & 0xff) as u8);
        match reg_pair {
            RegPair::BC => {
                self.rb = value_tuple.0;
                self.rc = value_tuple.1;
            }
            RegPair::DE => {
                self.rd = value_tuple.0;
                self.re = value_tuple.1;
            }
            RegPair::HL => {
                self.rh = value_tuple.0;
                self.rl = value_tuple.1;
            }
            RegPair::SP => {
                self.sp = value;
            }
        }
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

    /// Subtracts the content pointed to by (HL) from the accumulator.
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
    fn get_inc_or_dec_reg_pair(rh: u8, rl: u8, increment: bool) -> u16 {
        let value = (rh as u16) << 8 + rl as u16;
        if increment {
            value.wrapping_add(1)
        } else {
            value.wrapping_sub(1)
        }
    }

    /// Increments or decrements are register pair specified by `reg_pair`.
    pub fn inc_or_dec_reg_pair(&mut self, reg_pair: RegPair, increment: bool) {
        match reg_pair {
            RegPair::BC => {
                let result = Emulator8080::get_inc_or_dec_reg_pair(self.rb, self.rc, increment);
                self.set_reg_pair(result, RegPair::BC);
            }
            RegPair::DE => {
                let result = Emulator8080::get_inc_or_dec_reg_pair(self.rd, self.re, increment);
                self.set_reg_pair(result, RegPair::DE);
            }
            RegPair::HL => {
                let result = Emulator8080::get_inc_or_dec_reg_pair(self.rh, self.rl, increment);
                self.set_reg_pair(result, RegPair::HL);
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

    /// Adds the content of `reg_pair` into the content of (HL)
    pub fn dad(&mut self, reg_pair: RegPair) {
        let a = self.get_reg_pair(RegPair::HL);
        let b = self.get_reg_pair(reg_pair);
        let result = a.wrapping_add(b);

        if result < a {
            self.flags.cy = true;
        } else {
            self.flags.cy = false
        }

        self.set_reg_pair(result, RegPair::HL);
    }

    /// The eight-bit number in the accumulator is adjusted
    /// to form two four-bit Binary-Coded-Decimal by the following process:
    ///
    /// 1. If the value of the least significant 4 bits of the
    /// accumulator is greater than 9 or if the AC flag
    /// is set, 6 is added to the accumulator.
    /// 2. If the value of the most significant 4 bits of the
    /// accumulator is now greater than 9, or if the CY
    /// flag is set, 6 is added to the most significant 4
    /// bits of the accumulator.
    pub fn daa(&mut self) {
        // least significant 4 bits
        let ls4b = self.ra & 0x0f;
        if ls4b > 0x9 || self.flags.ac {
            self.ra = self.add(self.ra, 0x6, false);
        }
        // most significant 4 bits
        let mut ms4b = (self.ra & 0xf0) >> 4;
        if ms4b > 0x9 || self.flags.cy {
            ms4b = ms4b.wrapping_add(0x6) << 4;
        }
        let result = (self.ra & 0xf) | ms4b;
        self.set_flags(result, self.ra, false, true, true, true, true, true);
    }
    /* Branch group */
    /* Data transfer group */
    /* Logical group */
    /* Stack, I/O and machine control group */

    pub fn emulate_instruction(&mut self) {}
}
