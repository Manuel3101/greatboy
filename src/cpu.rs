use std::sync::{LazyLock, Mutex};

use crate::{
    cpu_instructions::INSTRUCTIONS, cpu_registers::GbCpuRegisters, memory_bus::memory_bus_read,
};

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

/**
 * Set the cpu registers to the initial values after boot.
 */
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

    let op_code = memory_bus_read(cpu.registers.pc as usize);
    let instruction = INSTRUCTIONS[op_code as usize];

    cpu.current_op_code = op_code;
    cpu.current_instruction_execute = instruction.execute;
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
