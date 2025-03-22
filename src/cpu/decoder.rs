use crate::bus::Bus;

use super::{
    error::Error,
    execution_state::ExecutionState,
    instruction::{
        BitIndex, Condition, Imm16, Imm8, Instruction, Register16, Register16Memory,
        Register16Stack, Register8, Target,
    },
};

#[derive(Debug, Clone)]
pub struct Decoder {}

impl Decoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn decode_one(&self, state: &ExecutionState, bus: &Bus) -> Result<Instruction, Error> {
        let ip = state.instruction_pointer();
        let opcode_byte = bus.read_u8(state.instruction_pointer())?;

        let opcode = Opcode::try_from(opcode_byte).map_err(|_| Error::InvalidInstruction(ip))?;

        let instruction = match opcode {
            Opcode::Nop => Instruction::Nop,
            Opcode::LdReg16 => {
                let r16 = self.read_r16(opcode_byte, 4);
                let imm16 = self.read_imm16(bus, ip)?;
                Instruction::LdReg16(r16, imm16)
            }
            Opcode::LdMemA => {
                let r16mem = self.read_r16_mem(opcode_byte, 4);
                Instruction::LdMemA(r16mem)
            }
            Opcode::LdAMem => {
                let r16mem = self.read_r16_mem(opcode_byte, 4);
                Instruction::LdAMem(r16mem)
            }
            Opcode::LdImm16Sp => {
                let imm16 = self.read_imm16(bus, ip)?;
                Instruction::LdImm16Sp(imm16)
            }
            Opcode::Inc16 => {
                let r16 = self.read_r16(opcode_byte, 4);
                Instruction::Inc16(r16)
            }
            Opcode::Dec16 => {
                let r16 = self.read_r16(opcode_byte, 4);
                Instruction::Dec16(r16)
            }
            Opcode::AddHl => {
                let r16 = self.read_r16(opcode_byte, 4);
                Instruction::AddHl(r16)
            }
            Opcode::Inc8 => {
                let r8 = self.read_r8(opcode_byte, 3);
                Instruction::Inc8(r8)
            }
            Opcode::Dec8 => {
                let r8 = self.read_r8(opcode_byte, 3);
                Instruction::Dec8(r8)
            }
            Opcode::LdReg8Imm => {
                let r8 = self.read_r8(opcode_byte, 3);
                let imm8 = self.read_imm8(bus, ip)?;
                Instruction::LdReg8Imm(r8, imm8)
            }
            Opcode::Rlca => Instruction::Rlca,
            Opcode::Rrca => Instruction::Rrca,
            Opcode::Rla => Instruction::Rla,
            Opcode::Rra => Instruction::Rra,
            Opcode::Daa => Instruction::Daa,
            Opcode::Cpl => Instruction::Cpl,
            Opcode::Scf => Instruction::Scf,
            Opcode::Ccf => Instruction::Ccf,
            Opcode::JrImm => {
                let imm8 = self.read_imm8(bus, ip)?;
                Instruction::JrImm(imm8)
            }
            Opcode::JrCond => {
                let cond = self.read_cond(opcode_byte);
                let imm8 = self.read_imm8(bus, ip)?;
                Instruction::JrCond(cond, imm8)
            }
            Opcode::Stop => Instruction::Stop,
            Opcode::LdReg8Reg8 => {
                let src = self.read_r8(opcode_byte, 0);
                let dest = self.read_r8(opcode_byte, 3);
                Instruction::LdReg8Reg8(dest, src)
            }
            Opcode::Halt => Instruction::Halt,
            Opcode::AddReg8
            | Opcode::AdcReg8
            | Opcode::SubReg8
            | Opcode::SbcReg8
            | Opcode::AndReg8
            | Opcode::XorReg8
            | Opcode::OrReg8
            | Opcode::CpReg8 => {
                let r8 = self.read_r8(opcode_byte, 0);

                match opcode {
                    Opcode::AddReg8 => Instruction::AddReg8(r8),
                    Opcode::AdcReg8 => Instruction::AdcReg8(r8),
                    Opcode::SubReg8 => Instruction::SubReg8(r8),
                    Opcode::SbcReg8 => Instruction::SbcReg8(r8),
                    Opcode::AndReg8 => Instruction::AndReg8(r8),
                    Opcode::XorReg8 => Instruction::XorReg8(r8),
                    Opcode::OrReg8 => Instruction::OrReg8(r8),
                    Opcode::CpReg8 => Instruction::CpReg8(r8),
                    _ => panic!(),
                }
            }
            Opcode::AddImm8
            | Opcode::AdcImm8
            | Opcode::SubImm8
            | Opcode::SbcImm8
            | Opcode::AndImm8
            | Opcode::XorImm8
            | Opcode::OrImm8
            | Opcode::CpImm8 => {
                let imm8 = self.read_imm8(bus, ip)?;

                match opcode {
                    Opcode::AddImm8 => Instruction::AddImm8(imm8),
                    Opcode::AdcImm8 => Instruction::AdcImm8(imm8),
                    Opcode::SubImm8 => Instruction::SubImm8(imm8),
                    Opcode::SbcImm8 => Instruction::SbcImm8(imm8),
                    Opcode::AndImm8 => Instruction::AndImm8(imm8),
                    Opcode::XorImm8 => Instruction::XorImm8(imm8),
                    Opcode::OrImm8 => Instruction::OrImm8(imm8),
                    Opcode::CpImm8 => Instruction::CpImm8(imm8),
                    _ => panic!(),
                }
            }
            Opcode::RetCond => {
                let cond = self.read_cond(opcode_byte);
                Instruction::RetCond(cond)
            }
            Opcode::Ret => Instruction::Ret,
            Opcode::Reti => Instruction::Reti,
            Opcode::JpCond => {
                let cond = self.read_cond(opcode_byte);
                let imm16 = self.read_imm16(bus, ip)?;
                Instruction::JpCond(cond, imm16)
            }
            Opcode::JpImm => {
                let imm16 = self.read_imm16(bus, ip)?;
                Instruction::JpImm(imm16)
            }
            Opcode::JpHl => Instruction::JpHl,
            Opcode::CallCond => {
                let cond = self.read_cond(opcode_byte);
                let imm16 = self.read_imm16(bus, ip)?;
                Instruction::CallCond(cond, imm16)
            }
            Opcode::CallImm => {
                let imm16 = self.read_imm16(bus, ip)?;
                Instruction::CallImm(imm16)
            }
            Opcode::Rst => {
                let tgt = self.read_tgt(opcode_byte);
                Instruction::Rst(tgt)
            }
            Opcode::Pop => {
                let r16stk = self.read_r16_stack(opcode_byte);
                Instruction::Pop(r16stk)
            }
            Opcode::Push => {
                let r16stk = self.read_r16_stack(opcode_byte);
                Instruction::Push(r16stk)
            }
            Opcode::Prefix => {
                let prefixed_byte = bus.read_u8(ip + 1)?;
                let prefixed = Prefixed::try_from(prefixed_byte).map_err(|_| Error::InvalidInstruction(ip))?;

                match prefixed {
                    Prefixed::Rlc
                    | Prefixed::Rrc
                    | Prefixed::Rl
                    | Prefixed::Rr
                    | Prefixed::Sla
                    | Prefixed::Sra
                    | Prefixed::Swap
                    | Prefixed::Srl => {
                        let r8 = self.read_r8(prefixed_byte, 0);

                        match prefixed {
                            Prefixed::Rlc => Instruction::Rlc(r8),
                            Prefixed::Rrc => Instruction::Rrc(r8),
                            Prefixed::Rl => Instruction::Rl(r8),
                            Prefixed::Rr => Instruction::Rr(r8),
                            Prefixed::Sla => Instruction::Sla(r8),
                            Prefixed::Sra => Instruction::Sra(r8),
                            Prefixed::Swap => Instruction::Swap(r8),
                            Prefixed::Srl => Instruction::Srl(r8),
                            _ => panic!(),
                        }
                    }
                    Prefixed::Bit | Prefixed::Res | Prefixed::Set => {
                        let bit_index = self.read_bit_index(prefixed_byte);
                        let r8 = self.read_r8(prefixed_byte, 0);

                        match prefixed {
                            Prefixed::Bit => Instruction::Bit(bit_index, r8),
                            Prefixed::Res => Instruction::Res(bit_index, r8),
                            Prefixed::Set => Instruction::Set(bit_index, r8),
                            _ => panic!(),
                        }
                    }
                }
            }
            Opcode::LdhMemA => Instruction::LdhMemA,
            Opcode::LdhImmA => {
                let imm8 = self.read_imm8(bus, ip)?;
                Instruction::LdhImmA(imm8)
            }
            Opcode::LdImmA => {
                let imm16 = self.read_imm16(bus, ip)?;
                Instruction::LdImmA(imm16)
            }
            Opcode::LdhAMem => Instruction::LdhAMem,
            Opcode::LdhAImm => {
                let imm8 = self.read_imm8(bus, ip)?;
                Instruction::LdhAImm(imm8)
            }
            Opcode::LdAImm => {
                let imm16 = self.read_imm16(bus, ip)?;
                Instruction::LdAImm(imm16)
            }
            Opcode::AddSp => {
                let imm8 = self.read_imm8(bus, ip)?;
                Instruction::AddSp(imm8)
            }
            Opcode::LdHlSpImm8 => {
                let imm8 = self.read_imm8(bus, ip)?;
                Instruction::LdHlSpImm8(imm8)
            }
            Opcode::LdSpHl => Instruction::LdSpHl,
            Opcode::Di => Instruction::Di,
            Opcode::Ei => Instruction::Ei,
        };

