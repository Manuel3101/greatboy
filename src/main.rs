use crate::cart::{cart_load, cart_print_info};

mod cart;

fn main() {
    const ROM_PATH: &str = "./roms/Tetris (World) (Rev 1).gb";

    if !cart_load(ROM_PATH) {
        println!("Failed to load ROM: {}", ROM_PATH);
        return;
    }

    cart_print_info();
}
