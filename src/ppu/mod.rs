use eframe::egui;
use oam::ObjectAttributeMemory;
use vram::{ColorId, Vram};

use crate::{
    io::SharedIO, DARKER_COLOR, DARKEST_COLOR, DISPLAY_HEIGHT_PIXELS, DISPLAY_WIDTH_PIXELS,
    LIGHTER_COLOR, LIGHTEST_COLOR,
};

pub mod oam;
pub mod vram;

pub struct Ppu {
    vram: Vram,
    oam: ObjectAttributeMemory,
    shared_io: SharedIO,
    pixel_buffer: Vec<egui::Color32>,
}

impl Ppu {
    pub fn new(vram: Vram, shared_io: SharedIO, oam: ObjectAttributeMemory) -> Self {
        Self {
            vram,
            oam,
            shared_io,
            pixel_buffer: Self::empty_pixel_buffer(),
        }
    }

    pub fn render(&mut self) -> Vec<egui::Color32> {
        let mut scroll_y = 0;
        let mut scroll_x = 0;

        self.shared_io.with_lcd_mut(|lcd| {
            scroll_y = lcd.read_scroll_y();
            scroll_x = lcd.read_scroll_x();
        });

        let bottom = scroll_y.wrapping_add(143);
        let top = bottom.wrapping_sub(144);
        let right = scroll_x.wrapping_add(159);
        let left = right.wrapping_sub(160);

        let map0 = self.vram.get_map_0();

        for y in 0..DISPLAY_HEIGHT_PIXELS {
            for x in 0..DISPLAY_WIDTH_PIXELS {
                let view_x = (left as usize) + x;
                let view_y = (top as usize) + y;

                let mut tile_location = ((view_y / 8) * 32) + (view_x / 8);
                tile_location = tile_location % 1024;

                let tile_id = map0[tile_location];
                let color_ids = self.vram.get_tile_colors(tile_id);

                let tile_y = view_y % 8;
                let tile_x = view_x % 8;

                let pixel_index = (y * DISPLAY_WIDTH_PIXELS) + x;
                let color_id = color_ids[tile_y][tile_x];
                self.pixel_buffer[pixel_index] = self.color_id_to_color(color_id);
            }
        }

        self.pixel_buffer.clone()
    }

    fn color_id_to_color(&self, color_id: ColorId) -> egui::Color32 {
        match color_id {
            ColorId::Zero => LIGHTEST_COLOR,
            ColorId::One => LIGHTER_COLOR,
            ColorId::Two => DARKER_COLOR,
            ColorId::Three => DARKEST_COLOR,
        }
    }

    fn empty_pixel_buffer() -> Vec<egui::Color32> {
        let mut pixels = Vec::new();

        for _ in 0..DISPLAY_HEIGHT_PIXELS {
            for _ in 0..DISPLAY_WIDTH_PIXELS {
                pixels.push(LIGHTEST_COLOR);
            }
        }

        pixels
    }
}