        Ok(instruction)
    }

    fn read_r16(&self, opcode_byte: u8, bit_offset: u8) -> Register16 {
        let value = (opcode_byte >> bit_offset) & 0b11;

        match value {
            0 => Register16::Bc,
            1 => Register16::De,
            2 => Register16::Hl,
            3 => Register16::Sp,
            _ => unreachable!(),
        }
    }

    fn read_r16_stack(&self, opcode_byte: u8) -> Register16Stack {
        let value = (opcode_byte >> 4) & 0b11;

        match value {
            0 => Register16Stack::Bc,
            1 => Register16Stack::De,
            2 => Register16Stack::Hl,
            3 => Register16Stack::Af,
            _ => unreachable!(),
        }
    }

    fn read_r16_mem(&self, opcode_byte: u8, bit_offset: u8) -> Register16Memory {
        let value = (opcode_byte >> bit_offset) & 0b11;

        match value {
            0 => Register16Memory::Bc,
            1 => Register16Memory::De,
            2 => Register16Memory::Hli,
            3 => Register16Memory::Hld,
            _ => unreachable!(),
        }
    }

    fn read_r8(&self, opcode_byte: u8, bit_offset: u8) -> Register8 {
        let value = (opcode_byte >> bit_offset) & 0b111;

        match value {
            0 => Register8::B,
            1 => Register8::C,
            2 => Register8::D,
            3 => Register8::E,
            4 => Register8::H,
            5 => Register8::L,
            6 => Register8::HlIndirect,
            7 => Register8::A,
            _ => unreachable!(),
        }
    }

    fn read_bit_index(&self, prefixed: u8) -> BitIndex {
        let value = (prefixed >> 3) & 0b111;
        BitIndex::from(value)
    }

    fn read_tgt(&self, opcode_byte: u8) -> Target {
        let value = (opcode_byte >> 3) & 0b111;
        Target::from(value)
    }

    fn read_cond(&self, opcode_byte: u8) -> Condition {
        let value = (opcode_byte >> 3) & 0b11;

        match value {
            0 => Condition::Nz,
            1 => Condition::Z,
            2 => Condition::Nc,
            3 => Condition::C,
            _ => unreachable!(),
        }
    }

    fn read_imm8(&self, bus: &Bus, ip: u16) -> Result<Imm8, Error> {
        let value = bus.read_u8(ip + 1)?;
        Ok(Imm8::from(value))
    }

    fn read_imm16(&self, bus: &Bus, ip: u16) -> Result<Imm16, Error> {
        let value = bus.read_u16(ip + 1)?;
        Ok(Imm16::from(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Block 0
    Nop,
    LdReg16,
    LdMemA,
    LdAMem,
    LdImm16Sp,
    Inc16,
    Dec16,
    AddHl,
    Inc8,
    Dec8,
    LdReg8Imm,
    Rlca,
    Rrca,
    Rla,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,
    JrImm,
    JrCond,
    Stop,
    // Block 1
    LdReg8Reg8,
    Halt,
    // Block 2
    AddReg8,
    AdcReg8,
    SubReg8,
    SbcReg8,
    AndReg8,
    XorReg8,
    OrReg8,
    CpReg8,
    // Block 3
    AddImm8,
    AdcImm8,
    SubImm8,
    SbcImm8,
    AndImm8,
    XorImm8,
    OrImm8,
    CpImm8,
    RetCond,
    Ret,
    Reti,
    JpCond,
    JpImm,
    JpHl,
    CallCond,
    CallImm,
    Rst,
    Pop,
    Push,
    LdhMemA,
    LdhImmA,
    LdImmA,
    LdhAMem,
    LdhAImm,
    LdAImm,
    AddSp,
    LdHlSpImm8,
    LdSpHl,
    Di,
    Ei,
    // 0xCB-prefixed instructions
    Prefix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prefixed {
    Rlc,
    Rrc,
    Rl,
    Rr,
    Sla,
    Sra,
    Swap,
    Srl,
    Bit,
    Res,
    Set,
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(if value == 0 {
            Opcode::Nop
        } else if value & 0b1100_1111 == 0b0000_0001 {
            Opcode::LdReg16
        } else if value & 0b1100_1111 == 0b0000_0010 {
            Opcode::LdMemA
        } else if value & 0b1100_1111 == 0b0000_1010 {
            Opcode::LdAMem
        } else if value & 0b1111_1111 == 0b0000_1000 {
            Opcode::LdImm16Sp
        } else if value & 0b1100_1111 == 0b0000_0011 {
            Opcode::Inc16
        } else if value & 0b1100_1111 == 0b0000_1011 {
            Opcode::Dec16
        } else if value & 0b1100_0111 == 0b0000_0100 {
            Opcode::Inc8
        } else if value & 0b1100_0111 == 0b0000_0101 {
            Opcode::Dec8
        } else if value & 0b1100_0111 == 0b0000_0110 {
            Opcode::LdReg8Imm
        } else if value == 0b0000_0111 {
            Opcode::Rlca
        } else if value == 0b0000_1111 {
            Opcode::Rrca
        } else if value == 0b0001_0111 {
            Opcode::Rla
        } else if value == 0b0001_1111 {
            Opcode::Rra
        } else if value == 0b0010_0111 {
            Opcode::Daa
        } else if value == 0b0010_1111 {
            Opcode::Cpl
        } else if value == 0b0011_0111 {
            Opcode::Scf
        } else if value == 0b0011_1111 {
            Opcode::Ccf
        } else if value == 0b0001_1000 {
            Opcode::JrImm
        } else if value & 0b1110_0111 == 0b0010_0000 {
            Opcode::JrCond
        } else if value == 0b0001_0000 {
            Opcode::Stop
        } else if value & 0b1100_0000 == 0b0100_0000 {
            Opcode::LdReg8Reg8
        } else if value == 0b0111_0110 {
            Opcode::Halt
        } else if value & 0b1111_1000 == 0b1000_0000 {
            Opcode::AddReg8
        } else if value & 0b1111_1000 == 0b1000_1000 {
            Opcode::AdcReg8
        } else if value & 0b1111_1000 == 0b1001_0000 {
            Opcode::SubReg8
        } else if value & 0b1111_1000 == 0b1001_1000 {
            Opcode::SbcReg8
        } else if value & 0b1111_1000 == 0b1010_0000 {
            Opcode::AndReg8
        } else if value & 0b1111_1000 == 0b1010_1000 {
            Opcode::XorReg8
        } else if value & 0b1111_1000 == 0b1011_0000 {
            Opcode::OrReg8
        } else if value & 0b1111_1000 == 0b1011_1000 {
            Opcode::CpReg8
        } else if value == 0b1100_0110 {
            Opcode::AddImm8
        } else if value == 0b1100_1110 {
            Opcode::AdcImm8
        } else if value == 0b1101_0110 {
            Opcode::SubImm8
        } else if value == 0b1101_1110 {
            Opcode::SbcImm8
        } else if value == 0b1110_0110 {
            Opcode::AndImm8
        } else if value == 0b1110_1110 {
            Opcode::XorImm8
        } else if value == 0b1111_0110 {
            Opcode::OrImm8
        } else if value == 0b1111_1110 {
            Opcode::CpImm8
        } else if value & 0b1110_0111 == 0b1100_0000 {
            Opcode::RetCond
        } else if value == 0b1100_1001 {
            Opcode::Ret
        } else if value == 0b1101_1001 {
            Opcode::Reti
        } else if value & 0b1110_0111 == 0b1100_0010 {
            Opcode::JpCond
        } else if value == 0b1100_0011 {
            Opcode::JpImm
        } else if value == 0b1110_1001 {
            Opcode::JpHl
        } else if value & 0b1110_0111 == 0b1100_0100 {
            Opcode::CallCond
        } else if value == 0b1100_1101 {
            Opcode::CallImm
        } else if value & 0b1100_0111 == 0b1100_0111 {
            Opcode::Rst
        } else if value & 0b1100_1111 == 0b1100_0001 {
            Opcode::Pop
        } else if value & 0b1100_1111 == 0b1100_0101 {
            Opcode::Push
        } else if value == 0xCB {
            Opcode::Prefix
        } else if value == 0b1110_0010 {
            Opcode::LdhMemA
        } else if value == 0b1110_0000 {
            Opcode::LdhImmA
        } else if value == 0b1110_1010 {
            Opcode::LdImmA
        } else if value == 0b1111_0010 {
            Opcode::LdhAMem
        } else if value == 0b1111_0000 {
            Opcode::LdhAImm
        } else if value == 0b1111_1010 {
            Opcode::LdAImm
        } else if value == 0b1110_1000 {
            Opcode::AddSp
        } else if value == 0b1111_1000 {
            Opcode::LdHlSpImm8
        } else if value == 0b1111_1001 {
            Opcode::LdSpHl
        } else if value == 0b1111_0011 {
            Opcode::Di
        } else if value == 0b1111_1011 {
            Opcode::Ei
        } else {
            return Err(());
        })
    }
}

impl TryFrom<u8> for Prefixed {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(if value & 0b1111_1000 == 0b0000_0000 {
            Prefixed::Rlc
        } else if value & 0b1111_1000 == 0b0000_1000 {
            Prefixed::Rrc
        } else if value & 0b1111_1000 == 0b0001_0000 {
            Prefixed::Rl
        } else if value & 0b1111_1000 == 0b0001_1000 {
            Prefixed::Rr
        } else if value & 0b1111_1000 == 0b0010_0000 {
            Prefixed::Sla
        } else if value & 0b1111_1000 == 0b0010_1000 {
            Prefixed::Sra
        } else if value & 0b1111_1000 == 0b0011_0000 {
            Prefixed::Swap
        } else if value & 0b1111_1000 == 0b0011_1000 {
            Prefixed::Srl
        } else if value & 0b1100_0000 == 0b0100_0000 {
            Prefixed::Bit
        } else if value & 0b1100_0000 == 0b1000_0000 {
            Prefixed::Res
        } else if value & 0b1100_0000 == 0b1100_0000 {
            Prefixed::Set
        } else {
            return Err(());
        })
    }
}
