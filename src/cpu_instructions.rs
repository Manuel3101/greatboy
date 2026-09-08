#[derive(Debug, Clone, Copy)]
pub struct GbCpuInstructions {
    pub dissasembly: &'static str, // name of the instruction
    pub operand_length: u8,        // n = 1 byte, nn = 2 bytes
    pub execute: Option<fn()>,     // function pointer to the instruction implementation
}

// TODO: implement all instructions
pub static INSTRUCTIONS: [GbCpuInstructions; 2] = [
    GbCpuInstructions {
        dissasembly: "NOP",
        operand_length: 0,
        execute: None,
    },
    GbCpuInstructions {
        dissasembly: "LD BC, d16",
        operand_length: 2,
        execute: None,
    },
];

fn nop() {
    println!("Running NOP")
}
