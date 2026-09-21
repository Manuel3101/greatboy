use crate::{
    cart::{cart_load, cart_print_info},
    gameboy::GameBoy,
};

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
    let mut gb = GameBoy::new();
    gb.cpu.cpu_reset();

    while !unsafe { CORE_QUIT_REQUESTED } {
        gb.cpu.cpu_fetch(&mut gb.bus);
        if !gb.cpu.cpu_execute(&mut gb.bus) {
            println!("CPU execution failed, requesting core shutdown");
            unsafe {
                CORE_QUIT_REQUESTED = true;
            }
        }
    }
}

/**
 * Sets `CORE_QUIT_REQUESTED` to true, which will cause the core to stop running and exit the main loop in `core_run()`.
 */
pub fn core_shutdown() {
    println!("Shutting down core");
    unsafe {
        CORE_QUIT_REQUESTED = true;
    }
}
