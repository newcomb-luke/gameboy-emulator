use eframe::egui::Color32;
use oam::ObjectAttributeMemory;
use vram::{ColorId, Vram};

use crate::io::{interrupts::Interrupt, lcd::Lcd};

pub mod oam;
pub mod vram;

pub const DISPLAY_HEIGHT_PIXELS: usize = 144;
pub const DISPLAY_WIDTH_PIXELS: usize = 160;
pub const DISPLAY_SIZE_PIXELS: &'static [usize; 2] = &[DISPLAY_WIDTH_PIXELS, DISPLAY_HEIGHT_PIXELS];
pub const TOTAL_PIXELS: usize = DISPLAY_HEIGHT_PIXELS * DISPLAY_WIDTH_PIXELS;

pub const DARKEST_COLOR: Color32 = Color32::from_rgb(8, 24, 32);
pub const DARKER_COLOR: Color32 = Color32::from_rgb(52, 104, 86);
pub const LIGHTER_COLOR: Color32 = Color32::from_rgb(136, 192, 112);
pub const LIGHTEST_COLOR: Color32 = Color32::from_rgb(224, 248, 208);
pub const OFF_COLOR: Color32 = Color32::from_rgb(234, 255, 218);

pub const SCANLINE_DOTS_LENGTH: usize = 456;
pub const SCANLINE_CYCLES_LENGTH: usize = SCANLINE_DOTS_LENGTH / 4;
pub const SCANLINES_PER_FRAME: usize = 156;
pub const FRAME_CYCLES_LENGTH: usize = SCANLINES_PER_FRAME * SCANLINE_CYCLES_LENGTH;
pub const VBLANK_START_SCANLINE: usize = 144;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PpuMode {
    /// Horizontal Blank (HBlank) or Mode 0
    HBlank,
    /// Vertical Blank (VBlank) or Mode 1
    VBlank,
    /// OAM Scan or Mode 2
    OAMScan,
    /// Drawing pixels or Mode 3
    PixelDraw,
}

impl From<PpuMode> for u8 {
    fn from(value: PpuMode) -> Self {
        match value {
            PpuMode::HBlank => 0,
            PpuMode::VBlank => 1,
            PpuMode::OAMScan => 2,
            PpuMode::PixelDraw => 3,
        }
    }
}

#[derive(Clone)]
pub struct Ppu {
    vram: Vram,
    oam: ObjectAttributeMemory,
    pixel_buffer: Box<[Color32; TOTAL_PIXELS]>,
    off_display: Box<[Color32; TOTAL_PIXELS]>,
    current_cycles: usize,
    current_scanline: usize,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            vram: Vram::zeroed(),
            oam: ObjectAttributeMemory::zeroed(),
            pixel_buffer: Self::empty_pixel_buffer(),
            off_display: Self::off_display(),
            current_cycles: 0,
            current_scanline: 0,
        }
    }

    pub fn vram(&self) -> &Vram {
        &self.vram
    }

    pub fn vram_mut(&mut self) -> &mut Vram {
        &mut self.vram
    }

    pub fn oam(&self) -> &ObjectAttributeMemory {
        &self.oam
    }

    pub fn oam_mut(&mut self) -> &mut ObjectAttributeMemory {
        &mut self.oam
    }

    pub fn step(&mut self, lcd: &mut Lcd, cycles: usize) -> (Option<Interrupt>, Option<Interrupt>) {
        if !lcd.control().lcd_enabled() {
            return (None, None);
        }

        self.current_cycles += cycles;
        self.current_cycles = self.current_cycles % FRAME_CYCLES_LENGTH;

        let scanline = self.current_cycles / SCANLINE_CYCLES_LENGTH;
        let within_scanline = self.current_cycles % SCANLINE_CYCLES_LENGTH;

        lcd.update_lcd_y(scanline as u8);

        let mut vblank_interrupt = false;
        let mut lcd_interrupt = false;

        let old_mode = lcd.status().ppu_mode();

        let new_mode = if scanline >= VBLANK_START_SCANLINE {
            PpuMode::VBlank
        } else {
            match within_scanline {
                0..=80 => PpuMode::OAMScan,
                81..=252 => PpuMode::PixelDraw,
                253.. => PpuMode::HBlank,
            }
        };

        if new_mode != old_mode {
            let status = lcd.status_mut();

            match new_mode {
                PpuMode::HBlank => {
                    if status.mode_0_interrupt_select() {
                        lcd_interrupt = true;
                    }
                }
                PpuMode::VBlank => {
                    vblank_interrupt = true;

                    if status.mode_1_interrupt_select() {
                        lcd_interrupt = true;
                    }
                }
                PpuMode::OAMScan => {
                    if status.mode_2_interrupt_select() {
                        lcd_interrupt = true;
                    }
                }
                PpuMode::PixelDraw => {}
            }

            status.set_ppu_mode(new_mode);
        }

        if lcd.status().lyc_interrupt_select() & lcd.status().lyc_equals_ly() {
            lcd_interrupt = true;
        }

        if self.current_scanline != scanline {
            self.current_scanline = scanline;

            if new_mode != PpuMode::VBlank {
                self.write_scanline(lcd);
            }
        }

        (
            vblank_interrupt.then_some(Interrupt::VBlank),
            lcd_interrupt.then_some(Interrupt::Lcd),
        )
    }

    fn write_scanline(&mut self, lcd: &mut Lcd) {
        let scroll_y = lcd.read_scroll_y();
        let scroll_x = lcd.read_scroll_x();

        let bottom = scroll_y.wrapping_add(143);
        let top = bottom.wrapping_sub(144);
        let right = scroll_x.wrapping_add(159);
        let left = right.wrapping_sub(160);

        let map0 = self.vram.get_map_0();

        let y = self.current_scanline;
        let view_y = (top as usize) + y;

        for x in 0..DISPLAY_WIDTH_PIXELS {
            let view_x = (left as usize) + x;

            let mut tile_location = ((view_y / 8) * 32) + (view_x / 8);
            tile_location = tile_location % 1024;

            let tile_id = map0[tile_location];
            let tile = self.vram.get_tile(tile_id);
            let color_ids = tile.color_data();

            let tile_y = view_y % 8;
            let tile_x = view_x % 8;

            let pixel_index = (y * DISPLAY_WIDTH_PIXELS) + x;
            let color_id = color_ids[tile_y][tile_x];
            self.pixel_buffer[pixel_index] = self.color_id_to_color(color_id);
        }
    }

    pub fn render(&mut self, lcd: &mut Lcd) -> &[Color32; TOTAL_PIXELS] {
        if !lcd.control().lcd_enabled() {
            return self.off_display.as_ref();
        }

        &self.pixel_buffer
    }

    fn color_id_to_color(&self, color_id: ColorId) -> Color32 {
        match color_id {
            ColorId::Zero => LIGHTEST_COLOR,
            ColorId::One => LIGHTER_COLOR,
            ColorId::Two => DARKER_COLOR,
            ColorId::Three => DARKEST_COLOR,
        }
    }

    fn empty_pixel_buffer() -> Box<[Color32; TOTAL_PIXELS]> {
        Box::new([LIGHTER_COLOR; TOTAL_PIXELS])
    }

    fn off_display() -> Box<[Color32; TOTAL_PIXELS]> {
        Box::new([OFF_COLOR; TOTAL_PIXELS])
    }
}
