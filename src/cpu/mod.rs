use crate::{
    bus::Bus,
    cpu::{cpu_instructions::INSTRUCTIONS, cpu_registers::GbCpuRegisters},
    timer::Timer,
};

mod cpu_instructions;
mod cpu_registers;
mod cpu_routines;

#[derive(Clone, Copy)]
pub enum TimerState {
    Running,
    Stopped,
    Halted,
}

pub static mut TIMER_STATE: TimerState = TimerState::Running;

pub struct Cpu {
    pub core_clock_counter: u32,
    pub registers: GbCpuRegisters,
    pub halted: bool,
    pub stopped: bool,
    pub ime: bool,

    pub current_op_code: u8,
    pub instruction_counter: u32,
    pub current_instruction_execute: Option<fn(&mut Cpu, &mut Bus)>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            core_clock_counter: 0,
            registers: GbCpuRegisters::new(),
            halted: false,
            stopped: false,
            ime: false,
            current_op_code: 0,
            instruction_counter: 0,
            current_instruction_execute: None,
        }
    }

    /**
     * Set the cpu registers to the initial values after boot.
     */
    pub fn cpu_reset(&mut self) {
        self.registers.set_af(0x01B0);
        self.registers.set_bc(0x0013);
        self.registers.set_de(0x00D8);
        self.registers.set_hl(0x014D);
        self.registers.sp = 0xFFFE;
        self.registers.pc = 0x0100;
    }

    pub fn cpu_fetch(&mut self, bus: &mut Bus) {
        let op_code = bus.memory.memory_bus_read(self.registers.pc as usize);
        let instruction = INSTRUCTIONS[op_code as usize];

        self.current_op_code = op_code;
        self.current_instruction_execute = instruction.execute;
    }

    pub fn cpu_execute(&mut self, mut bus: &mut Bus) -> bool {
        let Some(execute) = self.current_instruction_execute else {
            let instruction = INSTRUCTIONS[self.current_op_code as usize];
            let pchi: u8 = (self.registers.pc >> 8) as u8;
            let pclo: u8 = (self.registers.pc & 0xFF) as u8;
            println!(
                "Unknown instruction {:02X} at: 0x{:02X}{:02X} ({}), instruction_count {}",
                self.current_op_code, pchi, pclo, instruction.dissasembly, self.instruction_counter
            );
            return false;
        };

        execute(self, &mut bus);

        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.instruction_counter += 1;

        true
    }

    /**
     * Advances the CPU clock by the specified number of clock cycles.
     */
    pub fn core_advance_cpu_clock(&mut self, timer: &mut Timer, clocks: u8) {
        timer.advance_clocks(clocks);
        self.core_clock_counter += u32::from(clocks);
    }
}
