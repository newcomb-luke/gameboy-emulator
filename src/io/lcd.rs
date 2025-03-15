use super::IORegister;

#[derive(Clone, Copy)]
pub struct Lcd {
    control: IORegister,
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
            control: IORegister::new(),
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
        self.control.read()
    }

    pub fn write_control(&mut self, value: u8) {
        self.control.write(value);
    }

    pub fn read_lcd_y(&self) -> u8 {
        self.lcd_y.read()
    }

    pub fn write_lcd_y(&mut self, _value: u8) {}

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
