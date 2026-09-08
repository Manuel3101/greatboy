use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::System::Environment::GetCurrentDirectoryA;

pub const MAX_CARTRIDGE_SIZE: usize = 1024 * 1024;
pub static mut CARTRIDGE_DATA: [u8; MAX_CARTRIDGE_SIZE] = [0; MAX_CARTRIDGE_SIZE];
pub static mut CARTRIDGE_LOADED: bool = false;
pub static mut RUNTIME_PATH_BUFFER: [u8; MAX_PATH as usize] = [0; MAX_PATH as usize];

// TODO: Check sizes with technical reference to make sure they are correct
#[allow(dead_code)]
pub struct CartHeader {
    entry_point: [u8; 4],
    nintendo_logo: [u8; 48],
    title: [u8; 15],
    cgb_flag: u8,
    new_license_code: [u8; 2],
    sgb_flag: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination_code: u8,
    old_license_code: u8,
    mask_rom_version_number: u8,
    header_checksum: u8,
    global_checksum_hi: u8,
    global_checksum_lo: u8,
}

impl CartHeader {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            entry_point: bytes[0..4].try_into().unwrap(),
            nintendo_logo: bytes[4..52].try_into().unwrap(),
            title: bytes[52..67].try_into().unwrap(),
            cgb_flag: bytes[67],
            new_license_code: bytes[68..70].try_into().unwrap(),
            sgb_flag: bytes[70],
            cartridge_type: bytes[71],
            rom_size: bytes[72],
            ram_size: bytes[73],
            destination_code: bytes[74],
            old_license_code: bytes[75],
            mask_rom_version_number: bytes[76],
            header_checksum: bytes[77],
            global_checksum_hi: bytes[78],
            global_checksum_lo: bytes[79],
        }
    }
}

pub fn get_runtime_path() -> bool {
    unsafe {
        #[cfg(windows)]
        {
            let length_of_path = GetCurrentDirectoryA(Some(&mut RUNTIME_PATH_BUFFER[..]));

            return length_of_path != 0 && length_of_path < MAX_PATH as u32;
        }
    }
}

pub fn cart_print_info() {
    const HEADER_OFFSET: usize = 0x100;
    const HEADER_SIZE: usize = 80;

    let header = unsafe {
        if !CARTRIDGE_LOADED {
            println!("No cartridge loaded");
            return;
        }

        let mut header = [0u8; HEADER_SIZE];
        let source = std::slice::from_raw_parts(
            std::ptr::addr_of!(CARTRIDGE_DATA)
                .cast::<u8>()
                .add(HEADER_OFFSET),
            HEADER_SIZE,
        );
        header.copy_from_slice(source);
        header
    };
    // TODO: Seems to no display 0x0000
    let text = |bytes: &[u8]| {
        String::from_utf8_lossy(bytes)
            .trim_end_matches('\0')
            .trim_end()
            .to_owned()
    };

    let cart_header = CartHeader::from_bytes(&header);

    println!("Cartridge information:");
    println!("Entry point:       {:02X?}", cart_header.entry_point);
    println!("Title:             {}", text(&cart_header.title));
    println!("CGB flag:          0x{:02X}", cart_header.cgb_flag);
    println!("New license code:  {}", text(&cart_header.new_license_code));
    println!("SGB flag:          0x{:02X}", cart_header.sgb_flag);
    println!("Cartridge type:    0x{:02X}", cart_header.cartridge_type);
    println!("ROM size:          0x{:02X}", cart_header.rom_size);
    println!("RAM size:          0x{:02X}", cart_header.ram_size);
    println!("Destination code:  0x{:02X}", cart_header.destination_code);
    println!("Old license code:  0x{:02X}", cart_header.old_license_code);
    println!(
        "ROM version:       0x{:02X}",
        cart_header.mask_rom_version_number
    );
    println!(
        "cart_header checksum:   0x{:02X}",
        cart_header.header_checksum
    );
    println!(
        "Global checksum:   0x{:02X}{:02X}",
        cart_header.global_checksum_hi, cart_header.global_checksum_lo
    );
}

pub fn cart_load(filename: &str) -> bool {
    if !get_runtime_path() {
        println!("Failed to get runtime path");
        return false;
    }

    let data = match std::fs::read(filename) {
        Ok(data) => data,
        Err(_) => return false,
    };

    if data.len() > MAX_CARTRIDGE_SIZE {
        return false;
    }

    unsafe {
        CARTRIDGE_DATA[..data.len()].copy_from_slice(&data);
        CARTRIDGE_LOADED = true;
    }
    println!("ROM {} loaded, size: {} bytes", filename, data.len());

    true
}
