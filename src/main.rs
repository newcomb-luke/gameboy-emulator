use std::path::Path;

use gameboy_emulator::{boot::{BootRom, BootRomReader}, cartridge::Cartridge, cpu::{bus::{MainBus, SharedBus}, Cpu}, ppu::{vram::Vram, Ppu}};

fn main() {
    let boot_rom = read_boot_rom("dmg_boot.bin");
    let cartridge = read_cartridge("tests/roms/dmg-acid2.gb");
    
    let header = cartridge.header();

    println!("Title: {}", header.title());
    println!("Type: {:#?}", header.cartridge_type());
    println!("ROM Size: {:#?}", header.rom_size());
    println!("RAM Size: {:#?}", header.ram_size());
    println!("Destination code: {:#?}", header.destination_code());
    println!("Version number: {:#?}", header.version_number());
    println!("Header checksum (read): {:02x}", header.read_header_checksum());
    println!("Header checksum (computed): {:02x}", header.computed_header_checksum());
    println!("Header checksum valid: {}", header.header_checksum_valid());

    let vram = Vram::new();

    let bus = MainBus::new(boot_rom, cartridge, vram.clone());
    let shared_bus = SharedBus::new(bus);

    let mut cpu = Cpu::new(shared_bus);

    let mut ppu = Ppu::new(vram);

    loop {
        cpu.execute_one().unwrap();
    }
}

fn read_cartridge<P>(path: P) -> Cartridge
where 
    P: AsRef<Path> {
    let mut cartridge_file = std::fs::File::open(path).unwrap();
    Cartridge::read(&mut cartridge_file).unwrap()
}

fn read_boot_rom<P>(path: P) -> BootRom
where 
    P: AsRef<Path> {
    let mut boot_rom_file = std::fs::File::open(path).unwrap();
    BootRomReader::read(&mut boot_rom_file).unwrap()
}