use super::IORegister;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileMapArea {
    /// 9C00-9FFF
    Upper,
    /// 9800-9BFF
    Lower,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileDataArea {
    /// 8800-97FF
    Upper,
    /// 8000-8FFF
    Lower,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObjSize {
    /// 8x8
    Single,
    /// 8x16
    Double,
}

#[derive(Clone, Copy)]
pub struct LcdControl(IORegister);

impl LcdControl {
    pub fn new() -> Self {
        Self(IORegister::new())
    }

    pub fn lcd_enabled(&self) -> bool {
        (self.0 .0 >> 7) != 0
    }

    pub fn window_tile_map_area(&self) -> TileMapArea {
        if ((self.0 .0 >> 6) & 1) == 0 {
            TileMapArea::Lower
        } else {
            TileMapArea::Upper
        }
    }

    pub fn window_enabled(&self) -> bool {
        ((self.0 .0 >> 5) & 1) != 0
    }

    pub fn bg_and_window_tile_data_area(&self) -> TileDataArea {
        if ((self.0 .0 >> 4) & 1) == 0 {
            TileDataArea::Upper
        } else {
            TileDataArea::Lower
        }
    }

    pub fn bg_tile_map_area(&self) -> TileMapArea {
        if ((self.0 .0 >> 3) & 1) == 0 {
            TileMapArea::Lower
        } else {
            TileMapArea::Upper
        }
    }

    pub fn obj_size(&self) -> ObjSize {
        if ((self.0 .0 >> 2) & 1) == 0 {
            ObjSize::Single
        } else {
            ObjSize::Double
        }
    }

    pub fn obj_enabled(&self) -> bool {
        ((self.0 .0 >> 1) & 1) != 0
    }

    pub fn bg_and_window_enabled(&self) -> bool {
        (self.0 .0 & 1) != 0
    }
}

#[derive(Clone, Copy)]
pub struct Lcd {
    control: LcdControl,
    lcd_y: IORegister,
    lcd_y_compare: IORegister,
    status: IORegister,
    scroll_y: IORegister,
    scroll_x: IORegister,
    window_y: IORegister,
    window_x: IORegister,
    background_palette: IORegister,
    obj_palette_0: IORegister,
    obj_palette_1: IORegister,
}

impl Lcd {
    pub fn new() -> Self {
        Self {
            control: LcdControl::new(),
            lcd_y: IORegister::new(),
            lcd_y_compare: IORegister::new(),
            status: IORegister::new(),
            scroll_y: IORegister::new(),
            scroll_x: IORegister::new(),
            window_y: IORegister::new(),
            window_x: IORegister::new(),
            background_palette: IORegister::new(),
            obj_palette_0: IORegister::new(),
            obj_palette_1: IORegister::new(),
        }
    }

    pub fn read_control(&self) -> u8 {
        self.control.0.read()
    }

    pub fn write_control(&mut self, value: u8) {
        self.control.0.write(value);
    }

    pub fn get_control(&self) -> &LcdControl {
        &self.control
    }

    pub fn read_lcd_y(&self) -> u8 {
        self.lcd_y.read()
    }

    pub fn write_lcd_y(&mut self, value: u8) {
        self.lcd_y.write(value);
    }

    pub fn read_lcd_y_compare(&self) -> u8 {
        self.lcd_y_compare.read()
    }

    pub fn write_lcd_y_compare(&mut self, value: u8) {
        self.lcd_y_compare.write(value);
    }

    pub fn read_status(&self) -> u8 {
        self.status.read()
    }

    pub fn write_status(&mut self, value: u8) {
        self.status.write(value);
    }

    pub fn read_scroll_y(&self) -> u8 {
        self.scroll_y.read()
    }

    pub fn write_scroll_y(&mut self, value: u8) {
        self.scroll_y.write(value);
    }

    pub fn read_scroll_x(&self) -> u8 {
        self.scroll_x.read()
    }

    pub fn write_scroll_x(&mut self, value: u8) {
        self.scroll_x.write(value);
    }

    pub fn read_window_y(&self) -> u8 {
        self.window_y.read()
    }

    pub fn write_window_y(&mut self, value: u8) {
        self.window_y.write(value);
    }

    pub fn read_window_x(&self) -> u8 {
        self.window_x.read()
    }

    pub fn write_window_x(&mut self, value: u8) {
        self.window_x.write(value);
    }

    pub fn read_background_palette(&self) -> u8 {
        self.background_palette.read()
    }

    pub fn write_background_palette(&mut self, value: u8) {
        self.background_palette.write(value);
    }

    pub fn read_obj_palette_0(&self) -> u8 {
        self.obj_palette_0.read()
    }

    pub fn write_obj_palette_0(&mut self, value: u8) {
        self.obj_palette_0.write(value);
    }

    pub fn read_obj_palette_1(&self) -> u8 {
        self.obj_palette_1.read()
    }

    pub fn write_obj_palette_1(&mut self, value: u8) {
        self.obj_palette_1.write(value);
    }
}
