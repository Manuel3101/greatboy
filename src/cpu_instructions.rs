use crate::{
    cpu::CPU,
    cpu_registers::{Reg8, Reg16},
    cpu_routines::{
        cpu_routine_adc_a_8, cpu_routine_add_a_8, cpu_routine_add_hl_16, cpu_routine_and_a_8,
        cpu_routine_cp_a_8, cpu_routine_dec_8, cpu_routine_dec_16, cpu_routine_inc_8,
        cpu_routine_inc_16, cpu_routine_ld_8, cpu_routine_ld_16, cpu_routine_ld_ptr8,
        cpu_routine_ld_ptr16, cpu_routine_or_a_8, cpu_routine_pop_16, cpu_routine_push_16,
        cpu_routine_rst_nnnn, cpu_routine_sbc_a_8, cpu_routine_sub_a_8, cpu_routine_xor_a_8,
    },
    emulator_core::core_advance_cpu_clock,
    memory_bus::memory_bus_read,
};

#[derive(Debug, Clone, Copy)]
pub struct GbCpuInstructions {
    pub dissasembly: &'static str, // name of the instruction
    pub operand_length: u8,        // n = 1 byte, nn = 2 bytes
    pub execute: Option<fn()>,     // function pointer to the instruction implementation
}

pub static INSTRUCTIONS: [GbCpuInstructions; 256] = [
    GbCpuInstructions {
        dissasembly: "NOP",
        operand_length: 0,
        execute: Some(cpu_nop),
    }, // 0x00
    GbCpuInstructions {
        dissasembly: "LD BC, nn",
        operand_length: 1,
        execute: Some(cpu_ld_bc_nn),
    }, // 0x01
    GbCpuInstructions {
        dissasembly: "LD (BC), A",
        operand_length: 0,
        execute: Some(cpu_ld_bc_a),
    }, // 0x02
    GbCpuInstructions {
        dissasembly: "INC BC",
        operand_length: 0,
        execute: Some(cpu_inc_bc),
    }, // 0x03
    GbCpuInstructions {
        dissasembly: "INC B",
        operand_length: 0,
        execute: Some(cpu_inc_b),
    }, // 0x04
    GbCpuInstructions {
        dissasembly: "DEC B",
        operand_length: 0,
        execute: Some(cpu_dec_b),
    }, // 0x05
    GbCpuInstructions {
        dissasembly: "LD B, d8",
        operand_length: 1,
        execute: Some(cpu_ld_b_n),
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
        execute: Some(cpu_add_hl_bc),
    }, // 0x09
    GbCpuInstructions {
        dissasembly: "LD A , (BC)",
        operand_length: 0,
        execute: Some(cpu_ld_a_bc),
    }, // 0x0A
    GbCpuInstructions {
        dissasembly: "DEC BC",
        operand_length: 0,
        execute: Some(cpu_dec_bc),
    }, // 0x0B
    GbCpuInstructions {
        dissasembly: "INC C",
        operand_length: 0,
        execute: Some(cpu_inc_c),
    }, // 0x0C
    GbCpuInstructions {
        dissasembly: "DEC C",
        operand_length: 0,
        execute: Some(cpu_dec_c),
    }, // 0x0D
    GbCpuInstructions {
        dissasembly: "LD C, d8",
        operand_length: 1,
        execute: Some(cpu_ld_c_n),
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
        execute: Some(cpu_ld_de_nn),
    }, // 0x11
    GbCpuInstructions {
        dissasembly: "LD (DE), A",
        operand_length: 0,
        execute: Some(cpu_ld_de_a),
    }, // 0x12
    GbCpuInstructions {
        dissasembly: "INC DE",
        operand_length: 0,
        execute: Some(cpu_inc_de),
    }, // 0x13
    GbCpuInstructions {
        dissasembly: "INC D",
        operand_length: 0,
        execute: Some(cpu_inc_d),
    }, // 0x14
    GbCpuInstructions {
        dissasembly: "DEC D",
        operand_length: 0,
        execute: Some(cpu_dec_d),
    }, // 0x15
    GbCpuInstructions {
        dissasembly: "LD D, d8",
        operand_length: 1,
        execute: Some(cpu_ld_d_n),
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
        execute: Some(cpu_add_hl_de),
    }, // 0x19
    GbCpuInstructions {
        dissasembly: "LD A, (DE)",
        operand_length: 0,
        execute: Some(cpu_ld_a_de),
    }, // 0x1A
    GbCpuInstructions {
        dissasembly: "DEC DE",
        operand_length: 0,
        execute: Some(cpu_dec_de),
    }, // 0x1B
    GbCpuInstructions {
        dissasembly: "INC E",
        operand_length: 0,
        execute: Some(cpu_inc_e),
    }, // 0x1C
    GbCpuInstructions {
        dissasembly: "DEC E",
        operand_length: 0,
        execute: Some(cpu_dec_e),
    }, // 0x1D
    GbCpuInstructions {
        dissasembly: "LD E, d8",
        operand_length: 1,
        execute: Some(cpu_ld_e_n),
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
        execute: Some(cpu_ld_hl_nn),
    }, // 0x21
    GbCpuInstructions {
        dissasembly: "LD (HL+), A",
        operand_length: 0,
        execute: None,
    }, // 0x22
    GbCpuInstructions {
        dissasembly: "INC HL",
        operand_length: 0,
        execute: Some(cpu_inc_hl),
    }, // 0x23
    GbCpuInstructions {
        dissasembly: "INC H",
        operand_length: 0,
        execute: Some(cpu_inc_h),
    }, // 0x24
    GbCpuInstructions {
        dissasembly: "DEC H",
        operand_length: 0,
        execute: Some(cpu_dec_h),
    }, // 0x25
    GbCpuInstructions {
        dissasembly: "LD H, d8",
        operand_length: 1,
        execute: Some(cpu_ld_h_n),
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
        execute: Some(cpu_dec_hl),
    }, // 0x2B
    GbCpuInstructions {
        dissasembly: "INC L",
        operand_length: 0,
        execute: Some(cpu_inc_l),
    }, // 0x2C
    GbCpuInstructions {
        dissasembly: "DEC L",
        operand_length: 0,
        execute: Some(cpu_dec_l),
    }, // 0x2D
    GbCpuInstructions {
        dissasembly: "LD L, d8",
        operand_length: 1,
        execute: Some(cpu_ld_l_n),
    }, // 0x2E
    GbCpuInstructions {
        dissasembly: "CPL",
        operand_length: 0,
        execute: Some(cpu_cpl),
    }, // 0x2F
    GbCpuInstructions {
        dissasembly: "JR NC, s8",
        operand_length: 1,
        execute: None,
    }, // 0x30
    GbCpuInstructions {
        dissasembly: "LD SP, d16",
        operand_length: 2,
        execute: Some(cpu_ld_sp_nn),
    }, // 0x31
    GbCpuInstructions {
        dissasembly: "LD (HL-), A",
        operand_length: 0,
        execute: None,
    }, // 0x32
    GbCpuInstructions {
        dissasembly: "INC SP",
        operand_length: 0,
        execute: Some(cpu_inc_sp),
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
        execute: Some(cpu_add_hl_sp),
    }, // 0x39
    GbCpuInstructions {
        dissasembly: "LD A, (HL-)",
        operand_length: 0,
        execute: None,
    }, // 0x3A
    GbCpuInstructions {
        dissasembly: "DEC SP",
        operand_length: 0,
        execute: Some(cpu_dec_sp),
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
        execute: Some(cpu_ld_b_hl),
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
        execute: Some(cpu_ld_c_d),
    }, // 0x4A
    GbCpuInstructions {
        dissasembly: "LD C, E",
        operand_length: 0,
        execute: Some(cpu_ld_c_e),
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
        execute: Some(cpu_ld_c_hl),
    }, // 0x4E
    GbCpuInstructions {
        dissasembly: "LD C, A",
        operand_length: 0,
        execute: Some(cpu_ld_c_a),
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
        execute: Some(cpu_ld_d_hl),
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
        execute: Some(cpu_ld_e_e),
    }, // 0x5B
    GbCpuInstructions {
        dissasembly: "LD E, H",
        operand_length: 0,
        execute: Some(cpu_ld_e_h),
    }, // 0x5C
    GbCpuInstructions {
        dissasembly: "LD E, L",
        operand_length: 0,
        execute: None,
    }, // 0x5D
    GbCpuInstructions {
        dissasembly: "LD E, (HL)",
        operand_length: 0,
        execute: Some(cpu_ld_e_hl),
    }, // 0x5E
    GbCpuInstructions {
        dissasembly: "LD E, A",
        operand_length: 0,
        execute: None,
    }, // 0x5F
    GbCpuInstructions {
        dissasembly: "LD H, B",
        operand_length: 0,
        execute: Some(cpu_ld_h_b),
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
        execute: Some(cpu_ld_h_hl),
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
        execute: Some(cpu_ld_l_e),
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
        execute: Some(cpu_ld_l_hl),
    }, // 0x6E
    GbCpuInstructions {
        dissasembly: "LD L, A",
        operand_length: 0,
        execute: Some(cpu_ld_l_a),
    }, // 0x6F
    GbCpuInstructions {
        dissasembly: "LD (HL), B",
        operand_length: 0,
        execute: Some(cpu_ld_hl_b),
    }, // 0x70
    GbCpuInstructions {
        dissasembly: "LD (HL), C",
        operand_length: 0,
        execute: Some(cpu_ld_hl_c),
    }, // 0x71
    GbCpuInstructions {
        dissasembly: "LD (HL), D",
        operand_length: 0,
        execute: Some(cpu_ld_hl_d),
    }, // 0x72
    GbCpuInstructions {
        dissasembly: "LD (HL), E",
        operand_length: 0,
        execute: Some(cpu_ld_hl_e),
    }, // 0x73
    GbCpuInstructions {
        dissasembly: "LD (HL), H",
        operand_length: 0,
        execute: Some(cpu_ld_hl_h),
    }, // 0x74
    GbCpuInstructions {
        dissasembly: "LD (HL), L",
        operand_length: 0,
        execute: Some(cpu_ld_hl_l),
    }, // 0x75
    GbCpuInstructions {
        dissasembly: "HALT",
        operand_length: 0,
        execute: None,
    }, // 0x76
    GbCpuInstructions {
        dissasembly: "LD (HL), A",
        operand_length: 0,
        execute: Some(cpu_ld_hl_a),
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
        execute: Some(cpu_ld_a_hl),
    }, // 0x7E
    GbCpuInstructions {
        dissasembly: "LD A, A",
        operand_length: 0,
        execute: None,
    }, // 0x7F
    GbCpuInstructions {
        dissasembly: "ADD A, B",
        operand_length: 0,
        execute: Some(cpu_add_a_b),
    }, // 0x80
    GbCpuInstructions {
        dissasembly: "ADD A, C",
        operand_length: 0,
        execute: Some(cpu_add_a_c),
    }, // 0x81
    GbCpuInstructions {
        dissasembly: "ADD A, D",
        operand_length: 0,
        execute: Some(cpu_add_a_d),
    }, // 0x82
    GbCpuInstructions {
        dissasembly: "ADD A, E",
        operand_length: 0,
        execute: Some(cpu_add_a_e),
    }, // 0x83
    GbCpuInstructions {
        dissasembly: "ADD A, H",
        operand_length: 0,
        execute: Some(cpu_add_a_h),
    }, // 0x84
    GbCpuInstructions {
        dissasembly: "ADD A, L",
        operand_length: 0,
        execute: Some(cpu_add_a_l),
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
        execute: Some(cpu_adc_a_b),
    }, // 0x88
    GbCpuInstructions {
        dissasembly: "ADC A, C",
        operand_length: 0,
        execute: Some(cpu_adc_a_c),
    }, // 0x89
    GbCpuInstructions {
        dissasembly: "ADC A, D",
        operand_length: 0,
        execute: Some(cpu_adc_a_d),
    }, // 0x8A
    GbCpuInstructions {
        dissasembly: "ADC A, E",
        operand_length: 0,
        execute: Some(cpu_adc_a_e),
    }, // 0x8B
    GbCpuInstructions {
        dissasembly: "ADC A, H",
        operand_length: 0,
        execute: Some(cpu_adc_a_h),
    }, // 0x8C
    GbCpuInstructions {
        dissasembly: "ADC A, L",
        operand_length: 0,
        execute: Some(cpu_adc_a_l),
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
        execute: Some(cpu_sub_a_b),
    }, // 0x90
    GbCpuInstructions {
        dissasembly: "SUB C",
        operand_length: 0,
        execute: Some(cpu_sub_a_c),
    }, // 0x91
    GbCpuInstructions {
        dissasembly: "SUB D",
        operand_length: 0,
        execute: Some(cpu_sub_a_d),
    }, // 0x92
    GbCpuInstructions {
        dissasembly: "SUB E",
        operand_length: 0,
        execute: Some(cpu_sub_a_e),
    }, // 0x93
    GbCpuInstructions {
        dissasembly: "SUB H",
        operand_length: 0,
        execute: Some(cpu_sub_a_h),
    }, // 0x94
    GbCpuInstructions {
        dissasembly: "SUB L",
        operand_length: 0,
        execute: Some(cpu_sub_a_l),
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
        execute: Some(cpu_sbc_a_b),
    }, // 0x98
    GbCpuInstructions {
        dissasembly: "SBC A, C",
        operand_length: 0,
        execute: Some(cpu_sbc_a_c),
    }, // 0x99
    GbCpuInstructions {
        dissasembly: "SBC A, D",
        operand_length: 0,
        execute: Some(cpu_sbc_a_d),
    }, // 0x9A
    GbCpuInstructions {
        dissasembly: "SBC A, E",
        operand_length: 0,
        execute: Some(cpu_sbc_a_e),
    }, // 0x9B
    GbCpuInstructions {
        dissasembly: "SBC A, H",
        operand_length: 0,
        execute: Some(cpu_sbc_a_h),
    }, // 0x9C
    GbCpuInstructions {
        dissasembly: "SBC A, L",
        operand_length: 0,
        execute: Some(cpu_sbc_a_l),
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
        execute: Some(cpu_and_a_b),
    }, // 0xA0
    GbCpuInstructions {
        dissasembly: "AND C",
        operand_length: 0,
        execute: Some(cpu_and_a_c),
    }, // 0xA1
    GbCpuInstructions {
        dissasembly: "AND D",
        operand_length: 0,
        execute: Some(cpu_and_a_d),
    }, // 0xA2
    GbCpuInstructions {
        dissasembly: "AND E",
        operand_length: 0,
        execute: Some(cpu_and_a_e),
    }, // 0xA3
    GbCpuInstructions {
        dissasembly: "AND H",
        operand_length: 0,
        execute: Some(cpu_and_a_h),
    }, // 0xA4
    GbCpuInstructions {
        dissasembly: "AND L",
        operand_length: 0,
        execute: Some(cpu_and_a_l),
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
        execute: Some(cpu_xor_a_b),
    }, // 0xA8
    GbCpuInstructions {
        dissasembly: "XOR C",
        operand_length: 0,
        execute: Some(cpu_xor_a_c),
    }, // 0xA9
    GbCpuInstructions {
        dissasembly: "XOR D",
        operand_length: 0,
        execute: Some(cpu_xor_a_d),
    }, // 0xAA
    GbCpuInstructions {
        dissasembly: "XOR E",
        operand_length: 0,
        execute: Some(cpu_xor_a_e),
    }, // 0xAB
    GbCpuInstructions {
        dissasembly: "XOR H",
        operand_length: 0,
        execute: Some(cpu_xor_a_h),
    }, // 0xAC
    GbCpuInstructions {
        dissasembly: "XOR L",
        operand_length: 0,
        execute: Some(cpu_xor_a_l),
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
        execute: Some(cpu_or_a_b),
    }, // 0xB0
    GbCpuInstructions {
        dissasembly: "OR C",
        operand_length: 0,
        execute: Some(cpu_or_a_c),
    }, // 0xB1
    GbCpuInstructions {
        dissasembly: "OR D",
        operand_length: 0,
        execute: Some(cpu_or_a_d),
    }, // 0xB2
    GbCpuInstructions {
        dissasembly: "OR E",
        operand_length: 0,
        execute: Some(cpu_or_a_e),
    }, // 0xB3
    GbCpuInstructions {
        dissasembly: "OR H",
        operand_length: 0,
        execute: Some(cpu_or_a_h),
    }, // 0xB4
    GbCpuInstructions {
        dissasembly: "OR L",
        operand_length: 0,
        execute: Some(cpu_or_a_l),
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
        execute: Some(cpu_cp_a_b),
    }, // 0xB8
    GbCpuInstructions {
        dissasembly: "CP C",
        operand_length: 0,
        execute: Some(cpu_cp_a_c),
    }, // 0xB9
    GbCpuInstructions {
        dissasembly: "CP D",
        operand_length: 0,
        execute: Some(cpu_cp_a_d),
    }, // 0xBA
    GbCpuInstructions {
        dissasembly: "CP E",
        operand_length: 0,
        execute: Some(cpu_cp_a_e),
    }, // 0xBB
    GbCpuInstructions {
        dissasembly: "CP H",
        operand_length: 0,
        execute: Some(cpu_cp_a_h),
    }, // 0xBC
    GbCpuInstructions {
        dissasembly: "CP L",
        operand_length: 0,
        execute: Some(cpu_cp_a_l),
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
        execute: Some(cpu_pop_bc),
    }, // 0xC1
    GbCpuInstructions {
        dissasembly: "JP NZ, a16",
        operand_length: 2,
        execute: None,
    }, // 0xC2
    GbCpuInstructions {
        dissasembly: "JP a16",
        operand_length: 2,
        execute: Some(cpu_jp_nn),
    }, // 0xC3
    GbCpuInstructions {
        dissasembly: "CALL NZ, a16",
        operand_length: 2,
        execute: None,
    }, // 0xC4
    GbCpuInstructions {
        dissasembly: "PUSH BC",
        operand_length: 0,
        execute: Some(cpu_push_bc),
    }, // 0xC5
    GbCpuInstructions {
        dissasembly: "ADD A, a8",
        operand_length: 1,
        execute: None,
    }, // 0xC6
    GbCpuInstructions {
        dissasembly: "RST 0",
        operand_length: 0,
        execute: Some(cpu_rst_00),
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
        execute: Some(cpu_rst_08),
    }, // 0xCF
    GbCpuInstructions {
        dissasembly: "RET NC",
        operand_length: 0,
        execute: None,
    }, // 0xD0
    GbCpuInstructions {
        dissasembly: "POP DE",
        operand_length: 0,
        execute: Some(cpu_pop_de),
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
        execute: Some(cpu_push_de),
    }, // 0xD5
    GbCpuInstructions {
        dissasembly: "SUB d8",
        operand_length: 1,
        execute: None,
    }, // 0xD6
    GbCpuInstructions {
        dissasembly: "RST 2",
        operand_length: 0,
        execute: Some(cpu_rst_10),
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
        execute: Some(cpu_rst_18),
    }, // 0xDF
    GbCpuInstructions {
        dissasembly: "LD (a8), A",
        operand_length: 1,
        execute: None,
    }, // 0xE0
    GbCpuInstructions {
        dissasembly: "POP HL",
        operand_length: 0,
        execute: Some(cpu_pop_hl),
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
        execute: Some(cpu_push_hl),
    }, // 0xE5
    GbCpuInstructions {
        dissasembly: "AND d8",
        operand_length: 1,
        execute: None,
    }, // 0xE6
    GbCpuInstructions {
        dissasembly: "RST 4",
        operand_length: 0,
        execute: Some(cpu_rst_20),
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
        execute: Some(cpu_rst_28),
    }, // 0xEF
    GbCpuInstructions {
        dissasembly: "LD A, (a8)",
        operand_length: 1,
        execute: None,
    }, // 0xF0
    GbCpuInstructions {
        dissasembly: "POP AF",
        operand_length: 0,
        execute: Some(cpu_pop_af),
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
        execute: Some(cpu_push_af),
    }, // 0xF5
    GbCpuInstructions {
        dissasembly: "OR d8",
        operand_length: 1,
        execute: None,
    }, // 0xF6
    GbCpuInstructions {
        dissasembly: "RST 6",
        operand_length: 0,
        execute: Some(cpu_rst_30),
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
        execute: Some(cpu_rst_38),
    }, // 0xFF
];

