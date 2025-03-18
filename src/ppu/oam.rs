use super::vram::TileId;

#[derive(Debug, Clone, Copy)]
pub struct ObjectAttributes {
    y_position: u8,
    x_position: u8,
    tile_index: TileId,
    attributes: u8,
}

impl ObjectAttributes {
    pub fn zeroed() -> Self {
        Self {
            y_position: 0,
            x_position: 0,
            tile_index: TileId::zeroed(),
            attributes: 0,
        }
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

    pub fn read_u8(&self, address: u16) -> u8 {
        let oam_addr = address - 0xFE00;
        let object_index = (oam_addr / 4) as usize;
        let attribute_index = oam_addr % 4;
        let object = &self.objects[object_index];

        match attribute_index {
            0 => object.y_position,
            1 => object.x_position,
            2 => u8::from(object.tile_index),
            3 => object.attributes,
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
            3 => object.attributes = data,
            _ => panic!(),
        }
    }
}
