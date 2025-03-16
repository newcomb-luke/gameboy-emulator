use eframe::egui;
use vram::Vram;

use crate::{io::SharedIO, DARKER_COLOR, DARKEST_COLOR, DISPLAY_HEIGHT_PIXELS, DISPLAY_WIDTH_PIXELS, LIGHTER_COLOR, LIGHTEST_COLOR};

pub mod vram;

pub struct Ppu {
    vram: Vram,
    shared_io: SharedIO,
    pixel_buffer: Vec<egui::Color32>
}

impl Ppu {
    pub fn new(vram: Vram, shared_io: SharedIO) -> Self {
        Self { vram, shared_io, pixel_buffer: Self::empty_pixel_buffer() }
    }

    pub fn render(&mut self) -> Vec<egui::Color32> {
        let mut scroll_y = 0;

        self.shared_io.with_lcd_mut(|lcd| {
            scroll_y = lcd.read_scroll_y();
        });

        let map0 = self.vram.get_map_0();

        for y in 0..DISPLAY_HEIGHT_PIXELS {
            for x in 0..DISPLAY_WIDTH_PIXELS {
                let tile_location = ((y / 8) * 32) + (x / 8);
                let tile_id = map0[tile_location];
                let tile = self.vram.get_tile(tile_id);
                let color_ids = tile.as_color_ids();

                let tile_y = y % 8;
                let tile_x = x % 8;

                let pixel_index = (y * DISPLAY_WIDTH_PIXELS) + x;
                let color_id = color_ids[tile_y][tile_x];
                self.pixel_buffer[pixel_index] = self.color_id_to_color(color_id);
            }
        }

        self.pixel_buffer.clone()
    }

    fn color_id_to_color(&self, color_id: u8) -> egui::Color32 {
        match color_id {
            0 => LIGHTEST_COLOR,
            1 => LIGHTER_COLOR,
            2 => DARKER_COLOR,
            _ => DARKEST_COLOR
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
