#[allow(dead_code)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[allow(dead_code)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GbCpuRegisters {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum Flag {
    Z = 0b1000_0000, // Zero Flag
    N = 0b0100_0000, // Subtract Flag
    H = 0b0010_0000, // Half Carry Flag
    C = 0b0001_0000, // Carry Flag
}

impl GbCpuRegisters {
    pub fn new() -> Self {
        Self {
            sp: 0,
            pc: 0,
            ..Self::default()
        }
    }

    pub fn set8(&mut self, reg: Reg8, value: u8) {
        match reg {
            Reg8::A => self.a = value,
            Reg8::B => self.b = value,
            Reg8::C => self.c = value,
            Reg8::D => self.d = value,
            Reg8::E => self.e = value,
            Reg8::H => self.h = value,
            Reg8::L => self.l = value,
        }
    }

    pub fn get8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.a,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.h,
            Reg8::L => self.l,
        }
    }

    pub fn set16(&mut self, reg: Reg16, value: u16) {
        match reg {
            Reg16::AF => self.set_af(value),
            Reg16::BC => self.set_bc(value),
            Reg16::DE => self.set_de(value),
            Reg16::HL => self.set_hl(value),
            Reg16::SP => self.sp = value,
            Reg16::PC => self.pc = value,
        }
    }

    pub fn get16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => self.af(),
            Reg16::BC => self.bc(),
            Reg16::DE => self.de(),
            Reg16::HL => self.hl(),
            Reg16::SP => self.sp,
            Reg16::PC => self.pc,
        }
    }

    pub fn af(&self) -> u16 {
        u16::from_be_bytes([self.a, self.f])
    }

    pub fn set_af(&mut self, value: u16) {
        let [a, f] = value.to_be_bytes();
        self.a = a;
        self.f = self.f & 0x0F | (f & 0xF0); // lower bits are used to represent z,n,h,c flags
    }

    pub fn bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }

    pub fn set_bc(&mut self, value: u16) {
        let [b, c] = value.to_be_bytes();
        self.b = b;
        self.c = c;
    }

    pub fn de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }

    pub fn set_de(&mut self, value: u16) {
        let [d, e] = value.to_be_bytes();
        self.d = d;
        self.e = e;
    }

    pub fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }

    pub fn set_hl(&mut self, value: u16) {
        let [h, l] = value.to_be_bytes();
        self.h = h;
        self.l = l;
    }

    pub fn flag(&self, flag: Flag) -> bool {
        self.f & (flag as u8) != 0
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.f |= flag as u8;
        } else {
            self.f &= !(flag as u8);
        }

        self.f &= 0xF0; // lower nibble of F is always 0
    }

    pub fn zero(&self) -> bool {
        self.flag(Flag::Z)
    }

    pub fn set_zero(&mut self, value: bool) {
        self.set_flag(Flag::Z, value);
    }

    pub fn subtract(&self) -> bool {
        self.flag(Flag::N)
    }

    pub fn set_subtract(&mut self, value: bool) {
        self.set_flag(Flag::N, value);
    }

    pub fn half_carry(&self) -> bool {
        self.flag(Flag::H)
    }

    pub fn set_half_carry(&mut self, value: bool) {
        self.set_flag(Flag::H, value);
    }

    pub fn carry(&self) -> bool {
        self.flag(Flag::C)
    }

    pub fn set_carry(&mut self, value: bool) {
        self.set_flag(Flag::C, value);
    }
}
