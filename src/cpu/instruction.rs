#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    // Block 0
    Nop,
    /// ld r16, imm16
    LdReg16(Register16, Imm16),
    /// ld [r16mem], a
    LdMemA(Register16Memory),
    /// ld a, [r16mem]
    LdAMem(Register16Memory),
    /// ld [imm8], sp
    LdImm16Sp(Imm16),
    /// inc r16
    Inc16(Register16),
    /// dec r16
    Dec16(Register16),
    /// add hl, r16
    AddHl(Register16),
    /// inc r8
    Inc8(Register8),
    /// dec r8
    Dec8(Register8),
    /// ld r8, imm8
    LdReg8Imm(Register8, Imm8),
    /// rlca
    Rlca,
    /// rrca
    Rrca,
    /// rla
    Rla,
    /// rra
    Rra,
    /// daa
    Daa,
    /// cpl
    Cpl,
    /// scf
    Scf,
    /// ccf
    Ccf,
    /// jr imm8
    JrImm(Imm8),
    /// jr cond, imm8
    JrCond(Condition, Imm8),
    /// stop
    Stop,
    // Block 1
    /// ld r8, r8
    LdReg8Reg8(Register8, Register8),
    /// halt
    Halt,
    // Block 2
    /// add a, r8
    AddReg8(Register8),
    /// adc a, r8
    AdcReg8(Register8),
    /// sub a, r8
    SubReg8(Register8),
    /// sbc a, r8
    SbcReg8(Register8),
    /// and a, r8
    AndReg8(Register8),
    /// xor a, r8
    XorReg8(Register8),
    /// or a, r8
    OrReg8(Register8),
    /// cp a, r8
    CpReg8(Register8),
    // Block 3
    /// add a, imm8
    AddImm8(Imm8),
    /// adc a, imm8
    AdcImm8(Imm8),
    /// sub a, imm8
    SubImm8(Imm8),
    /// sbc a, imm8
    SbcImm8(Imm8),
    /// and a, imm8
    AndImm8(Imm8),
    /// xor a, imm8
    XorImm8(Imm8),
    /// or a, imm8
    OrImm8(Imm8),
    /// cp a, imm8
    CpImm8(Imm8),
    /// ret cond
    RetCond(Condition),
    /// ret
    Ret,
    /// reti
    Reti,
    /// jp cond, imm18
    JpCond(Condition, Imm16),
    /// jp imm18
    JpImm(Imm16),
    /// jp hl
    JpHl,
    /// call cond, imm16
    CallCond(Condition, Imm16),
    /// call imm16
    CallImm(Imm16),
    /// rst tgt3
    Rst(Target),
    /// pop r16stk
    Pop(Register16Stack),
    /// push r16stk
    Push(Register16Stack),
    /// ldh [c], a
    LdhMemA,
    /// ldh [imm8], a
    LdhImmA(Imm8),
    /// ld [imm16], a
    LdImmA(Imm16),
    /// ldh a, [c]
    LdhAMem,
    /// ldh a, [imm8]
    LdhAImm(Imm8),
    /// ld a, [imm16]
    LdAImm(Imm16),
    /// add sp, imm8
    AddSp(Imm8),
    /// ld hl, sp + imm8
    LdHlSpImm8(Imm8),
    /// ld sp, hl
    LdSpHl,
    /// di
    Di,
    /// ei
    Ei,
    // 0xCB-prefixed instructions
    /// rlc r8
    Rlc(Register8),
    /// rrc r8
    Rrc(Register8),
    /// rl r8
    Rl(Register8),
    /// rr r8
    Rr(Register8),
    /// sla r8
    Sla(Register8),
    /// sra r8
    Sra(Register8),
    /// swap r8
    Swap(Register8),
    /// srl r8
    Srl(Register8),
    /// bit b3, r8
    Bit(BitIndex, Register8),
    /// res b3, r8
    Res(BitIndex, Register8),
    /// set b3, r8
    Set(BitIndex, Register8),
}

