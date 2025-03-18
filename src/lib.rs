use std::path::Path;

use boot::{BootRom, BootRomReader};
use bus::Bus;
use cartridge::Cartridge;
use cpu::{
    error::Error,
    execution_state::ExecutionState,
    Cpu,
};
use eframe::egui::Color32;

pub mod boot;
pub mod cartridge;
pub mod cpu;
pub mod io;
pub mod memory;
pub mod ppu;
pub mod bus;

pub const DISPLAY_HEIGHT_PIXELS: usize = 144;
pub const DISPLAY_WIDTH_PIXELS: usize = 160;
pub const DISPLAY_SIZE_PIXELS: &'static [usize; 2] = &[DISPLAY_WIDTH_PIXELS, DISPLAY_HEIGHT_PIXELS];

pub const DARKEST_COLOR: Color32 = Color32::from_rgb(8, 24, 32);
pub const DARKER_COLOR: Color32 = Color32::from_rgb(52, 104, 86);
pub const LIGHTER_COLOR: Color32 = Color32::from_rgb(136, 192, 112);
pub const LIGHTEST_COLOR: Color32 = Color32::from_rgb(224, 248, 208);
pub const OFF_COLOR: Color32 = Color32::from_rgb(133, 159, 88);

pub struct Emulator {
    cpu: Cpu,
    empty_display: Vec<Color32>,
    breakpoints: Vec<u16>,
}

impl Emulator {
    pub fn new(boot_rom: BootRom, cartridge: Cartridge) -> Self {
        let bus = Bus::new(
            boot_rom,
            cartridge,
        );

        Self {
            cpu: Cpu::new(bus),
            empty_display: Self::off_display(),
            breakpoints: Vec::new(),
        }
    }

    pub fn add_breakpoint(&mut self, address: u16) {
        self.breakpoints.push(address);
    }

    pub fn execution_state(&self) -> &ExecutionState {
        self.cpu.execution_state()
    }

    pub fn step(&mut self) -> Result<(), Error> {
        self.cpu.execute_one()
    }

    pub fn get_pixels(&mut self) -> Vec<Color32> {
        let lcd = self.cpu.bus_mut().io_mut().lcd_mut();

        lcd.write_lcd_y(0x90);

        let screen_on = lcd.get_control().lcd_enabled();

        // let lcd_y = lcd.read_lcd_y();
        // if lcd_y >= 153 {
        //     lcd.write_lcd_y(0);
        // } else {
        //     lcd.write_lcd_y(lcd_y + 1);
        // }

        if screen_on {
            self.cpu.bus_mut().render()
        } else {
            self.empty_display.clone()
        }
    }

    pub fn breakpoint_reached(&self) -> Option<u16> {
        let pc = self.cpu.execution_state().instruction_pointer();

        for breakpoint in &self.breakpoints {
            if *breakpoint == pc {
                return Some(*breakpoint);
            }
        }

        None
    }

    fn off_display() -> Vec<Color32> {
        let mut pixels = Vec::new();

        for _ in 0..DISPLAY_HEIGHT_PIXELS {
            for _ in 0..DISPLAY_WIDTH_PIXELS {
                pixels.push(OFF_COLOR);
            }
        }

        pixels
    }
}


pub fn read_cartridge<P>(path: P) -> Cartridge
where
    P: AsRef<Path>,
{
    let mut cartridge_file = std::fs::File::open(path).unwrap();
    Cartridge::read(&mut cartridge_file).unwrap()
}

pub fn read_boot_rom<P>(path: P) -> BootRom
where
    P: AsRef<Path>,
{
    let mut boot_rom_file = std::fs::File::open(path).unwrap();
    BootRomReader::read(&mut boot_rom_file).unwrap()
}