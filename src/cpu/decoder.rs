use super::{error::Error, instruction::{self, Imm16, Imm8, Instruction, Register16, Register16Memory, Register16Stack, Register8}, memory::Memory, ExecutionState};

#[derive(Debug, Clone)]
pub struct Decoder {

}

impl Decoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn decode_one(&self, state: &ExecutionState, memory: &Memory) -> Result<Instruction, Error> {
        let ip = state.instruction_pointer();
        let opcode_byte = memory.read_u8(state.instruction_pointer())?;

        let opcode = Opcode::try_from(opcode_byte)?;

        let instruction = match opcode {
            Opcode::Nop => Instruction::Nop,
            Opcode::LdReg16 => {
                let r16 = self.read_r16(opcode_byte, 4);
                let imm16 = self.read_imm16(memory, ip + 1)?;
                Instruction::LdReg16(r16, imm16)
            },
            Opcode::XorReg8 => {
                let r8 = self.read_r8(opcode_byte, 0);
                Instruction::XorReg8(r8)
            }
            _ => {
                unimplemented!("Undecoded opcode: {:08b}", opcode_byte)
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

    fn read_imm8(&self, memory: &Memory, address: u16) -> Result<Imm8, Error> {
        let value = memory.read_u8(address)?;
        Ok(Imm8::from(value))
    }

    fn read_imm16(&self, memory: &Memory, address: u16) -> Result<Imm16, Error> {
        let value = memory.read_u16(address)?;
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
    LdSpImm16,
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
    LdReg8Reg,
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
    LdhAMem,
    LdhImmMem,
    LdImmMem,
    LdhMemA,
    LdhImmA,
    LdImmA,
    AddSp,
    LdSpImm8,
    LdSpHl,
    Di,
    Ei,
    // 0xCB-prefixed instructions
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
            } else if value & 0b1111_1000 == 0b1010_1000 {
                Opcode::XorReg8
            } else {
                return Err(Error::InvalidInstruction);
            }
        )
    }
}