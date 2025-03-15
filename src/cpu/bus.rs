use std::{cell::RefCell, rc::Rc};

use crate::{
    boot::BootRom, cartridge::Cartridge, io::SharedIO, memory::ram::HighRam, ppu::vram::Vram,
};

use super::error::Error;

pub trait Bus {
    fn read_u8(&self, address: u16) -> Result<u8, Error>;
    fn read_u16(&self, address: u16) -> Result<u16, Error>;
    fn write_u8(&self, address: u16, data: u8) -> Result<(), Error>;
    fn write_u16(&self, address: u16, data: u16) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct SharedBus {
    inner: Rc<RefCell<MainBus>>,
}

impl SharedBus {
    pub fn new(bus: MainBus) -> Self {
        Self {
            inner: Rc::new(RefCell::new(bus)),
        }
    }
}

impl Bus for SharedBus {
    fn read_u8(&self, address: u16) -> Result<u8, Error> {
        self.inner.borrow().read_u8(address)
    }

    fn read_u16(&self, address: u16) -> Result<u16, Error> {
        self.inner.borrow().read_u16(address)
    }

    fn write_u8(&self, address: u16, data: u8) -> Result<(), Error> {
        self.inner.borrow_mut().write_u8(address, data)
    }

    fn write_u16(&self, address: u16, data: u16) -> Result<(), Error> {
        self.inner.borrow_mut().write_u16(address, data)
    }
}

#[derive(Clone)]
pub struct MainBus {
    boot_rom: BootRom,
    cartridge: Cartridge,
    vram: Vram,
    io: SharedIO,
    high_ram: HighRam,
    boot_rom_enabled: bool,
}

impl MainBus {
    pub fn new(boot_rom: BootRom, cartridge: Cartridge, vram: Vram, io: SharedIO) -> Self {
        Self {
            boot_rom,
            cartridge,
            vram,
            io,
            high_ram: HighRam::new(),
            boot_rom_enabled: true,
        }
    }

    fn read_u8(&self, address: u16) -> Result<u8, Error> {
        Ok(match address {
            0x0000..=0x00FF => {
                if self.boot_rom_enabled {
                    self.boot_rom.contents()[address as usize]
                } else {
                    self.cartridge.bank0()[address as usize]
                }
            }
            0x0100..=0x3FFF => self.cartridge.bank0()[address as usize],
            0x4000..=0x7FFF => self.cartridge.bank1()[(address as usize) - 0x4000],
            0x8000..=0x9FFF => self.vram.read_u8(address)?,
            0xFF00..=0xFF7F => self.io.read_u8(address)?,
            0xFF80..=0xFFFE => self.high_ram.read_u8(address),
            _ => {
                return Err(Error::MemoryFault(address));
            }
        })
    }

    fn read_u16(&self, address: u16) -> Result<u16, Error> {
        let lower = self.read_u8(address)?;
        let higher = self.read_u8(address + 1)?;

        Ok(((higher as u16) << 8) | lower as u16)
    }

    fn write_u8(&mut self, address: u16, data: u8) -> Result<(), Error> {
        Ok(match address {
            0x0000..=0x7FFF => {}
            0x8000..=0x9FFF => self.vram.write_u8(address, data)?,
            0xFF00..=0xFF7F => self.io.write_u8(address, data)?,
            0xFF80..=0xFFFE => self.high_ram.write_u8(address, data),
            _ => {
                return Err(Error::MemoryFault(address));
            }
        })
    }

    fn write_u16(&mut self, address: u16, data: u16) -> Result<(), Error> {
        self.write_u8(address + 1, (data >> 8) as u8)?;
        self.write_u8(address, (data & 0xFF) as u8)
    }
}
