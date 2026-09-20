use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, Default)]
pub struct GbTimerRegisters {
    pub div: u8,  // Divider Register
    pub tima: u8, // Timer Counter
    pub tma: u8,  // Timer Modulo
    pub tac: u8,  // Timer Control
}

pub static mut TIMER_INTERNAL_SYSCLK: u16 = 0;
pub static mut TIMER_INTERRUPT_DELAY: u8 = 0;
pub static mut TIMER_REGISTERS: LazyLock<GbTimerRegisters> =
    LazyLock::new(|| GbTimerRegisters::new());

impl GbTimerRegisters {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn timer_init() {
    unsafe {
        TIMER_INTERNAL_SYSCLK = 0xABCC;
        TIMER_REGISTERS.div = 0xAB;
        TIMER_REGISTERS.tac = 0;
        TIMER_REGISTERS.tima = 0;
        TIMER_REGISTERS.tma = 0;
    }
}

fn timer_check_clock_edges(prev_sysclk: u16) {
    let div_prev: u8 = unsafe { TIMER_REGISTERS.div };
    let div_bit_4_prev: bool = (unsafe { TIMER_REGISTERS.div } >> 4 & 1) != 0;

    // Show upper 8 bits
    unsafe { TIMER_REGISTERS.div = (TIMER_INTERNAL_SYSCLK >> 8) as u8 };

    let div_bit_4_now: bool = (unsafe { TIMER_REGISTERS.div } >> 4 & 1) != 0;
    if div_bit_4_prev && !div_bit_4_now { // falling edge on div 4 bit
        // TODO: Tick APU counter here
    }

    // Check bit 2 of the timer control register to see if the timer is enabled
    let tac_timer_enabled: bool = (unsafe { TIMER_REGISTERS.tac } >> 2 & 1) != 0;
    if !tac_timer_enabled {
        return;
    }

    let tac_clock_select: u8 = unsafe { TIMER_REGISTERS.tac & 0b00000011 };
    let tac_divider_bit = match tac_clock_select {
        0b00 => 9, // 4096 Hz
        0b01 => 3, // 262144 Hz
        0b10 => 5, // 65536 Hz
        0b11 => 7, // 16384 Hz
        _ => unreachable!(),
    };

    let prev_edge: bool = (prev_sysclk >> tac_divider_bit & 1) != 0;
    let curr_edge: bool = (unsafe { TIMER_INTERNAL_SYSCLK } >> tac_divider_bit & 1) != 0;
    if prev_edge && !curr_edge {
        timer_tick_tima();
    }
}

fn timer_increase_div(cycles: u8) {
    for _ in 0..cycles {
        let prev_sysclk: u16 = unsafe { TIMER_INTERNAL_SYSCLK };
        unsafe { TIMER_INTERNAL_SYSCLK = TIMER_INTERNAL_SYSCLK.wrapping_add(1) };

        timer_check_clock_edges(prev_sysclk);
    }
}

pub fn timer_advance_clocks(cycles: u8) {
    if unsafe { TIMER_INTERRUPT_DELAY } > 0 {
        for _ in 0..cycles {
            unsafe {
                TIMER_INTERRUPT_DELAY = TIMER_INTERRUPT_DELAY.saturating_sub(1);
                if TIMER_INTERRUPT_DELAY == 0 {
                    // TODO: Raise timer interrupt flag
                    break;
                }
            }
        }
    }
    timer_increase_div(cycles);
}

pub fn timer_div_write(value: u8) {
    let prev_sysclk: u16 = unsafe { TIMER_INTERNAL_SYSCLK };
    unsafe { TIMER_INTERNAL_SYSCLK = 0x0000 };
    timer_check_clock_edges(prev_sysclk);
}

pub fn timer_tick_tima() {
    unsafe {
        if TIMER_REGISTERS.tima < 255 {
            TIMER_REGISTERS.tima = TIMER_REGISTERS.tima.wrapping_add(1);
        } else {
            TIMER_REGISTERS.tima = TIMER_REGISTERS.tma;
            TIMER_INTERRUPT_DELAY = 4;
        }
    }
}
