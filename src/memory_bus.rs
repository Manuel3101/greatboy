use crate::cart::CARTRIDGE_DATA;

const MEMORY_BUS_SIZE: u32 = 64 * 1024; // 64KB

pub struct MemoryBus {
    pub size: u32,
    pub memory: [u8; MEMORY_BUS_SIZE as usize],
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            size: 64 * 1024,
            memory: [0; MEMORY_BUS_SIZE as usize],
        }
    }

    pub fn memory_bus_read(&self, addr: usize) -> u8 {
        return unsafe { CARTRIDGE_DATA[addr] };
    }

    pub fn memory_bus_write(&self, _addr: usize, _value: u8) {
        // TODO: Implement memory bus write
    }
}
