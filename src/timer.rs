use crate::cpu::{TIMER_STATE, TimerState};

pub struct Timer {
    pub registers: GbTimerRegisters,
    internal_sysclk: u16,
    interrupt_delay: u8,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GbTimerRegisters {
    pub div: u8,  // Divider Register
    pub tima: u8, // Timer Counter
    pub tma: u8,  // Timer Modulo
    pub tac: u8,  // Timer Control
}

impl Timer {
    pub fn new() -> Self {
        Self {
            registers: GbTimerRegisters::default(),
            internal_sysclk: 0xABCC,
            interrupt_delay: 0,
        }
    }

    fn check_clock_edges(&mut self, prev_sysclk: u16) {
        let div_bit_4_prev: bool = (self.registers.div >> 4 & 1) != 0;

        // Show upper 8 bits
        self.registers.div = (self.internal_sysclk >> 8) as u8;

        let div_bit_4_now: bool = (self.registers.div >> 4 & 1) != 0;
        if div_bit_4_prev && !div_bit_4_now { // falling edge on div 4 bit
            // TODO: Tick APU counter here
        }

        // Check bit 2 of the timer control register to see if the timer is enabled
        let tac_timer_enabled: bool = (self.registers.tac >> 2 & 1) != 0;
        if !tac_timer_enabled {
            return;
        }

        let tac_clock_select: u8 = self.registers.div & 0b00000011;
        let tac_divider_bit = match tac_clock_select {
            0b00 => 9, // 4096 Hz
            0b01 => 3, // 262144 Hz
            0b10 => 5, // 65536 Hz
            0b11 => 7, // 16384 Hz
            _ => unreachable!(),
        };

        let prev_edge: bool = (prev_sysclk >> tac_divider_bit & 1) != 0;
        let curr_edge: bool = (self.internal_sysclk >> tac_divider_bit & 1) != 0;
        if prev_edge && !curr_edge {
            self.tick_tima();
        }
    }

    fn increase_div(&mut self, cycles: u8) {
        for _ in 0..cycles {
            let prev_sysclk: u16 = self.internal_sysclk;
            self.internal_sysclk = self.internal_sysclk.wrapping_add(1);

            self.check_clock_edges(prev_sysclk);
        }
    }

    pub fn advance_clocks(&mut self, cycles: u8) {
        if self.interrupt_delay > 0 {
            for _ in 0..cycles {
                self.interrupt_delay = self.interrupt_delay.saturating_sub(1);
                if self.interrupt_delay == 0 {
                    // TODO: Raise timer interrupt flag
                    break;
                }
            }
        }

        match unsafe { TIMER_STATE } {
            TimerState::Running => {
                self.increase_div(cycles);
            }
            TimerState::Halted => {
                // Do nothing, timer is halted
            }
            TimerState::Stopped => {
                return;
            }
        }
    }

    pub fn div_write(&mut self, value: u8) {
        let prev_sysclk: u16 = self.internal_sysclk;
        self.internal_sysclk = 0x0000;
        self.check_clock_edges(prev_sysclk);
    }

    fn tick_tima(&mut self) {
        if self.registers.tima < 255 {
            self.registers.tima = self.registers.tima.wrapping_add(1);
        } else {
            self.registers.tima = self.registers.tma;
            self.interrupt_delay = 4;
        }
    }
}

impl GbTimerRegisters {
    pub fn new() -> Self {
        Self {
            div: 0xAB,
            tac: 0,
            tima: 0,
            tma: 0,
        }
    }
}
