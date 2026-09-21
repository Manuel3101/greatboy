use crate::{memory_bus::MemoryBus, timer::Timer};

pub struct Bus {
    pub memory: MemoryBus,
    pub timer: Timer,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: MemoryBus::new(),
            timer: Timer::new(),
        }
    }

    pub fn advance_timer_cycles(&mut self, cycles: u8) {
        self.timer.advance_clocks(cycles);
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
