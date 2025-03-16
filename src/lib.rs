use boot::BootRom;
use cartridge::Cartridge;
use cpu::{
    bus::{MainBus, SharedBus},
    error::Error,
    execution_state::SharedExecutionState,
    Cpu,
};
use eframe::egui::Color32;
use io::SharedIO;
use ppu::{
    oam::ObjectAttributeMemory,
    vram::Vram,
    Ppu,
};

pub mod boot;
pub mod cartridge;
pub mod cpu;
pub mod io;
pub mod memory;
pub mod ppu;

pub const DISPLAY_HEIGHT_PIXELS: usize = 144;
pub const DISPLAY_WIDTH_PIXELS: usize = 160;
pub const DISPLAY_SIZE_PIXELS: &'static [usize; 2] = &[DISPLAY_WIDTH_PIXELS, DISPLAY_HEIGHT_PIXELS];

pub const DARKEST_COLOR: Color32 = Color32::from_rgb(8, 24, 32);
pub const DARKER_COLOR: Color32 = Color32::from_rgb(52, 104, 86);
pub const LIGHTER_COLOR: Color32 = Color32::from_rgb(136, 192, 112);
pub const LIGHTEST_COLOR: Color32 = Color32::from_rgb(224, 248, 208);
pub const OFF_COLOR: Color32 = Color32::from_rgb(133, 159, 88);

pub struct Emulator {
    shared_io: SharedIO,
    cpu: Cpu<SharedBus>,
    ppu: Ppu,
    empty_display: Vec<Color32>,
    breakpoints: Vec<u16>,
}

impl Emulator {
    pub fn new(boot_rom: BootRom, cartridge: Cartridge) -> Self {
        let vram = Vram::new();
        let shared_io = SharedIO::new();
        let oam = ObjectAttributeMemory::new();

        let bus = MainBus::new(
            boot_rom,
            cartridge,
            vram.clone(),
            shared_io.clone(),
            oam.clone(),
        );
        let shared_bus = SharedBus::new(bus);

        let cpu = Cpu::new(shared_bus);
        let ppu = Ppu::new(vram, shared_io.clone(), oam);

        Self {
            shared_io,
            cpu,
            ppu,
            empty_display: Self::off_display(),
            breakpoints: Vec::new(),
        }
    }

    pub fn add_breakpoint(&mut self, address: u16) {
        self.breakpoints.push(address);
    }

    pub fn execution_state(&self) -> SharedExecutionState {
        self.cpu.execution_state()
    }

    pub fn step(&mut self) -> Result<(), Error> {
        self.cpu.execute_one()
    }

    pub fn get_pixels(&mut self) -> Vec<Color32> {
        let mut screen_on = false;

        self.shared_io.with_lcd_mut(|lcd| {
            screen_on = lcd.get_control().lcd_enabled();
            lcd.write_lcd_y(0x90);
            // let lcd_y = lcd.read_lcd_y();
            // if lcd_y >= 153 {
            //     lcd.write_lcd_y(0);
            // } else {
            //     lcd.write_lcd_y(lcd_y + 1);
            // }
        });

        if screen_on {
            self.ppu.render()
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
