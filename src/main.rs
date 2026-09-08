use crate::cart::{cart_load, cart_print_info};
use crate::cpu::{cpu_execute, cpu_fetch, cpu_reset};

mod cart;
mod cpu;
mod cpu_instructions;

fn main() {
    const ROM_PATH: &str = "./roms/Tetris (World) (Rev 1).gb";

    if !cart_load(ROM_PATH) {
        println!("Failed to load ROM: {}", ROM_PATH);
        return;
    }

    cart_print_info();

    cpu_reset();
    cpu_fetch();
    cpu_execute();
}