fn cpu_nop() {
    core_advance_cpu_clock(4);
} // 0x00

fn cpu_ld_bc_nn() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_16(&mut cpu, Reg16::BC);
} // 0x01

fn cpu_ld_bc_a() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr8(&mut cpu, Reg16::BC, Reg8::A);
} // 0x02

fn cpu_inc_bc() {
    let mut cpu = CPU.lock().unwrap();

    cpu_routine_inc_16(&mut cpu, Reg16::BC);
} // 0x03

fn cpu_inc_b() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_inc_8(&mut cpu, Reg8::B);
} // 0x04

fn cpu_dec_b() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_dec_8(&mut cpu, Reg8::B);
} // 0x05

fn cpu_ld_b_n() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_8(&mut cpu, Reg8::B);
} // 0x06

fn cpu_add_hl_bc() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_add_hl_16(&mut cpu, Reg16::BC);
} // 0x09

fn cpu_ld_a_bc() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr16(&mut cpu, Reg8::A, Reg16::BC);
} // 0x0A

fn cpu_dec_bc() {
    let mut cpu = CPU.lock().unwrap();

    cpu_routine_dec_16(&mut cpu, Reg16::BC);
} // 0x0B

fn cpu_inc_c() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_inc_8(&mut cpu, Reg8::C);
} // 0x0C

