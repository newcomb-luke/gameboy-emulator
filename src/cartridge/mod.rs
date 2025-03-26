use std::io::Read;

pub use error::Error;
use header::{CartridgeHeader, CartridgeHeaderReader, ManufacturerCode};

mod error;
pub mod header;
pub mod ram;

const BANK_SIZE: usize = 16 * 1024;

#[derive(Debug, Clone)]
pub struct Cartridge {
    bank0: Box<[u8; BANK_SIZE]>,
    extra_banks: Vec<[u8; BANK_SIZE]>,
    header: CartridgeHeader,
    bank_selected: usize,
}

impl Cartridge {
    pub fn empty() -> Self {
        let bank0 = Box::new([0u8; BANK_SIZE]);
        let bank1 = [0u8; BANK_SIZE];
        let header = CartridgeHeader::new(
            "EMPTY",
            ManufacturerCode::zeroed(),
            header::CgbFlag::No,
            header::NewLicenseeCode::Unknown('\0', '\0'),
            header::SgbFlag::No,
            header::CartridgeType::RomOnly,
            header::RomSize::Size32KiB,
            header::RamSize::NoRam,
            header::DestinationCode::Japan,
            header::OldLicenseeCode::UseNewLicenseeCode,
            0,
            0,
            0,
        );

        Self {
            bank0,
            extra_banks: vec![bank1],
            header,
            bank_selected: 0,
        }
    }

    pub fn read(reader: &mut impl Read) -> Result<Self, Error> {
        let mut bank0 = Box::new([0u8; BANK_SIZE]);
        reader.read_exact(bank0.as_mut_slice()).map_err(|e| Error::from(e))?;

        let mut remaining_rom_bytes = Vec::new();
        reader
            .read_to_end(&mut remaining_rom_bytes)
            .map_err(|e| Error::from(e))?;

        let header = CartridgeHeaderReader::read(bank0.as_slice(), &remaining_rom_bytes)?;

        println!("{:#?}", header);

        if header.cartridge_type() != header::CartridgeType::RomOnly {
            return Err(Error::UnsupportedCartridgeType);
        }

        if (remaining_rom_bytes.len() % BANK_SIZE) != 0 {
            return Err(Error::FileSizeError);
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
