use std::io;
use std::mem;
use std::slice;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
struct INESHeader {
    magic_bytes: [u8; 4],
    prg_rom_size: u8,
    chr_rom_size: u8,
    flags: [u8; 5],
    padding: [u8;5]
}

fn parse_pgr_rom_size(size: u8) -> &'static str {
    match size {
        0x01 => "(1x16kbpages)",
        0x02 => "(2x16kbpages)",
        0x03 => "(3x16kbpages)",
        0x04 => "(4x16kbpages)",
        0x05 => "(5x16kbpages)",
        0x06 => "(6x16kbpages)",
        0x07 => "(7x16kbpages)",
        0x08 => "(8x16kbpages)",
        0x09 => "(9x16kbpages)",
        0x0A => "(10x16kbpages)",
        0x0B => "(11x16kbpages)",
        0x0C => "(12x16kbpages)",
        0x0D => "(13x16kbpages)",
        0x0E => "(14x16kbpages)",
        0x0F => "(15x16kbpages)",
        0x10 => "(16x16kbpages)",
        0x11 => "(17x16kbpages)",
        0x12 => "(18x16kbpages)",
        0x13 => "(19x16kbpages)",
        0x14 => "(20x16kbpages)",
        0x15 => "(21x16kbpages)",
        0x16 => "(22x16kbpages)",
        0x17 => "(23x16kbpages)",
        0x18 => "(24x16kbpages)",
        0x19 => "(25x16kbpages)",
        0x1A => "(26x16kbpages)",
        0x1B => "(27x16kbpages)",
        0x1C => "(28x16kbpages)",
        0x1D => "(29x16kbpages)",
        0x1E => "(30x16kbpages)",
        0x1F => "(31x16kbpages)",
        0x20 => "(32x16kbpages)",
        0x21 => "(33x16kbpages)",
        0x22 => "(34x16kbpages)",
        0x23 => "(35x16kbpages)",
        0x24 => "(36x16kbpages)",
        0x25 => "(37x16kbpages)",
        0x26 => "(38x16kbpages)",
        0x27 => "(39x16kbpages)",
        0x28 => "(40x16kbpages)",
        0x29 => "(41x16kbpages)",
        0x2A => "(42x16kbpages)",
        0x2B => "(43x16kbpages)",
        0x2C => "(44x16kbpages)",
        0x2D => "(45x16kbpages)",
        0x2E => "(46x16kbpages)",
        0x2F => "(47x16kbpages)",
        0x30 => "(48x16kbpages)",
        0x31 => "(49x16kbpages)",
        0x32 => "(50x16kbpages)",
        0x33 => "(51x16kbpages)",
        0x34 => "(52x16kbpages)",
        0x35 => "(53x16kbpages)",
        0x36 => "(54x16kbpages)",
        0x37 => "(55x16kbpages)",
        0x38 => "(56x16kbpages)",
        0x39 => "(57x16kbpages)",
        0x3A => "(58x16kbpages)",
        0x3B => "(59x16kbpages)",
        0x3C => "(60x16kbpages)",
        0x3D => "(61x16kbpages)",
        0x3E => "(62x16kbpages)",
        0x3F => "(63x16kbpages)",
        0x40 => "(64x16kbpages)",
          _  => "(Unknown)"
    }
}

fn parse_chr_rom_size(size: u8) -> &'static str {
    match size {
        0x01 => "(1x8kbpages)",
        0x02 => "(2x8kbpages)",
        0x03 => "(3x8kbpages)",
        0x04 => "(4x8kbpages)",
        0x05 => "(5x8kbpages)",
        0x06 => "(6x8kbpages)",
        0x07 => "(7x8kbpages)",
        0x08 => "(8x8kbpages)",
        0x09 => "(9x8kbpages)",
        0x0A => "(10x8kbpages)",
        0x0B => "(11x8kbpages)",
        0x0C => "(12x8kbpages)",
        0x0D => "(13x8kbpages)",
        0x0E => "(14x8kbpages)",
        0x0F => "(15x8kbpages)",
        0x10 => "(16x8kbpages)",
        0x11 => "(17x8kbpages)",
        0x12 => "(18x8kbpages)",
        0x13 => "(19x8kbpages)",
        0x14 => "(20x8kbpages)",
        0x15 => "(21x8kbpages)",
        0x16 => "(22x8kbpages)",
        0x17 => "(23x8kbpages)",
        0x18 => "(24x8kbpages)",
        0x19 => "(25x8kbpages)",
        0x1A => "(26x8kbpages)",
        0x1B => "(27x8kbpages)",
        0x1C => "(28x8kbpages)",
        0x1D => "(29x8kbpages)",
        0x1E => "(30x8kbpages)",
        0x1F => "(31x8kbpages)",
        0x20 => "(32x8kbpages)",
        0x21 => "(33x8kbpages)",
        0x22 => "(34x8kbpages)",
        0x23 => "(35x8kbpages)",
        0x24 => "(36x8kbpages)",
        0x25 => "(37x8kbpages)",
        0x26 => "(38x8kbpages)",
        0x27 => "(39x8kbpages)",
        0x28 => "(40x8kbpages)",
        0x29 => "(41x8kbpages)",
        0x2A => "(42x8kbpages)",
        0x2B => "(43x8kbpages)",
        0x2C => "(44x8kbpages)",
        0x2D => "(45x8kbpages)",
        0x2E => "(46x8kbpages)",
        0x2F => "(47x8kbpages)",
        0x30 => "(48x8kbpages)",
        0x31 => "(49x8kbpages)",
        0x32 => "(50x8kbpages)",
        0x33 => "(51x8kbpages)",
        0x34 => "(52x8kbpages)",
        0x35 => "(53x8kbpages)",
        0x36 => "(54x8kbpages)",
        0x37 => "(55x8kbpages)",
        0x38 => "(56x8kbpages)",
        0x39 => "(57x8kbpages)",
        0x3A => "(58x8kbpages)",
        0x3B => "(59x8kbpages)",
        0x3C => "(60x8kbpages)",
        0x3D => "(61x8kbpages)",
        0x3E => "(62x8kbpages)",
        0x3F => "(63x8kbpages)",
        0x40 => "(64x8kbpages)",
           _ => "(Unknown)"
    }
}

