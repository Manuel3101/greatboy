use crate::{cpu::CPU, emulator_core::core_advance_cpu_clock, memory_bus::memory_bus_read};

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
        execute: Some(cpu_nop),
    }, // 0x00
    GbCpuInstructions {
        dissasembly: "LD BC, nn",
        operand_length: 1,
        execute: None,
    }, // 0x01
    GbCpuInstructions {
        dissasembly: "LD (BC), A",
        operand_length: 0,
        execute: None,
    }, // 0x02
    GbCpuInstructions {
        dissasembly: "INC BC",
        operand_length: 0,
        execute: None,
    }, // 0x03
    GbCpuInstructions {
        dissasembly: "INC B",
        operand_length: 0,
        execute: None,
    }, // 0x04
    GbCpuInstructions {
        dissasembly: "DEC B",
        operand_length: 0,
        execute: None,
    }, // 0x05
    GbCpuInstructions {
        dissasembly: "LD B, d8",
        operand_length: 1,
        execute: None,
    }, // 0x06
    GbCpuInstructions {
        dissasembly: "RLCA",
        operand_length: 0,
        execute: None,
    }, // 0x07
    GbCpuInstructions {
        dissasembly: "LD (a16), SP",
        operand_length: 2,
        execute: None,
    }, // 0x08
    GbCpuInstructions {
        dissasembly: "ADD HL, BC",
        operand_length: 0,
        execute: None,
    }, // 0x09
    GbCpuInstructions {
        dissasembly: "LD A , (BC)",
        operand_length: 0,
        execute: None,
    }, // 0x0A
    GbCpuInstructions {
        dissasembly: "DEC BC",
        operand_length: 0,
        execute: None,
    }, // 0x0B
    GbCpuInstructions {
        dissasembly: "INC C",
        operand_length: 0,
        execute: None,
    }, // 0x0C
    GbCpuInstructions {
        dissasembly: "DEC C",
        operand_length: 0,
        execute: None,
    }, // 0x0D
    GbCpuInstructions {
        dissasembly: "LD C, d8",
        operand_length: 1,
        execute: None,
    }, // 0x0E
    GbCpuInstructions {
        dissasembly: "RRCA",
        operand_length: 0,
        execute: None,
    }, // 0x0F
    GbCpuInstructions {
        dissasembly: "STOP",
        operand_length: 1,
        execute: None,
    }, // 0x10
    GbCpuInstructions {
        dissasembly: "LD DE, d16",
        operand_length: 2,
        execute: None,
    }, // 0x11
    GbCpuInstructions {
        dissasembly: "LD (DE), A",
        operand_length: 0,
        execute: None,
    }, // 0x12
    GbCpuInstructions {
        dissasembly: "INC DE",
        operand_length: 0,
        execute: None,
    }, // 0x13
    GbCpuInstructions {
        dissasembly: "INC D",
        operand_length: 0,
        execute: None,
    }, // 0x14
    GbCpuInstructions {
        dissasembly: "DEC D",
        operand_length: 0,
        execute: None,
    }, // 0x15
    GbCpuInstructions {
        dissasembly: "LD D, d8",
        operand_length: 1,
        execute: None,
    }, // 0x16
    GbCpuInstructions {
        dissasembly: "RLA",
        operand_length: 0,
        execute: None,
    }, // 0x17
    GbCpuInstructions {
        dissasembly: "JR s8",
        operand_length: 1,
        execute: None,
    }, // 0x18
    GbCpuInstructions {
        dissasembly: "ADD HL, DE",
        operand_length: 0,
        execute: None,
    }, // 0x19
    GbCpuInstructions {
        dissasembly: "LD A, (DE)",
        operand_length: 0,
        execute: None,
    }, // 0x1A
    GbCpuInstructions {
        dissasembly: "DEC DE",
        operand_length: 0,
        execute: None,
    }, // 0x1B
    GbCpuInstructions {
        dissasembly: "INC E",
        operand_length: 0,
        execute: None,
    }, // 0x1C
    GbCpuInstructions {
        dissasembly: "DEC E",
        operand_length: 0,
        execute: None,
    }, // 0x1D
    GbCpuInstructions {
        dissasembly: "LD E, d8",
        operand_length: 1,
        execute: None,
    }, // 0x1E
    GbCpuInstructions {
        dissasembly: "RRA",
        operand_length: 0,
        execute: None,
    }, // 0x1F
    GbCpuInstructions {
        dissasembly: "JR NZ, s8",
        operand_length: 1,
        execute: None,
    }, // 0x20
    GbCpuInstructions {
        dissasembly: "LD HL, d16",
        operand_length: 2,
        execute: None,
    }, // 0x21
    GbCpuInstructions {
        dissasembly: "LD (HL+), A",
        operand_length: 0,
        execute: None,
    }, // 0x22
    GbCpuInstructions {
        dissasembly: "INC HL",
        operand_length: 0,
        execute: None,
    }, // 0x23
    GbCpuInstructions {
        dissasembly: "INC H",
        operand_length: 0,
        execute: None,
    }, // 0x24
    GbCpuInstructions {
        dissasembly: "DEC H",
        operand_length: 0,
        execute: None,
    }, // 0x25
    GbCpuInstructions {
        dissasembly: "LD H, d8",
        operand_length: 1,
        execute: None,
    }, // 0x26
    GbCpuInstructions {
        dissasembly: "DAA",
        operand_length: 0,
        execute: None,
    }, // 0x27
    GbCpuInstructions {
        dissasembly: "JR Z, s8",
        operand_length: 1,
        execute: None,
    }, // 0x28
    GbCpuInstructions {
        dissasembly: "ADD HL, HL",
        operand_length: 0,
        execute: None,
    }, // 0x29
    GbCpuInstructions {
        dissasembly: "LDA A, (HL+)",
        operand_length: 0,
        execute: None,
    }, // 0x2A
    GbCpuInstructions {
        dissasembly: "DEC HL",
        operand_length: 0,
        execute: None,
    }, // 0x2B
    GbCpuInstructions {
        dissasembly: "INC L",
        operand_length: 0,
        execute: None,
    }, // 0x2C
    GbCpuInstructions {
        dissasembly: "DEC L",
        operand_length: 0,
        execute: None,
    }, // 0x2D
    GbCpuInstructions {
        dissasembly: "LD L, d8",
        operand_length: 1,
        execute: None,
    }, // 0x2E
    GbCpuInstructions {
        dissasembly: "CPL",
        operand_length: 0,
        execute: None,
    }, // 0x2F
    GbCpuInstructions {
        dissasembly: "JR NC, s8",
        operand_length: 1,
        execute: None,
    }, // 0x30
    GbCpuInstructions {
        dissasembly: "LD SP, d16",
        operand_length: 2,
        execute: None,
    }, // 0x31
    GbCpuInstructions {
        dissasembly: "LD (HL-), A",
        operand_length: 0,
        execute: None,
    }, // 0x32
    GbCpuInstructions {
        dissasembly: "INC SP",
        operand_length: 0,
        execute: None,
    }, // 0x33
    GbCpuInstructions {
        dissasembly: "INC (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x34
    GbCpuInstructions {
        dissasembly: "DEC (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x35
    GbCpuInstructions {
        dissasembly: "LD (HL), d8",
        operand_length: 1,
        execute: None,
    }, // 0x36
    GbCpuInstructions {
        dissasembly: "SCF",
        operand_length: 0,
        execute: None,
    }, // 0x37
    GbCpuInstructions {
        dissasembly: "JR C, s8",
        operand_length: 1,
        execute: None,
    }, // 0x38
    GbCpuInstructions {
        dissasembly: "ADD HL, SP",
        operand_length: 0,
        execute: None,
    }, // 0x39
    GbCpuInstructions {
        dissasembly: "LD A, (HL-)",
        operand_length: 0,
        execute: None,
    }, // 0x3A
    GbCpuInstructions {
        dissasembly: "DEC SP",
        operand_length: 0,
        execute: None,
    }, // 0x3B
    GbCpuInstructions {
        dissasembly: "INC A",
        operand_length: 0,
        execute: None,
    }, // 0x3C
    GbCpuInstructions {
        dissasembly: "DEC A",
        operand_length: 0,
        execute: None,
    }, // 0x3D
    GbCpuInstructions {
        dissasembly: "LD A, d8",
        operand_length: 1,
        execute: None,
    }, // 0x3E
    GbCpuInstructions {
        dissasembly: "CCF",
        operand_length: 0,
        execute: None,
    }, // 0x3F
    GbCpuInstructions {
        dissasembly: "LD B, B",
        operand_length: 0,
        execute: None,
    }, // 0x40
    GbCpuInstructions {
        dissasembly: "LD B, C",
        operand_length: 0,
        execute: None,
    }, // 0x41
    GbCpuInstructions {
        dissasembly: "LD B, D",
        operand_length: 0,
        execute: None,
    }, // 0x42
    GbCpuInstructions {
        dissasembly: "LD B, E",
        operand_length: 0,
        execute: None,
    }, // 0x43
    GbCpuInstructions {
        dissasembly: "LD B, H",
        operand_length: 0,
        execute: None,
    }, // 0x44
    GbCpuInstructions {
        dissasembly: "LD B, L",
        operand_length: 0,
        execute: None,
    }, // 0x45
    GbCpuInstructions {
        dissasembly: "LD B, (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x46
    GbCpuInstructions {
        dissasembly: "LD B, A",
        operand_length: 0,
        execute: None,
    }, // 0x47
    GbCpuInstructions {
        dissasembly: "LD C, B",
        operand_length: 0,
        execute: None,
    }, // 0x48
    GbCpuInstructions {
        dissasembly: "LD C, C",
        operand_length: 0,
        execute: None,
    }, // 0x49
    GbCpuInstructions {
        dissasembly: "LD C, D",
        operand_length: 0,
        execute: None,
    }, // 0x4A
    GbCpuInstructions {
        dissasembly: "LD C, E",
        operand_length: 0,
        execute: None,
    }, // 0x4B
    GbCpuInstructions {
        dissasembly: "LD C, H",
        operand_length: 0,
        execute: None,
    }, // 0x4C
    GbCpuInstructions {
        dissasembly: "LD C, L",
        operand_length: 0,
        execute: None,
    }, // 0x4D
    GbCpuInstructions {
        dissasembly: "LD C, (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x4E
    GbCpuInstructions {
        dissasembly: "LD C, A",
        operand_length: 0,
        execute: None,
    }, // 0x4F
    GbCpuInstructions {
        dissasembly: "LD D, B",
        operand_length: 0,
        execute: None,
    }, // 0x50
    GbCpuInstructions {
        dissasembly: "LD D, C",
        operand_length: 0,
        execute: None,
    }, // 0x51
    GbCpuInstructions {
        dissasembly: "LD D, D",
        operand_length: 0,
        execute: None,
    }, // 0x52
    GbCpuInstructions {
        dissasembly: "LD D, E",
        operand_length: 0,
        execute: None,
    }, // 0x53
    GbCpuInstructions {
        dissasembly: "LD D, H",
        operand_length: 0,
        execute: None,
    }, // 0x54
    GbCpuInstructions {
        dissasembly: "LD D, L",
        operand_length: 0,
        execute: None,
    }, // 0x55
    GbCpuInstructions {
        dissasembly: "LD D, (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x56
    GbCpuInstructions {
        dissasembly: "LD D, A",
        operand_length: 0,
        execute: None,
    }, // 0x57
    GbCpuInstructions {
        dissasembly: "LD E, B",
        operand_length: 0,
        execute: None,
    }, // 0x58
    GbCpuInstructions {
        dissasembly: "LD E, C",
        operand_length: 0,
        execute: None,
    }, // 0x59
    GbCpuInstructions {
        dissasembly: "LD E, D",
        operand_length: 0,
        execute: None,
    }, // 0x5A
    GbCpuInstructions {
        dissasembly: "LD E, E",
        operand_length: 0,
        execute: None,
    }, // 0x5B
    GbCpuInstructions {
        dissasembly: "LD E, H",
        operand_length: 0,
        execute: None,
    }, // 0x5C
    GbCpuInstructions {
        dissasembly: "LD E, L",
        operand_length: 0,
        execute: None,
    }, // 0x5D
    GbCpuInstructions {
        dissasembly: "LD E, (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x5E
    GbCpuInstructions {
        dissasembly: "LD E, A",
        operand_length: 0,
        execute: None,
    }, // 0x5F
    GbCpuInstructions {
        dissasembly: "LD H, B",
        operand_length: 0,
        execute: None,
    }, // 0x60
    GbCpuInstructions {
        dissasembly: "LD H, C",
        operand_length: 0,
        execute: None,
    }, // 0x61
    GbCpuInstructions {
        dissasembly: "LD H, D",
        operand_length: 0,
        execute: None,
    }, // 0x62
    GbCpuInstructions {
        dissasembly: "LD H, E",
        operand_length: 0,
        execute: None,
    }, // 0x63
    GbCpuInstructions {
        dissasembly: "LD H, H",
        operand_length: 0,
        execute: None,
    }, // 0x64
    GbCpuInstructions {
        dissasembly: "LD H, L",
        operand_length: 0,
        execute: None,
    }, // 0x65
    GbCpuInstructions {
        dissasembly: "LD H, (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x66
    GbCpuInstructions {
        dissasembly: "LD H, A",
        operand_length: 0,
        execute: None,
    }, // 0x67
    GbCpuInstructions {
        dissasembly: "LD L, B",
        operand_length: 0,
        execute: None,
    }, // 0x68
    GbCpuInstructions {
        dissasembly: "LD L, C",
        operand_length: 0,
        execute: None,
    }, // 0x69
    GbCpuInstructions {
        dissasembly: "LD L, D",
        operand_length: 0,
        execute: None,
    }, // 0x6A
    GbCpuInstructions {
        dissasembly: "LD L, E",
        operand_length: 0,
        execute: None,
    }, // 0x6B
    GbCpuInstructions {
        dissasembly: "LD L, H",
        operand_length: 0,
        execute: None,
    }, // 0x6C
    GbCpuInstructions {
        dissasembly: "LD L, L",
        operand_length: 0,
        execute: None,
    }, // 0x6D
    GbCpuInstructions {
        dissasembly: "LD L, (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x6E
    GbCpuInstructions {
        dissasembly: "LD L, A",
        operand_length: 0,
        execute: None,
    }, // 0x6F
    GbCpuInstructions {
        dissasembly: "LD (HL), B",
        operand_length: 0,
        execute: None,
    }, // 0x70
    GbCpuInstructions {
        dissasembly: "LD (HL), C",
        operand_length: 0,
        execute: None,
    }, // 0x71
    GbCpuInstructions {
        dissasembly: "LD (HL), D",
        operand_length: 0,
        execute: None,
    }, // 0x72
    GbCpuInstructions {
        dissasembly: "LD (HL), E",
        operand_length: 0,
        execute: None,
    }, // 0x73
    GbCpuInstructions {
        dissasembly: "LD (HL), H",
        operand_length: 0,
        execute: None,
    }, // 0x74
    GbCpuInstructions {
        dissasembly: "LD (HL), L",
        operand_length: 0,
        execute: None,
    }, // 0x75
    GbCpuInstructions {
        dissasembly: "HALT",
        operand_length: 0,
        execute: None,
    }, // 0x76
    GbCpuInstructions {
        dissasembly: "LD (HL), A",
        operand_length: 0,
        execute: None,
    }, // 0x77
    GbCpuInstructions {
        dissasembly: "LD A, B",
        operand_length: 0,
        execute: None,
    }, // 0x78
    GbCpuInstructions {
        dissasembly: "LD A, C",
        operand_length: 0,
        execute: None,
    }, // 0x79
    GbCpuInstructions {
        dissasembly: "LD A, D",
        operand_length: 0,
        execute: None,
    }, // 0x7A
    GbCpuInstructions {
        dissasembly: "LD A, E",
        operand_length: 0,
        execute: None,
    }, // 0x7B
    GbCpuInstructions {
        dissasembly: "LD A, H",
        operand_length: 0,
        execute: None,
    }, // 0x7C
    GbCpuInstructions {
        dissasembly: "LD A, L",
        operand_length: 0,
        execute: None,
    }, // 0x7D
    GbCpuInstructions {
        dissasembly: "LD A, (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x7E
    GbCpuInstructions {
        dissasembly: "LD A, A",
        operand_length: 0,
        execute: None,
    }, // 0x7F
    GbCpuInstructions {
        dissasembly: "ADD A, B",
        operand_length: 0,
        execute: None,
    }, // 0x80
    GbCpuInstructions {
        dissasembly: "ADD A, C",
        operand_length: 0,
        execute: None,
    }, // 0x81
    GbCpuInstructions {
        dissasembly: "ADD A, D",
        operand_length: 0,
        execute: None,
    }, // 0x82
    GbCpuInstructions {
        dissasembly: "ADD A, E",
        operand_length: 0,
        execute: None,
    }, // 0x83
    GbCpuInstructions {
        dissasembly: "ADD A, H",
        operand_length: 0,
        execute: None,
    }, // 0x84
    GbCpuInstructions {
        dissasembly: "ADD A, L",
        operand_length: 0,
        execute: None,
    }, // 0x85
    GbCpuInstructions {
        dissasembly: "ADD A, (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x86
    GbCpuInstructions {
        dissasembly: "ADD A, A",
        operand_length: 0,
        execute: None,
    }, // 0x87
    GbCpuInstructions {
        dissasembly: "ADC A, B",
        operand_length: 0,
        execute: None,
    }, // 0x88
    GbCpuInstructions {
        dissasembly: "ADC A, C",
        operand_length: 0,
        execute: None,
    }, // 0x89
    GbCpuInstructions {
        dissasembly: "ADC A, D",
        operand_length: 0,
        execute: None,
    }, // 0x8A
    GbCpuInstructions {
        dissasembly: "ADC A, E",
        operand_length: 0,
        execute: None,
    }, // 0x8B
    GbCpuInstructions {
        dissasembly: "ADC A, H",
        operand_length: 0,
        execute: None,
    }, // 0x8C
    GbCpuInstructions {
        dissasembly: "ADC A, L",
        operand_length: 0,
        execute: None,
    }, // 0x8D
    GbCpuInstructions {
        dissasembly: "ADC A, (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x8E
    GbCpuInstructions {
        dissasembly: "ADC A, A",
        operand_length: 0,
        execute: None,
    }, // 0x8F
    GbCpuInstructions {
        dissasembly: "SUB B",
        operand_length: 0,
        execute: None,
    }, // 0x90
    GbCpuInstructions {
        dissasembly: "SUB C",
        operand_length: 0,
        execute: None,
    }, // 0x91
    GbCpuInstructions {
        dissasembly: "SUB D",
        operand_length: 0,
        execute: None,
    }, // 0x92
    GbCpuInstructions {
        dissasembly: "SUB E",
        operand_length: 0,
        execute: None,
    }, // 0x93
    GbCpuInstructions {
        dissasembly: "SUB H",
        operand_length: 0,
        execute: None,
    }, // 0x94
    GbCpuInstructions {
        dissasembly: "SUB L",
        operand_length: 0,
        execute: None,
    }, // 0x95
    GbCpuInstructions {
        dissasembly: "SUB (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x96
    GbCpuInstructions {
        dissasembly: "SUB A",
        operand_length: 0,
        execute: None,
    }, // 0x97
    GbCpuInstructions {
        dissasembly: "SBC A, B",
        operand_length: 0,
        execute: None,
    }, // 0x98
    GbCpuInstructions {
        dissasembly: "SBC A, C",
        operand_length: 0,
        execute: None,
    }, // 0x99
    GbCpuInstructions {
        dissasembly: "SBC A, D",
        operand_length: 0,
        execute: None,
    }, // 0x9A
    GbCpuInstructions {
        dissasembly: "SBC A, E",
        operand_length: 0,
        execute: None,
    }, // 0x9B
    GbCpuInstructions {
        dissasembly: "SBC A, H",
        operand_length: 0,
        execute: None,
    }, // 0x9C
    GbCpuInstructions {
        dissasembly: "SBC A, L",
        operand_length: 0,
        execute: None,
    }, // 0x9D
    GbCpuInstructions {
        dissasembly: "SBC A, (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x9E
    GbCpuInstructions {
        dissasembly: "SBC A, A",
        operand_length: 0,
        execute: None,
    }, // 0x9F
    GbCpuInstructions {
        dissasembly: "AND B",
        operand_length: 0,
        execute: None,
    }, // 0xA0
    GbCpuInstructions {
        dissasembly: "AND C",
        operand_length: 0,
        execute: None,
    }, // 0xA1
    GbCpuInstructions {
        dissasembly: "AND D",
        operand_length: 0,
        execute: None,
    }, // 0xA2
    GbCpuInstructions {
        dissasembly: "AND E",
        operand_length: 0,
        execute: None,
    }, // 0xA3
    GbCpuInstructions {
        dissasembly: "AND H",
        operand_length: 0,
        execute: None,
    }, // 0xA4
    GbCpuInstructions {
        dissasembly: "AND L",
        operand_length: 0,
        execute: None,
    }, // 0xA5
    GbCpuInstructions {
        dissasembly: "AND (HL)",
        operand_length: 0,
        execute: None,
    }, // 0xA6
    GbCpuInstructions {
        dissasembly: "AND A",
        operand_length: 0,
        execute: None,
    }, // 0xA7
    GbCpuInstructions {
        dissasembly: "XOR B",
        operand_length: 0,
        execute: None,
    }, // 0xA8
    GbCpuInstructions {
        dissasembly: "XOR C",
        operand_length: 0,
        execute: None,
    }, // 0xA9
    GbCpuInstructions {
        dissasembly: "XOR D",
        operand_length: 0,
        execute: None,
    }, // 0xAA
    GbCpuInstructions {
        dissasembly: "XOR E",
        operand_length: 0,
        execute: None,
    }, // 0xAB
    GbCpuInstructions {
        dissasembly: "XOR H",
        operand_length: 0,
        execute: None,
    }, // 0xAC
    GbCpuInstructions {
        dissasembly: "XOR L",
        operand_length: 0,
        execute: None,
    }, // 0xAD
    GbCpuInstructions {
        dissasembly: "XOR (HL)",
        operand_length: 0,
        execute: None,
    }, // 0xAE
    GbCpuInstructions {
        dissasembly: "XOR A",
        operand_length: 0,
        execute: None,
    }, // 0xAF
    GbCpuInstructions {
        dissasembly: "OR B",
        operand_length: 0,
        execute: None,
    }, // 0xB0
    GbCpuInstructions {
        dissasembly: "OR C",
        operand_length: 0,
        execute: None,
    }, // 0xB1
    GbCpuInstructions {
        dissasembly: "OR D",
        operand_length: 0,
        execute: None,
    }, // 0xB2
    GbCpuInstructions {
        dissasembly: "OR E",
        operand_length: 0,
        execute: None,
    }, // 0xB3
    GbCpuInstructions {
        dissasembly: "OR H",
        operand_length: 0,
        execute: None,
    }, // 0xB4
    GbCpuInstructions {
        dissasembly: "OR L",
        operand_length: 0,
        execute: None,
    }, // 0xB5
    GbCpuInstructions {
        dissasembly: "OR (HL)",
        operand_length: 0,
        execute: None,
    }, // 0xB6
    GbCpuInstructions {
        dissasembly: "OR A",
        operand_length: 0,
        execute: None,
    }, // 0xB7
    GbCpuInstructions {
        dissasembly: "CP B",
        operand_length: 0,
        execute: None,
    }, // 0xB8
    GbCpuInstructions {
        dissasembly: "CP C",
        operand_length: 0,
        execute: None,
    }, // 0xB9
    GbCpuInstructions {
        dissasembly: "CP D",
        operand_length: 0,
        execute: None,
    }, // 0xBA
    GbCpuInstructions {
        dissasembly: "CP E",
        operand_length: 0,
        execute: None,
    }, // 0xBB
    GbCpuInstructions {
        dissasembly: "CP H",
        operand_length: 0,
        execute: None,
    }, // 0xBC
    GbCpuInstructions {
        dissasembly: "CP L",
        operand_length: 0,
        execute: None,
    }, // 0xBD
    GbCpuInstructions {
        dissasembly: "CP (HL)",
        operand_length: 0,
        execute: None,
    }, // 0xBE
    GbCpuInstructions {
        dissasembly: "CP A",
        operand_length: 0,
        execute: None,
    }, // 0xBF
    GbCpuInstructions {
        dissasembly: "RET NZ",
        operand_length: 0,
        execute: None,
    }, // 0xC0
    GbCpuInstructions {
        dissasembly: "POP BC",
        operand_length: 0,
        execute: None,
    }, // 0xC1
    GbCpuInstructions {
        dissasembly: "JP NZ, a16",
        operand_length: 2,
        execute: None,
    }, // 0xC2
    GbCpuInstructions {
        dissasembly: "JP a16",
        operand_length: 2,
        execute: Some(cpu_jp_a16),
    }, // 0xC3
    GbCpuInstructions {
        dissasembly: "CALL NZ, a16",
        operand_length: 2,
        execute: None,
    }, // 0xC4
    GbCpuInstructions {
        dissasembly: "PUSH BC",
        operand_length: 0,
        execute: None,
    }, // 0xC5
    GbCpuInstructions {
        dissasembly: "ADD A, a8",
        operand_length: 1,
        execute: None,
    }, // 0xC6
    GbCpuInstructions {
        dissasembly: "RST 0",
        operand_length: 0,
        execute: None,
    }, // 0xC7
    GbCpuInstructions {
        dissasembly: "RET Z",
        operand_length: 0,
        execute: None,
    }, // 0xC8
    GbCpuInstructions {
        dissasembly: "RET",
        operand_length: 0,
        execute: None,
    }, // 0xC9
    GbCpuInstructions {
        dissasembly: "JP Z, a16",
        operand_length: 2,
        execute: None,
    }, // 0xCA
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xCB
    GbCpuInstructions {
        dissasembly: "CALL Z, a16",
        operand_length: 2,
        execute: None,
    }, // 0xCC
    GbCpuInstructions {
        dissasembly: "CALL a16",
        operand_length: 2,
        execute: None,
    }, // 0xCD
    GbCpuInstructions {
        dissasembly: "ADC A, d8",
        operand_length: 1,
        execute: None,
    }, // 0xCE
    GbCpuInstructions {
        dissasembly: "RST 1",
        operand_length: 0,
        execute: None,
    }, // 0xCF
    GbCpuInstructions {
        dissasembly: "RET NC",
        operand_length: 0,
        execute: None,
    }, // 0xD0
    GbCpuInstructions {
        dissasembly: "POP DE",
        operand_length: 0,
        execute: None,
    }, // 0xD1
    GbCpuInstructions {
        dissasembly: "JP NC, a16",
        operand_length: 2,
        execute: None,
    }, // 0xD2
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xD3
    GbCpuInstructions {
        dissasembly: "CALL NC, a16",
        operand_length: 2,
        execute: None,
    }, // 0xD4
    GbCpuInstructions {
        dissasembly: "PUSH DE",
        operand_length: 0,
        execute: None,
    }, // 0xD5
    GbCpuInstructions {
        dissasembly: "SUB d8",
        operand_length: 1,
        execute: None,
    }, // 0xD6
    GbCpuInstructions {
        dissasembly: "RST 2",
        operand_length: 0,
        execute: None,
    }, // 0xD7
    GbCpuInstructions {
        dissasembly: "RET C",
        operand_length: 0,
        execute: None,
    }, // 0xD8
    GbCpuInstructions {
        dissasembly: "RETI",
        operand_length: 0,
        execute: None,
    }, // 0xD9
    GbCpuInstructions {
        dissasembly: "JP C, a16",
        operand_length: 2,
        execute: None,
    }, // 0xDA
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xDB
    GbCpuInstructions {
        dissasembly: "CALL C, a16",
        operand_length: 2,
        execute: None,
    }, // 0xDC
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xDD
    GbCpuInstructions {
        dissasembly: "SBC A, d8",
        operand_length: 1,
        execute: None,
    }, // 0xDE
    GbCpuInstructions {
        dissasembly: "RST 3",
        operand_length: 0,
        execute: None,
    }, // 0xDF
    GbCpuInstructions {
        dissasembly: "LD (a8), A",
        operand_length: 1,
        execute: None,
    }, // 0xE0
    GbCpuInstructions {
        dissasembly: "POP HL",
        operand_length: 0,
        execute: None,
    }, // 0xE1
    GbCpuInstructions {
        dissasembly: "LD (C), A",
        operand_length: 0,
        execute: None,
    }, // 0xE2
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xE3
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xE4
    GbCpuInstructions {
        dissasembly: "PUSH HL",
        operand_length: 0,
        execute: None,
    }, // 0xE5
    GbCpuInstructions {
        dissasembly: "AND d8",
        operand_length: 1,
        execute: None,
    }, // 0xE6
    GbCpuInstructions {
        dissasembly: "RST 4",
        operand_length: 0,
        execute: None,
    }, // 0xE7
    GbCpuInstructions {
        dissasembly: "ADD Sp, s8",
        operand_length: 1,
        execute: None,
    }, // 0xE8
    GbCpuInstructions {
        dissasembly: "JP HL",
        operand_length: 0,
        execute: None,
    }, // 0xE9
    GbCpuInstructions {
        dissasembly: "LD (a16), A",
        operand_length: 2,
        execute: None,
    }, // 0xEA
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xEB
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xEC
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xED
    GbCpuInstructions {
        dissasembly: "XOR d8",
        operand_length: 1,
        execute: None,
    }, // 0xEE
    GbCpuInstructions {
        dissasembly: "RST 5",
        operand_length: 0,
        execute: None,
    }, // 0xEF
    GbCpuInstructions {
        dissasembly: "LD A, (a8)",
        operand_length: 1,
        execute: None,
    }, // 0xF0
    GbCpuInstructions {
        dissasembly: "POP AF",
        operand_length: 0,
        execute: None,
    }, // 0xF1
    GbCpuInstructions {
        dissasembly: "LD A, (C)",
        operand_length: 0,
        execute: None,
    }, // 0xF2
    GbCpuInstructions {
        dissasembly: "DI",
        operand_length: 0,
        execute: None,
    }, // 0xF3
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xF4
    GbCpuInstructions {
        dissasembly: "PUSH AF",
        operand_length: 0,
        execute: None,
    }, // 0xF5
    GbCpuInstructions {
        dissasembly: "OR d8",
        operand_length: 1,
        execute: None,
    }, // 0xF6
    GbCpuInstructions {
        dissasembly: "RST 6",
        operand_length: 0,
        execute: None,
    }, // 0xF7
    GbCpuInstructions {
        dissasembly: "LD HL, SP+s8",
        operand_length: 1,
        execute: None,
    }, // 0xF8
    GbCpuInstructions {
        dissasembly: "LD SP, HL",
        operand_length: 0,
        execute: None,
    }, // 0xF9
    GbCpuInstructions {
        dissasembly: "LD A, (a16)",
        operand_length: 2,
        execute: None,
    }, // 0xFA
    GbCpuInstructions {
        dissasembly: "EI",
        operand_length: 0,
        execute: None,
    }, // 0xFB
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xFC
    GbCpuInstructions {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xFD
    GbCpuInstructions {
        dissasembly: "CP d8",
        operand_length: 1,
        execute: None,
    }, // 0xFE
    GbCpuInstructions {
        dissasembly: "RST 7",
        operand_length: 0,
        execute: None,
    }, // 0xFF
];

fn cpu_nop() {
    core_advance_cpu_clock(4);
}

fn cpu_jp_a16() {
    core_advance_cpu_clock(4);
    let mut cpu = CPU.lock().unwrap();

    let low = memory_bus_read(usize::from(cpu.registers.pc));
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

    let high = memory_bus_read(usize::from(cpu.registers.pc));
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

    let address = u16::from_le_bytes([low, high]);

    core_advance_cpu_clock(4);
    cpu.registers.pc = address;
    core_advance_cpu_clock(4);
}
