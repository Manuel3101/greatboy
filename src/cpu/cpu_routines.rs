use std::ops::{Shl, Shr};

use crate::{
    bus::Bus,
    cpu::{
        Cpu,
        cpu_registers::{Reg8, Reg16},
    },
};

/**
 * Loads an 8-bit value from memory into the specified 8-bit register of the CPU.
 */
pub fn cpu_routine_ld_8(cpu: &mut Cpu, bus: &mut Bus, reg: Reg8) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    let value = bus.memory.memory_bus_read(cpu.registers.pc as usize);

    cpu.registers.set8(reg, value);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_ld_16(cpu: &mut Cpu, bus: &mut Bus, reg: Reg16) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    let reg_a = bus.memory.memory_bus_read(cpu.registers.pc as usize);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

    let reg_b = bus.memory.memory_bus_read(cpu.registers.pc as usize);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);

    let value = u16::from_le_bytes([reg_a, reg_b]);
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
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_ld_ptr8(cpu: &mut Cpu, bus: &mut Bus, reg16: Reg16, reg8: Reg8) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    let address = cpu.registers.get16(&reg16) as usize;
    let reg8_value = cpu.registers.get8(&reg8);
    bus.memory.memory_bus_write(address, reg8_value);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}
// TODO: check that this is correct
// reg16 should be loaded into reg8
pub fn cpu_routine_ld_ptr16(cpu: &mut Cpu, bus: &mut Bus, reg8: Reg8, reg16: Reg16) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    let address = cpu.registers.get8(&reg8) as usize;
    let reg16_value = cpu.registers.get16(&reg16) as u8;
    bus.memory.memory_bus_write(address, reg16_value);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_dec_8(cpu: &mut Cpu, bus: &mut Bus, reg: Reg8) {
    let selected_reg = cpu.registers.get8(&reg);
    cpu.registers.set_subtract(true);
    cpu.registers.set_subtract(false);
    cpu.registers.set_half_carry(selected_reg & 0x0F == 0x0F);

    let result = selected_reg.wrapping_sub(1);
    cpu.registers.set8(reg, result);

    cpu.registers.set_zero(result == 0);

    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_inc_8(cpu: &mut Cpu, bus: &mut Bus, reg: Reg8) {
    let selected_reg = cpu.registers.get8(&reg);
    cpu.registers.set_subtract(false);
    cpu.registers.set_half_carry(selected_reg & 0x0F == 0x0F);

    let result = selected_reg.wrapping_add(1);
    cpu.registers.set8(reg, result);

    cpu.registers.set_zero(result == 0);

    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_inc_16(cpu: &mut Cpu, bus: &mut Bus, reg: Reg16) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);

    let result = cpu.registers.get16(&reg).wrapping_add(1);
    cpu.registers.set16(reg, result);

    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_dec_16(cpu: &mut Cpu, bus: &mut Bus, reg: Reg16) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);

    let result = cpu.registers.get16(&reg).wrapping_sub(1);
    cpu.registers.set16(reg, result);

    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_add_a_8(cpu: &mut Cpu, bus: &mut Bus, reg8: Reg8) {
    cpu.registers.set_subtract(false);
    let reg8_value = cpu.registers.get8(&reg8);
    let temp = cpu.registers.a;
    cpu.registers
        .set_half_carry(temp & 0x0F + reg8_value & 0x0F > 0x0F);
    cpu.registers.a = cpu.registers.a.wrapping_add(reg8_value);
    cpu.registers.set_zero(cpu.registers.a == 0);
    cpu.registers.set_carry(temp > cpu.registers.a);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_adc_a_8(cpu: &mut Cpu, bus: &mut Bus, reg8: Reg8) {
    cpu.registers.set_subtract(false);
    // TODO: how to get carry flag?
    let carry = cpu.registers.carry();
    let reg8_value = cpu.registers.get8(&reg8);
    let mut temp = cpu.registers.a + reg8_value + carry as u8;

    cpu.registers
        .set_half_carry(cpu.registers.a & 0x0F + reg8_value & 0x0F > 0x0F);
    cpu.registers.set_carry(temp >= 0xFF);

    temp &= 0xFF;
    cpu.registers.a = temp;

    cpu.registers.set_zero(temp == 0);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_sub_a_8(cpu: &mut Cpu, bus: &mut Bus, reg8: Reg8) {
    cpu.registers.set_subtract(true);
    let reg8_value = cpu.registers.get8(&reg8);
    cpu.registers
        .set_half_carry((cpu.registers.a & 0x0F) < (reg8_value & 0x0F));
    cpu.registers.set_carry(cpu.registers.a < reg8_value);
    cpu.registers.a = cpu.registers.a.wrapping_sub(reg8_value);
    cpu.registers.set_zero(cpu.registers.a == 0);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_sbc_a_8(cpu: &mut Cpu, bus: &mut Bus, reg8: Reg8) {
    // TODO: how to get carry flag?
    let carry = cpu.registers.carry();
    let reg8_value = cpu.registers.get8(&reg8);
    let temp = cpu.registers.a - (reg8_value + carry as u8);

    cpu.registers.set_subtract(true);
    cpu.registers
        .set_carry(if temp & 0xFF != 0 { true } else { false });
    cpu.registers
        .set_zero(if temp & 0xFF == 0 { false } else { true });
    cpu.registers
        .set_half_carry((cpu.registers.a ^ reg8_value ^ temp) & 0x10 != 0);

    cpu.registers.a = temp;
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_and_a_8(cpu: &mut Cpu, bus: &mut Bus, reg8: Reg8) {
    cpu.registers.set_half_carry(true);
    cpu.registers.set_subtract(false);
    cpu.registers.set_carry(false);
    let reg8_value = cpu.registers.get8(&reg8);
    cpu.registers.a &= reg8_value;
    cpu.registers.set_zero(cpu.registers.a == 0);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_xor_a_8(cpu: &mut Cpu, bus: &mut Bus, reg8: Reg8) {
    cpu.registers.set_half_carry(false);
    cpu.registers.set_subtract(false);
    cpu.registers.set_carry(false);
    let reg8_value = cpu.registers.get8(&reg8);
    cpu.registers.a ^= reg8_value;
    cpu.registers.set_zero(cpu.registers.a == 0);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_or_a_8(cpu: &mut Cpu, bus: &mut Bus, reg8: Reg8) {
    cpu.registers.set_half_carry(false);
    cpu.registers.set_subtract(false);
    cpu.registers.set_carry(false);
    let reg8_value = cpu.registers.get8(&reg8);
    cpu.registers.a |= reg8_value;
    cpu.registers.set_zero(cpu.registers.a == 0);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_cp_a_8(cpu: &mut Cpu, bus: &mut Bus, reg8: Reg8) {
    let reg8_value = cpu.registers.get8(&reg8);
    cpu.registers.set_subtract(false);
    cpu.registers
        .set_half_carry((cpu.registers.a & 0x0F) < (reg8_value & 0x0F));
    cpu.registers.set_carry(cpu.registers.a < reg8_value);
    cpu.registers.set_zero(cpu.registers.a == reg8_value);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_rst_nnnn(cpu: &mut Cpu, bus: &mut Bus, address: u16) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
    cpu.registers.sp &= 0xFFFF;
    let pchi: u8 = (cpu.registers.pc & 0xFF00).shr(8) as u8;
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    bus.memory.memory_bus_write(cpu.registers.sp as usize, pchi);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
    cpu.registers.sp &= 0xFFFF;
    let pclo: u8 = (cpu.registers.pc & 0xFF) as u8;
    bus.memory.memory_bus_write(cpu.registers.sp as usize, pclo);
    cpu.registers.pc = address;
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_push_16(cpu: &mut Cpu, bus: &mut Bus, reg_hi: Reg8, reg_lo: Reg8) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
    cpu.registers.sp &= 0xFFFF;
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    let reg_hi_value = cpu.registers.get8(&reg_hi);
    bus.memory
        .memory_bus_write(cpu.registers.sp as usize, reg_hi_value);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
    cpu.registers.sp &= 0xFFFF;
    let reg_lo_value = cpu.registers.get8(&reg_lo);
    bus.memory
        .memory_bus_write(cpu.registers.sp as usize, reg_lo_value);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_pop_16(cpu: &mut Cpu, bus: &mut Bus, reg_hi: Reg8, reg_lo: Reg8) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    let reg_lo_value = bus.memory.memory_bus_read(cpu.registers.sp as usize);
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    cpu.registers.sp &= 0xFFFF;
    cpu.registers.set8(reg_lo, reg_lo_value);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    let reg_hi_value = bus.memory.memory_bus_read(cpu.registers.sp as usize);
    cpu.registers.sp &= 0xFFFF;
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    cpu.registers.set8(reg_hi, reg_hi_value);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}

pub fn cpu_routine_call_conditional_nnnn(cpu: &mut Cpu, bus: &mut Bus, condition: bool) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    if condition {
        let mut temp = bus.memory.memory_bus_read(cpu.registers.pc as usize);
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
        temp |= bus.memory.memory_bus_read(cpu.registers.pc as usize).shl(8);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        cpu.registers.sp &= 0xFFFF;
        let pchi: u8 = (cpu.registers.pc & 0xFF00).shr(8) as u8;
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
        bus.memory.memory_bus_write(cpu.registers.sp as usize, pchi);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        cpu.registers.sp &= 0xFFFF;
        let pclo: u8 = (cpu.registers.pc & 0xFF) as u8;
        bus.memory.memory_bus_write(cpu.registers.sp as usize, pclo);
        cpu.registers.pc = temp as u16;
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    } else {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
        cpu.registers.pc &= 0xFFFF;
    }
}

pub fn cpu_routine_ret_conditional(cpu: &mut Cpu, bus: &mut Bus, condition: bool) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    if condition {
        let mut temp = bus.memory.memory_bus_read(cpu.registers.sp as usize);
        cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
        temp |= bus.memory.memory_bus_read(cpu.registers.sp as usize).shl(8);
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        cpu.registers.sp &= 0xFFFF;
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
        cpu.registers.pc = temp as u16;
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    } else {
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    }
}

pub fn cpu_routine_jp_conditional_nnnn(cpu: &mut Cpu, bus: &mut Bus, condition: bool) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    if condition {
        let mut temp = bus.memory.memory_bus_read(cpu.registers.pc as usize);
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
        temp |= bus.memory.memory_bus_read(cpu.registers.pc as usize).shl(8);
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
        cpu.registers.pc = temp as u16;
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    } else {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    }
}

pub fn cpu_routine_jr_conditional_n(cpu: &mut Cpu, bus: &mut Bus, condition: bool) {
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    if condition {
        let temp = bus.memory.memory_bus_read(cpu.registers.pc as usize);
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
        cpu.registers.pc = cpu.registers.pc.wrapping_add((temp as i8) as u16);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    } else {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
        cpu.core_advance_cpu_clock(&mut bus.timer, 4);
    }
}

pub fn cpu_routine_add_hl_16(cpu: &mut Cpu, bus: &mut Bus, reg16: Reg16) {
    cpu.registers.set_subtract(false);

    let reg16_value = cpu.registers.get16(&reg16);
    let temp = cpu.registers.hl().wrapping_add(reg16_value);

    cpu.registers.set_carry(temp >= 0xFFFF);
    cpu.registers
        .set_half_carry((cpu.registers.hl() & 0x0FFF) + (reg16_value & 0x0FFF) > 0x0FFF);

    cpu.registers.set_hl(temp);
    cpu.core_advance_cpu_clock(&mut bus.timer, 4);
}
