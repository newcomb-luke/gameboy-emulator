use alu::Alu;
use bus::Bus;
use decoder::Decoder;
use error::Error;
use execution_state::SharedExecutionState;
use instruction::{Condition, Instruction, Register16, Register16Memory, Register8};

pub mod alu;
pub mod bus;
pub mod decoder;
pub mod error;
pub mod execution_state;
pub mod instruction;

pub struct Cpu<B> {
    state: SharedExecutionState,
    bus: B,
    decoder: Decoder,
    alu: Alu,
}

impl<B: Bus> Cpu<B> {
    pub fn new(bus: B) -> Self {
        let state = SharedExecutionState::new();
        Self {
            state: state.clone(),
            bus,
            decoder: Decoder::new(),
            alu: Alu::new(state),
        }
    }

    pub fn execute_one(&mut self) -> Result<(), Error> {
        let current_instruction = self.decoder.decode_one(&self.state, &self.bus)?;
        let mut next_instruction_address = self
            .state
            .instruction_pointer()
            .wrapping_add(current_instruction.length());

        println!("{:#?}", current_instruction);

        match current_instruction {
            Instruction::Nop => {}
            Instruction::LdReg16(r16, imm16) => {
                self.update_r16(r16, imm16.into());
            }
            Instruction::LdMemA(r16mem) => {
                self.update_r16_mem_u8(r16mem, self.get_r8(Register8::A)?)?;
            }
            Instruction::LdAMem(r16mem) => {
                let new_a = self.get_r16_mem_u8(r16mem)?;
                self.update_r8(Register8::A, new_a)?;
            }
            Instruction::LdImm16Sp(imm16) => {
                self.bus
                    .write_u16(imm16.into(), self.state.stack_pointer())?;
            }
            Instruction::Inc16(r16) => {
                self.update_r16(r16, self.alu.inc_u16(self.get_r16(r16)));
            }
            Instruction::Dec16(r16) => {
                self.update_r16(r16, self.alu.dec_u16(self.get_r16(r16)));
            }
            Instruction::AddHl(r16) => {
                let val1 = self.get_r16(Register16::Hl);
                let val2 = self.get_r16(r16);
                let result = self.alu.add_u16(val1, val2);
                self.update_r16(r16, result);
            }
            Instruction::Inc8(r8) => {
                self.update_r8(r8, self.alu.inc_u8(self.get_r8(r8)?))?;
            }
            Instruction::Dec8(r8) => {
                self.update_r8(r8, self.alu.dec_u8(self.get_r8(r8)?))?;
            }
            Instruction::LdReg8Imm(r8, imm8) => {
                self.update_r8(r8, imm8.into())?;
            }
            Instruction::Rlca => {
                self.update_r8(
                    Register8::A,
                    self.alu.rotate_left_u8(self.get_r8(Register8::A)?),
                )?;
            }
            Instruction::JrImm(imm8) => {
                next_instruction_address =
                    self.rel_jump_dest(imm8.into(), current_instruction.length());
            }
            Instruction::JrCond(cond, imm8) => {
                let dest = self.rel_jump_dest(imm8.into(), current_instruction.length());

                if self.is_condition_met(cond) {
                    next_instruction_address = dest;
                }
            }
            Instruction::XorReg8(r8) => {
                let val = self.get_r8(r8)?;
                let result = self.alu.xor_u8(val, self.get_r8(Register8::A)?);
                self.update_r8(r8, result)?;
            }
            Instruction::Bit(idx, r8) => {
                let val = self.get_r8(r8)?;
                self.alu.test_bit_u8(idx.into(), val);
            }
            _ => unimplemented!(
                "Instruction execution not yet implemented for {:#?}",
                current_instruction
            ),
        }

        self.state.set_instruction_pointer(next_instruction_address);

        println!("{}", self.state);

        Ok(())
    }

    fn is_condition_met(&self, cond: Condition) -> bool {
        match cond {
            Condition::Nz => !self.state.flags().zero,
            Condition::Z => self.state.flags().zero,
            Condition::Nc => !self.state.flags().carry,
            Condition::C => self.state.flags().carry,
        }
    }

    fn rel_jump_dest(&self, offset: i8, instr_len: u16) -> u16 {
        let address_after = self.state.instruction_pointer().wrapping_add(instr_len);
        ((address_after as i32 + (offset as i32)) & 0xFFFF) as u16
    }

