use std::ops::{Shl, Shr};

use crate::{
    bus::Bus,
    cpu::{
        Cpu,
        cpu_registers::{Reg8, Reg16},
        cpu_routines::{
            cpu_routine_adc_a_8, cpu_routine_add_a_8, cpu_routine_add_hl_16, cpu_routine_and_a_8,
            cpu_routine_call_conditional_nnnn, cpu_routine_cp_a_8, cpu_routine_dec_8,
            cpu_routine_dec_16, cpu_routine_inc_8, cpu_routine_inc_16,
            cpu_routine_jp_conditional_nnnn, cpu_routine_jr_conditional_n, cpu_routine_ld_8,
            cpu_routine_ld_16, cpu_routine_ld_ptr8, cpu_routine_ld_ptr16, cpu_routine_or_a_8,
            cpu_routine_pop_16, cpu_routine_push_16, cpu_routine_ret_conditional,
            cpu_routine_rst_nnnn, cpu_routine_sbc_a_8, cpu_routine_sub_a_8, cpu_routine_xor_a_8,
        },
    },
};

pub type Execute = fn(&mut Cpu, &mut Bus);

#[derive(Debug, Clone, Copy)]
pub struct GbCpuInstruction {
    pub dissasembly: &'static str, // name of the instruction
    pub operand_length: u8,        // n = 1 byte, nn = 2 bytes
    pub execute: Option<Execute>,  // function pointer to the instruction implementation
}

