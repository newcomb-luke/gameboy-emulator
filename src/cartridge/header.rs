use std::fmt::Display;

use super::error::Error;

use super::BANK_SIZE;

#[derive(Debug, Clone)]
pub struct CartridgeHeader {
    title: String,
    manufacturer_code: ManufacturerCode,
    cgb_flag: CgbFlag,
    new_licensee_code: NewLicenseeCode,
    sgb_flag: SgbFlag,
    cartridge_type: CartridgeType,
    rom_size: RomSize,
    ram_size: RamSize,
    destination_code: DestinationCode,
    old_licensee_code: OldLicenseeCode,
    version_number: u8,
    read_header_checksum: u8,
    computed_header_checksum: u8,
    read_global_checksum: u16,
    computed_global_checksum: u16,
}

impl CartridgeHeader {
    pub fn new(
        title: impl Into<String>,
        manufacturer_code: ManufacturerCode,
        cgb_flag: CgbFlag,
        new_licensee_code: NewLicenseeCode,
        sgb_flag: SgbFlag,
        cartridge_type: CartridgeType,
        rom_size: RomSize,
        ram_size: RamSize,
        destination_code: DestinationCode,
        old_licensee_code: OldLicenseeCode,
        version_number: u8,
        header_checksum: u8,
        global_checksum: u16,
    ) -> Self {
        Self {
            title: title.into(),
            manufacturer_code,
            new_licensee_code,
            sgb_flag,
            cgb_flag,
            cartridge_type,
            rom_size,
            ram_size,
            destination_code,
            old_licensee_code,
            version_number,
            read_header_checksum: header_checksum,
            computed_header_checksum: header_checksum,
            read_global_checksum: global_checksum,
            computed_global_checksum: global_checksum,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn manufacturer_code(&self) -> &ManufacturerCode {
        &self.manufacturer_code
    }

    pub fn new_licensee_code(&self) -> NewLicenseeCode {
        self.new_licensee_code
    }

    pub fn sgb_flag(&self) -> SgbFlag {
        self.sgb_flag
    }

    pub fn cgb_flag(&self) -> CgbFlag {
        self.cgb_flag
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

    pub fn old_licensee_code(&self) -> OldLicenseeCode {
        self.old_licensee_code
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

    pub fn read_global_checksum(&self) -> u16 {
        self.read_global_checksum
    }

    pub fn computed_global_checksum(&self) -> u16 {
        self.computed_global_checksum
    }

    pub fn global_checksum_valid(&self) -> bool {
        self.read_global_checksum == self.computed_global_checksum
    }

    pub fn licensee(&self) -> Licensee {
        match self.old_licensee_code {
            OldLicenseeCode::UseNewLicenseeCode => Licensee::New(self.new_licensee_code),
            c => Licensee::Old(c),
        }
    }
}

pub struct CartridgeHeaderReader {}

impl CartridgeHeaderReader {
    pub fn read(bank0: &[u8], extra_banks: &[u8]) -> Result<CartridgeHeader, Error> {
        let title = Self::read_title(bank0)?;
        let manufacturer_code = Self::read_manufacturer_code(bank0)?;
        let cgb_flag = Self::read_cgb_flag(bank0);
        let new_licensee_code = Self::read_new_licensee_code(bank0);
        let sgb_flag = Self::read_sgb_flag(bank0);
        let cartridge_type = Self::read_cartridge_type(bank0)?;
        let rom_size = Self::read_rom_size(bank0)?;
        let ram_size = Self::read_ram_size(bank0)?;
        let destination_code = Self::read_destination_code(bank0)?;
        let old_licensee_code = Self::read_old_licensee_code(bank0);
        let version_number = Self::read_rom_version_number(bank0);
        let read_header_checksum = Self::read_header_checksum(bank0);
        let computed_header_checksum = Self::calculate_header_checksum(bank0);
        let read_global_checksum = Self::read_global_checksum(bank0);
        let computed_global_checksum = Self::calculate_global_checksum(bank0, extra_banks);

        Ok(CartridgeHeader {
            title: title.to_string(),
            manufacturer_code,
            cgb_flag,
            new_licensee_code,
            sgb_flag,
            cartridge_type,
            rom_size,
            ram_size,
            destination_code,
            old_licensee_code,
            version_number,
            read_header_checksum,
            computed_header_checksum,
            read_global_checksum,
            computed_global_checksum,
        })
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

    fn read_manufacturer_code(bank0: &[u8]) -> Result<ManufacturerCode, Error> {
        let start = 0x013F;

        let mut code = [' '; 4];

        for i in 0..4 {
            code[i] = bank0[start + i] as char;
        }

        Ok(ManufacturerCode::new(code))
    }

    fn read_cgb_flag(bank0: &[u8]) -> CgbFlag {
        let flag_byte = bank0[0x0143];
        CgbFlag::from(flag_byte)
    }

    fn read_new_licensee_code(bank0: &[u8]) -> NewLicenseeCode {
        let code_byte1 = bank0[0x0144];
        let code_byte2 = bank0[0x0145];

        let code = (code_byte1 as char, code_byte2 as char);

        NewLicenseeCode::from(code)
    }

    fn read_sgb_flag(bank0: &[u8]) -> SgbFlag {
        let flag_byte = bank0[0x0146];
        SgbFlag::from(flag_byte)
    }

    fn read_cartridge_type(bank0: &[u8]) -> Result<CartridgeType, Error> {
        let type_byte = bank0[0x0147];
        CartridgeType::try_from(type_byte)
    }

    fn read_rom_size(bank0: &[u8]) -> Result<RomSize, Error> {
        let size_byte = bank0[0x0148];
        RomSize::try_from(size_byte)
    }

    fn read_ram_size(bank0: &[u8]) -> Result<RamSize, Error> {
        let size_byte = bank0[0x0149];
        RamSize::try_from(size_byte)
    }

    fn read_destination_code(bank0: &[u8]) -> Result<DestinationCode, Error> {
        let dest_byte = bank0[0x014A];
        DestinationCode::try_from(dest_byte)
    }

    fn read_old_licensee_code(bank0: &[u8]) -> OldLicenseeCode {
        let code_byte = bank0[0x014B];
        OldLicenseeCode::from(code_byte)
    }

    fn read_rom_version_number(bank0: &[u8]) -> u8 {
        bank0[0x014C]
    }

    fn read_header_checksum(bank0: &[u8]) -> u8 {
        bank0[0x014D]
    }

    fn calculate_header_checksum(bank0: &[u8]) -> u8 {
        let mut checksum: u8 = 0;

        for b in &bank0[0x0134..=0x014C] {
            checksum = checksum.wrapping_sub(*b).wrapping_sub(1);
        }

        checksum
    }

    fn read_global_checksum(bank0: &[u8]) -> u16 {
        let mut bytes = [0u8; 2];

        bytes[0] = bank0[0x014E];
        bytes[1] = bank0[0x014F];

        u16::from_be_bytes(bytes)
    }

    fn calculate_global_checksum(bank0: &[u8], extra_banks: &[u8]) -> u16 {
        let mut checksum: u16 = 0;

        for b in &bank0[0x0000..0x014E] {
            checksum = checksum.wrapping_add((*b) as u16);
        }

        // Exclude the header checksum itself

        for b in &bank0[0x0150..BANK_SIZE] {
            checksum = checksum.wrapping_add((*b) as u16);
        }

        // Add the rest of the ROM too
        for b in extra_banks {
            checksum = checksum.wrapping_add((*b) as u16);
        }

        checksum
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
    Huc1RamBattery,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManufacturerCode {
    code: String,
}

impl ManufacturerCode {
    pub fn new(code: [char; 4]) -> Self {
        Self {
            code: format!("{}{}{}{}", code[0], code[1], code[2], code[3]),
        }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgbFlag {
    No,
    BackwardsCompatible,
    CgbOnly,
}

impl From<u8> for CgbFlag {
    fn from(value: u8) -> Self {
        match value {
            0x80 => Self::BackwardsCompatible,
            0xC0 => Self::CgbOnly,
            _ => Self::No,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestinationCode {
    Japan,
    OverseasOnly,
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
pub enum NewLicenseeCode {
    None,
    NintendoResearchAndDevelopment,
    Capcom,
    ElectronicArts,
    HudsonSoft,
    BAi,
    Kss,
    PlanningOfficeWada,
    PcmComplete,
    SanX,
    Kemco,
    SetaCorporation,
    Viacom,
    Nintendo,
    Bandai,
    OceanSoftwareAcclaimEntertainment,
    Konami,
    HectorSoft,
    Taito,
    Banpresto,
    UbiSoft,
    Altus,
    MalibuInteractive,
    Angel,
    BulletProofSoftware,
    Irem,
    Absolute,
    AcclaimEntertainment,
    Activision,
    SammyUsaCorporation,
    HiTechExpressions,
    Ljn,
    Matchbox,
    Mattel,
    MiltonBradleyCompany,
    TitusInteractive,
    VirginGamesLtd,
    LucasfilmGames,
    OceanSoftware,
    Infogrames,
    InterplayEntertainment,
    Broderbund,
    SculpturedSoftware,
    TheSalesCurveLimited,
    Thq,
    Accolade,
    MisawaEntertainment,
    Lozc,
    TokumaShoten,
    TsukudaOriginal,
    ChunsoftCo,
    VideoSystem,
    Varie,
    YonezawaSpal,
    Kaneko,
    PackInVideo,
    BottomUp,
    KonamiYuGiOh,
    Mto,
    Kodansha,
    Unknown(char, char),
}

impl From<(char, char)> for NewLicenseeCode {
    fn from(value: (char, char)) -> Self {
        let s = format!("{}{}", value.0, value.1);

        match s.as_str() {
            "00" => Self::None,
            "01" => Self::NintendoResearchAndDevelopment,
            "08" => Self::Capcom,
            "13" => Self::ElectronicArts,
            "18" => Self::HudsonSoft,
            "19" => Self::BAi,
            "20" => Self::Kss,
            "22" => Self::PlanningOfficeWada,
            "24" => Self::PcmComplete,
            "25" => Self::SanX,
            "28" => Self::Kemco,
            "29" => Self::SetaCorporation,
            "30" => Self::Viacom,
            "31" => Self::Nintendo,
            "32" => Self::Bandai,
            "33" => Self::OceanSoftwareAcclaimEntertainment,
            "34" => Self::Konami,
            "35" => Self::HectorSoft,
            "37" => Self::Taito,
            "38" => Self::HudsonSoft,
            "39" => Self::Banpresto,
            "41" => Self::UbiSoft,
            "42" => Self::Altus,
            "44" => Self::MalibuInteractive,
            "46" => Self::Angel,
            "47" => Self::BulletProofSoftware,
            "49" => Self::Irem,
            "50" => Self::Absolute,
            "51" => Self::AcclaimEntertainment,
            "52" => Self::Activision,
            "53" => Self::SammyUsaCorporation,
            "54" => Self::Konami,
            "55" => Self::HiTechExpressions,
            "56" => Self::Ljn,
            "57" => Self::Matchbox,
            "58" => Self::Mattel,
            "59" => Self::MiltonBradleyCompany,
            "60" => Self::TitusInteractive,
            "61" => Self::VirginGamesLtd,
            "64" => Self::LucasfilmGames,
            "67" => Self::OceanSoftware,
            "69" => Self::ElectronicArts,
            "70" => Self::Infogrames,
            "71" => Self::InterplayEntertainment,
            "72" => Self::Broderbund,
            "73" => Self::SculpturedSoftware,
            "75" => Self::TheSalesCurveLimited,
            "78" => Self::Thq,
            "79" => Self::Accolade,
            "80" => Self::MisawaEntertainment,
            "83" => Self::Lozc,
            "86" => Self::TokumaShoten,
            "87" => Self::TsukudaOriginal,
            "91" => Self::ChunsoftCo,
            "92" => Self::VideoSystem,
            "93" => Self::OceanSoftwareAcclaimEntertainment,
            "95" => Self::Varie,
            "96" => Self::YonezawaSpal,
            "97" => Self::Kaneko,
            "99" => Self::PackInVideo,
            "9H" => Self::BottomUp,
            "A4" => Self::KonamiYuGiOh,
            "BL" => Self::Mto,
            "DK" => Self::Kodansha,
            _ => Self::Unknown(value.0, value.1),
        }
    }
}

impl Display for NewLicenseeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::None => "None",
            Self::NintendoResearchAndDevelopment => "Nintendo Research & Development 1",
            Self::Capcom => "Capcom",
            Self::ElectronicArts => "EA (Electronic Arts)",
            Self::HudsonSoft => "Hudson Soft",
            Self::BAi => "B-AI",
            Self::Kss => "KSS",
            Self::PlanningOfficeWada => "Planning Office WADA",
            Self::PcmComplete => "PCM Complete",
            Self::SanX => "San-X",
            Self::Kemco => "Kemco",
            Self::SetaCorporation => "SETA Corporation",
            Self::Viacom => "Viacom",
            Self::Nintendo => "Nintendo",
            Self::Bandai => "Bandai",
            Self::OceanSoftwareAcclaimEntertainment => "Ocean Software/Acclaim Entertainment",
            Self::Konami => "Konami",
            Self::HectorSoft => "HectorSoft",
            Self::Taito => "Taito",
            Self::Banpresto => "Banpresto",
            Self::UbiSoft => "Ubi Soft",
            Self::Altus => "Atlus",
            Self::MalibuInteractive => "Malibu Interactive",
            Self::Angel => "Angel",
            Self::BulletProofSoftware => "Bullet-Proof Software",
            Self::Irem => "Irem",
            Self::Absolute => "Absolute",
            Self::AcclaimEntertainment => "Acclaim Entertainment",
            Self::Activision => "Activision",
            Self::SammyUsaCorporation => "Sammy USA Corporation",
            Self::HiTechExpressions => "Hi Tech Expressions",
            Self::Ljn => "LJN",
            Self::Matchbox => "Matchbox",
            Self::Mattel => "Mattel",
            Self::MiltonBradleyCompany => "Milton Bradley Company",
            Self::TitusInteractive => "Titus Interactive",
            Self::VirginGamesLtd => "Virgin Games Ltd.",
            Self::LucasfilmGames => "Lucasfilm Games",
            Self::OceanSoftware => "Ocean Software",
            Self::Infogrames => "Infogrames",
            Self::InterplayEntertainment => "Interplay Entertainment",
            Self::Broderbund => "Broderbund",
            Self::SculpturedSoftware => "Sculptured Software",
            Self::TheSalesCurveLimited => "The Sales Curve Limited",
            Self::Thq => "THQ",
            Self::Accolade => "Accolade",
            Self::MisawaEntertainment => "Misawa Entertainment",
            Self::Lozc => "lozc",
            Self::TokumaShoten => "Tokuma Shoten",
            Self::TsukudaOriginal => "Tsukuda Original",
            Self::ChunsoftCo => "Chunsoft Co.",
            Self::VideoSystem => "Video System",
            Self::Varie => "Varie",
            Self::YonezawaSpal => "Yonezawa/S'Pal",
            Self::Kaneko => "Kaneko",
            Self::PackInVideo => "Pack-In-Video",
            Self::BottomUp => "Bottom Up",
            Self::KonamiYuGiOh => "Konami (Yu-Gi-Oh!)",
            Self::Mto => "MTO",
            Self::Kodansha => "Kodansha",
            Self::Unknown(c1, c2) => {
                return write!(f, "Unknown ({c1}{c2})");
            }
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SgbFlag {
    No,
    Yes,
}

impl From<u8> for SgbFlag {
    fn from(value: u8) -> Self {
        match value {
            0x03 => Self::Yes,
            _ => Self::No,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OldLicenseeCode {
    None,
    Nintendo,
    Capcom,
    HotB,
    Jaleco,
    CoconutsJapan,
    EliteSystems,
    ElectronicArts,
    HudsonSoft,
    ItcEntertainment,
    Yanoman,
    JapanClary,
    VirginGamesLtd,
    PcmComplete,
    SanX,
    Kemco,
    SetaCorporation,
    Infogrames,
    Bandai,
    UseNewLicenseeCode,
    Konami,
    HectorSoft,
    Banpresto,
    EntertainmentInteractive,
    Gremlin,
    UbiSoft,
    Atlus,
    MalibuInteractive,
    Angel,
    SpectrumHoloByte,
    Irem,
    UsGold,
    Absolute,
    AcclaimEntertainment,
    Activision,
    SammyUsaCorporation,
    GameTek,
    ParkPlace,
    Ljn,
    Matchbox,
    MiltonBradleyCompany,
    Mindscape,
    Romstar,
    NaxatSoft,
    Tradewest,
    TitusInteractive,
    OceanSoftware,
    ElectroBrain,
    InterplayEntertainment,
    Broderbund,
    SculpturedSoftware,
    TheSalesCurveLimited,
    Thq,
    Accolade,
    TriffixEntertainment,
    MicroProse,
    LozcG,
    BulletProofSoftware,
    VicTokaiCorp,
    ApeInc,
    IMax,
    ChunsoftCo,
    VideoSystem,
    TsubarayaProductions,
    Varie,
    YonezawaSpal,
    Arc,
    NihonBussan,
    Tecmo,
    Imagineer,
    Nova,
    HoriElectric,
    Kawada,
    Takara,
    TechnosJapan,
    ToeiAnimation,
    Toho,
    Namco,
    AsciiCorporation,
    SquareEnix,
    HalLaboratory,
    Snk,
    PonyCanyon,
    CultureBrain,
    Sunsoft,
    SonyImagesoft,
    SammyCorporation,
    Taito,
    Square,
    DataEast,
    TonkinHouse,
    Koei,
    Ufl,
    UltraGames,
    VapInc,
    UseCorporation,
    Meldac,
    Epoch,
    Athena,
    AsmikAceEntertainment,
    Natsume,
    KingRecords,
    EpicSonyRecords,
    Igs,
    AWave,
    ExtremeEntertainment,
    Unknown(u8),
}

impl From<u8> for OldLicenseeCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::None,
            0x01 => Self::Nintendo,
            0x08 => Self::Capcom,
            0x09 => Self::HotB,
            0x0A => Self::Jaleco,
            0x0B => Self::CoconutsJapan,
            0x0C => Self::EliteSystems,
            0x13 => Self::ElectronicArts,
            0x18 => Self::HudsonSoft,
            0x19 => Self::ItcEntertainment,
            0x1A => Self::Yanoman,
            0x1D => Self::JapanClary,
            0x1F => Self::VirginGamesLtd,
            0x24 => Self::PcmComplete,
            0x25 => Self::SanX,
            0x28 => Self::Kemco,
            0x29 => Self::SetaCorporation,
            0x30 => Self::Infogrames,
            0x31 => Self::Nintendo,
            0x32 => Self::Bandai,
            0x33 => Self::UseNewLicenseeCode,
            0x34 => Self::Konami,
            0x35 => Self::HectorSoft,
            0x39 => Self::Banpresto,
            0x3C => Self::EntertainmentInteractive,
            0x3E => Self::Gremlin,
            0x41 => Self::UbiSoft,
            0x42 => Self::Atlus,
            0x44 => Self::MalibuInteractive,
            0x46 => Self::Angel,
            0x47 => Self::SpectrumHoloByte,
            0x49 => Self::Irem,
            0x4F => Self::UsGold,
            0x50 => Self::Absolute,
            0x51 => Self::AcclaimEntertainment,
            0x52 => Self::Activision,
            0x53 => Self::SammyUsaCorporation,
            0x54 => Self::GameTek,
            0x55 => Self::ParkPlace,
            0x56 => Self::Ljn,
            0x57 => Self::Matchbox,
            0x59 => Self::MiltonBradleyCompany,
            0x5A => Self::Mindscape,
            0x5B => Self::Romstar,
            0x5C => Self::NaxatSoft,
            0x5D => Self::Tradewest,
            0x60 => Self::TitusInteractive,
            0x67 => Self::OceanSoftware,
            0x6F => Self::ElectroBrain,
            0x71 => Self::InterplayEntertainment,
            0x72 => Self::Broderbund,
            0x73 => Self::SculpturedSoftware,
            0x75 => Self::TheSalesCurveLimited,
            0x78 => Self::Thq,
            0x79 => Self::Accolade,
            0x7A => Self::TriffixEntertainment,
            0x7C => Self::MicroProse,
            0x83 => Self::LozcG,
            0x8B => Self::BulletProofSoftware,
            0x8C => Self::VicTokaiCorp,
            0x8E => Self::ApeInc,
            0x8F => Self::IMax,
            0x91 => Self::ChunsoftCo,
            0x92 => Self::VideoSystem,
            0x93 => Self::TsubarayaProductions,
            0x95 => Self::Varie,
            0x96 => Self::YonezawaSpal,
            0x99 => Self::Arc,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for OldLicenseeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::None => "None",
            Self::Nintendo => "Nintendo",
            Self::Capcom => "Capcom",
            Self::HotB => "HOT-B",
            Self::Jaleco => "Jaleco",
            Self::CoconutsJapan => "Coconuts Japan",
            Self::EliteSystems => "Elite Systems",
            Self::ElectronicArts => "EA (Electronic Arts)",
            Self::HudsonSoft => "Hudson Soft",
            Self::ItcEntertainment => "ITC Entertainment",
            Self::Yanoman => "Yanoman",
            Self::JapanClary => "Japan Clary",
            Self::VirginGamesLtd => "Virgin Games Ltd.",
            Self::PcmComplete => "PCM Complete",
            Self::SanX => "San-X",
            Self::Kemco => "Kemco",
            Self::SetaCorporation => "SETA Corporation",
            Self::Infogrames => "Infogrames",
            Self::Bandai => "Bandai",
            Self::UseNewLicenseeCode => "[Use New Licensee Code]",
            Self::Konami => "Konami",
            Self::HectorSoft => "HectorSoft",
            Self::Banpresto => "Banpresto",
            Self::EntertainmentInteractive => "Entertainment Interactive",
            Self::Gremlin => "Gremlin",
            Self::UbiSoft => "Ubi Soft",
            Self::Atlus => "Atlus",
            Self::MalibuInteractive => "Malibu Interactive",
            Self::Angel => "Angel",
            Self::SpectrumHoloByte => "Spectrum HoloByte",
            Self::Irem => "Irem",
            Self::UsGold => "U.S. Gold",
            Self::Absolute => "Absolute",
            Self::AcclaimEntertainment => "Acclaim Entertainment",
            Self::Activision => "Activision",
            Self::SammyUsaCorporation => "Sammy USA Corporation",
            Self::GameTek => "GameTek",
            Self::ParkPlace => "Park Place",
            Self::Ljn => "LJN",
            Self::Matchbox => "Matchbox",
            Self::MiltonBradleyCompany => "Milton Bradley Company",
            Self::Mindscape => "Mindscape",
            Self::Romstar => "Romstar",
            Self::NaxatSoft => "Naxat Soft",
            Self::Tradewest => "Tradewest",
            Self::TitusInteractive => "Titus Interactive",
            Self::OceanSoftware => "Ocean Software",
            Self::ElectroBrain => "Electro Brain",
            Self::InterplayEntertainment => "Interplay Entertainment",
            Self::Broderbund => "Broderbund",
            Self::SculpturedSoftware => "Sculptured Software",
            Self::TheSalesCurveLimited => "The Sales Curve Limited",
            Self::Thq => "THQ",
            Self::Accolade => "Accolade",
            Self::TriffixEntertainment => "Triffix Entertainment",
            Self::MicroProse => "MicroProse",
            Self::LozcG => "LOZC G.",
            Self::BulletProofSoftware => "Bullet-Proof Software",
            Self::VicTokaiCorp => "Vic Tokai Corp.",
            Self::ApeInc => "Ape Inc.",
            Self::IMax => "I’Max",
            Self::ChunsoftCo => "Chunsoft Co.",
            Self::VideoSystem => "Video System",
            Self::TsubarayaProductions => "Tsubaraya Productions",
            Self::Varie => "Varie",
            Self::YonezawaSpal => "Yonezawa/S'Pal",
            Self::Arc => "Arc",
            Self::NihonBussan => "Nihon Bussan",
            Self::Tecmo => "Tecmo",
            Self::Imagineer => "Imagineer",
            Self::Nova => "Nova",
            Self::HoriElectric => "Hori Electric",
            Self::Kawada => "Kawada",
            Self::Takara => "Takara",
            Self::TechnosJapan => "Technos Japan",
            Self::ToeiAnimation => "Toei Animation",
            Self::Toho => "Toho",
            Self::Namco => "Namco",
            Self::AsciiCorporation => "ASCII Corporation",
            Self::SquareEnix => "Square Enix",
            Self::HalLaboratory => "HAL Laboratory",
            Self::Snk => "SNK",
            Self::PonyCanyon => "Pony Canyon",
            Self::CultureBrain => "Culture Brain",
            Self::Sunsoft => "Sunsoft",
            Self::SonyImagesoft => "Sony Imagesoft",
            Self::SammyCorporation => "Sammy Corporation",
            Self::Taito => "Taito",
            Self::Square => "Square",
            Self::DataEast => "Data East",
            Self::TonkinHouse => "Tonkin House",
            Self::Koei => "Koei",
            Self::Ufl => "UFL",
            Self::UltraGames => "Ultra Games",
            Self::VapInc => "VAP, Inc.",
            Self::UseCorporation => "Use Corporation",
            Self::Meldac => "Meldac",
            Self::Epoch => "Epoch",
            Self::Athena => "Athena",
            Self::AsmikAceEntertainment => "Asmik Ace Entertainment",
            Self::Natsume => "Natsume",
            Self::KingRecords => "King Records",
            Self::EpicSonyRecords => "Epic/Sony Records",
            Self::Igs => "IGS",
            Self::AWave => "A Wave",
            Self::ExtremeEntertainment => "Extreme Entertainment",
            Self::Unknown(v) => {
                return write!(f, "Unknown ({:02X})", v);
            }
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Licensee {
    Old(OldLicenseeCode),
    New(NewLicenseeCode),
}

impl Display for Licensee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Old(o) => write!(f, "{}", o),
            Self::New(n) => write!(f, "{}", n),
        }
    }
}