fn cpu_dec_c() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_dec_8(&mut cpu, Reg8::C);
} // 0x0D

fn cpu_ld_c_n() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_8(&mut cpu, Reg8::C);
} // 0x0E

fn cpu_ld_de_nn() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_16(&mut cpu, Reg16::DE);
} // 0x11

fn cpu_ld_de_a() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr8(&mut cpu, Reg16::DE, Reg8::A);
} // 0x12

fn cpu_inc_de() {
    let mut cpu = CPU.lock().unwrap();

    cpu_routine_inc_16(&mut cpu, Reg16::DE);
} // 0x13

fn cpu_inc_d() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_inc_8(&mut cpu, Reg8::D);
} // 0x14

fn cpu_dec_d() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_dec_8(&mut cpu, Reg8::D);
} // 0x15

fn cpu_ld_d_n() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_8(&mut cpu, Reg8::D);
} // 0x16

fn cpu_add_hl_de() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_add_hl_16(&mut cpu, Reg16::DE);
} // 0x19

fn cpu_ld_a_de() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr16(&mut cpu, Reg8::A, Reg16::DE);
} // 0x1A

fn cpu_dec_de() {
    let mut cpu = CPU.lock().unwrap();

    cpu_routine_dec_16(&mut cpu, Reg16::DE);
} // 0x1B

