#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    BootRomReadFailure,
    InvalidInstruction,
    MemoryFault
}