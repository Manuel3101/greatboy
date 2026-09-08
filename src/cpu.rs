use std::sync::{LazyLock, Mutex};

use crate::{cart::CARTRIDGE_DATA, cpu_instructions::INSTRUCTIONS};

pub static CPU: LazyLock<Mutex<Cpu>> = std::sync::LazyLock::new(|| Mutex::new(Cpu::new()));

pub struct Cpu {
    pub registers: GbCpuRegisters,
    pub halted: bool,
    pub stopped: bool,
    pub ime: bool,

    pub current_op_code: u8,
    pub instruction_counter: u32,
    pub current_instruction_execute: Option<fn()>,
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

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: GbCpuRegisters::new(),
            halted: false,
            stopped: false,
            ime: false,
            current_op_code: 0,
            instruction_counter: 0,
            current_instruction_execute: None,
        }
    }
}

impl GbCpuRegisters {
    pub fn new() -> Self {
        Self {
            sp: 0,
            pc: 0,
            ..Self::default()
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

pub fn cpu_reset() {
    let mut cpu = CPU.lock().unwrap();

    cpu.registers.set_af(0x01B0);
    cpu.registers.set_bc(0x0013);
    cpu.registers.set_de(0x00D8);
    cpu.registers.set_hl(0x014D);
    cpu.registers.sp = 0xFFFE;
    cpu.registers.pc = 0x0100;
}

pub fn cpu_fetch() {
    let mut cpu = CPU.lock().unwrap();

    unsafe {
        // TODO: read from memory bus instead of directly from ROM data
        let op_code = CARTRIDGE_DATA[cpu.registers.pc as usize];
        let instruction = INSTRUCTIONS[op_code as usize];

        cpu.current_op_code = op_code;
        cpu.current_instruction_execute = instruction.execute;
    }
}

pub fn cpu_execute() -> bool {
    let execute = {
        let cpu = CPU.lock().unwrap();

        let Some(execute) = cpu.current_instruction_execute else {
            let instruction = INSTRUCTIONS[cpu.current_op_code as usize];
            let pchi: u8 = (cpu.registers.pc >> 8) as u8;
            let pclo: u8 = (cpu.registers.pc & 0xFF) as u8;
            println!(
                "Unknown instruction {:02X} at: 0x{:02X}{:02X} ({}), instruction_count {}",
                cpu.current_op_code, pchi, pclo, instruction.dissasembly, cpu.instruction_counter
            );
            return false;
        };

        execute
    };

    // Lock must be released before running the execute function
    execute();

    let mut cpu = CPU.lock().unwrap();
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    cpu.instruction_counter += 1;

    true
}
