use eframe::egui::Color32;
use oam::{ObjectAttributeMemory, ObjectAttributes, PaletteSelection};
use vram::{ColorId, Vram};

use crate::io::{
    interrupts::Interrupt,
    lcd::{Color, Lcd, ObjSize, Palette, TileMapArea},
};

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
pub const SCANLINES_PER_FRAME: usize = 154;
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
    bg_priority: [bool; TOTAL_PIXELS],
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
            bg_priority: [false; TOTAL_PIXELS],
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

    pub fn step(
        &mut self,
        lcd: &mut Lcd,
        cycles: usize,
    ) -> (Option<Interrupt>, Option<Interrupt>, bool) {
        if !lcd.control().lcd_enabled() {
            return (None, None, false);
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
                0..=20 => PpuMode::OAMScan,
                21..=63 => PpuMode::PixelDraw,
                64.. => PpuMode::HBlank,
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

        if (new_mode == PpuMode::HBlank) & (old_mode != PpuMode::HBlank) {
            self.write_scanline(lcd);
        }

        let mut new_frame = false;

        if self.current_scanline != scanline {
            if lcd.status().lyc_interrupt_select() & lcd.status().lyc_equals_ly() {
                lcd_interrupt = true;
            }

            self.current_scanline = scanline;

            if new_mode == PpuMode::VBlank {
                new_frame = old_mode != PpuMode::VBlank;
            }
        }

        (
            vblank_interrupt.then_some(Interrupt::VBlank),
            lcd_interrupt.then_some(Interrupt::Lcd),
            new_frame,
        )
    }

    fn write_scanline(&mut self, lcd: &mut Lcd) {
        let scroll_y = lcd.read_scroll_y();
        let scroll_x = lcd.read_scroll_x();

        let bottom = scroll_y.wrapping_add(143);
        let top = bottom.wrapping_sub(143);
        let right = scroll_x.wrapping_add(159);
        let left = right.wrapping_sub(159);

        let map = match lcd.control().bg_tile_map_area() {
            TileMapArea::Lower => self.vram.get_map_0(),
            TileMapArea::Upper => self.vram.get_map_1(),
        };

        let bg_palette = lcd.background_palette();
        let data_mode = lcd.control().bg_and_window_tile_data_area();

        let y = self.current_scanline;

        let view_y = ((top as usize) + y) % 256;

        let bg_enabled = lcd.control().bg_and_window_enabled();

        for x in 0..DISPLAY_WIDTH_PIXELS {
            let view_x = ((left as usize) + x) % 256;

            let tile_location = ((view_y / 8) * 32) + (view_x / 8);

            let tile_id = map[tile_location];
            let tile = self.vram.get_tile(data_mode, tile_id);

            let tile_y = view_y % 8;
            let tile_x = view_x % 8;

            let color_ids = tile.color_data();

            let pixel_index = (y * DISPLAY_WIDTH_PIXELS) + x;
            let color_id = color_ids[tile_y][tile_x];

            if bg_enabled {
                self.bg_priority[pixel_index] = color_id != ColorId::Zero;
                self.pixel_buffer[pixel_index] = self.color_id_to_color(bg_palette, color_id);
            } else {
                self.bg_priority[pixel_index] = false;
                self.pixel_buffer[pixel_index] = self.color_id_to_color(bg_palette, ColorId::Zero);
            }
        }

        let obj_size = lcd.control().obj_size();
        let height = match obj_size {
            ObjSize::Single => 8,
            ObjSize::Double => 16,
        };
        let mut line_objects = Vec::new();

        if lcd.control().obj_enabled() {
            for obj in self.oam.objects() {
                if obj.y_pos() < 16 {
                    continue;
                }

                let obj_y = (obj.y_pos() - 16) as usize;

                if (y >= obj_y) & (y < (obj_y + height)) {
                    line_objects.push(*obj);
                }
            }
        }

        line_objects.sort_by(|a, b| a.x_pos().cmp(&b.x_pos()));

        match obj_size {
            ObjSize::Single => {
                for obj in line_objects.iter().take(10).rev() {
                    self.draw_object_8(lcd, *obj, y);
                }
            }
            ObjSize::Double => {
                for obj in line_objects.iter().take(10).rev() {
                    self.draw_object_16(lcd, *obj, y);
                }
            }
        }
    }

    fn draw_object_8(&mut self, lcd: &mut Lcd, obj: ObjectAttributes, y: usize) {
        let obj_palette_0 = lcd.obj_palette_0();
        let obj_palette_1 = lcd.obj_palette_1();

        let obj_y = (obj.y_pos() - 16) as usize;
        let obj_x = (obj.x_pos() - 8) as usize;
        let tile = self.vram.get_tile_upper(obj.tile_index());
        let color_ids = tile.color_data();
        let obj_palette = match obj.attributes().palette() {
            PaletteSelection::Pallete0 => obj_palette_0,
            PaletteSelection::Pallete1 => obj_palette_1,
        };
        let bg_priority = obj.attributes().priority();

        let screen_y = y;
        let y = y - obj_y;

        for x in 0..8 {
            let screen_x = obj_x + x;

            let x = if obj.attributes().x_flip() { 7 - x } else { x };

            let y = if obj.attributes().y_flip() { 7 - y } else { y };

            let color_id = color_ids[y][x];
            let pixel_index = (screen_y * DISPLAY_WIDTH_PIXELS) + screen_x;

            if !(bg_priority & self.bg_priority[pixel_index]) & (color_id != ColorId::Zero) {
                self.pixel_buffer[pixel_index] = self.color_id_to_color(obj_palette, color_id);
            }
        }
    }

    fn draw_object_16(&mut self, lcd: &mut Lcd, obj: ObjectAttributes, y: usize) {
        let obj_palette_0 = lcd.obj_palette_0();
        let obj_palette_1 = lcd.obj_palette_1();

        let obj_y = (obj.y_pos() - 16) as usize;
        let obj_x = (obj.x_pos() - 8) as usize;
        let (top_id, bottom_id) = obj.tile_index().as_double();
        let top = self.vram.get_tile_upper(top_id);
        let bottom = self.vram.get_tile_upper(bottom_id);
        let top_color_ids = top.color_data();
        let bottom_color_ids = bottom.color_data();
        let obj_palette = match obj.attributes().palette() {
            PaletteSelection::Pallete0 => obj_palette_0,
            PaletteSelection::Pallete1 => obj_palette_1,
        };
        let bg_priority = obj.attributes().priority();

        let screen_y = y;
        let y = y - obj_y;

        let y = if obj.attributes().y_flip() {
            15 - y
        } else {
            y
        };

        let color_ids = if y < 8{
            top_color_ids[y]
        } else {
            bottom_color_ids[y % 8]
        };

        for x in 0..8 {
            let screen_x = obj_x + x;

            let x = if obj.attributes().x_flip() { 7 - x } else { x };

            let color_id = color_ids[x];

            let pixel_index = (screen_y * DISPLAY_WIDTH_PIXELS) + screen_x;

            if !(bg_priority & self.bg_priority[pixel_index]) & (color_id != ColorId::Zero) {
                self.pixel_buffer[pixel_index] = self.color_id_to_color(obj_palette, color_id);
            }
        }
    }

    pub fn render(&mut self, lcd: &mut Lcd) -> &[Color32; TOTAL_PIXELS] {
        if !lcd.control().lcd_enabled() {
            return self.off_display.as_ref();
        }

        &self.pixel_buffer
    }

    fn color_id_to_color(&self, palette: Palette, color_id: ColorId) -> Color32 {
        match color_id {
            ColorId::Zero => self.color_to_color32(palette.id0),
            ColorId::One => self.color_to_color32(palette.id1),
            ColorId::Two => self.color_to_color32(palette.id2),
            ColorId::Three => self.color_to_color32(palette.id3),
        }
    }

    fn color_to_color32(&self, color: Color) -> Color32 {
        match color {
            Color::White => LIGHTEST_COLOR,
            Color::LightGray => LIGHTER_COLOR,
            Color::DarkGray => DARKER_COLOR,
            Color::Black => DARKEST_COLOR,
        }
    }

    fn empty_pixel_buffer() -> Box<[Color32; TOTAL_PIXELS]> {
        Box::new([LIGHTEST_COLOR; TOTAL_PIXELS])
    }

    fn off_display() -> Box<[Color32; TOTAL_PIXELS]> {
        Box::new([OFF_COLOR; TOTAL_PIXELS])
    }
}