fn cpu_inc_e() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_inc_8(&mut cpu, Reg8::E);
} // 0x1C

fn cpu_dec_e() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_dec_8(&mut cpu, Reg8::E);
} // 0x1D

fn cpu_ld_e_n() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_8(&mut cpu, Reg8::E);
} // 0x1E

fn cpu_ld_hl_nn() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_16(&mut cpu, Reg16::HL);
} // 0x21

fn cpu_inc_hl() {
    let mut cpu = CPU.lock().unwrap();

    cpu_routine_inc_16(&mut cpu, Reg16::HL);
} // 0x23

fn cpu_inc_h() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_inc_8(&mut cpu, Reg8::H);
} // 0x24

fn cpu_dec_hl() {
    let mut cpu = CPU.lock().unwrap();

    cpu_routine_dec_16(&mut cpu, Reg16::HL);
} // 0x2B

fn cpu_inc_l() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_inc_8(&mut cpu, Reg8::L);
} // 0x2C

fn cpu_ld_h_n() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_8(&mut cpu, Reg8::H);
} // 0x26

fn cpu_ld_l_n() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_8(&mut cpu, Reg8::L);
} // 0x2E

fn cpu_ld_sp_nn() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_16(&mut cpu, Reg16::SP);
} // 0x31

fn cpu_cpl() {
    let mut cpu = CPU.lock().unwrap();
    cpu.registers.a = !cpu.registers.a;
    cpu.registers.set_subtract(true);
    cpu.registers.set_half_carry(true);
    core_advance_cpu_clock(4);
} // 0x2F

