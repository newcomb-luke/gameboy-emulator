
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    // Block 0
    Nop,
    LdReg16(Register16, Imm16),
    LdMemA(Register16Memory),
    LdAMem(Register16Memory),
    LdSpImm16(Imm16),
    Inc16(Register16),
    Dec16(Register16),
    Add(Register16),
    Inc8(Register8),
    Dec8(Register8),
    LdReg8Imm(Register8, Imm8),
    Rlca,
    Rrca,
    Rla,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,
    JrImm(Imm8),
    JrCond(Condition, Imm8),
    Stop,
    // Block 1
    LdReg8Reg(Register8, Register8),
    Halt,
    // Block 2
    AddReg8(Register8),
    AdcReg8(Register8),
    SubReg8(Register8),
    SbcReg8(Register8),
    AndReg8(Register8),
    XorReg8(Register8),
    OrReg8(Register8),
    CpReg8(Register8),
    // Block 3
    AddImm8(Imm8),
    AdcImm8(Imm8),
    SubImm8(Imm8),
    SbcImm8(Imm8),
    AndImm8(Imm8),
    XorImm8(Imm8),
    OrImm8(Imm8),
    CpImm8(Imm8),
    RetCond(Condition),
    Ret,
    Reti,
    JpCond(Condition, Imm16),
    JpImm(Imm16),
    JpHl,
    CallCond(Condition, Imm16),
    CallImm(Imm16),
    Rst(Target),
    Pop(Register16Stack),
    Push(Register16Stack),
    LdhAMem,
    LdhImmMem(Imm8),
    LdImmMem(Imm16),
    LdhMemA,
    LdhImmA(Imm8),
    LdImmA(Imm16),
    AddSp(Imm8),
    LdSpImm8(Imm8),
    LdSpHl,
    Di,
    Ei,
    // 0xCB-prefixed instructions
    Rlc(Register8),
    Rrc(Register8),
    Rl(Register8),
    Rr(Register8),
    Sla(Register8),
    Sra(Register8),
    Swap(Register8),
    Srl(Register8),
    Bit(BitIndex, Register8),
    Res(BitIndex, Register8),
    Set(BitIndex, Register8),
}

impl Instruction {
    pub fn length(&self) -> u16 {
        match self {
            Self::Nop => 1,
            Self::LdReg16(_, _) => 3,
            Self::XorReg8(_) => 1,
            _ => unimplemented!()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register16 {
    Bc,
    De,
    Hl,
    Sp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register16Stack {
    Bc,
    De,
    Hl,
    Af,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register16Memory {
    Bc,
    De,
    Hli,
    Hld,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HlIndirect
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    Nz,
    Z,
    Nc,
    C
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitIndex(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Target(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Imm8(u8);

impl From<u8> for Imm8 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Imm8> for u8 {
    fn from(value: Imm8) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Imm16(u16);

impl From<u16> for Imm16 {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Imm16> for u16 {
    fn from(value: Imm16) -> Self {
        value.0
    }
}