use std::fmt::Display;

use decoder::Decoder;
use error::Error;
use instruction::{Instruction, Register16, Register8};
use memory::Memory;

mod decoder;
mod error;
mod instruction;
pub mod memory;

#[derive(Debug, Clone, Copy)]
pub struct ExecutionState {
    instruction_pointer: u16,
    stack_pointer: u16,
    reg_bc: u16,
    reg_de: u16,
    reg_hl: u16,
    reg_af: u16,
}

impl ExecutionState {
    pub fn new() -> Self {
        Self {
            instruction_pointer: 0,
            stack_pointer: 0xFFFF,
            reg_bc: 0,
            reg_de: 0,
            reg_hl: 0,
            reg_af: 0,
        }
    }

    pub fn instruction_pointer(&self) -> u16 {
        self.instruction_pointer
    }

    pub fn stack_pointer(&self) -> u16 {
        self.stack_pointer
    }

    pub fn carry_flag(&self) -> bool {
        ((self.reg_af >> 4) & 0b1) != 0
    }

    pub fn half_carry_flag(&self) -> bool {
        ((self.reg_af >> 5) & 0b1) != 0
    }

    pub fn subtraction_flag(&self) -> bool {
        ((self.reg_af >> 6) & 0b1) != 0
    }

    pub fn zero_flag(&self) -> bool {
        ((self.reg_af >> 7) & 0b1) != 0
    }
}

impl Display for ExecutionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IP: {:04x} SP: {:04x} BC: {:04x} DE: {:04x} HL: {:04x} AF: {:04x} {}{}{}{}",
                self.instruction_pointer,
                self.stack_pointer,
                self.reg_bc,
                self.reg_de,
                self.reg_hl,
                self.reg_af,
                if self.zero_flag() { 'z' } else { '-' },
                if self.subtraction_flag() { 'n' } else { '-' },
                if self.half_carry_flag() { 'h' } else { '-' },
                if self.carry_flag() { 'c' } else { '-' },
            )
    }
}

#[derive(Debug, Clone)]
pub struct Cpu<'mem> {
    state: ExecutionState,
    memory: &'mem Memory,
    decoder: Decoder
}

impl<'mem> Cpu<'mem> {
    pub fn new(memory: &'mem Memory) -> Self {
        Self {
            state: ExecutionState::new(),
            memory,
            decoder: Decoder::new()
        }
    }

    pub fn execute_one(&mut self) -> Result<(), Error> {
        let current_instruction = self.decoder.decode_one(&self.state, &self.memory)?;

        println!("{:#?}", current_instruction);

        match current_instruction {
            Instruction::Nop => {}
            Instruction::LdReg16(r16, imm16) => {
                self.update_r16(r16, u16::from(imm16));
            },
            Instruction::XorReg8(r8) => {
                let value = self.get_r8(Register8::A) ^ self.get_r8(r8);
                self.update_r8(r8, value);
            }
            _ => todo!()
        }

        self.state.instruction_pointer += current_instruction.length();

        println!("{}", self.state);

        Ok(())
    }

    fn update_r16(&mut self, r16: Register16, value: u16) {
        match r16 {
            Register16::Bc => {
                self.state.reg_bc = value;
            },
            Register16::De => {
                self.state.reg_de = value;
            },
            Register16::Hl => {
                self.state.reg_hl = value;
            },
            Register16::Sp => {
                self.state.stack_pointer = value;
            }
        }
    }

    fn get_r16(&self, r16: Register16) -> u16 {
        match r16 {
            Register16::Bc => {
                self.state.reg_bc
            },
            Register16::De => {
                self.state.reg_de
            },
            Register16::Hl => {
                self.state.reg_hl
            },
            Register16::Sp => {
                self.state.stack_pointer
            }
        }
    }

    fn update_r8(&mut self, r8: Register8, value: u8) {
        match r8 {
            Register8::A => {
                self.state.reg_af = (self.state.reg_af & 0x00FF) | ((value as u16) << 8);
            },
            Register8::B => {
                self.state.reg_bc = (self.state.reg_bc & 0x00FF) | ((value as u16) << 8);
            },
            Register8::C => {
                self.state.reg_bc = (self.state.reg_bc & 0xFF00) | (value as u16);
            },
            Register8::D => {
                self.state.reg_de = (self.state.reg_de & 0x00FF) | ((value as u16) << 8);
            },
            Register8::E => {
                self.state.reg_de = (self.state.reg_de & 0xFF00) | (value as u16);
            },
            Register8::H => {
                self.state.reg_hl = (self.state.reg_hl & 0x00FF) | ((value as u16) << 8);
            },
            Register8::L => {
                self.state.reg_hl = (self.state.reg_hl & 0xFF00) | (value as u16);
            },
            Register8::HlIndirect => {
                // Write r8 to [HL]
                todo!()
            }
        }
    }

    fn get_r8(&self, r8: Register8) -> u8 {
        match r8 {
            Register8::A => {
                (self.state.reg_af >> 8) as u8
            },
            Register8::B => {
                (self.state.reg_bc >> 8) as u8
            },
            Register8::C => {
                (self.state.reg_bc & 0x00FF) as u8
            },
            Register8::D => {
                (self.state.reg_de >> 8) as u8
            },
            Register8::E => {
                (self.state.reg_de & 0x00FF) as u8
            },
            Register8::H => {
                (self.state.reg_hl >> 8) as u8
            },
            Register8::L => {
                (self.state.reg_hl & 0x00FF) as u8
            },
            Register8::HlIndirect => {
                // Write r8 to [HL]
                todo!()
            }
        }
    }
}