fn cpu_dec_h() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_dec_8(&mut cpu, Reg8::H);
} // 0x25

fn cpu_dec_l() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_dec_8(&mut cpu, Reg8::L);
} // 0x2D

fn cpu_inc_sp() {
    let mut cpu = CPU.lock().unwrap();

    cpu_routine_inc_16(&mut cpu, Reg16::SP);
} // 0x33

fn cpu_add_hl_sp() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_add_hl_16(&mut cpu, Reg16::SP);
} // 0x39

fn cpu_dec_sp() {
    let mut cpu = CPU.lock().unwrap();

    cpu_routine_dec_16(&mut cpu, Reg16::SP);
} // 0x3B

fn cpu_ld_b_hl() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr16(&mut cpu, Reg8::B, Reg16::HL);
} // 0x46

fn cpu_ld_c_d() {
    let mut cpu = CPU.lock().unwrap();
    cpu.registers.c = cpu.registers.d;
    core_advance_cpu_clock(4);
} // 0x4A

fn cpu_ld_c_e() {
    let mut cpu = CPU.lock().unwrap();
    cpu.registers.c = cpu.registers.e;
    core_advance_cpu_clock(4);
} // 0x4B

fn cpu_ld_c_hl() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr16(&mut cpu, Reg8::C, Reg16::HL);
} // 0x4E

