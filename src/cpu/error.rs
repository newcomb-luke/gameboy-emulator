use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Error {
    BootRomReadFailure,
    InvalidInstruction,
    MemoryReadFault(u16),
    MemoryWriteFault(u16, u8),
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BootRomReadFailure => write!(f, "BootRomReadFailure"),
            Self::InvalidInstruction => write!(f, "InvalidInstruction"),
            Self::MemoryReadFault(addr) => write!(f, "MemoryReadFault(0x{:04x})", addr),
            Self::MemoryWriteFault(addr, val) => {
                write!(f, "MemoryWriteFault(0x{:04x}, 0x{:02x})", addr, val)
            }
        }
    }
}
