use std::{ops::BitOr, path::Path};

use boot::{BootRom, BootRomReader};
use bus::Bus;
use cartridge::Cartridge;
use cpu::{error::Error, execution_state::ExecutionState, Cpu};
use eframe::egui::Color32;

pub mod boot;
pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod io;
pub mod memory;
pub mod ppu;

pub const DISPLAY_HEIGHT_PIXELS: usize = 144;
pub const DISPLAY_WIDTH_PIXELS: usize = 160;
pub const DISPLAY_SIZE_PIXELS: &'static [usize; 2] = &[DISPLAY_WIDTH_PIXELS, DISPLAY_HEIGHT_PIXELS];
pub const TOTAL_PIXELS: usize = DISPLAY_HEIGHT_PIXELS * DISPLAY_WIDTH_PIXELS;

pub const DARKEST_COLOR: Color32 = Color32::from_rgb(8, 24, 32);
pub const DARKER_COLOR: Color32 = Color32::from_rgb(52, 104, 86);
pub const LIGHTER_COLOR: Color32 = Color32::from_rgb(136, 192, 112);
pub const LIGHTEST_COLOR: Color32 = Color32::from_rgb(224, 248, 208);
pub const OFF_COLOR: Color32 = Color32::from_rgb(234, 255, 218);

pub struct Emulator {
    cpu: Cpu,
    empty_display: Box<[Color32; TOTAL_PIXELS]>,
    breakpoints: Vec<u16>,
}

impl Emulator {
    pub fn new(boot_rom: BootRom, cartridge: Cartridge) -> Self {
        let bus = Bus::new(boot_rom, cartridge);

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

    pub fn step(&mut self, input_state: InputState) -> Result<(), Error> {
        self.cpu.bus_mut().io_mut().joypad_mut().set_inputs(input_state);
        self.cpu.execute_one()
    }

    pub fn get_pixels(&mut self) -> &[Color32] {
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
            self.empty_display.as_slice()
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

    fn off_display() -> Box<[Color32; TOTAL_PIXELS]> {
        Box::new([OFF_COLOR; TOTAL_PIXELS])
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DPadButtonState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool
}

impl DPadButtonState {
    pub fn new(up: bool, down: bool, left: bool, right: bool) -> Self {
        Self {
            up,
            down,
            left,
            right
        }
    }

    pub fn empty() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

impl BitOr for DPadButtonState {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            up: self.up | rhs.up,
            down: self.down | rhs.down,
            left: self.left | rhs.left,
            right: self.right | rhs.right,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DPadState {
    None,
    Left,
    Up,
    Right,
    Down,
    LeftUp,
    RightUp,
    LeftDown,
    RightDown
}

impl DPadState {
    pub fn from_buttons(state: DPadButtonState) -> Self {
        let left_or_right = match (state.left, state.right) {
            (true, true) => DPadState::None,
            (true, false) => DPadState::Left,
            (false, true) => DPadState::Right,
            (false, false) => DPadState::None
        };

        let up_or_down = match (state.up, state.down) {
            (true, true) => DPadState::None,
            (true, false) => DPadState::Up,
            (false, true) => DPadState::Down,
            (false, false) => DPadState::None,
        };

        match (left_or_right, up_or_down) {
            (Self::None, Self::None) => Self::None,
            (Self::Left, Self::None) => Self::Left,
            (Self::Right, Self::None) => Self::Right,
            (Self::Left, Self::Up) => Self::LeftUp,
            (Self::Right, Self::Up) => Self::RightUp,
            (Self::Left, Self::Down) => Self::LeftDown,
            (Self::Right, Self::Down) => Self::RightDown,
            (Self::None, Self::Up) => Self::Up,
            (Self::None, Self::Down) => Self::Down,
            _ => panic!()
        }
    }

    pub fn is_left(self) -> bool {
        (self == Self::Left) | (self == Self::LeftDown) | (self == Self::LeftUp)
    }

    pub fn is_right(self) -> bool {
        (self == Self::Right) | (self == Self::RightDown) | (self == Self::RightUp)
    }

    pub fn is_up(self) -> bool {
        (self == Self::Up) | (self == Self::LeftUp) | (self == Self::RightUp)
    }

    pub fn is_down(self) -> bool {
        (self == Self::Down) | (self == Self::LeftDown) | (self == Self::RightDown)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct InputState {
    pub a_pressed: bool,
    pub b_pressed: bool,
    pub start_pressed: bool,
    pub select_pressed: bool,
    pub dpad_state: DPadState
}

impl InputState {
    pub fn empty() -> Self {
        Self {
            a_pressed: false,
            b_pressed: false,
            start_pressed: false,
            select_pressed: false,
            dpad_state: DPadState::None
        }
    }
}