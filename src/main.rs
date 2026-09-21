use crate::emulator_core::{core_init, core_run, core_shutdown};

mod bus;
mod cart;
mod cpu;
mod emulator_core;
mod gameboy;
mod memory_bus;
mod timer;

fn main() {
    let init = core_init();

    if !init {
        println!("Failed to initialize core");
        return;
    }

    core_run();
    core_shutdown();
}
