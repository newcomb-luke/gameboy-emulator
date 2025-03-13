use std::io::Read;

use error::Error;

mod error;

#[derive(Debug, Clone)]
pub struct Cartridge {
    bank0: [u8; 512],
    bank1: [u8; 512],
    header: CartridgeHeader
}

impl Cartridge {
    pub fn read(reader: &mut impl Read) -> Result<Self, Error> {
        let mut bank0 = [0u8; 512];
        reader.read_exact(&mut bank0).map_err(|e| Error::from(e))?;

        let header = CartridgeHeaderReader::read(&bank0)?;

        let mut bank1 = [0u8; 512];
        reader.read_exact(&mut bank1).map_err(|e| Error::from(e))?;

        Ok(Self {
            bank0,
            header,
            bank1
        })
    }

    pub fn header(&self) -> &CartridgeHeader {
        &self.header
    }
    
    pub fn bank0(&self) -> &[u8; 512] {
        &self.bank0
    }
    
    pub fn bank1(&self) -> &[u8; 512] {
        &self.bank1
    }
}

#[derive(Debug, Clone)]
pub struct CartridgeHeader {
    title: String,
    cartridge_type: CartridgeType,
    rom_size: RomSize,
    ram_size: RamSize,
    destination_code: DestinationCode,
    version_number: u8,
    read_header_checksum: u8,
    computed_header_checksum: u8
}