fn main() -> io::Result<()> {
    // Open file and read into vector.
    let f = File::open("./test/test_roms/Tetris.nes")?;
    let mut reader = BufReader::new(f);
    let mut file_buffer = Vec::new();
    reader.read_to_end(&mut file_buffer)?;

    // Parse bytes into INESHeader struct
    let mut header: INESHeader = unsafe { mem::zeroed() };
    let mut header_buffer: &[u8] = &file_buffer[..mem::size_of::<INESHeader>()];

    unsafe {
        let header_slice = slice::from_raw_parts_mut(&mut header as  *mut _ as *mut u8, mem::size_of::<INESHeader>());
        header_buffer.read_exact(header_slice).unwrap();
    }

    // Display header information

    println!("INES ROM Parser");

    print!("Magic Bytes: {:02X} {:02X} {:02X} {:02X} ", 
             header.magic_bytes[0], 
             header.magic_bytes[1], 
             header.magic_bytes[2], 
             header.magic_bytes[3]);

    if header.magic_bytes == [0x4e, 0x45, 0x53, 0x1A] {
        println!(" [VALID]");
    }
    else {
        println!(" [INVALID!]");
    }
    
    let prg_rom_size = parse_pgr_rom_size(header.prg_rom_size);
    println!("PRG ROM Size: {prg_rom_size}");

    let chr_rom_size = parse_chr_rom_size(header.chr_rom_size);
    println!("CHR ROM Size: {chr_rom_size}");

    println!("\nBytes 7-15 are ignored by many emulators and are either empty or populated with messages on many ROMs.");
    println!("Interpret with caution...\n");

    print!("Tail Bytes: ");
    for flag_byte in header.flags[1..].iter() {
        print!("{:02X} ", flag_byte);
    }
    for padding_byte in header.padding.iter() {
        print!("{:02X} ", padding_byte);
    }
    println!("\n");

    let prg_ram_size = header.flags[2] * 8;
    print!("PRG RAM Size: ");
    if prg_ram_size == 0 {
        println!("8KB Compatible");
    }
    else {
        println!("{}KB", prg_ram_size);
    }

    let tv_system_type_a = header.flags[3];
    print!("TV System Type (a): ");
    if tv_system_type_a == 0 {
        println!("NTSC");
    }
    else if tv_system_type_a == 1 {
        println!("PAL");
    }
    else {
        println!("(Unknown)");
    }

    let tv_system_type_b = header.flags[4] & 0b00000011;
    print!("TV System Type (b): ");
    if tv_system_type_b  == 0 {
        println!("NTSC");
    }
    else if tv_system_type_b == 1 {
        println!("PAL");
    }
    else {
        println!("Dual Compatible");
    }

    let prg_ram_present: bool = (header.flags[4] & 0b00010000) == 0;
    print!("PRG RAM Present: ");
    if prg_ram_present {
        println!("YES");
    }
    else {
        println!("NO");
    }

    let board_has_bus_conflicts: bool = (header.flags[4] & 0b00100000) != 0;
    print!("Board Has No Conflicts: ");
    if board_has_bus_conflicts {
        println!("YES");
    }
    else {
        println!("NO");
    }

    print!("Padding Bytes: ");
    let mut padding_valid = true;
    for padding_byte in header.padding.iter() {
        print!("{:02X} ", padding_byte);
        if *padding_byte != 0u8 {
            padding_valid = false;
        }
    }
    if padding_valid {
        println!("[VALID]");
    }
    else {
        println!("[INVALID]");
    }

    Ok(())
}
