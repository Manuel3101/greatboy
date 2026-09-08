use crate::emulator_core::core_advance_cpu_clock;

#[derive(Debug, Clone, Copy)]
pub struct GbCpuInstructions {
    pub dissasembly: &'static str, // name of the instruction
    pub operand_length: u8,        // n = 1 byte, nn = 2 bytes
    pub execute: Option<fn()>,     // function pointer to the instruction implementation
}

// TODO: implement all instructions
pub static INSTRUCTIONS: [GbCpuInstructions; 256] = [
    GbCpuInstructions {
        dissasembly: "NOP",
        operand_length: 0,
        execute: Some(nop),
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    },
];

fn nop() {
    core_advance_cpu_clock(4);
}
