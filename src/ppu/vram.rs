#[derive(Debug, Clone, Copy)]
pub enum ColorId {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    data: [u8; 16],
    colors: [[ColorId; 8]; 8],
}

impl Tile {
    pub fn zeroed() -> Self {
        Self {
            data: [0u8; 16],
            colors: [[ColorId::Zero; 8]; 8],
        }
    }

    pub fn data(&self) -> &[u8; 16] {
        &self.data
    }

    pub fn color_data(&self) -> &[[ColorId; 8]; 8] {
        &self.colors
    }

    pub fn read(&self, index: usize) -> u8 {
        self.data[index]
    }

    pub fn write(&mut self, index: usize, data: u8) {
        self.data[index] = data;

        let row_start = index & 0xFFFE;

        let lo_bits = self.data[row_start];
        let hi_bits = self.data[row_start + 1];

        let row_idx = index / 2;

        for col_idx in 0..8 {
            let mask = 1 << (7 - col_idx);

            let left_bit = (hi_bits & mask) != 0;
            let right_bit = (lo_bits & mask) != 0;

            let color_id = match (left_bit, right_bit) {
                (false, false) => ColorId::Zero,
                (false, true) => ColorId::One,
                (true, false) => ColorId::Two,
                (true, true) => ColorId::Three,
            };

            self.colors[row_idx][col_idx] = color_id;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TileId(u8);

impl TileId {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn zeroed() -> Self {
        Self(0)
    }
}

impl From<u8> for TileId {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<TileId> for u8 {
    fn from(value: TileId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VramBank {}

impl VramBank {}

#[derive(Clone)]
pub struct Vram {
    tiles: [Tile; 384],
    map0: [TileId; 1024],
    map1: [TileId; 1024],
}

impl Vram {
    const TILE_MAP_OFFSET: u16 = 0x1800;
    const TILE_MAP_SIZE: u16 = 0x0400;

    pub fn zeroed() -> Self {
        Self {
            tiles: [Tile::zeroed(); 384],
            map0: [TileId::zeroed(); 1024],
            map1: [TileId::zeroed(); 1024],
        }
    }

    pub fn get_tile(&self, id: TileId) -> &Tile {
        &self.tiles[id.0 as usize]
    }

    pub fn get_map_0(&self) -> &[TileId; 1024] {
        &self.map0
    }

    pub fn get_map_1(&self) -> &[TileId; 1024] {
        &self.map1
    }

    pub fn read_u8(&self, address: u16) -> Result<u8, crate::cpu::error::Error> {
        let vram_addr = address - 0x8000;

        Ok(match vram_addr {
            0x0000..=0x17FF => {
                let tile_index = vram_addr / 16;
                let pixel_index = vram_addr % 16;

                self.tiles[tile_index as usize].read(pixel_index as usize)
            }
            0x1800..=0x1BFF => self.map0[(vram_addr - Self::TILE_MAP_OFFSET) as usize].0,
            0x1C00..=0x1FFF => {
                self.map1[(vram_addr - Self::TILE_MAP_OFFSET - Self::TILE_MAP_SIZE) as usize].0
            }
            _ => {
                return Err(crate::cpu::error::Error::MemoryReadFault(address));
            }
        })
    }

    pub fn write_u8(&mut self, address: u16, data: u8) -> Result<(), crate::cpu::error::Error> {
        let vram_addr = address - 0x8000;

        match vram_addr {
            0x0000..=0x17FF => {
                let tile_index = vram_addr / 16;
                let pixel_index = vram_addr % 16;

                self.tiles[tile_index as usize].write(pixel_index as usize, data);
            }
            0x1800..=0x1BFF => {
                self.map0[(vram_addr - Self::TILE_MAP_OFFSET) as usize].0 = data;
            }
            0x1C00..=0x1FFF => {
                self.map1[(vram_addr - Self::TILE_MAP_OFFSET - Self::TILE_MAP_SIZE) as usize].0 =
                    data;
            }
            _ => {
                return Err(crate::cpu::error::Error::MemoryWriteFault(address, data));
            }
        }

        Ok(())
    }
}
