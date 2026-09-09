use crate::cart::CARTRIDGE_DATA;

pub fn memory_bus_read(addr: usize) -> u8 {
    return unsafe { CARTRIDGE_DATA[addr] };
}

pub fn memory_bus_write(addr: usize, value: u8) {
    todo!("Implement memory bus write");
}
