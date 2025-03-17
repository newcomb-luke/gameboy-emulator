use std::io::Read;

use error::Error;
use header::{CartridgeHeader, CartridgeHeaderReader};

mod error;
pub mod header;
pub mod ram;

const BANK_SIZE: usize = 16 * 1024;

#[derive(Debug, Clone)]
pub struct Cartridge {
    bank0: [u8; BANK_SIZE],
    extra_banks: Vec<[u8; BANK_SIZE]>,
    header: CartridgeHeader,
    bank_selected: usize,
}

impl Cartridge {
    pub fn read(reader: &mut impl Read) -> Result<Self, Error> {
        let mut bank0 = [0u8; BANK_SIZE];
        reader.read_exact(&mut bank0).map_err(|e| Error::from(e))?;

        let mut remaining_rom_bytes = Vec::new();
        reader
            .read_to_end(&mut remaining_rom_bytes)
            .map_err(|e| Error::from(e))?;

        let header = CartridgeHeaderReader::read(&bank0, &remaining_rom_bytes)?;

        if (remaining_rom_bytes.len() % BANK_SIZE) != 0 {
            panic!("ROM doesn't have a size in a multiple of banks, possibly a not yet supported format: {}", (remaining_rom_bytes.len() + BANK_SIZE));
        }

        let mut extra_banks = Vec::new();

        for chunk in remaining_rom_bytes.chunks_exact(BANK_SIZE) {
            let mut bank = [0u8; BANK_SIZE];
            bank.copy_from_slice(chunk);
            extra_banks.push(bank);
        }

        Ok(Self {
            bank0,
            header,
            extra_banks,
            bank_selected: 0,
        })
    }

    pub fn header(&self) -> &CartridgeHeader {
        &self.header
    }

    pub fn bank0(&self) -> &[u8; BANK_SIZE] {
        &self.bank0
    }

    pub fn bank1(&self) -> &[u8; BANK_SIZE] {
        &self.extra_banks[self.bank_selected]
    }
}