pub static INSTRUCTIONS: [GbCpuInstruction; 256] = [
    GbCpuInstruction {
        dissasembly: "NOP",
        operand_length: 0,
        execute: Some(cpu_nop),
    }, // 0x00
    GbCpuInstruction {
        dissasembly: "LD BC, nn",
        operand_length: 1,
        execute: Some(cpu_ld_bc_nn),
    }, // 0x01
    GbCpuInstruction {
        dissasembly: "LD (BC), A",
        operand_length: 0,
        execute: Some(cpu_ld_bc_a),
    }, // 0x02
    GbCpuInstruction {
        dissasembly: "INC BC",
        operand_length: 0,
        execute: Some(cpu_inc_bc),
    }, // 0x03
    GbCpuInstruction {
        dissasembly: "INC B",
        operand_length: 0,
        execute: Some(cpu_inc_b),
    }, // 0x04
    GbCpuInstruction {
        dissasembly: "DEC B",
        operand_length: 0,
        execute: Some(cpu_dec_b),
    }, // 0x05
    GbCpuInstruction {
        dissasembly: "LD B, d8",
        operand_length: 1,
        execute: Some(cpu_ld_b_n),
    }, // 0x06
    GbCpuInstruction {
        dissasembly: "RLCA",
        operand_length: 0,
        execute: Some(cpu_rlca),
    }, // 0x07
    GbCpuInstruction {
        dissasembly: "LD (a16), SP",
        operand_length: 2,
        execute: Some(cpu_ld_nn_sp),
    }, // 0x08
    GbCpuInstruction {
        dissasembly: "ADD HL, BC",
        operand_length: 0,
        execute: Some(cpu_add_hl_bc),
    }, // 0x09
    GbCpuInstruction {
        dissasembly: "LD A , (BC)",
        operand_length: 0,
        execute: Some(cpu_ld_a_bc),
    }, // 0x0A
    GbCpuInstruction {
        dissasembly: "DEC BC",
        operand_length: 0,
        execute: Some(cpu_dec_bc),
    }, // 0x0B
    GbCpuInstruction {
        dissasembly: "INC C",
        operand_length: 0,
        execute: Some(cpu_inc_c),
    }, // 0x0C
    GbCpuInstruction {
        dissasembly: "DEC C",
        operand_length: 0,
        execute: Some(cpu_dec_c),
    }, // 0x0D
    GbCpuInstruction {
        dissasembly: "LD C, d8",
        operand_length: 1,
        execute: Some(cpu_ld_c_n),
    }, // 0x0E
    GbCpuInstruction {
        dissasembly: "RRCA",
        operand_length: 0,
        execute: Some(cpu_rrca),
    }, // 0x0F
    GbCpuInstruction {
        dissasembly: "STOP",
        operand_length: 1,
        execute: Some(cpu_stop),
    }, // 0x10
    GbCpuInstruction {
        dissasembly: "LD DE, d16",
        operand_length: 2,
        execute: Some(cpu_ld_de_nn),
    }, // 0x11
    GbCpuInstruction {
        dissasembly: "LD (DE), A",
        operand_length: 0,
        execute: Some(cpu_ld_de_a),
    }, // 0x12
    GbCpuInstruction {
        dissasembly: "INC DE",
        operand_length: 0,
        execute: Some(cpu_inc_de),
    }, // 0x13
    GbCpuInstruction {
        dissasembly: "INC D",
        operand_length: 0,
        execute: Some(cpu_inc_d),
    }, // 0x14
    GbCpuInstruction {
        dissasembly: "DEC D",
        operand_length: 0,
        execute: Some(cpu_dec_d),
    }, // 0x15
    GbCpuInstruction {
        dissasembly: "LD D, d8",
        operand_length: 1,
        execute: Some(cpu_ld_d_n),
    }, // 0x16
    GbCpuInstruction {
        dissasembly: "RLA",
        operand_length: 0,
        execute: Some(cpu_rla),
    }, // 0x17
    GbCpuInstruction {
        dissasembly: "JR s8",
        operand_length: 1,
        execute: Some(cpu_jr_n),
    }, // 0x18
    GbCpuInstruction {
        dissasembly: "ADD HL, DE",
        operand_length: 0,
        execute: Some(cpu_add_hl_de),
    }, // 0x19
    GbCpuInstruction {
        dissasembly: "LD A, (DE)",
        operand_length: 0,
        execute: Some(cpu_ld_a_de),
    }, // 0x1A
    GbCpuInstruction {
        dissasembly: "DEC DE",
        operand_length: 0,
        execute: Some(cpu_dec_de),
    }, // 0x1B
    GbCpuInstruction {
        dissasembly: "INC E",
        operand_length: 0,
        execute: Some(cpu_inc_e),
    }, // 0x1C
    GbCpuInstruction {
        dissasembly: "DEC E",
        operand_length: 0,
        execute: Some(cpu_dec_e),
    }, // 0x1D
    GbCpuInstruction {
        dissasembly: "LD E, d8",
        operand_length: 1,
        execute: Some(cpu_ld_e_n),
    }, // 0x1E
    GbCpuInstruction {
        dissasembly: "RRA",
        operand_length: 0,
        execute: Some(cpu_rra),
    }, // 0x1F
    GbCpuInstruction {
        dissasembly: "JR NZ, s8",
        operand_length: 1,
        execute: Some(cpu_jr_nz_n),
    }, // 0x20
    GbCpuInstruction {
        dissasembly: "LD HL, d16",
        operand_length: 2,
        execute: Some(cpu_ld_hl_nn),
    }, // 0x21
    GbCpuInstruction {
        dissasembly: "LD (HL+), A",
        operand_length: 0,
        execute: Some(cpu_ldi_hl_a),
    }, // 0x22
    GbCpuInstruction {
        dissasembly: "INC HL",
        operand_length: 0,
        execute: Some(cpu_inc_hl),
    }, // 0x23
    GbCpuInstruction {
        dissasembly: "INC H",
        operand_length: 0,
        execute: Some(cpu_inc_h),
    }, // 0x24
    GbCpuInstruction {
        dissasembly: "DEC H",
        operand_length: 0,
        execute: Some(cpu_dec_h),
    }, // 0x25
    GbCpuInstruction {
        dissasembly: "LD H, d8",
        operand_length: 1,
        execute: Some(cpu_ld_h_n),
    }, // 0x26
    GbCpuInstruction {
        dissasembly: "DAA",
        operand_length: 0,
        execute: Some(cpu_daa),
    }, // 0x27
    GbCpuInstruction {
        dissasembly: "JR Z, s8",
        operand_length: 1,
        execute: Some(cpu_jr_z_n),
    }, // 0x28
    GbCpuInstruction {
        dissasembly: "ADD HL, HL",
        operand_length: 0,
        execute: Some(cpu_add_hl_hl),
    }, // 0x29
    GbCpuInstruction {
        dissasembly: "LDI A, (HL+)",
        operand_length: 0,
        execute: Some(cpu_ldi_a_hl),
    }, // 0x2A
    GbCpuInstruction {
        dissasembly: "DEC HL",
        operand_length: 0,
        execute: Some(cpu_dec_hl),
    }, // 0x2B
    GbCpuInstruction {
        dissasembly: "INC L",
        operand_length: 0,
        execute: Some(cpu_inc_l),
    }, // 0x2C
    GbCpuInstruction {
        dissasembly: "DEC L",
        operand_length: 0,
        execute: Some(cpu_dec_l),
    }, // 0x2D
    GbCpuInstruction {
        dissasembly: "LD L, d8",
        operand_length: 1,
        execute: Some(cpu_ld_l_n),
    }, // 0x2E
    GbCpuInstruction {
        dissasembly: "CPL",
        operand_length: 0,
        execute: Some(cpu_cpl),
    }, // 0x2F
    GbCpuInstruction {
        dissasembly: "JR NC, s8",
        operand_length: 1,
        execute: Some(cpu_jr_nc_n),
    }, // 0x30
    GbCpuInstruction {
        dissasembly: "LD SP, d16",
        operand_length: 2,
        execute: Some(cpu_ld_sp_nn),
    }, // 0x31
    GbCpuInstruction {
        dissasembly: "LD (HL-), A",
        operand_length: 0,
        execute: None,
    }, // 0x32
    GbCpuInstruction {
        dissasembly: "INC SP",
        operand_length: 0,
        execute: Some(cpu_inc_sp),
    }, // 0x33
    GbCpuInstruction {
        dissasembly: "INC (HL)",
        operand_length: 0,
        execute: Some(cpu_inc_nn_hl),
    }, // 0x34
    GbCpuInstruction {
        dissasembly: "DEC (HL)",
        operand_length: 0,
        execute: Some(cpu_dec_nn_hl),
    }, // 0x35
    GbCpuInstruction {
        dissasembly: "LD (HL), d8",
        operand_length: 1,
        execute: Some(cpu_ld_hl_n),
    }, // 0x36
    GbCpuInstruction {
        dissasembly: "SCF",
        operand_length: 0,
        execute: Some(cpu_scf),
    }, // 0x37
    GbCpuInstruction {
        dissasembly: "JR C, s8",
        operand_length: 1,
        execute: Some(cpu_jr_c_n),
    }, // 0x38
    GbCpuInstruction {
        dissasembly: "ADD HL, SP",
        operand_length: 0,
        execute: Some(cpu_add_hl_sp),
    }, // 0x39
    GbCpuInstruction {
        dissasembly: "LDD A, (HL-)",
        operand_length: 0,
        execute: Some(cpu_ldd_a_hl),
    }, // 0x3A
    GbCpuInstruction {
        dissasembly: "DEC SP",
        operand_length: 0,
        execute: Some(cpu_dec_sp),
    }, // 0x3B
    GbCpuInstruction {
        dissasembly: "INC A",
        operand_length: 0,
        execute: None,
    }, // 0x3C
    GbCpuInstruction {
        dissasembly: "DEC A",
        operand_length: 0,
        execute: None,
    }, // 0x3D
    GbCpuInstruction {
        dissasembly: "LD A, d8",
        operand_length: 1,
        execute: None,
    }, // 0x3E
    GbCpuInstruction {
        dissasembly: "CCF",
        operand_length: 0,
        execute: Some(cpu_ccf),
    }, // 0x3F
    GbCpuInstruction {
        dissasembly: "LD B, B",
        operand_length: 0,
        execute: Some(cpu_ld_b_b),
    }, // 0x40
    GbCpuInstruction {
        dissasembly: "LD B, C",
        operand_length: 0,
        execute: Some(cpu_ld_b_c),
    }, // 0x41
    GbCpuInstruction {
        dissasembly: "LD B, D",
        operand_length: 0,
        execute: Some(cpu_ld_b_d),
    }, // 0x42
    GbCpuInstruction {
        dissasembly: "LD B, E",
        operand_length: 0,
        execute: Some(cpu_ld_b_e),
    }, // 0x43
    GbCpuInstruction {
        dissasembly: "LD B, H",
        operand_length: 0,
        execute: Some(cpu_ld_b_h),
    }, // 0x44
    GbCpuInstruction {
        dissasembly: "LD B, L",
        operand_length: 0,
        execute: Some(cpu_ld_b_l),
    }, // 0x45
    GbCpuInstruction {
        dissasembly: "LD B, (HL)",
        operand_length: 0,
        execute: Some(cpu_ld_b_hl),
    }, // 0x46
    GbCpuInstruction {
        dissasembly: "LD B, A",
        operand_length: 0,
        execute: Some(cpu_ld_b_a),
    }, // 0x47
    GbCpuInstruction {
        dissasembly: "LD C, B",
        operand_length: 0,
        execute: Some(cpu_ld_c_b),
    }, // 0x48
    GbCpuInstruction {
        dissasembly: "LD C, C",
        operand_length: 0,
        execute: Some(cpu_ld_c_c),
    }, // 0x49
    GbCpuInstruction {
        dissasembly: "LD C, D",
        operand_length: 0,
        execute: Some(cpu_ld_c_d),
    }, // 0x4A
    GbCpuInstruction {
        dissasembly: "LD C, E",
        operand_length: 0,
        execute: Some(cpu_ld_c_e),
    }, // 0x4B
    GbCpuInstruction {
        dissasembly: "LD C, H",
        operand_length: 0,
        execute: Some(cpu_ld_c_h),
    }, // 0x4C
    GbCpuInstruction {
        dissasembly: "LD C, L",
        operand_length: 0,
        execute: Some(cpu_ld_c_l),
    }, // 0x4D
    GbCpuInstruction {
        dissasembly: "LD C, (HL)",
        operand_length: 0,
        execute: Some(cpu_ld_c_hl),
    }, // 0x4E
    GbCpuInstruction {
        dissasembly: "LD C, A",
        operand_length: 0,
        execute: Some(cpu_ld_c_a),
    }, // 0x4F
    GbCpuInstruction {
        dissasembly: "LD D, B",
        operand_length: 0,
        execute: Some(cpu_ld_d_b),
    }, // 0x50
    GbCpuInstruction {
        dissasembly: "LD D, C",
        operand_length: 0,
        execute: Some(cpu_ld_d_c),
    }, // 0x51
    GbCpuInstruction {
        dissasembly: "LD D, D",
        operand_length: 0,
        execute: Some(cpu_ld_d_d),
    }, // 0x52
    GbCpuInstruction {
        dissasembly: "LD D, E",
        operand_length: 0,
        execute: Some(cpu_ld_d_e),
    }, // 0x53
    GbCpuInstruction {
        dissasembly: "LD D, H",
        operand_length: 0,
        execute: Some(cpu_ld_d_h),
    }, // 0x54
    GbCpuInstruction {
        dissasembly: "LD D, L",
        operand_length: 0,
        execute: Some(cpu_ld_d_l),
    }, // 0x55
    GbCpuInstruction {
        dissasembly: "LD D, (HL)",
        operand_length: 0,
        execute: Some(cpu_ld_d_hl),
    }, // 0x56
    GbCpuInstruction {
        dissasembly: "LD D, A",
        operand_length: 0,
        execute: Some(cpu_ld_d_a),
    }, // 0x57
    GbCpuInstruction {
        dissasembly: "LD E, B",
        operand_length: 0,
        execute: Some(cpu_ld_e_b),
    }, // 0x58
    GbCpuInstruction {
        dissasembly: "LD E, C",
        operand_length: 0,
        execute: Some(cpu_ld_e_c),
    }, // 0x59
    GbCpuInstruction {
        dissasembly: "LD E, D",
        operand_length: 0,
        execute: Some(cpu_ld_e_d),
    }, // 0x5A
    GbCpuInstruction {
        dissasembly: "LD E, E",
        operand_length: 0,
        execute: Some(cpu_ld_e_e),
    }, // 0x5B
    GbCpuInstruction {
        dissasembly: "LD E, H",
        operand_length: 0,
        execute: Some(cpu_ld_e_h),
    }, // 0x5C
    GbCpuInstruction {
        dissasembly: "LD E, L",
        operand_length: 0,
        execute: Some(cpu_ld_e_l),
    }, // 0x5D
    GbCpuInstruction {
        dissasembly: "LD E, (HL)",
        operand_length: 0,
        execute: Some(cpu_ld_e_hl),
    }, // 0x5E
    GbCpuInstruction {
        dissasembly: "LD E, A",
        operand_length: 0,
        execute: Some(cpu_ld_e_a),
    }, // 0x5F
    GbCpuInstruction {
        dissasembly: "LD H, B",
        operand_length: 0,
        execute: Some(cpu_ld_h_b),
    }, // 0x60
    GbCpuInstruction {
        dissasembly: "LD H, C",
        operand_length: 0,
        execute: Some(cpu_ld_h_c),
    }, // 0x61
    GbCpuInstruction {
        dissasembly: "LD H, D",
        operand_length: 0,
        execute: Some(cpu_ld_h_d),
    }, // 0x62
    GbCpuInstruction {
        dissasembly: "LD H, E",
        operand_length: 0,
        execute: Some(cpu_ld_h_e),
    }, // 0x63
    GbCpuInstruction {
        dissasembly: "LD H, H",
        operand_length: 0,
        execute: Some(cpu_ld_h_h),
    }, // 0x64
    GbCpuInstruction {
        dissasembly: "LD H, L",
        operand_length: 0,
        execute: Some(cpu_ld_h_l),
    }, // 0x65
    GbCpuInstruction {
        dissasembly: "LD H, (HL)",
        operand_length: 0,
        execute: Some(cpu_ld_h_hl),
    }, // 0x66
    GbCpuInstruction {
        dissasembly: "LD H, A",
        operand_length: 0,
        execute: Some(cpu_ld_h_a),
    }, // 0x67
    GbCpuInstruction {
        dissasembly: "LD L, B",
        operand_length: 0,
        execute: Some(cpu_ld_l_b),
    }, // 0x68
    GbCpuInstruction {
        dissasembly: "LD L, C",
        operand_length: 0,
        execute: Some(cpu_ld_l_c),
    }, // 0x69
    GbCpuInstruction {
        dissasembly: "LD L, D",
        operand_length: 0,
        execute: Some(cpu_ld_l_d),
    }, // 0x6A
    GbCpuInstruction {
        dissasembly: "LD L, E",
        operand_length: 0,
        execute: Some(cpu_ld_l_e),
    }, // 0x6B
    GbCpuInstruction {
        dissasembly: "LD L, H",
        operand_length: 0,
        execute: Some(cpu_ld_l_h),
    }, // 0x6C
    GbCpuInstruction {
        dissasembly: "LD L, L",
        operand_length: 0,
        execute: Some(cpu_ld_l_l),
    }, // 0x6D
    GbCpuInstruction {
        dissasembly: "LD L, (HL)",
        operand_length: 0,
        execute: Some(cpu_ld_l_hl),
    }, // 0x6E
    GbCpuInstruction {
        dissasembly: "LD L, A",
        operand_length: 0,
        execute: Some(cpu_ld_l_a),
    }, // 0x6F
    GbCpuInstruction {
        dissasembly: "LD (HL), B",
        operand_length: 0,
        execute: Some(cpu_ld_hl_b),
    }, // 0x70
    GbCpuInstruction {
        dissasembly: "LD (HL), C",
        operand_length: 0,
        execute: Some(cpu_ld_hl_c),
    }, // 0x71
    GbCpuInstruction {
        dissasembly: "LD (HL), D",
        operand_length: 0,
        execute: Some(cpu_ld_hl_d),
    }, // 0x72
    GbCpuInstruction {
        dissasembly: "LD (HL), E",
        operand_length: 0,
        execute: Some(cpu_ld_hl_e),
    }, // 0x73
    GbCpuInstruction {
        dissasembly: "LD (HL), H",
        operand_length: 0,
        execute: Some(cpu_ld_hl_h),
    }, // 0x74
    GbCpuInstruction {
        dissasembly: "LD (HL), L",
        operand_length: 0,
        execute: Some(cpu_ld_hl_l),
    }, // 0x75
    GbCpuInstruction {
        dissasembly: "HALT",
        operand_length: 0,
        execute: None,
    }, // 0x76
    GbCpuInstruction {
        dissasembly: "LD (HL), A",
        operand_length: 0,
        execute: Some(cpu_ld_hl_a),
    }, // 0x77
    GbCpuInstruction {
        dissasembly: "LD A, B",
        operand_length: 0,
        execute: Some(cpu_ld_a_b),
    }, // 0x78
    GbCpuInstruction {
        dissasembly: "LD A, C",
        operand_length: 0,
        execute: Some(cpu_ld_a_c),
    }, // 0x79
    GbCpuInstruction {
        dissasembly: "LD A, D",
        operand_length: 0,
        execute: Some(cpu_ld_a_d),
    }, // 0x7A
    GbCpuInstruction {
        dissasembly: "LD A, E",
        operand_length: 0,
        execute: Some(cpu_ld_a_e),
    }, // 0x7B
    GbCpuInstruction {
        dissasembly: "LD A, H",
        operand_length: 0,
        execute: Some(cpu_ld_a_h),
    }, // 0x7C
    GbCpuInstruction {
        dissasembly: "LD A, L",
        operand_length: 0,
        execute: Some(cpu_ld_a_l),
    }, // 0x7D
    GbCpuInstruction {
        dissasembly: "LD A, (HL)",
        operand_length: 0,
        execute: Some(cpu_ld_a_hl),
    }, // 0x7E
    GbCpuInstruction {
        dissasembly: "LD A, A",
        operand_length: 0,
        execute: Some(cpu_ld_a_a),
    }, // 0x7F
    GbCpuInstruction {
        dissasembly: "ADD A, B",
        operand_length: 0,
        execute: Some(cpu_add_a_b),
    }, // 0x80
    GbCpuInstruction {
        dissasembly: "ADD A, C",
        operand_length: 0,
        execute: Some(cpu_add_a_c),
    }, // 0x81
    GbCpuInstruction {
        dissasembly: "ADD A, D",
        operand_length: 0,
        execute: Some(cpu_add_a_d),
    }, // 0x82
    GbCpuInstruction {
        dissasembly: "ADD A, E",
        operand_length: 0,
        execute: Some(cpu_add_a_e),
    }, // 0x83
    GbCpuInstruction {
        dissasembly: "ADD A, H",
        operand_length: 0,
        execute: Some(cpu_add_a_h),
    }, // 0x84
    GbCpuInstruction {
        dissasembly: "ADD A, L",
        operand_length: 0,
        execute: Some(cpu_add_a_l),
    }, // 0x85
    GbCpuInstruction {
        dissasembly: "ADD A, (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x86
    GbCpuInstruction {
        dissasembly: "ADD A, A",
        operand_length: 0,
        execute: None,
    }, // 0x87
    GbCpuInstruction {
        dissasembly: "ADC A, B",
        operand_length: 0,
        execute: Some(cpu_adc_a_b),
    }, // 0x88
    GbCpuInstruction {
        dissasembly: "ADC A, C",
        operand_length: 0,
        execute: Some(cpu_adc_a_c),
    }, // 0x89
    GbCpuInstruction {
        dissasembly: "ADC A, D",
        operand_length: 0,
        execute: Some(cpu_adc_a_d),
    }, // 0x8A
    GbCpuInstruction {
        dissasembly: "ADC A, E",
        operand_length: 0,
        execute: Some(cpu_adc_a_e),
    }, // 0x8B
    GbCpuInstruction {
        dissasembly: "ADC A, H",
        operand_length: 0,
        execute: Some(cpu_adc_a_h),
    }, // 0x8C
    GbCpuInstruction {
        dissasembly: "ADC A, L",
        operand_length: 0,
        execute: Some(cpu_adc_a_l),
    }, // 0x8D
    GbCpuInstruction {
        dissasembly: "ADC A, (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x8E
    GbCpuInstruction {
        dissasembly: "ADC A, A",
        operand_length: 0,
        execute: None,
    }, // 0x8F
    GbCpuInstruction {
        dissasembly: "SUB B",
        operand_length: 0,
        execute: Some(cpu_sub_a_b),
    }, // 0x90
    GbCpuInstruction {
        dissasembly: "SUB C",
        operand_length: 0,
        execute: Some(cpu_sub_a_c),
    }, // 0x91
    GbCpuInstruction {
        dissasembly: "SUB D",
        operand_length: 0,
        execute: Some(cpu_sub_a_d),
    }, // 0x92
    GbCpuInstruction {
        dissasembly: "SUB E",
        operand_length: 0,
        execute: Some(cpu_sub_a_e),
    }, // 0x93
    GbCpuInstruction {
        dissasembly: "SUB H",
        operand_length: 0,
        execute: Some(cpu_sub_a_h),
    }, // 0x94
    GbCpuInstruction {
        dissasembly: "SUB L",
        operand_length: 0,
        execute: Some(cpu_sub_a_l),
    }, // 0x95
    GbCpuInstruction {
        dissasembly: "SUB (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x96
    GbCpuInstruction {
        dissasembly: "SUB A",
        operand_length: 0,
        execute: None,
    }, // 0x97
    GbCpuInstruction {
        dissasembly: "SBC A, B",
        operand_length: 0,
        execute: Some(cpu_sbc_a_b),
    }, // 0x98
    GbCpuInstruction {
        dissasembly: "SBC A, C",
        operand_length: 0,
        execute: Some(cpu_sbc_a_c),
    }, // 0x99
    GbCpuInstruction {
        dissasembly: "SBC A, D",
        operand_length: 0,
        execute: Some(cpu_sbc_a_d),
    }, // 0x9A
    GbCpuInstruction {
        dissasembly: "SBC A, E",
        operand_length: 0,
        execute: Some(cpu_sbc_a_e),
    }, // 0x9B
    GbCpuInstruction {
        dissasembly: "SBC A, H",
        operand_length: 0,
        execute: Some(cpu_sbc_a_h),
    }, // 0x9C
    GbCpuInstruction {
        dissasembly: "SBC A, L",
        operand_length: 0,
        execute: Some(cpu_sbc_a_l),
    }, // 0x9D
    GbCpuInstruction {
        dissasembly: "SBC A, (HL)",
        operand_length: 0,
        execute: None,
    }, // 0x9E
    GbCpuInstruction {
        dissasembly: "SBC A, A",
        operand_length: 0,
        execute: None,
    }, // 0x9F
    GbCpuInstruction {
        dissasembly: "AND B",
        operand_length: 0,
        execute: Some(cpu_and_a_b),
    }, // 0xA0
    GbCpuInstruction {
        dissasembly: "AND C",
        operand_length: 0,
        execute: Some(cpu_and_a_c),
    }, // 0xA1
    GbCpuInstruction {
        dissasembly: "AND D",
        operand_length: 0,
        execute: Some(cpu_and_a_d),
    }, // 0xA2
    GbCpuInstruction {
        dissasembly: "AND E",
        operand_length: 0,
        execute: Some(cpu_and_a_e),
    }, // 0xA3
    GbCpuInstruction {
        dissasembly: "AND H",
        operand_length: 0,
        execute: Some(cpu_and_a_h),
    }, // 0xA4
    GbCpuInstruction {
        dissasembly: "AND L",
        operand_length: 0,
        execute: Some(cpu_and_a_l),
    }, // 0xA5
    GbCpuInstruction {
        dissasembly: "AND (HL)",
        operand_length: 0,
        execute: None,
    }, // 0xA6
    GbCpuInstruction {
        dissasembly: "AND A",
        operand_length: 0,
        execute: None,
    }, // 0xA7
    GbCpuInstruction {
        dissasembly: "XOR B",
        operand_length: 0,
        execute: Some(cpu_xor_a_b),
    }, // 0xA8
    GbCpuInstruction {
        dissasembly: "XOR C",
        operand_length: 0,
        execute: Some(cpu_xor_a_c),
    }, // 0xA9
    GbCpuInstruction {
        dissasembly: "XOR D",
        operand_length: 0,
        execute: Some(cpu_xor_a_d),
    }, // 0xAA
    GbCpuInstruction {
        dissasembly: "XOR E",
        operand_length: 0,
        execute: Some(cpu_xor_a_e),
    }, // 0xAB
    GbCpuInstruction {
        dissasembly: "XOR H",
        operand_length: 0,
        execute: Some(cpu_xor_a_h),
    }, // 0xAC
    GbCpuInstruction {
        dissasembly: "XOR L",
        operand_length: 0,
        execute: Some(cpu_xor_a_l),
    }, // 0xAD
    GbCpuInstruction {
        dissasembly: "XOR (HL)",
        operand_length: 0,
        execute: None,
    }, // 0xAE
    GbCpuInstruction {
        dissasembly: "XOR A",
        operand_length: 0,
        execute: None,
    }, // 0xAF
    GbCpuInstruction {
        dissasembly: "OR B",
        operand_length: 0,
        execute: Some(cpu_or_a_b),
    }, // 0xB0
    GbCpuInstruction {
        dissasembly: "OR C",
        operand_length: 0,
        execute: Some(cpu_or_a_c),
    }, // 0xB1
    GbCpuInstruction {
        dissasembly: "OR D",
        operand_length: 0,
        execute: Some(cpu_or_a_d),
    }, // 0xB2
    GbCpuInstruction {
        dissasembly: "OR E",
        operand_length: 0,
        execute: Some(cpu_or_a_e),
    }, // 0xB3
    GbCpuInstruction {
        dissasembly: "OR H",
        operand_length: 0,
        execute: Some(cpu_or_a_h),
    }, // 0xB4
    GbCpuInstruction {
        dissasembly: "OR L",
        operand_length: 0,
        execute: Some(cpu_or_a_l),
    }, // 0xB5
    GbCpuInstruction {
        dissasembly: "OR (HL)",
        operand_length: 0,
        execute: None,
    }, // 0xB6
    GbCpuInstruction {
        dissasembly: "OR A",
        operand_length: 0,
        execute: None,
    }, // 0xB7
    GbCpuInstruction {
        dissasembly: "CP B",
        operand_length: 0,
        execute: Some(cpu_cp_a_b),
    }, // 0xB8
    GbCpuInstruction {
        dissasembly: "CP C",
        operand_length: 0,
        execute: Some(cpu_cp_a_c),
    }, // 0xB9
    GbCpuInstruction {
        dissasembly: "CP D",
        operand_length: 0,
        execute: Some(cpu_cp_a_d),
    }, // 0xBA
    GbCpuInstruction {
        dissasembly: "CP E",
        operand_length: 0,
        execute: Some(cpu_cp_a_e),
    }, // 0xBB
    GbCpuInstruction {
        dissasembly: "CP H",
        operand_length: 0,
        execute: Some(cpu_cp_a_h),
    }, // 0xBC
    GbCpuInstruction {
        dissasembly: "CP L",
        operand_length: 0,
        execute: Some(cpu_cp_a_l),
    }, // 0xBD
    GbCpuInstruction {
        dissasembly: "CP (HL)",
        operand_length: 0,
        execute: None,
    }, // 0xBE
    GbCpuInstruction {
        dissasembly: "CP A",
        operand_length: 0,
        execute: None,
    }, // 0xBF
    GbCpuInstruction {
        dissasembly: "RET NZ",
        operand_length: 0,
        execute: Some(cpu_ret_nz),
    }, // 0xC0
    GbCpuInstruction {
        dissasembly: "POP BC",
        operand_length: 0,
        execute: Some(cpu_pop_bc),
    }, // 0xC1
    GbCpuInstruction {
        dissasembly: "JP NZ, a16",
        operand_length: 2,
        execute: Some(cpu_jp_nz_nn),
    }, // 0xC2
    GbCpuInstruction {
        dissasembly: "JP a16",
        operand_length: 2,
        execute: Some(cpu_jp_nn),
    }, // 0xC3
    GbCpuInstruction {
        dissasembly: "CALL NZ, a16",
        operand_length: 2,
        execute: Some(cpu_call_nz_nn),
    }, // 0xC4
    GbCpuInstruction {
        dissasembly: "PUSH BC",
        operand_length: 0,
        execute: Some(cpu_push_bc),
    }, // 0xC5
    GbCpuInstruction {
        dissasembly: "ADD A, a8",
        operand_length: 1,
        execute: None,
    }, // 0xC6
    GbCpuInstruction {
        dissasembly: "RST 0",
        operand_length: 0,
        execute: Some(cpu_rst_00),
    }, // 0xC7
    GbCpuInstruction {
        dissasembly: "RET Z",
        operand_length: 0,
        execute: Some(cpu_ret_z),
    }, // 0xC8
    GbCpuInstruction {
        dissasembly: "RET",
        operand_length: 0,
        execute: None,
    }, // 0xC9
    GbCpuInstruction {
        dissasembly: "JP Z, a16",
        operand_length: 2,
        execute: Some(cpu_jp_z_nn),
    }, // 0xCA
    GbCpuInstruction {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xCB
    GbCpuInstruction {
        dissasembly: "CALL Z, a16",
        operand_length: 2,
        execute: Some(cpu_call_z_nn),
    }, // 0xCC
    GbCpuInstruction {
        dissasembly: "CALL a16",
        operand_length: 2,
        execute: None,
    }, // 0xCD
    GbCpuInstruction {
        dissasembly: "ADC A, d8",
        operand_length: 1,
        execute: None,
    }, // 0xCE
    GbCpuInstruction {
        dissasembly: "RST 1",
        operand_length: 0,
        execute: Some(cpu_rst_08),
    }, // 0xCF
    GbCpuInstruction {
        dissasembly: "RET NC",
        operand_length: 0,
        execute: Some(cpu_ret_nc),
    }, // 0xD0
    GbCpuInstruction {
        dissasembly: "POP DE",
        operand_length: 0,
        execute: Some(cpu_pop_de),
    }, // 0xD1
    GbCpuInstruction {
        dissasembly: "JP NC, a16",
        operand_length: 2,
        execute: Some(cpu_jp_nc_nn),
    }, // 0xD2
    GbCpuInstruction {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xD3
    GbCpuInstruction {
        dissasembly: "CALL NC, a16",
        operand_length: 2,
        execute: Some(cpu_call_nc_nn),
    }, // 0xD4
    GbCpuInstruction {
        dissasembly: "PUSH DE",
        operand_length: 0,
        execute: Some(cpu_push_de),
    }, // 0xD5
    GbCpuInstruction {
        dissasembly: "SUB d8",
        operand_length: 1,
        execute: None,
    }, // 0xD6
    GbCpuInstruction {
        dissasembly: "RST 2",
        operand_length: 0,
        execute: Some(cpu_rst_10),
    }, // 0xD7
    GbCpuInstruction {
        dissasembly: "RET C",
        operand_length: 0,
        execute: Some(cpu_ret_c),
    }, // 0xD8
    GbCpuInstruction {
        dissasembly: "RETI",
        operand_length: 0,
        execute: None,
    }, // 0xD9
    GbCpuInstruction {
        dissasembly: "JP C, a16",
        operand_length: 2,
        execute: Some(cpu_jp_c_nn),
    }, // 0xDA
    GbCpuInstruction {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xDB
    GbCpuInstruction {
        dissasembly: "CALL C, a16",
        operand_length: 2,
        execute: Some(cpu_call_c_nn),
    }, // 0xDC
    GbCpuInstruction {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xDD
    GbCpuInstruction {
        dissasembly: "SBC A, d8",
        operand_length: 1,
        execute: None,
    }, // 0xDE
    GbCpuInstruction {
        dissasembly: "RST 3",
        operand_length: 0,
        execute: Some(cpu_rst_18),
    }, // 0xDF
    GbCpuInstruction {
        dissasembly: "LD (a8), A",
        operand_length: 1,
        execute: None,
    }, // 0xE0
    GbCpuInstruction {
        dissasembly: "POP HL",
        operand_length: 0,
        execute: Some(cpu_pop_hl),
    }, // 0xE1
    GbCpuInstruction {
        dissasembly: "LD (C), A",
        operand_length: 0,
        execute: None,
    }, // 0xE2
    GbCpuInstruction {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xE3
    GbCpuInstruction {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xE4
    GbCpuInstruction {
        dissasembly: "PUSH HL",
        operand_length: 0,
        execute: Some(cpu_push_hl),
    }, // 0xE5
    GbCpuInstruction {
        dissasembly: "AND d8",
        operand_length: 1,
        execute: None,
    }, // 0xE6
    GbCpuInstruction {
        dissasembly: "RST 4",
        operand_length: 0,
        execute: Some(cpu_rst_20),
    }, // 0xE7
    GbCpuInstruction {
        dissasembly: "ADD Sp, s8",
        operand_length: 1,
        execute: None,
    }, // 0xE8
    GbCpuInstruction {
        dissasembly: "JP HL",
        operand_length: 0,
        execute: None,
    }, // 0xE9
    GbCpuInstruction {
        dissasembly: "LD (a16), A",
        operand_length: 2,
        execute: None,
    }, // 0xEA
    GbCpuInstruction {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xEB
    GbCpuInstruction {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xEC
    GbCpuInstruction {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xED
    GbCpuInstruction {
        dissasembly: "XOR d8",
        operand_length: 1,
        execute: None,
    }, // 0xEE
    GbCpuInstruction {
        dissasembly: "RST 5",
        operand_length: 0,
        execute: Some(cpu_rst_28),
    }, // 0xEF
    GbCpuInstruction {
        dissasembly: "LD A, (a8)",
        operand_length: 1,
        execute: None,
    }, // 0xF0
    GbCpuInstruction {
        dissasembly: "POP AF",
        operand_length: 0,
        execute: Some(cpu_pop_af),
    }, // 0xF1
    GbCpuInstruction {
        dissasembly: "LD A, (C)",
        operand_length: 0,
        execute: None,
    }, // 0xF2
    GbCpuInstruction {
        dissasembly: "DI",
        operand_length: 0,
        execute: None,
    }, // 0xF3
    GbCpuInstruction {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xF4
    GbCpuInstruction {
        dissasembly: "PUSH AF",
        operand_length: 0,
        execute: Some(cpu_push_af),
    }, // 0xF5
    GbCpuInstruction {
        dissasembly: "OR d8",
        operand_length: 1,
        execute: None,
    }, // 0xF6
    GbCpuInstruction {
        dissasembly: "RST 6",
        operand_length: 0,
        execute: Some(cpu_rst_30),
    }, // 0xF7
    GbCpuInstruction {
        dissasembly: "LD HL, SP+s8",
        operand_length: 1,
        execute: None,
    }, // 0xF8
    GbCpuInstruction {
        dissasembly: "LD SP, HL",
        operand_length: 0,
        execute: None,
    }, // 0xF9
    GbCpuInstruction {
        dissasembly: "LD A, (a16)",
        operand_length: 2,
        execute: None,
    }, // 0xFA
    GbCpuInstruction {
        dissasembly: "EI",
        operand_length: 0,
        execute: None,
    }, // 0xFB
    GbCpuInstruction {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xFC
    GbCpuInstruction {
        dissasembly: "???",
        operand_length: 0,
        execute: None,
    }, // 0xFD
    GbCpuInstruction {
        dissasembly: "CP d8",
        operand_length: 1,
        execute: None,
    }, // 0xFE
    GbCpuInstruction {
        dissasembly: "RST 7",
        operand_length: 0,
        execute: Some(cpu_rst_38),
    }, // 0xFF
];

fn cpu_nop(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x00

fn cpu_ld_bc_nn(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_16(&mut cpu, &mut bus, Reg16::BC);
} // 0x01

fn cpu_ld_bc_a(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr8(&mut cpu, &mut bus, Reg16::BC, Reg8::A);
} // 0x02

fn cpu_inc_bc(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_inc_16(&mut cpu, &mut bus, Reg16::BC);
} // 0x03

fn cpu_inc_b(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_inc_8(&mut cpu, &mut bus, Reg8::B);
} // 0x04

fn cpu_dec_b(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_dec_8(&mut cpu, &mut bus, Reg8::B);
} // 0x05

fn cpu_ld_b_n(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_8(&mut cpu, &mut bus, Reg8::B);
} // 0x06

fn cpu_rlca(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);

    let temp = cpu.registers.a;
    cpu.registers.set_carry(temp & 0x80 != 0);
    cpu.registers.set_half_carry(false);
    cpu.registers.set_zero(false);
    cpu.registers.set_subtract(false);

    cpu.registers.a = cpu.registers.a.rotate_left(1);
} // 0x07

fn cpu_ld_nn_sp(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    let mut temp = bus.memory.memory_bus_read(cpu.registers.pc as usize);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    temp |= bus.memory.memory_bus_read(cpu.registers.pc as usize).shl(8);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    bus.memory
        .memory_bus_write(temp as usize, (cpu.registers.sp & 0xFF) as u8);
    temp = temp.wrapping_add(1);
    bus.memory
        .memory_bus_write(temp as usize, (cpu.registers.sp & 0xFF00) as u8);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x08

fn cpu_add_hl_bc(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_add_hl_16(&mut cpu, &mut bus, Reg16::BC);
} // 0x09

fn cpu_ld_a_bc(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr16(&mut cpu, &mut bus, Reg8::A, Reg16::BC);
} // 0x0A

fn cpu_dec_bc(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_dec_16(&mut cpu, &mut bus, Reg16::BC);
} // 0x0B

fn cpu_inc_c(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_inc_8(&mut cpu, &mut bus, Reg8::C);
} // 0x0C

fn cpu_dec_c(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_dec_8(&mut cpu, &mut bus, Reg8::C);
} // 0x0D

fn cpu_ld_c_n(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_8(&mut cpu, &mut bus, Reg8::C);
} // 0x0E

fn cpu_rrca(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);

    let temp = cpu.registers.a;
    cpu.registers.set_carry(temp & 0x01 != 0);
    cpu.registers.set_half_carry(false);
    cpu.registers.set_zero(false);
    cpu.registers.set_subtract(false);

    cpu.registers.a = cpu.registers.a.rotate_right(1);
} // 0x0F

fn cpu_stop(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    if bus.memory.memory_bus_read(cpu.registers.pc as usize) != 0 {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
        println!(
            "CPU: Corrupted STOP at PC: {:0x}, should be 0x00",
            cpu.registers.pc
        )
    }

    bus.timer.div_write(0);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.stopped = true;
} // 0x10

fn cpu_ld_de_nn(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_16(&mut cpu, &mut bus, Reg16::DE);
} // 0x11

fn cpu_ld_de_a(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr8(&mut cpu, &mut bus, Reg16::DE, Reg8::A);
} // 0x12

fn cpu_inc_de(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_inc_16(&mut cpu, &mut bus, Reg16::DE);
} // 0x13

fn cpu_inc_d(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_inc_8(&mut cpu, &mut bus, Reg8::D);
} // 0x14

fn cpu_dec_d(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_dec_8(&mut cpu, &mut bus, Reg8::D);
} // 0x15

fn cpu_ld_d_n(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_8(&mut cpu, &mut bus, Reg8::D);
} // 0x16

fn cpu_rla(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);

    cpu.registers.set_half_carry(false);
    cpu.registers.set_zero(false);
    cpu.registers.set_subtract(false);
    let temp = cpu.registers.a;
    let carry = cpu.registers.carry();
    cpu.registers.set_carry(temp & 0x80 != 0);

    cpu.registers.a = (cpu.registers.a << 1) | if carry { 1 } else { 0 };
} // 0x17

fn cpu_jr_n(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);

    let temp = bus.memory.memory_bus_read(usize::from(cpu.registers.pc));
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);

    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.pc += u16::from(temp);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x18

fn cpu_add_hl_de(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_add_hl_16(&mut cpu, &mut bus, Reg16::DE);
} // 0x19

fn cpu_ld_a_de(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr16(&mut cpu, &mut bus, Reg8::A, Reg16::DE);
} // 0x1A

fn cpu_dec_de(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_dec_16(&mut cpu, &mut bus, Reg16::DE);
} // 0x1B

fn cpu_inc_e(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_inc_8(&mut cpu, &mut bus, Reg8::E);
} // 0x1C

fn cpu_dec_e(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_dec_8(&mut cpu, &mut bus, Reg8::E);
} // 0x1D

fn cpu_ld_e_n(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_8(&mut cpu, &mut bus, Reg8::E);
} // 0x1E

fn cpu_rra(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);

    cpu.registers.set_half_carry(false);
    cpu.registers.set_zero(false);
    cpu.registers.set_subtract(false);
    let temp = cpu.registers.a;
    cpu.registers.set_carry(cpu.registers.a & 0x01 != 0);

    cpu.registers.a = cpu.registers.a.shr(1) | temp.shl(7);
} // 0x1F

fn cpu_jr_nz_n(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.zero() == false;
    cpu_routine_jr_conditional_n(&mut cpu, &mut bus, condition);
} // 0x20

fn cpu_ld_hl_nn(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_16(&mut cpu, &mut bus, Reg16::HL);
} // 0x21

fn cpu_ldi_hl_a(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    bus.memory
        .memory_bus_write(cpu.registers.hl() as usize, cpu.registers.a);
    let temp = cpu.registers.hl().wrapping_add(1);
    cpu.registers.set_hl(temp);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x22

fn cpu_inc_hl(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_inc_16(&mut cpu, &mut bus, Reg16::HL);
} // 0x23

fn cpu_inc_h(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_inc_8(&mut cpu, &mut bus, Reg8::H);
} // 0x24

fn cpu_dec_h(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_dec_8(&mut cpu, &mut bus, Reg8::H);
} // 0x25

fn cpu_ld_h_n(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_8(&mut cpu, &mut bus, Reg8::H);
} // 0x26

fn cpu_daa(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);

    if !cpu.registers.subtract() {
        if cpu.registers.carry() || (cpu.registers.a > 0x99) {
            cpu.registers.a += 0x60;
            cpu.registers.set_carry(true);
        }
        if cpu.registers.half_carry() || ((cpu.registers.a & 0x0F) > 0x09) {
            cpu.registers.a += 0x6;
        }
    } else {
        if cpu.registers.carry() {
            cpu.registers.a -= 0x60;
        }
        if cpu.registers.half_carry() {
            cpu.registers.a -= 0x6;
        }
    }
    cpu.registers.set_zero(cpu.registers.a == 0);
    cpu.registers.set_half_carry(false);
} // 0x27

fn cpu_jr_z_n(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.zero() == true;
    cpu_routine_jr_conditional_n(&mut cpu, &mut bus, condition);
} // 0x28

fn cpu_add_hl_hl(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.set_subtract(false);
    cpu.registers.set_carry((cpu.registers.hl() & 0xB000) != 0);
    cpu.registers
        .set_half_carry((cpu.registers.hl() & 0x0800) != 0);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.set_hl(cpu.registers.hl().shl(1));
} // 0x29

fn cpu_ldi_a_hl(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.a = bus.memory.memory_bus_read(cpu.registers.hl() as usize);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.set_hl(cpu.registers.hl() + 1);
} // 0x2A

fn cpu_dec_hl(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_dec_16(&mut cpu, &mut bus, Reg16::HL);
} // 0x2B

fn cpu_inc_l(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_inc_8(&mut cpu, &mut bus, Reg8::L);
} // 0x2C

fn cpu_dec_l(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_dec_8(&mut cpu, &mut bus, Reg8::L);
} // 0x2D

fn cpu_ld_l_n(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_8(&mut cpu, &mut bus, Reg8::L);
} // 0x2E

fn cpu_cpl(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.set_subtract(true);
    cpu.registers.set_half_carry(true);
    cpu.registers.a = !cpu.registers.a;
} // 0x2F

fn cpu_jr_nc_n(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.carry() == false;
    cpu_routine_jr_conditional_n(&mut cpu, &mut bus, condition);
} // 0x30

fn cpu_ld_sp_nn(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_16(&mut cpu, &mut bus, Reg16::SP);
} // 0x31

fn cpu_inc_sp(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_inc_16(&mut cpu, &mut bus, Reg16::SP);
} // 0x33

fn cpu_inc_nn_hl(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    let mut temp = bus.memory.memory_bus_read(cpu.registers.hl() as usize);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.set_subtract(false);
    cpu.registers.set_half_carry(temp & 0x0F == 0);
    temp = temp.wrapping_add(1);
    cpu.registers.set_zero(temp == 0);
    bus.memory
        .memory_bus_write(cpu.registers.hl() as usize, temp);
} // 0x34

fn cpu_dec_nn_hl(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    let mut temp = bus.memory.memory_bus_read(cpu.registers.hl() as usize);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.set_subtract(false);
    cpu.registers.set_half_carry(temp & 0x0F == 0);
    temp = temp.wrapping_sub(1);
    cpu.registers.set_zero(temp == 0);
    bus.memory
        .memory_bus_write(cpu.registers.hl() as usize, temp);
} // 0x35

fn cpu_ld_hl_n(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    let temp = bus.memory.memory_bus_read(cpu.registers.pc as usize);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    bus.memory
        .memory_bus_write(cpu.registers.hl() as usize, temp);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x36

fn cpu_scf(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.registers.set_subtract(false);
    cpu.registers.set_half_carry(false);
    cpu.registers.set_carry(true);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x37

fn cpu_jr_c_n(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.carry() == true;
    cpu_routine_jr_conditional_n(&mut cpu, &mut bus, condition);
} // 0x38

fn cpu_add_hl_sp(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_add_hl_16(&mut cpu, &mut bus, Reg16::SP);
} // 0x39

fn cpu_ldd_a_hl(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.a = bus.memory.memory_bus_read(cpu.registers.hl() as usize);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.set_hl(cpu.registers.hl() - 1);
} // 0x3A

fn cpu_dec_sp(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_dec_16(&mut cpu, &mut bus, Reg16::SP);
} // 0x3B

fn cpu_ccf(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.set_subtract(false);
    cpu.registers.set_half_carry(false);
    cpu.registers.set_carry(!cpu.registers.carry());
} // 0x3B

fn cpu_ld_b_b(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x40

fn cpu_ld_b_c(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.b = cpu.registers.c;
} // 0x41

fn cpu_ld_b_d(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.b = cpu.registers.d;
} // 0x42

fn cpu_ld_b_e(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.b = cpu.registers.e;
} // 0x43

fn cpu_ld_b_h(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.b = cpu.registers.h;
} // 0x44

fn cpu_ld_b_l(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.b = cpu.registers.l;
} // 0x45

fn cpu_ld_b_hl(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr16(&mut cpu, &mut bus, Reg8::B, Reg16::HL);
} // 0x46

fn cpu_ld_b_a(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.b = cpu.registers.a;
} // 0x47

fn cpu_ld_c_b(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.c = cpu.registers.b;
} // 0x48

fn cpu_ld_c_c(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x49

fn cpu_ld_c_d(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.c = cpu.registers.d;
} // 0x4A

fn cpu_ld_c_e(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.c = cpu.registers.e;
} // 0x4B

fn cpu_ld_c_h(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.c = cpu.registers.h;
} // 0x4C

fn cpu_ld_c_l(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.c = cpu.registers.l;
} // 0x4D

fn cpu_ld_c_hl(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr16(&mut cpu, &mut bus, Reg8::C, Reg16::HL);
} // 0x4E

fn cpu_ld_c_a(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.c = cpu.registers.a;
} // 0x4F

fn cpu_ld_d_b(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x50

fn cpu_ld_d_c(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.d = cpu.registers.c;
} // 0x51

fn cpu_ld_d_d(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x52

fn cpu_ld_d_e(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.d = cpu.registers.e;
} // 0x53

fn cpu_ld_d_h(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.d = cpu.registers.h;
} // 0x54

fn cpu_ld_d_l(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.d = cpu.registers.l;
} // 0x55

fn cpu_ld_d_hl(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr16(&mut cpu, &mut bus, Reg8::D, Reg16::HL);
} // 0x56

fn cpu_ld_d_a(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.d = cpu.registers.a;
} // 0x57

fn cpu_ld_e_b(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.e = cpu.registers.b;
} // 0x58

fn cpu_ld_e_c(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.e = cpu.registers.c;
} // 0x59

fn cpu_ld_e_d(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.e = cpu.registers.d;
} // 0x5A

fn cpu_ld_e_e(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x5B

fn cpu_ld_e_h(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.e = cpu.registers.h;
} // 0x5C

fn cpu_ld_e_l(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.e = cpu.registers.l;
} // 0x5D

fn cpu_ld_e_hl(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr16(&mut cpu, &mut bus, Reg8::E, Reg16::HL);
} // 0x5E

fn cpu_ld_e_a(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.e = cpu.registers.a;
} // 0x5F

fn cpu_ld_h_b(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.registers.h = cpu.registers.b;
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x60

fn cpu_ld_h_c(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.h = cpu.registers.c;
} // 0x61

fn cpu_ld_h_d(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.h = cpu.registers.d;
} // 0x62

fn cpu_ld_h_e(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.h = cpu.registers.e;
} // 0x63

fn cpu_ld_h_h(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x64

fn cpu_ld_h_l(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.h = cpu.registers.l;
} // 0x65

fn cpu_ld_h_hl(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr16(&mut cpu, &mut bus, Reg8::H, Reg16::HL);
} // 0x66

fn cpu_ld_h_a(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.h = cpu.registers.a;
} // 0x67

fn cpu_ld_l_b(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.l = cpu.registers.b;
} // 0x68

fn cpu_ld_l_c(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.l = cpu.registers.c;
} // 0x69

fn cpu_ld_l_d(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.l = cpu.registers.d;
} // 0x6A

fn cpu_ld_l_e(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.l = cpu.registers.e;
} // 0x6B

fn cpu_ld_l_h(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.l = cpu.registers.h;
} // 0x6C

fn cpu_ld_l_l(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x6D

fn cpu_ld_l_hl(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr16(&mut cpu, &mut bus, Reg8::L, Reg16::HL);
} // 0x6E

fn cpu_ld_l_a(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.l = cpu.registers.a;
} // 0x6F

fn cpu_ld_hl_b(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr8(&mut cpu, &mut bus, Reg16::HL, Reg8::B);
} // 0x70

fn cpu_ld_hl_c(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr8(&mut cpu, &mut bus, Reg16::HL, Reg8::C);
} // 0x71

fn cpu_ld_hl_d(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr8(&mut cpu, &mut bus, Reg16::HL, Reg8::D);
} // 0x72

fn cpu_ld_hl_e(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr8(&mut cpu, &mut bus, Reg16::HL, Reg8::E);
} // 0x73

fn cpu_ld_hl_h(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr8(&mut cpu, &mut bus, Reg16::HL, Reg8::H);
} // 0x74

fn cpu_ld_hl_l(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr8(&mut cpu, &mut bus, Reg16::HL, Reg8::L);
} // 0x75

fn cpu_ld_hl_a(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr8(&mut cpu, &mut bus, Reg16::HL, Reg8::A);
} // 0x77

fn cpu_ld_a_b(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.a = cpu.registers.b;
} // 0x78

fn cpu_ld_a_c(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.a = cpu.registers.c;
} // 0x79

fn cpu_ld_a_d(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.a = cpu.registers.d;
} // 0x7A

fn cpu_ld_a_e(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.a = cpu.registers.e;
} // 0x7B

fn cpu_ld_a_h(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.a = cpu.registers.h;
} // 0x7C

fn cpu_ld_a_l(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.a = cpu.registers.l;
} // 0x7D

fn cpu_ld_a_hl(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_ld_ptr16(&mut cpu, &mut bus, Reg8::A, Reg16::HL);
} // 0x7E

fn cpu_ld_a_a(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0x7F

fn cpu_add_a_b(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_add_a_8(&mut cpu, &mut bus, Reg8::B);
} // 0x80

fn cpu_add_a_c(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_add_a_8(&mut cpu, &mut bus, Reg8::C);
} // 0x81

fn cpu_add_a_d(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_add_a_8(&mut cpu, &mut bus, Reg8::D);
} // 0x82

fn cpu_add_a_e(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_add_a_8(&mut cpu, &mut bus, Reg8::E);
} // 0x83

fn cpu_add_a_h(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_add_a_8(&mut cpu, &mut bus, Reg8::H);
} // 0x84

fn cpu_add_a_l(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_add_a_8(&mut cpu, &mut bus, Reg8::L);
} // 0x85

fn cpu_adc_a_b(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_adc_a_8(&mut cpu, &mut bus, Reg8::B);
} // 0x88

fn cpu_adc_a_c(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_adc_a_8(&mut cpu, &mut bus, Reg8::C);
} // 0x89

fn cpu_adc_a_d(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_adc_a_8(&mut cpu, &mut bus, Reg8::D);
} // 0x8A

fn cpu_adc_a_e(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_adc_a_8(&mut cpu, &mut bus, Reg8::E);
} // 0x8B

fn cpu_adc_a_h(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_adc_a_8(&mut cpu, &mut bus, Reg8::H);
} // 0x8C

fn cpu_adc_a_l(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_adc_a_8(&mut cpu, &mut bus, Reg8::L);
} // 0x8D

fn cpu_sub_a_b(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_sub_a_8(&mut cpu, &mut bus, Reg8::B);
} // 0x90

fn cpu_sub_a_c(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_sub_a_8(&mut cpu, &mut bus, Reg8::C);
} // 0x91

fn cpu_sub_a_d(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_sub_a_8(&mut cpu, &mut bus, Reg8::D);
} // 0x92

fn cpu_sub_a_e(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_sub_a_8(&mut cpu, &mut bus, Reg8::E);
} // 0x93

fn cpu_sub_a_h(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_sub_a_8(&mut cpu, &mut bus, Reg8::H);
} // 0x94

fn cpu_sub_a_l(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_sub_a_8(&mut cpu, &mut bus, Reg8::L);
} // 0x95

fn cpu_sbc_a_b(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_sbc_a_8(&mut cpu, &mut bus, Reg8::B);
} // 0x98

fn cpu_sbc_a_c(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_sbc_a_8(&mut cpu, &mut bus, Reg8::C);
} // 0x99

fn cpu_sbc_a_d(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_sbc_a_8(&mut cpu, &mut bus, Reg8::D);
} // 0x9A

fn cpu_sbc_a_e(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_sbc_a_8(&mut cpu, &mut bus, Reg8::E);
} // 0x9B

fn cpu_sbc_a_h(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_sbc_a_8(&mut cpu, &mut bus, Reg8::H);
} // 0x9C

fn cpu_sbc_a_l(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_sbc_a_8(&mut cpu, &mut bus, Reg8::L);
} // 0x9D

fn cpu_and_a_b(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_and_a_8(&mut cpu, &mut bus, Reg8::B);
} // 0xA0

fn cpu_and_a_c(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_and_a_8(&mut cpu, &mut bus, Reg8::C);
} // 0xA1

fn cpu_and_a_d(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_and_a_8(&mut cpu, &mut bus, Reg8::D);
} // 0xA2

fn cpu_and_a_e(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_and_a_8(&mut cpu, &mut bus, Reg8::E);
} // 0xA3

fn cpu_and_a_h(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_and_a_8(&mut cpu, &mut bus, Reg8::H);
} // 0xA4

fn cpu_and_a_l(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_and_a_8(&mut cpu, &mut bus, Reg8::L);
} // 0xA5

fn cpu_xor_a_b(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_xor_a_8(&mut cpu, &mut bus, Reg8::B);
} // 0xA8

fn cpu_xor_a_c(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_xor_a_8(&mut cpu, &mut bus, Reg8::C);
} // 0xA9

fn cpu_xor_a_d(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_xor_a_8(&mut cpu, &mut bus, Reg8::D);
} // 0xAA

fn cpu_xor_a_e(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_xor_a_8(&mut cpu, &mut bus, Reg8::E);
} // 0xAB

fn cpu_xor_a_h(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_xor_a_8(&mut cpu, &mut bus, Reg8::H);
} // 0xAC

fn cpu_xor_a_l(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_xor_a_8(&mut cpu, &mut bus, Reg8::L);
} // 0xAD

fn cpu_or_a_b(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_or_a_8(&mut cpu, &mut bus, Reg8::B);
} // 0xB0

fn cpu_or_a_c(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_or_a_8(&mut cpu, &mut bus, Reg8::C);
} // 0xB1

fn cpu_or_a_d(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_or_a_8(&mut cpu, &mut bus, Reg8::D);
} // 0xB2

fn cpu_or_a_e(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_or_a_8(&mut cpu, &mut bus, Reg8::E);
} // 0xB3

fn cpu_or_a_h(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_or_a_8(&mut cpu, &mut bus, Reg8::H);
} // 0xB4

fn cpu_or_a_l(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_or_a_8(&mut cpu, &mut bus, Reg8::L);
} // 0xB5

fn cpu_cp_a_b(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_cp_a_8(&mut cpu, &mut bus, Reg8::B);
} // 0xB8

fn cpu_cp_a_c(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_cp_a_8(&mut cpu, &mut bus, Reg8::C);
} // 0xB9

fn cpu_cp_a_d(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_cp_a_8(&mut cpu, &mut bus, Reg8::D);
} // 0xBA

fn cpu_cp_a_e(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_cp_a_8(&mut cpu, &mut bus, Reg8::E);
} // 0xBB

fn cpu_cp_a_h(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_cp_a_8(&mut cpu, &mut bus, Reg8::H);
} // 0xBC

fn cpu_cp_a_l(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_cp_a_8(&mut cpu, &mut bus, Reg8::L);
} // 0xBD

fn cpu_ret_nz(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.zero() == false;
    cpu_routine_ret_conditional(&mut cpu, &mut bus, condition);
} // 0xC0

fn cpu_pop_bc(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_pop_16(&mut cpu, &mut bus, Reg8::B, Reg8::C);
} // 0xC1

fn cpu_jp_nz_nn(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.zero() == false;
    cpu_routine_jp_conditional_nnnn(&mut cpu, &mut bus, condition);
} // 0xC2

fn cpu_jp_nn(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);

    let low = bus.memory.memory_bus_read(usize::from(cpu.registers.pc));
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);

    let high = bus.memory.memory_bus_read(usize::from(cpu.registers.pc));
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

    let address = u16::from_le_bytes([low, high]);

    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.pc = address;
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
} // 0xC3

fn cpu_call_nz_nn(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.zero() == false;
    cpu_routine_call_conditional_nnnn(&mut cpu, &mut bus, condition);
} // 0xC4

fn cpu_push_bc(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_push_16(&mut cpu, &mut bus, Reg8::B, Reg8::C);
} // 0xC5

fn cpu_rst_00(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_rst_nnnn(&mut cpu, &mut bus, 0x0000);
} // 0xC7

fn cpu_ret_z(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.zero() == true;
    cpu_routine_ret_conditional(&mut cpu, &mut bus, condition);
} // 0xC8

fn cpu_jp_z_nn(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.zero() == true;
    cpu_routine_jp_conditional_nnnn(&mut cpu, &mut bus, condition);
} // 0xCA

fn cpu_call_z_nn(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.zero() == true;
    cpu_routine_call_conditional_nnnn(&mut cpu, &mut bus, condition);
} // 0xCC

fn cpu_rst_08(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_rst_nnnn(&mut cpu, &mut bus, 0x0008);
} // 0xCF

fn cpu_ret_nc(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.carry() == false;
    cpu_routine_ret_conditional(&mut cpu, &mut bus, condition);
} // 0xD0

fn cpu_pop_de(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_pop_16(&mut cpu, &mut bus, Reg8::D, Reg8::E);
} // 0xD1

fn cpu_jp_nc_nn(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.carry() == false;
    cpu_routine_jp_conditional_nnnn(&mut cpu, &mut bus, condition);
} // 0xD2

fn cpu_call_nc_nn(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.carry() == false;
    cpu_routine_call_conditional_nnnn(&mut cpu, &mut bus, condition);
} // 0xD4

fn cpu_push_de(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_push_16(&mut cpu, &mut bus, Reg8::D, Reg8::E);
} // 0xD5

fn cpu_rst_10(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_rst_nnnn(&mut cpu, &mut bus, 0x0010);
} // 0xD7

fn cpu_ret_c(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.carry() == true;
    cpu_routine_ret_conditional(&mut cpu, &mut bus, condition);
} // 0xD8

fn cpu_jp_c_nn(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.carry() == true;
    cpu_routine_jp_conditional_nnnn(&mut cpu, &mut bus, condition);
} // 0xDA

fn cpu_call_c_nn(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    let condition = cpu.registers.carry() == true;
    cpu_routine_call_conditional_nnnn(&mut cpu, &mut bus, condition);
} // 0xDC

fn cpu_rst_18(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_rst_nnnn(&mut cpu, &mut bus, 0x0018);
} // 0xDF

fn cpu_pop_hl(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_pop_16(&mut cpu, &mut bus, Reg8::H, Reg8::L);
} // 0xE1

fn cpu_push_hl(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_push_16(&mut cpu, &mut bus, Reg8::H, Reg8::L);
} // 0xE5

fn cpu_rst_20(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_rst_nnnn(&mut cpu, &mut bus, 0x0020);
} // 0xE7

fn cpu_rst_28(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_rst_nnnn(&mut cpu, &mut bus, 0x0028);
} // 0xEF

fn cpu_pop_af(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_pop_16(&mut cpu, &mut bus, Reg8::A, Reg8::F);
    cpu.registers.f &= 0xF0; // All flags are reset to 0
} // 0xF1

fn cpu_push_af(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_push_16(&mut cpu, &mut bus, Reg8::A, Reg8::F);
} // 0xF5

fn cpu_rst_30(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_rst_nnnn(&mut cpu, &mut bus, 0x0030);
} // 0xF7

fn cpu_rst_38(mut cpu: &mut Cpu, mut bus: &mut Bus) {
    cpu_routine_rst_nnnn(&mut cpu, &mut bus, 0x0038);
} // 0xFF
