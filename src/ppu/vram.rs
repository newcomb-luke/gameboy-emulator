use std::{cell::RefCell, rc::Rc};

use crate::cpu::bus::Bus;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    data: [u8; 16],
}

impl Tile {
    pub fn zeroed() -> Self {
        Self { data: [0u8; 16] }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TileId(u8);

impl TileId {
    pub fn zeroed() -> Self {
        Self(0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VramBank {
    tiles: [Tile; 384],
    map0: [TileId; 1024],
    map1: [TileId; 1024],
}

impl VramBank {
    pub fn zeroed() -> Self {
        Self {
            tiles: [Tile::zeroed(); 384],
            map0: [TileId::zeroed(); 1024],
            map1: [TileId::zeroed(); 1024],
        }
    }
}

#[derive(Clone)]
pub struct Vram {
    inner: Rc<RefCell<VramBank>>,
}

impl Vram {
    const TILE_MAP_OFFSET: u16 = 0x1800;
    const TILE_MAP_SIZE: u16 = 0x0400;

    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(VramBank::zeroed())),
        }
    }
}

impl Bus for Vram {
    fn read_u8(&self, address: u16) -> Result<u8, crate::cpu::error::Error> {
        let vram_addr = address - 0x8000;

        Ok(match vram_addr {
            0x0000..=0x17FF => {
                let tile_index = vram_addr / 16;
                let pixel_index = vram_addr % 16;

                self.inner.borrow().tiles[tile_index as usize].data[pixel_index as usize]
            }
            0x1800..=0x1BFF => {
                self.inner.borrow().map0[(vram_addr - Self::TILE_MAP_OFFSET) as usize].0
            }
            0x1C00..=0x1FFF => {
                self.inner.borrow().map1
                    [(vram_addr - Self::TILE_MAP_OFFSET - Self::TILE_MAP_SIZE) as usize]
                    .0
            }
            _ => {
                return Err(crate::cpu::error::Error::MemoryFault(address));
            }
        })
    }

    fn read_u16(&self, address: u16) -> Result<u16, crate::cpu::error::Error> {
        let lower = self.read_u8(address)?;
        let higher = self.read_u8(address + 1)?;

        Ok(((higher as u16) << 8) | lower as u16)
    }

    fn write_u8(&self, address: u16, data: u8) -> Result<(), crate::cpu::error::Error> {
        let vram_addr = address - 0x8000;

        match vram_addr {
            0x0000..=0x17FF => {
                let tile_index = vram_addr / 16;
                let pixel_index = vram_addr % 16;

                self.inner.borrow_mut().tiles[tile_index as usize].data[pixel_index as usize] =
                    data;
            }
            0x1800..=0x1BFF => {
                self.inner.borrow_mut().map0[(vram_addr - Self::TILE_MAP_OFFSET) as usize].0 = data;
            }
            0x1C00..=0x1FFF => {
                self.inner.borrow_mut().map1
                    [(vram_addr - Self::TILE_MAP_OFFSET - Self::TILE_MAP_SIZE) as usize]
                    .0 = data;
            }
            _ => {
                return Err(crate::cpu::error::Error::MemoryFault(address));
            }
        }

        Ok(())
    }

    fn write_u16(&self, address: u16, data: u16) -> Result<(), crate::cpu::error::Error> {
        self.write_u8(address + 1, (data >> 8) as u8)?;
        self.write_u8(address, (data & 0xFF) as u8)
    }
}