impl CartridgeHeader {
    pub fn new(
        title: impl Into<String>,
        cartridge_type: CartridgeType,
        rom_size: RomSize,
        ram_size: RamSize,
        destination_code: DestinationCode,
        version_number: u8,
        read_header_checksum: u8,
        computed_header_checksum: u8) -> Self {
        Self {
            title: title.into(),
            cartridge_type,
            rom_size,
            ram_size,
            destination_code,
            version_number,
            read_header_checksum,
            computed_header_checksum
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn cartridge_type(&self) -> CartridgeType {
        self.cartridge_type
    }

    pub fn rom_size(&self) -> RomSize {
        self.rom_size
    }

    pub fn ram_size(&self) -> RamSize {
        self.ram_size
    }

    pub fn destination_code(&self) -> DestinationCode {
        self.destination_code
    }

    pub fn version_number(&self) -> u8 {
        self.version_number
    }

    pub fn read_header_checksum(&self) -> u8 {
        self.read_header_checksum
    }

    pub fn computed_header_checksum(&self) -> u8 {
        self.computed_header_checksum
    }

    pub fn header_checksum_valid(&self) -> bool {
        self.read_header_checksum == self.computed_header_checksum
    }
}

struct CartridgeHeaderReader {}

impl CartridgeHeaderReader {
    fn read(bank0: &[u8]) -> Result<CartridgeHeader, Error> {
        let title = Self::read_title(bank0)?;
        let cartridge_type = Self::read_cartridge_type(bank0)?;
        let rom_size = Self::read_rom_size(bank0)?;
        let ram_size = Self::read_ram_size(bank0)?;
        let destination_code = Self::read_destination_code(bank0)?;
        let version_number = Self::read_rom_version_number(bank0);
        let read_header_checksum = Self::read_header_checksum(bank0);
        let computed_header_checksum = Self::calculate_header_checksum(bank0);

        Ok(CartridgeHeader::new(title, cartridge_type, rom_size, ram_size, destination_code, version_number, read_header_checksum, computed_header_checksum))
    }

    fn calculate_header_checksum(bank0: &[u8]) -> u8 {
        let mut checksum: u8 = 0;

        for b in &bank0[0x0134..=0x014C] {
            checksum = checksum.wrapping_sub(*b).wrapping_sub(1);
        }

        checksum
    }

    fn read_header_checksum(bank0: &[u8]) -> u8 {
        bank0[0x014D]
    }

    fn read_rom_version_number(bank0: &[u8]) -> u8 {
        bank0[0x014C]
    }

    fn read_destination_code(bank0: &[u8]) -> Result<DestinationCode, Error> {
        let dest_byte = bank0[0x014A];
        DestinationCode::try_from(dest_byte)
    }

    fn read_ram_size(bank0: &[u8]) -> Result<RamSize, Error> {
        let size_byte = bank0[0x0149];
        RamSize::try_from(size_byte)
    }

    fn read_rom_size(bank0: &[u8]) -> Result<RomSize, Error> {
        let size_byte = bank0[0x0148];
        RomSize::try_from(size_byte)
    }

    fn read_cartridge_type(bank0: &[u8]) -> Result<CartridgeType, Error> {
        let type_byte = bank0[0x0147];
        CartridgeType::try_from(type_byte)
    }

    fn read_title(bank0: &[u8]) -> Result<&'_ str, Error> {
        let start = 0x0134;
        let mut end = start;

        for b in &bank0[start..0x0143] {
            if *b == 0 {
                break;
            }

            end += 1;
        }

        std::str::from_utf8(&bank0[start..end]).map_err(|_| Error::InvalidCartridgeTitle)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestinationCode {
    Japan,
    OverseasOnly
}

impl TryFrom<u8> for DestinationCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x00 => Self::Japan,
            0x01 => Self::OverseasOnly,
            _ => {
                return Err(Error::InvalidCartridgeDestinationCode);
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RamSize {
    NoRam,
    Size8KiB,
    Size32KiB,
    Size64KiB,
    Size128KiB,
}

impl TryFrom<u8> for RamSize {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x00 => Self::NoRam,
            0x02 => Self::Size8KiB,
            0x03 => Self::Size32KiB,
            0x04 => Self::Size128KiB,
            0x05 => Self::Size64KiB,
            _ => {
                return Err(Error::InvalidCartridgeRamSize);
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RomSize {
    Size32KiB,
    Size64KiB,
    Size128KiB,
    Size256KiB,
    Size512KiB,
    Size1MiB,
    Size2MiB,
    Size4MiB,
    Size8MiB,
}

impl TryFrom<u8> for RomSize {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x00 => Self::Size32KiB,
            0x01 => Self::Size64KiB,
            0x02 => Self::Size128KiB,
            0x03 => Self::Size256KiB,
            0x04 => Self::Size512KiB,
            0x05 => Self::Size1MiB,
            0x06 => Self::Size2MiB,
            0x07 => Self::Size4MiB,
            0x08 => Self::Size8MiB,
            _ => {
                return Err(Error::InvalidCartridgeRomSize);
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CartridgeType {
    RomOnly,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery,
    Mbc2,
    Mbc2Battery,
    RomRam,
    RomRamBattery,
    Mmm01,
    Mmm01Ram,
    Mmm01RamBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery,
    Mbc3,
    Mbc3Ram,
    Mbc3RamBattery,
    Mbc5,
    Mbc5Ram,
    Mbc5RamBattery,
    Mbc5Rumble,
    Mbc5RumbleRam,
    Mbc5RumbleRamBattery,
    Mbc6,
    Mbc7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    Huc3,
    Huc1RamBattery
}

impl TryFrom<u8> for CartridgeType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x00 => Self::RomOnly,
            0x01 => Self::Mbc1,
            0x02 => Self::Mbc1Ram,
            0x03 => Self::Mbc1RamBattery,
            0x05 => Self::Mbc2,
            0x06 => Self::Mbc2Battery,
            0x08 => Self::RomRam,
            0x09 => Self::RomRamBattery,
            0x0B => Self::Mmm01,
            0x0C => Self::Mmm01Ram,
            0x0D => Self::Mmm01RamBattery,
            0x0F => Self::Mbc3TimerBattery,
            0x10 => Self::Mbc3TimerRamBattery,
            0x11 => Self::Mbc3,
            0x12 => Self::Mbc3Ram,
            0x13 => Self::Mbc3RamBattery,
            0x19 => Self::Mbc5,
            0x1A => Self::Mbc5Ram,
            0x1B => Self::Mbc5RamBattery,
            0x1C => Self::Mbc5Rumble,
            0x1D => Self::Mbc5RumbleRam,
            0x1E => Self::Mbc5RumbleRamBattery,
            0x20 => Self::Mbc6,
            0x22 => Self::Mbc7SensorRumbleRamBattery,
            0xFC => Self::PocketCamera,
            0xFD => Self::BandaiTama5,
            0xFE => Self::Huc3,
            0xFF => Self::Huc1RamBattery,
            _ => {
                return Err(Error::InvalidCartridgeType);
            }
        })
    }
}