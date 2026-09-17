use crate::{
    cpu::Cpu,
    cpu_registers::{Reg8, Reg16},
    emulator_core::core_advance_cpu_clock,
    memory_bus::{memory_bus_read, memory_bus_write},
};

/**
 * Loads an 8-bit value from memory into the specified 8-bit register of the CPU.
 */
pub fn cpu_routine_ld_8(cpu: &mut Cpu, reg: Reg8) {
    core_advance_cpu_clock(4);
    let value = memory_bus_read(cpu.registers.pc as usize);

    cpu.registers.set8(reg, value);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    core_advance_cpu_clock(4);
}

pub fn cpu_routine_ld_16(cpu: &mut Cpu, reg: Reg16) {
    core_advance_cpu_clock(4);
    let reg_a = memory_bus_read(cpu.registers.pc as usize);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

    let reg_b = memory_bus_read(cpu.registers.pc as usize);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    core_advance_cpu_clock(4);

    let value = u16::from_be_bytes([reg_a, reg_b]);
    match reg {
        Reg16::AF => {
            cpu.registers.set_af(value);
        }
        Reg16::BC => {
            cpu.registers.set_bc(value);
        }
        Reg16::DE => {
            cpu.registers.set_de(value);
        }
        Reg16::HL => {
            cpu.registers.set_hl(value);
        }
        Reg16::SP => {
            cpu.registers.sp = value;
        }
        Reg16::PC => {
            cpu.registers.pc = value;
        }
    }
    core_advance_cpu_clock(4);
}

pub fn cpu_routine_ld_ptr8(cpu: &mut Cpu, reg16: Reg16, reg8: Reg8) {
    core_advance_cpu_clock(4);
    let address = cpu.registers.get16(reg16) as usize;
    let reg8_value = cpu.registers.get8(reg8);
    memory_bus_write(address, reg8_value);
    core_advance_cpu_clock(4);
}

pub fn cpu_routine_dec_8(cpu: &mut Cpu, reg: Reg8) {
    core_advance_cpu_clock(4);

    cpu.registers.set_subtract(true);
    let result: u8 = match reg {
        Reg8::A => {
            cpu.registers.set_half_carry(cpu.registers.a & 0x0F == 0x0);
            cpu.registers.a = cpu.registers.a.wrapping_sub(1);
            cpu.registers.a
        }
        Reg8::B => {
            cpu.registers.set_half_carry(cpu.registers.b & 0x0F == 0x0);
            cpu.registers.b = cpu.registers.b.wrapping_sub(1);
            cpu.registers.b
        }
        Reg8::C => {
            cpu.registers.set_half_carry(cpu.registers.c & 0x0F == 0x0);
            cpu.registers.c = cpu.registers.c.wrapping_sub(1);
            cpu.registers.c
        }
        Reg8::D => {
            cpu.registers.set_half_carry(cpu.registers.d & 0x0F == 0x0);
            cpu.registers.d = cpu.registers.d.wrapping_sub(1);
            cpu.registers.d
        }
        Reg8::E => {
            cpu.registers.set_half_carry(cpu.registers.e & 0x0F == 0x0);
            cpu.registers.e = cpu.registers.e.wrapping_sub(1);
            cpu.registers.e
        }
        Reg8::H => {
            cpu.registers.set_half_carry(cpu.registers.h & 0x0F == 0x0);
            cpu.registers.h = cpu.registers.h.wrapping_sub(1);
            cpu.registers.h
        }
        Reg8::L => {
            cpu.registers.set_half_carry(cpu.registers.l & 0x0F == 0x0);
            cpu.registers.l = cpu.registers.l.wrapping_sub(1);
            cpu.registers.l
        }
    };
    cpu.registers.set_zero(result == 0);

    core_advance_cpu_clock(4);
}
