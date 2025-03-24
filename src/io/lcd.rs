use crate::{io::IORegister, ppu::PpuMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileMapArea {
    /// 9C00-9FFF
    Upper,
    /// 9800-9BFF
    Lower,
}

impl From<TileMapArea> for u8 {
    fn from(value: TileMapArea) -> Self {
        match value {
            TileMapArea::Lower => 0,
            TileMapArea::Upper => 1,
        }
    }
}

impl From<u8> for TileMapArea {
    fn from(value: u8) -> Self {
        match value {
            0 => TileMapArea::Lower,
            _ => TileMapArea::Upper,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileDataArea {
    /// 8800-97FF
    Upper,
    /// 8000-8FFF
    Lower,
}

impl From<TileDataArea> for u8 {
    fn from(value: TileDataArea) -> Self {
        match value {
            TileDataArea::Lower => 0,
            TileDataArea::Upper => 1,
        }
    }
}

impl From<u8> for TileDataArea {
    fn from(value: u8) -> Self {
        match value {
            0 => TileDataArea::Lower,
            _ => TileDataArea::Upper,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjSize {
    /// 8x8
    Single,
    /// 8x16
    Double,
}

impl From<ObjSize> for u8 {
    fn from(value: ObjSize) -> Self {
        match value {
            ObjSize::Single => 0,
            ObjSize::Double => 1,
        }
    }
}

impl From<u8> for ObjSize {
    fn from(value: u8) -> Self {
        match value {
            0 => ObjSize::Single,
            _ => ObjSize::Double,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub id0: Color,
    pub id1: Color,
    pub id2: Color,
    pub id3: Color,
}

impl From<u8> for Palette {
    fn from(value: u8) -> Self {
        let id0 = Color::from(value & 0b11);
        let id1 = Color::from((value >> 2) & 0b11);
        let id2 = Color::from((value >> 4) & 0b11);
        let id3 = Color::from((value >> 6) & 0b11);

        Self {
            id0,
            id1,
            id2,
            id3
        }
    }
}

impl From<Palette> for u8 {
    fn from(value: Palette) -> Self {
        let mut v = 0;
        v |= u8::from(value.id0);
        v |= u8::from(value.id1) << 2;
        v |= u8::from(value.id2) << 4;
        v |= u8::from(value.id3) << 6;
        v
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            id0: Color::White,
            id1: Color::LightGray,
            id2: Color::DarkGray,
            id3: Color::Black
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    LightGray,
    DarkGray,
    Black
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::White,
            1 => Self::LightGray,
            2 => Self::DarkGray,
            _ => Self::Black
        }
    }
}

impl From<Color> for u8 {
    fn from(value: Color) -> Self {
        match value {
            Color::White => 0,
            Color::LightGray => 1,
            Color::DarkGray => 2,
            Color::Black => 3
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LcdStatus {
    lyc_interrupt_select: bool,
    mode_2_interrupt_select: bool,
    mode_1_interrupt_select: bool,
    mode_0_interrupt_select: bool,
    lyc_equals_ly: bool,
    ppu_mode: PpuMode,
}

impl LcdStatus {
    pub fn new() -> Self {
        Self {
            lyc_interrupt_select: false,
            mode_2_interrupt_select: false,
            mode_1_interrupt_select: false,
            mode_0_interrupt_select: false,
            lyc_equals_ly: false,
            ppu_mode: PpuMode::HBlank,
        }
    }

    pub fn ppu_mode(&self) -> PpuMode {
        self.ppu_mode
    }

    pub fn set_ppu_mode(&mut self, mode: PpuMode) {
        self.ppu_mode = mode;
    }

    pub fn lyc_interrupt_select(&self) -> bool {
        self.lyc_interrupt_select
    }

    pub fn mode_2_interrupt_select(&self) -> bool {
        self.mode_2_interrupt_select
    }

    pub fn mode_1_interrupt_select(&self) -> bool {
        self.mode_1_interrupt_select
    }

    pub fn mode_0_interrupt_select(&self) -> bool {
        self.mode_0_interrupt_select
    }

    pub fn lyc_equals_ly(&self) -> bool {
        self.lyc_equals_ly
    }

    pub fn set_from_u8(&mut self, value: u8) {
        self.lyc_interrupt_select = (value & (1 << 6)) != 0;
        self.mode_2_interrupt_select = (value & (1 << 5)) != 0;
        self.mode_1_interrupt_select = (value & (1 << 4)) != 0;
        self.mode_0_interrupt_select = (value & (1 << 3)) != 0;

        if self.mode_0_interrupt_select
            | self.mode_1_interrupt_select
            | self.mode_2_interrupt_select
        {
            unimplemented!("Mode 0, 1, or 2 interrupts are not yet supported");
        }
    }
}

impl From<&LcdStatus> for u8 {
    fn from(value: &LcdStatus) -> Self {
        let mut v = 0;
        v |= if value.lyc_interrupt_select {
            1 << 6
        } else {
            0
        };
        v |= if value.mode_2_interrupt_select {
            1 << 5
        } else {
            0
        };
        v |= if value.mode_1_interrupt_select {
            1 << 4
        } else {
            0
        };
        v |= if value.mode_0_interrupt_select {
            1 << 3
        } else {
            0
        };
        v |= if value.lyc_equals_ly { 1 << 2 } else { 0 };
        v |= u8::from(value.ppu_mode);
        v
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LcdControl {
    lcd_and_ppu_enable: bool,
    window_tile_map: TileMapArea,
    window_enable: bool,
    bg_and_window_data: TileDataArea,
    bg_tile_map: TileMapArea,
    obj_size: ObjSize,
    obj_enable: bool,
    bg_and_window_enable: bool,
}

impl LcdControl {
    pub fn new() -> Self {
        Self {
            lcd_and_ppu_enable: false,
            window_tile_map: TileMapArea::Lower,
            window_enable: false,
            bg_and_window_data: TileDataArea::Lower,
            bg_tile_map: TileMapArea::Lower,
            obj_size: ObjSize::Single,
            obj_enable: false,
            bg_and_window_enable: false,
        }
    }

    pub fn lcd_enabled(&self) -> bool {
        self.lcd_and_ppu_enable
    }

    pub fn window_tile_map_area(&self) -> TileMapArea {
        self.window_tile_map
    }

    pub fn window_enabled(&self) -> bool {
        self.window_enable
    }

    pub fn bg_and_window_tile_data_area(&self) -> TileDataArea {
        self.bg_and_window_data
    }

    pub fn bg_tile_map_area(&self) -> TileMapArea {
        self.bg_tile_map
    }

    pub fn obj_size(&self) -> ObjSize {
        self.obj_size
    }

    pub fn obj_enabled(&self) -> bool {
        self.obj_enable
    }

    pub fn bg_and_window_enabled(&self) -> bool {
        self.bg_and_window_enable
    }

    pub fn set_from_u8(&mut self, value: u8) {
        self.lcd_and_ppu_enable = (value & (1 << 7)) != 0;
        self.window_tile_map = TileMapArea::from((value >> 6) & 1);
        self.window_enable = (value & (1 << 5)) != 0;
        self.bg_and_window_data = TileDataArea::from((value >> 4) & 1);
        self.bg_tile_map = TileMapArea::from((value >> 3) & 1);
        self.obj_size = ObjSize::from((value >> 2) & 1);
        self.obj_enable = (value & (1 << 1)) != 0;
        self.bg_and_window_enable = (value & 1) != 0;
    }
}

impl From<&LcdControl> for u8 {
    fn from(value: &LcdControl) -> Self {
        let mut v = 0;
        v |= if value.lcd_and_ppu_enable { 1 << 7 } else { 0 };
        v |= u8::from(value.window_tile_map) << 6;
        v |= if value.window_enable { 1 << 5 } else { 0 };
        v |= u8::from(value.bg_and_window_data) << 4;
        v |= u8::from(value.bg_tile_map) << 3;
        v |= u8::from(value.obj_size) << 2;
        v |= if value.obj_enable { 1 << 1 } else { 0 };
        v |= if value.bg_and_window_enable { 1 } else { 0 };
        v
    }
}

#[derive(Clone, Copy)]
pub struct Lcd {
    control: LcdControl,
    lcd_y: IORegister,
    lcd_y_compare: IORegister,
    status: LcdStatus,
    scroll_y: IORegister,
    scroll_x: IORegister,
    window_y: IORegister,
    window_x: IORegister,
    background_palette: Palette,
    obj_palette_0: Palette,
    obj_palette_1: Palette,
}

impl Lcd {
    pub fn new() -> Self {
        Self {
            control: LcdControl::new(),
            lcd_y: IORegister::new(),
            lcd_y_compare: IORegister::new(),
            status: LcdStatus::new(),
            scroll_y: IORegister::new(),
            scroll_x: IORegister::new(),
            window_y: IORegister::new(),
            window_x: IORegister::new(),
            background_palette: Palette::default(),
            obj_palette_0: Palette::default(),
            obj_palette_1: Palette::default(),
        }
    }

    pub fn read_control(&self) -> u8 {
        u8::from(&self.control)
    }

    pub fn write_control(&mut self, value: u8) {
        self.control.set_from_u8(value);
    }

    pub fn control(&self) -> &LcdControl {
        &self.control
    }

    pub fn control_mut(&mut self) -> &mut LcdControl {
        &mut self.control
    }

    pub fn status(&self) -> &LcdStatus {
        &self.status
    }

    pub fn status_mut(&mut self) -> &mut LcdStatus {
        &mut self.status
    }

    pub fn update_lcd_y(&mut self, value: u8) {
        self.lcd_y.write(value);
        self.status.lyc_equals_ly = self.lcd_y.read() == self.lcd_y_compare.read();
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
        u8::from(&self.status)
    }

    pub fn write_status(&mut self, value: u8) {
        self.status.set_from_u8(value);
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

    pub fn background_palette(&self) -> Palette {
        self.background_palette
    }

    pub fn read_background_palette(&self) -> u8 {
        self.background_palette.into()
    }

    pub fn write_background_palette(&mut self, value: u8) {
        self.background_palette = Palette::from(value);
    }

    pub fn obj_palette_0(&self) -> Palette {
        self.obj_palette_0
    }

    pub fn read_obj_palette_0(&self) -> u8 {
        self.obj_palette_0.into()
    }

    pub fn write_obj_palette_0(&mut self, value: u8) {
        self.obj_palette_0 = Palette::from(value);
    }

    pub fn obj_palette_1(&self) -> Palette {
        self.obj_palette_1
    }

    pub fn read_obj_palette_1(&self) -> u8 {
        self.obj_palette_1.into()
    }

    pub fn write_obj_palette_1(&mut self, value: u8) {
        self.obj_palette_1 = Palette::from(value);
    }
}