    fn update_r16_mem_u16(&mut self, r16mem: Register16Memory, value: u16) -> Result<(), Error> {
        match r16mem {
            Register16Memory::Bc => {
                self.bus.write_u16(self.state.reg_bc(), value)?;
            }
            Register16Memory::De => {
                self.bus.write_u16(self.state.reg_de(), value)?;
            }
            Register16Memory::Hli | Register16Memory::Hld => {
                self.bus.write_u16(self.state.reg_hl(), value)?;
            }
        }

        self.after_r16_mem(r16mem);

        Ok(())
    }

    fn update_r16_mem_u8(&mut self, r16mem: Register16Memory, value: u8) -> Result<(), Error> {
        match r16mem {
            Register16Memory::Bc => {
                self.bus.write_u8(self.state.reg_bc(), value)?;
            }
            Register16Memory::De => {
                self.bus.write_u8(self.state.reg_de(), value)?;
            }
            Register16Memory::Hli | Register16Memory::Hld => {
                self.bus.write_u8(self.state.reg_hl(), value)?;
            }
        }

        self.after_r16_mem(r16mem);

        Ok(())
    }

    fn get_r16_mem_u8(&mut self, r16mem: Register16Memory) -> Result<u8, Error> {
        let val = match r16mem {
            Register16Memory::Bc => self.bus.read_u8(self.state.reg_bc()),
            Register16Memory::De => self.bus.read_u8(self.state.reg_de()),
            Register16Memory::Hli | Register16Memory::Hld => self.bus.read_u8(self.state.reg_hl()),
        }?;

        self.after_r16_mem(r16mem);

        Ok(val)
    }

    fn get_r16_mem_u16(&mut self, r16mem: Register16Memory) -> Result<u16, Error> {
        let val = match r16mem {
            Register16Memory::Bc => self.bus.read_u16(self.state.reg_bc()),
            Register16Memory::De => self.bus.read_u16(self.state.reg_de()),
            Register16Memory::Hli | Register16Memory::Hld => self.bus.read_u16(self.state.reg_hl()),
        }?;

        self.after_r16_mem(r16mem);

        Ok(val)
    }

    fn after_r16_mem(&mut self, r16mem: Register16Memory) {
        match r16mem {
            Register16Memory::Hld => {
                self.state.set_reg_hl(self.state.reg_hl().wrapping_sub(1));
            }
            Register16Memory::Hli => {
                self.state.set_reg_hl(self.state.reg_hl().wrapping_add(1));
            }
            _ => {}
        }
    }

    fn update_r16(&mut self, r16: Register16, value: u16) {
        match r16 {
            Register16::Bc => {
                self.state.set_reg_bc(value);
            }
            Register16::De => {
                self.state.set_reg_de(value);
            }
            Register16::Hl => {
                self.state.set_reg_hl(value);
            }
            Register16::Sp => {
                self.state.set_stack_pointer(value);
            }
        }
    }

    fn get_r16(&self, r16: Register16) -> u16 {
        match r16 {
            Register16::Bc => self.state.reg_bc(),
            Register16::De => self.state.reg_de(),
            Register16::Hl => self.state.reg_hl(),
            Register16::Sp => self.state.stack_pointer(),
        }
    }

    fn update_r8(&mut self, r8: Register8, value: u8) -> Result<(), Error> {
        match r8 {
            Register8::A => {
                self.state.set_reg_a(value);
            }
            Register8::B => {
                self.state.set_reg_b(value);
            }
            Register8::C => {
                self.state.set_reg_c(value);
            }
            Register8::D => {
                self.state.set_reg_d(value);
            }
            Register8::E => {
                self.state.set_reg_e(value);
            }
            Register8::H => {
                self.state.set_reg_h(value);
            }
            Register8::L => {
                self.state.set_reg_l(value);
            }
            Register8::HlIndirect => {
                self.bus.write_u8(self.state.reg_hl(), value)?;
            }
        }
        Ok(())
    }

    fn get_r8(&self, r8: Register8) -> Result<u8, Error> {
        let v = match r8 {
            Register8::A => self.state.reg_a(),
            Register8::B => self.state.reg_b(),
            Register8::C => self.state.reg_c(),
            Register8::D => self.state.reg_d(),
            Register8::E => self.state.reg_e(),
            Register8::H => self.state.reg_h(),
            Register8::L => self.state.reg_l(),
            Register8::HlIndirect => self.bus.read_u8(self.state.reg_hl())?,
        };
        Ok(v)
    }
}
