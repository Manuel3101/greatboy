use crate::{
    cart::{cart_load, cart_print_info},
    cpu::{cpu_execute, cpu_fetch, cpu_reset},
};

pub static mut CORE_CLOCK_COUNTER: u32 = 0;
static mut CORE_QUIT_REQUESTED: bool = false;

pub fn core_init() -> bool {
    const ROM_PATH: &str = "./roms/Tetris (World) (Rev 1).gb";

    if !cart_load(ROM_PATH) {
        println!("Failed to load ROM: {}", ROM_PATH);
        unsafe {
            CORE_QUIT_REQUESTED = true;
        }
        return false;
    }

    cart_print_info();
    true
}

pub fn core_run() {
    cpu_reset();

    while !unsafe { CORE_QUIT_REQUESTED } {
        cpu_fetch();
        if !cpu_execute() {
            println!("CPU execution failed, requesting core shutdown");
            unsafe {
                CORE_QUIT_REQUESTED = true;
            }
        }
    }
}

pub fn core_shutdown() {
    println!("Shutting down core");
    unsafe {
        CORE_QUIT_REQUESTED = true;
    }
}

pub fn core_advance_cpu_clock(cycles: u32) {
    unsafe {
        CORE_CLOCK_COUNTER += cycles;
    }
}
