use crate::{boot::BootRom, cartridge::Cartridge};

use super::error::Error;

#[derive(Debug, Clone)]
pub struct Memory {
    boot_rom: BootRom,
    cartridge: Cartridge,
    boot_rom_enabled: bool,
}

impl Memory {
    pub fn new(boot_rom: BootRom, cartridge: Cartridge) -> Self {
        Self {
            boot_rom,
            cartridge,
            boot_rom_enabled: true,
        }
    }

    pub fn read_u8(&self, address: u16) -> Result<u8, Error> {
        Ok(match address {
            0x0000..=0x00FF => {
                if self.boot_rom_enabled {
                    self.boot_rom.contents()[address as usize]
                } else {
                    self.cartridge.bank0()[address as usize]
                }
            },
            0x0100..=0x3FFF => {
                self.cartridge.bank0()[address as usize]
            },
            _ => {
                return Err(Error::MemoryFault);
            }
        })
    }

    pub fn read_u16(&self, address: u16) -> Result<u16, Error> {
        let lower = self.read_u8(address)?;
        let higher = self.read_u8(address + 1)?;

        Ok(((higher as u16) << 8) | lower as u16)
    }
}