impl Instruction {
    pub fn length(&self) -> u16 {
        match self {
            Self::Nop => 1,
            Self::LdReg16(_, _) => 3,
            Self::LdMemA(_) => 1,
            Self::LdAMem(_) => 1,
            Self::LdImm16Sp(_) => 3,
            Self::Inc16(_) => 1,
            Self::Dec16(_) => 1,
            Self::AddHl(_) => 1,
            Self::Inc8(_) => 1,
            Self::Dec8(_) => 1,
            Self::LdReg8Imm(_, _) => 2,
            Self::Rlca => 1,
            Self::Rrca => 1,
            Self::Rla => 1,
            Self::Rra => 1,
            Self::Daa => 1,
            Self::Cpl => 1,
            Self::Scf => 1,
            Self::Ccf => 1,
            Self::JrImm(_) => 2,
            Self::JrCond(_, _) => 2,
            Self::Stop => 1,
            Self::LdReg8Reg8(_, _) => 1,
            Self::Halt => 1,
            Self::AddReg8(_) => 1,
            Self::AdcReg8(_) => 1,
            Self::SubReg8(_) => 1,
            Self::SbcReg8(_) => 1,
            Self::AndReg8(_) => 1,
            Self::XorReg8(_) => 1,
            Self::OrReg8(_) => 1,
            Self::CpReg8(_) => 1,
            Self::AddImm8(_) => 2,
            Self::AdcImm8(_) => 2,
            Self::SubImm8(_) => 2,
            Self::SbcImm8(_) => 2,
            Self::AndImm8(_) => 2,
            Self::XorImm8(_) => 2,
            Self::OrImm8(_) => 2,
            Self::CpImm8(_) => 2,
            Self::RetCond(_) => 1,
            Self::Ret => 1,
            Self::Reti => 1,
            Self::JpCond(_, _) => 3,
            Self::JpImm(_) => 3,
            Self::JpHl => 1,
            Self::CallCond(_, _) => 3,
            Self::CallImm(_) => 3,
            Self::Rst(_) => 1,
            Self::Pop(_) => 1,
            Self::Push(_) => 1,
            Self::LdhMemA => 1,
            Self::LdhImmA(_) => 2,
            Self::LdImmA(_) => 3,
            Self::LdhAMem => 1,
            Self::LdhAImm(_) => 2,
            Self::LdAImm(_) => 3,
            Self::AddSp(_) => 2,
            Self::LdHlSpImm8(_) => 2,
            Self::LdSpHl => 1,
            Self::Di => 1,
            Self::Ei => 1,
            // 0xCB-Prefixed
            Self::Rlc(_) => 2,
            Self::Rrc(_) => 2,
            Self::Rl(_) => 2,
            Self::Rr(_) => 2,
            Self::Sla(_) => 2,
            Self::Sra(_) => 2,
            Self::Swap(_) => 2,
            Self::Srl(_) => 2,
            Self::Bit(_, _) => 2,
            Self::Res(_, _) => 2,
            Self::Set(_, _) => 2,
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
    HlIndirect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    Nz,
    Z,
    Nc,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitIndex(u8);

impl From<u8> for BitIndex {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<BitIndex> for u8 {
    fn from(value: BitIndex) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Target(u8);

impl From<u8> for Target {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Target> for u8 {
    fn from(value: Target) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl From<Imm8> for i8 {
    fn from(value: Imm8) -> Self {
        value.0 as i8
    }
}

impl From<Imm8> for u16 {
    fn from(value: Imm8) -> Self {
        value.0 as u16
    }
}

impl std::fmt::Debug for Imm8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Imm8")
            .field(&format_args!("0x{:02x}", self.0))
            .finish()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl std::fmt::Debug for Imm16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Imm16")
            .field(&format_args!("0x{:04x}", self.0))
            .finish()
    }
}
