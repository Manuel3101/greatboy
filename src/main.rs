use crate::emulator_core::{core_init, core_run, core_shutdown};

mod cart;
mod cpu;
mod cpu_instructions;
mod emulator_core;

fn main() {
    let init = core_init();

    if !init {
        println!("Failed to initialize core");
        return;
    }

    core_run();
    core_shutdown();
}
