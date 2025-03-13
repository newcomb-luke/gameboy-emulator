use super::{bus::Bus, error::Error, instruction::{BitIndex, Condition, Imm16, Imm8, Instruction, Register16, Register16Memory, Register16Stack, Register8}, ExecutionState};

#[derive(Debug, Clone)]
pub struct Decoder {

}

impl Decoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn decode_one(&self, state: &ExecutionState, bus: & impl Bus) -> Result<Instruction, Error> {
        let ip = state.instruction_pointer();
        let opcode_byte = bus.read_u8(state.instruction_pointer())?;

        let opcode = Opcode::try_from(opcode_byte)?;

        let instruction = match opcode {
            Opcode::Nop => Instruction::Nop,
            Opcode::LdReg16 => {
                let r16 = self.read_r16(opcode_byte, 4);
                let imm16 = self.read_imm16(bus, ip + 1)?;
                Instruction::LdReg16(r16, imm16)
            },
            Opcode::LdMemA => {
                let r16mem = self.read_r16_mem(opcode_byte, 4);
                Instruction::LdMemA(r16mem)
            }
            Opcode::LdAMem => {
                let r16mem = self.read_r16_mem(opcode_byte, 4);
                Instruction::LdAMem(r16mem)
            }
            Opcode::LdImm16Sp => {
                let imm16 = self.read_imm16(bus, ip + 1)?;
                Instruction::LdImm16Sp(imm16)
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
                let imm8 = self.read_imm8(bus, ip + 1)?;
                Instruction::LdReg8Imm(r8,imm8)
            }
            Opcode::Rlca => {
                Instruction::Rlca
            }
            Opcode::JrImm => {
                let imm8 = self.read_imm8(bus, ip + 1)?;
                Instruction::JrImm(imm8)
            }
            Opcode::JrCond => {
                let cond = self.read_cond(opcode_byte);
                let imm8 = self.read_imm8(bus, ip + 1)?;
                Instruction::JrCond(cond, imm8)
            }
            Opcode::XorReg8 => {
                let r8 = self.read_r8(opcode_byte, 0);
                Instruction::XorReg8(r8)
            }
            Opcode::Prefix => {
                let prefixed_byte = bus.read_u8(ip + 1)?;

                let prefixed = Prefixed::try_from(prefixed_byte)?;

                match prefixed {
                    Prefixed::Bit => {
                        let bit_index = self.read_bit_index(prefixed_byte);
                        let r8 = self.read_r8(prefixed_byte, 0);
                        Instruction::Bit(bit_index, r8)
                    }
                    _ => {
                        unimplemented!("Unimplemented CB-prefixed instruction decoding for {:#?}", prefixed);
                    }
                }
            }
            _ => {
                unimplemented!("Unimplemented instruction decoding for {:#?}", opcode);
            }
        };

        Ok(instruction)
    }

    fn read_r16(&self, opcode: u8, bit_offset: u8) -> Register16 {
        let value = (opcode >> bit_offset) & 0b11;

        match value {
            0 => Register16::Bc,
            1 => Register16::De,
            2 => Register16::Hl,
            3 => Register16::Sp,
            _ => unreachable!()
        }
    }

    fn read_r16_stack(&self, opcode: u8, bit_offset: u8) -> Register16Stack {
        let value = (opcode >> bit_offset) & 0b11;

        match value {
            0 => Register16Stack::Bc,
            1 => Register16Stack::De,
            2 => Register16Stack::Hl,
            3 => Register16Stack::Af,
            _ => unreachable!()
        }
    }

    fn read_r16_mem(&self, opcode: u8, bit_offset: u8) -> Register16Memory {
        let value = (opcode >> bit_offset) & 0b11;

        match value {
            0 => Register16Memory::Bc,
            1 => Register16Memory::De,
            2 => Register16Memory::Hli,
            3 => Register16Memory::Hld,
            _ => unreachable!()
        }
    }

    fn read_r8(&self, opcode: u8, bit_offset: u8) -> Register8 {
        let value = (opcode >> bit_offset) & 0b111;

        match value {
            0 => Register8::B,
            1 => Register8::C,
            2 => Register8::D,
            3 => Register8::E,
            4 => Register8::H,
            5 => Register8::L,
            6 => Register8::HlIndirect,
            7 => Register8::A,
            _ => unreachable!()
        }
    }

    fn read_bit_index(&self, prefixed: u8) -> BitIndex {
        let value = (prefixed >> 3) & 0b111;
        BitIndex::from(value)
    }

    fn read_cond(&self, opcode: u8) -> Condition {
        let value = (opcode >> 3) & 0b111;

        match value {
            0 => Condition::Nz,
            1 => Condition::Z,
            2 => Condition::Nc,
            3 => Condition::C,
            _ => unreachable!()
        }
    }

    fn read_imm8(&self, bus: & impl Bus, address: u16) -> Result<Imm8, Error> {
        let value = bus.read_u8(address)?;
        Ok(Imm8::from(value))
    }

    fn read_imm16(&self, bus: & impl Bus, address: u16) -> Result<Imm16, Error> {
        let value = bus.read_u16(address)?;
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
    Add,
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
    LdhImmMem,
    LdImmMem,
    LdhAMem,
    LdhAImm,
    LdAImm,
    AddSp,
    LdSpImm8,
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
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(
            if value == 0 {
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
                Opcode::LdhImmMem
            } else if value == 0b1110_1010 {
                Opcode::LdImmMem
            } else if value == 0b1111_0010 {
                Opcode::LdhAMem
            } else if value == 0b1111_0000 {
                Opcode::LdhAImm
            } else if value == 0b1111_1010 {
                Opcode::LdAImm
            } else if value == 0b1110_1000 {
                Opcode::AddSp
            } else if value == 0b1111_1000 {
                Opcode::LdSpImm8
            } else if value == 0b1111_1001 {
                Opcode::LdSpHl
            } else if value == 0b1111_0011 {
                Opcode::Di
            } else if value == 0b1111_1011 {
                Opcode::Ei
            } else {
                return Err(Error::InvalidInstruction);
            }
        )
    }
}

impl TryFrom<u8> for Prefixed {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(
            if value & 0b1111_1000 == 0b0000_0000 {
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
                return Err(Error::InvalidInstruction);
            }
        )
    }
}