use super::vram::TileId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteSelection {
    Pallete0,
    Pallete1,
}

#[derive(Debug, Clone, Copy)]
pub struct Flags {
    priority: bool,
    y_flip: bool,
    x_flip: bool,
    palette: PaletteSelection,
}

impl Flags {
    pub fn zeroed() -> Self {
        Self {
            priority: false,
            y_flip: false,
            x_flip: false,
            palette: PaletteSelection::Pallete0,
        }
    }

    pub fn priority(&self) -> bool {
        self.priority
    }

    pub fn y_flip(&self) -> bool {
        self.y_flip
    }

    pub fn x_flip(&self) -> bool {
        self.x_flip
    }

    pub fn palette(&self) -> PaletteSelection {
        self.palette
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Self {
            priority: ((value >> 7) & 1) != 0,
            y_flip: ((value >> 6) & 1) != 0,
            x_flip: ((value >> 5) & 1) != 0,
            palette: if ((value >> 4) & 1) == 0 {
                PaletteSelection::Pallete0
            } else {
                PaletteSelection::Pallete1
            },
        }
    }
}

impl From<&Flags> for u8 {
    fn from(value: &Flags) -> Self {
        let mut v = 0;
        v |= if value.priority { 1 << 7 } else { 0 };
        v |= if value.y_flip { 1 << 6 } else { 0 };
        v |= if value.x_flip { 1 << 5 } else { 0 };
        v |= if value.palette == PaletteSelection::Pallete0 {
            0
        } else {
            1 << 4
        };
        v
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ObjectAttributes {
    y_position: u8,
    x_position: u8,
    tile_index: TileId,
    attributes: Flags,
}

impl ObjectAttributes {
    pub fn zeroed() -> Self {
        Self {
            y_position: 0,
            x_position: 0,
            tile_index: TileId::zeroed(),
            attributes: Flags::zeroed(),
        }
    }

    pub fn y_pos(&self) -> u8 {
        self.y_position
    }

    pub fn x_pos(&self) -> u8 {
        self.x_position
    }

    pub fn tile_index(&self) -> TileId {
        self.tile_index
    }

    pub fn attributes(&self) -> Flags {
        self.attributes
    }
}

#[derive(Clone)]
pub struct ObjectAttributeMemory {
    objects: [ObjectAttributes; 40],
}

impl ObjectAttributeMemory {
    pub fn zeroed() -> Self {
        Self {
            objects: [ObjectAttributes::zeroed(); 40],
        }
    }

    pub fn objects(&self) -> &[ObjectAttributes] {
        &self.objects
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        let oam_addr = address - 0xFE00;
        let object_index = (oam_addr / 4) as usize;
        let attribute_index = oam_addr % 4;
        let object = &self.objects[object_index];

        match attribute_index {
            0 => object.y_position,
            1 => object.x_position,
            2 => u8::from(object.tile_index),
            3 => u8::from(&object.attributes),
            _ => panic!(),
        }
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        let oam_addr = address - 0xFE00;
        let object_index = (oam_addr / 4) as usize;
        let attribute_index = oam_addr % 4;
        let object = &mut self.objects[object_index];

        match attribute_index {
            0 => object.y_position = data,
            1 => object.x_position = data,
            2 => object.tile_index = TileId::new(data),
            3 => object.attributes = Flags::from(data),
            _ => panic!(),
        }
    }
}