fn cpu_ld_c_a() {
    let mut cpu = CPU.lock().unwrap();
    cpu.registers.c = cpu.registers.a;
    core_advance_cpu_clock(4);
} // 0x4F

fn cpu_ld_d_hl() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr16(&mut cpu, Reg8::D, Reg16::HL);
} // 0x56

fn cpu_ld_e_e() {
    core_advance_cpu_clock(4);
} // 0x5B

fn cpu_ld_e_h() {
    let mut cpu = CPU.lock().unwrap();
    cpu.registers.e = cpu.registers.h;
    core_advance_cpu_clock(4);
} // 0x5C

fn cpu_ld_e_hl() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr16(&mut cpu, Reg8::E, Reg16::HL);
} // 0x5E

fn cpu_ld_h_b() {
    let mut cpu = CPU.lock().unwrap();
    cpu.registers.h = cpu.registers.b;
    core_advance_cpu_clock(4);
} // 0x60

fn cpu_ld_h_hl() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr16(&mut cpu, Reg8::H, Reg16::HL);
} // 0x66

fn cpu_ld_l_e() {
    let mut cpu = CPU.lock().unwrap();
    cpu.registers.l = cpu.registers.e;
    core_advance_cpu_clock(4);
} // 0x6B

fn cpu_ld_l_hl() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr16(&mut cpu, Reg8::L, Reg16::HL);
} // 0x6E

