#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidCartridgeTitle,
    InvalidCartridgeType,
    InvalidCartridgeRomSize,
    InvalidCartridgeRamSize,
    InvalidCartridgeDestinationCode,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
