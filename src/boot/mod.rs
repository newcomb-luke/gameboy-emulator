use std::io::Read;

use error::Error;

pub mod error;

pub const DEFAULT_BOOT_ROM: BootRom = BootRom::new(*include_bytes!("dmg_boot.bin"));

#[derive(Debug, Clone, Copy)]
pub struct BootRom {
    contents: [u8; 256],
}

impl BootRom {
    pub const fn new(contents: [u8; 256]) -> Self {
        Self { contents }
    }

    pub fn contents(&self) -> &[u8; 256] {
        &self.contents
    }
}

pub struct BootRomReader {}

impl BootRomReader {
    pub fn read(reader: &mut impl Read) -> Result<BootRom, Error> {
        let mut contents = [0u8; 256];
        reader
            .read_exact(&mut contents)
            .map_err(|e| Error::from(e))?;

        Ok(BootRom::new(contents))
    }
}