fn cpu_ld_l_a() {
    let mut cpu = CPU.lock().unwrap();
    cpu.registers.l = cpu.registers.a;
    core_advance_cpu_clock(4);
} // 0x6F

fn cpu_ld_hl_b() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr8(&mut cpu, Reg16::HL, Reg8::B);
} // 0x70

fn cpu_ld_hl_c() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr8(&mut cpu, Reg16::HL, Reg8::C);
} // 0x71

fn cpu_ld_hl_d() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr8(&mut cpu, Reg16::HL, Reg8::D);
} // 0x72

fn cpu_ld_hl_e() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr8(&mut cpu, Reg16::HL, Reg8::E);
} // 0x73

fn cpu_ld_hl_h() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr8(&mut cpu, Reg16::HL, Reg8::H);
} // 0x74

fn cpu_ld_hl_l() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr8(&mut cpu, Reg16::HL, Reg8::L);
} // 0x75

fn cpu_ld_hl_a() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr8(&mut cpu, Reg16::HL, Reg8::A);
} // 0x77

fn cpu_ld_a_hl() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_ld_ptr16(&mut cpu, Reg8::A, Reg16::HL);
} // 0x7E

fn cpu_add_a_b() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_add_a_8(&mut cpu, Reg8::B);
} // 0x80

fn cpu_add_a_c() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_add_a_8(&mut cpu, Reg8::C);
} // 0x81

fn cpu_add_a_d() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_add_a_8(&mut cpu, Reg8::D);
} // 0x82

fn cpu_add_a_e() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_add_a_8(&mut cpu, Reg8::E);
} // 0x83

fn cpu_add_a_h() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_add_a_8(&mut cpu, Reg8::H);
} // 0x84

fn cpu_add_a_l() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_add_a_8(&mut cpu, Reg8::L);
} // 0x85

fn cpu_adc_a_b() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_adc_a_8(&mut cpu, Reg8::B);
} // 0x88

fn cpu_adc_a_c() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_adc_a_8(&mut cpu, Reg8::C);
} // 0x89

fn cpu_adc_a_d() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_adc_a_8(&mut cpu, Reg8::D);
} // 0x8A

fn cpu_adc_a_e() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_adc_a_8(&mut cpu, Reg8::E);
} // 0x8B

fn cpu_adc_a_h() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_adc_a_8(&mut cpu, Reg8::H);
} // 0x8C

fn cpu_adc_a_l() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_adc_a_8(&mut cpu, Reg8::L);
} // 0x8D

fn cpu_sub_a_b() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_sub_a_8(&mut cpu, Reg8::B);
} // 0x90

fn cpu_sub_a_c() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_sub_a_8(&mut cpu, Reg8::C);
} // 0x91

fn cpu_sub_a_d() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_sub_a_8(&mut cpu, Reg8::D);
} // 0x92

fn cpu_sub_a_e() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_sub_a_8(&mut cpu, Reg8::E);
} // 0x93

fn cpu_sub_a_h() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_sub_a_8(&mut cpu, Reg8::H);
} // 0x94

fn cpu_sub_a_l() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_sub_a_8(&mut cpu, Reg8::L);
} // 0x95

fn cpu_sbc_a_b() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_sbc_a_8(&mut cpu, Reg8::B);
} // 0x98

fn cpu_sbc_a_c() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_sbc_a_8(&mut cpu, Reg8::C);
} // 0x99

fn cpu_sbc_a_d() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_sbc_a_8(&mut cpu, Reg8::D);
} // 0x9A

fn cpu_sbc_a_e() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_sbc_a_8(&mut cpu, Reg8::E);
} // 0x9B

fn cpu_sbc_a_h() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_sbc_a_8(&mut cpu, Reg8::H);
} // 0x9C

fn cpu_sbc_a_l() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_sbc_a_8(&mut cpu, Reg8::L);
} // 0x9D

fn cpu_and_a_b() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_and_a_8(&mut cpu, Reg8::B);
} // 0xA0

fn cpu_and_a_c() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_and_a_8(&mut cpu, Reg8::C);
} // 0xA1

fn cpu_and_a_d() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_and_a_8(&mut cpu, Reg8::D);
} // 0xA2

fn cpu_and_a_e() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_and_a_8(&mut cpu, Reg8::E);
} // 0xA3

fn cpu_and_a_h() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_and_a_8(&mut cpu, Reg8::H);
} // 0xA4

