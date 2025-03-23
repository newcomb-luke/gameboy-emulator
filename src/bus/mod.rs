use eframe::egui;

use crate::{
    boot::BootRom,
    cartridge::Cartridge,
    cpu::error::Error,
    io::{interrupts::Interrupt, IO},
    memory::ram::{HighRam, WorkRam},
    ppu::{Ppu, TOTAL_PIXELS},
};

#[derive(Clone)]
pub struct Bus {
    ppu: Ppu,
    boot_rom: BootRom,
    cartridge: Cartridge,
    work_ram: WorkRam,
    io: IO,
    high_ram: HighRam,
}

impl Bus {
    pub fn new(boot_rom: BootRom, cartridge: Cartridge) -> Self {
        Self {
            ppu: Ppu::new(),
            boot_rom,
            cartridge,
            work_ram: WorkRam::new(),
            io: IO::new(),
            high_ram: HighRam::new(),
        }
    }

    pub fn read_u8(&self, address: u16) -> Result<u8, Error> {
        Ok(match address {
            0x0000..=0x00FF => {
                if self.boot_rom_enabled() {
                    self.boot_rom.contents()[address as usize]
                } else {
                    self.cartridge.bank0()[address as usize]
                }
            }
            0x0100..=0x3FFF => self.cartridge.bank0()[address as usize],
            0x4000..=0x7FFF => self.cartridge.bank1()[(address as usize) - 0x4000],
            0x8000..=0x9FFF => self.ppu.vram().read_u8(address)?,
            0xC000..=0xDFFF => self.work_ram.read_u8(address),
            0xFE00..=0xFE9F => self.ppu.oam().read_u8(address),
            0xFF00..=0xFF7F => self.io.read_u8(address)?,
            0xFF80..=0xFFFE => self.high_ram.read_u8(address),
            _ => {
                return Err(Error::MemoryReadFault(address));
            }
        })
    }

    pub fn read_u16(&self, address: u16) -> Result<u16, Error> {
        let lower = self.read_u8(address)?;
        let higher = self.read_u8(address + 1)?;

        Ok(((higher as u16) << 8) | lower as u16)
    }

    pub fn write_u8(&mut self, address: u16, data: u8) -> Result<(), Error> {
        Ok(match address {
            0x0000..=0x7FFF => {}
            0x8000..=0x9FFF => self.ppu.vram_mut().write_u8(address, data)?,
            0xC000..=0xDFFF => self.work_ram.write_u8(address, data),
            0xFE00..=0xFE9F => self.ppu.oam_mut().write_u8(address, data),
            0xFF00..=0xFF7F => self.io.write_u8(address, data)?,
            0xFF80..=0xFFFE => self.high_ram.write_u8(address, data),
            0xFFFF => self.io.write_u8(address, data)?,
            _ => {
                return Err(Error::MemoryWriteFault(address, data));
            }
        })
    }

    pub fn write_u16(&mut self, address: u16, data: u16) -> Result<(), Error> {
        self.write_u8(address + 1, (data >> 8) as u8)?;
        self.write_u8(address, (data & 0xFF) as u8)
    }

    fn boot_rom_enabled(&self) -> bool {
        self.io.boot_rom_enable() == 0
    }

    pub fn step_ppu(&mut self, cycles: usize) -> (Option<Interrupt>, Option<Interrupt>, bool) {
        self.ppu.step(self.io.lcd_mut(), cycles)
    }

    pub fn render(&mut self) -> &[egui::Color32; TOTAL_PIXELS] {
        self.ppu.render(self.io.lcd_mut())
    }

    pub fn io(&self) -> &IO {
        &self.io
    }

    pub fn io_mut(&mut self) -> &mut IO {
        &mut self.io
    }

    pub fn ppu_mut(&mut self) -> &mut Ppu {
        &mut self.ppu
    }
}
