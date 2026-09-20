use crate::cart::CARTRIDGE_DATA;

const MEMORY_BUS_SIZE: u32 = 64 * 1024;
pub static mut MEMORY: [u8; MEMORY_BUS_SIZE as usize] = [0; MEMORY_BUS_SIZE as usize];

pub fn memory_bus_read(addr: usize) -> u8 {
    return unsafe { CARTRIDGE_DATA[addr] };
}

pub fn memory_bus_write(addr: usize, value: u8) {
    todo!("Implement memory bus write");
}