fn cpu_and_a_l() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_and_a_8(&mut cpu, Reg8::L);
} // 0xA5

fn cpu_xor_a_b() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_xor_a_8(&mut cpu, Reg8::B);
} // 0xA8

fn cpu_xor_a_c() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_xor_a_8(&mut cpu, Reg8::C);
} // 0xA9

fn cpu_xor_a_d() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_xor_a_8(&mut cpu, Reg8::D);
} // 0xAA

fn cpu_xor_a_e() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_xor_a_8(&mut cpu, Reg8::E);
} // 0xAB

fn cpu_xor_a_h() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_xor_a_8(&mut cpu, Reg8::H);
} // 0xAC

fn cpu_xor_a_l() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_xor_a_8(&mut cpu, Reg8::L);
} // 0xAD

fn cpu_or_a_b() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_or_a_8(&mut cpu, Reg8::B);
} // 0xB0

fn cpu_or_a_c() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_or_a_8(&mut cpu, Reg8::C);
} // 0xB1

fn cpu_or_a_d() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_or_a_8(&mut cpu, Reg8::D);
} // 0xB2

fn cpu_or_a_e() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_or_a_8(&mut cpu, Reg8::E);
} // 0xB3

fn cpu_or_a_h() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_or_a_8(&mut cpu, Reg8::H);
} // 0xB4

fn cpu_or_a_l() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_or_a_8(&mut cpu, Reg8::L);
} // 0xB5

fn cpu_cp_a_b() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_cp_a_8(&mut cpu, Reg8::B);
} // 0xB8

fn cpu_cp_a_c() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_cp_a_8(&mut cpu, Reg8::C);
} // 0xB9

fn cpu_cp_a_d() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_cp_a_8(&mut cpu, Reg8::D);
} // 0xBA

fn cpu_cp_a_e() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_cp_a_8(&mut cpu, Reg8::E);
} // 0xBB

fn cpu_cp_a_h() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_cp_a_8(&mut cpu, Reg8::H);
} // 0xBC

fn cpu_cp_a_l() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_cp_a_8(&mut cpu, Reg8::L);
} // 0xBD

fn cpu_pop_bc() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_pop_16(&mut cpu, Reg8::B, Reg8::C);
} // 0xC1

fn cpu_jp_nn() {
    core_advance_cpu_clock(4);
    let mut cpu = CPU.lock().unwrap();

    let low = memory_bus_read(usize::from(cpu.registers.pc));
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    core_advance_cpu_clock(4);

    let high = memory_bus_read(usize::from(cpu.registers.pc));
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

    let address = u16::from_le_bytes([low, high]);

    core_advance_cpu_clock(4);
    cpu.registers.pc = address;
    core_advance_cpu_clock(4);
} // 0xC3

fn cpu_push_bc() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_push_16(&mut cpu, Reg8::B, Reg8::C);
} // 0xC5

fn cpu_rst_00() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_rst_nnnn(&mut cpu, 0x0000);
} // 0xC7

fn cpu_rst_08() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_rst_nnnn(&mut cpu, 0x0008);
} // 0xCF

fn cpu_pop_de() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_pop_16(&mut cpu, Reg8::D, Reg8::E);
} // 0xD1

fn cpu_push_de() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_push_16(&mut cpu, Reg8::D, Reg8::E);
} // 0xD5

fn cpu_rst_10() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_rst_nnnn(&mut cpu, 0x0010);
} // 0xD7

fn cpu_rst_18() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_rst_nnnn(&mut cpu, 0x0018);
} // 0xDF

fn cpu_pop_hl() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_pop_16(&mut cpu, Reg8::H, Reg8::L);
} // 0xE1

fn cpu_push_hl() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_push_16(&mut cpu, Reg8::H, Reg8::L);
} // 0xE5

fn cpu_rst_20() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_rst_nnnn(&mut cpu, 0x0020);
} // 0xE7

fn cpu_rst_28() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_rst_nnnn(&mut cpu, 0x0028);
} // 0xEF

fn cpu_pop_af() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_pop_16(&mut cpu, Reg8::A, Reg8::F);
    cpu.registers.f &= 0xF0; // All flags are reset to 0
} // 0xF1

fn cpu_push_af() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_push_16(&mut cpu, Reg8::A, Reg8::F);
} // 0xF5

fn cpu_rst_30() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_rst_nnnn(&mut cpu, 0x0030);
} // 0xF7

fn cpu_rst_38() {
    let mut cpu = CPU.lock().unwrap();
    cpu_routine_rst_nnnn(&mut cpu, 0x0038);
} // 0xFF
