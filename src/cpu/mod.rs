use alu::Alu;
use bus::Bus;
use decoder::Decoder;
use error::Error;
use execution_state::SharedExecutionState;
use instruction::{
    Condition, Instruction, Register16, Register16Memory, Register16Stack, Register8,
};

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

    pub fn execution_state(&self) -> SharedExecutionState {
        self.state.clone()
    }

    pub fn execute_one(&mut self) -> Result<(), Error> {
        let current_instruction = self.decoder.decode_one(&self.state, &self.bus)?;
        let mut next_instruction_address = self
            .state
            .instruction_pointer()
            .wrapping_add(current_instruction.length());

        // println!("{:#?}", current_instruction);

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
                    self.alu
                        .rotate_left_u8(self.get_r8(Register8::A)?, false, false),
                )?;
            }
            Instruction::Rrca => {
                self.update_r8(
                    Register8::A,
                    self.alu
                        .rotate_right_u8(self.get_r8(Register8::A)?, false, false),
                )?;
            }
            Instruction::Rla => {
                self.update_r8(
                    Register8::A,
                    self.alu
                        .rotate_left_u8(self.get_r8(Register8::A)?, false, true),
                )?;
            }
            Instruction::Rra => {
                self.update_r8(
                    Register8::A,
                    self.alu
                        .rotate_right_u8(self.get_r8(Register8::A)?, false, true),
                )?;
            }
            Instruction::Daa => {
                let a = self.get_r8(Register8::A)?;
                self.update_r8(Register8::A, self.alu.decimal_adjust(a))?;
            }
            Instruction::Cpl => {
                let a = self.get_r8(Register8::A)?;
                self.update_r8(Register8::A, self.alu.not_u8(a))?;
            }
            Instruction::Scf => {
                self.state.modify_flags(|flags| {
                    flags.subtraction = false;
                    flags.half_carry = false;
                    flags.carry = true;
                });
            }
            Instruction::Ccf => {
                self.state.modify_flags(|flags| {
                    flags.subtraction = false;
                    flags.half_carry = false;
                    flags.carry = !flags.carry;
                });
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
            Instruction::Stop => todo!(),
            Instruction::LdReg8Reg8(dest, src) => {
                let val = self.get_r8(src)?;
                self.update_r8(dest, val)?;
            }
            Instruction::Halt => todo!(),
            Instruction::AddReg8(r8)
            | Instruction::AdcReg8(r8)
            | Instruction::SubReg8(r8)
            | Instruction::SbcReg8(r8)
            | Instruction::AndReg8(r8)
            | Instruction::XorReg8(r8)
            | Instruction::OrReg8(r8) => {
                let val = self.get_r8(r8)?;
                let a = self.get_r8(Register8::A)?;

                let result = match current_instruction {
                    Instruction::AddReg8(_) => self.alu.add_u8(val, a),
                    Instruction::AdcReg8(_) => self.alu.adc_u8(val, a),
                    Instruction::SubReg8(_) => self.alu.sub_u8(val, a),
                    Instruction::SbcReg8(_) => self.alu.sbc_u8(val, a),
                    Instruction::AndReg8(_) => self.alu.and_u8(val, a),
                    Instruction::XorReg8(_) => self.alu.xor_u8(val, a),
                    Instruction::OrReg8(_) => self.alu.or_u8(val, a),
                    _ => panic!(),
                };

                self.update_r8(Register8::A, result)?;
            }
            Instruction::CpReg8(r8) => {
                let val = self.get_r8(r8)?;
                let a = self.get_r8(Register8::A)?;
                self.alu.cp_u8(val, a);
            }
            Instruction::AddImm8(imm8)
            | Instruction::AdcImm8(imm8)
            | Instruction::SubImm8(imm8)
            | Instruction::SbcImm8(imm8)
            | Instruction::AndImm8(imm8)
            | Instruction::XorImm8(imm8)
            | Instruction::OrImm8(imm8) => {
                let val = imm8.into();
                let a = self.get_r8(Register8::A)?;

                let result = match current_instruction {
                    Instruction::AddImm8(_) => self.alu.add_u8(val, a),
                    Instruction::AdcImm8(_) => self.alu.adc_u8(val, a),
                    Instruction::SubImm8(_) => self.alu.sub_u8(val, a),
                    Instruction::SbcImm8(_) => self.alu.sbc_u8(val, a),
                    Instruction::AndImm8(_) => self.alu.and_u8(val, a),
                    Instruction::XorImm8(_) => self.alu.xor_u8(val, a),
                    Instruction::OrImm8(_) => self.alu.or_u8(val, a),
                    _ => panic!(),
                };

                self.update_r8(Register8::A, result)?;
            }
            Instruction::CpImm8(imm8) => {
                let val = imm8.into();
                let a = self.get_r8(Register8::A)?;
                self.alu.cp_u8(val, a);
            }
            Instruction::RetCond(cond) => {
                if self.is_condition_met(cond) {
                    next_instruction_address = self.pop_u16()?;
                }
            }
            Instruction::Ret => {
                next_instruction_address = self.pop_u16()?;
            }
            Instruction::Reti => {
                self.state.set_interrupts_enabled(true);
                next_instruction_address = self.pop_u16()?;
            }
            Instruction::JpCond(cond, imm16) => {
                if self.is_condition_met(cond) {
                    next_instruction_address = imm16.into();
                }
            }
            Instruction::JpImm(imm16) => {
                next_instruction_address = imm16.into();
            }
            Instruction::JpHl => {
                next_instruction_address = self.get_r16(Register16::Hl);
            }
            Instruction::CallCond(cond, imm16) => {
                if self.is_condition_met(cond) {
                    self.push_u16(next_instruction_address)?;
                    next_instruction_address = imm16.into();
                }
            }
            Instruction::CallImm(imm16) => {
                self.push_u16(next_instruction_address)?;
                next_instruction_address = imm16.into();
            }
            Instruction::Rst(tgt) => {
                self.push_u16(next_instruction_address)?;
                next_instruction_address = tgt.into();
            }
            Instruction::Pop(r16stk) => {
                let value = self.pop_u16()?;
                self.update_r16_stack(r16stk, value);
            }
            Instruction::Push(r16stk) => {
                let value = self.get_r16_stack(r16stk);
                self.push_u16(value)?;
            }
            Instruction::LdhMemA => {
                let val = self.get_r8(Register8::A)?;
                let addr = 0xFF00 + (self.get_r8(Register8::C)? as u16);
                self.bus.write_u8(addr, val)?;
            }
            Instruction::LdhImmA(imm8) => {
                let val = self.get_r8(Register8::A)?;
                let addr = 0xFF00 + u16::from(imm8);
                self.bus.write_u8(addr, val)?;
            }
            Instruction::LdImmA(imm16) => {
                let val = self.get_r8(Register8::A)?;
                self.bus.write_u8(u16::from(imm16), val)?;
            }
            Instruction::LdhAMem => {
                let addr = 0xFF00 + (self.get_r8(Register8::C)? as u16);
                let val = self.bus.read_u8(addr)?;
                self.update_r8(Register8::A, val)?;
            }
            Instruction::LdhAImm(imm8) => {
                let addr = 0xFF00 + u16::from(imm8);
                let val = self.bus.read_u8(addr)?;
                self.update_r8(Register8::A, val)?;
            }
            Instruction::LdAImm(imm16) => {
                let val = self.bus.read_u8(u16::from(imm16))?;
                self.update_r8(Register8::A, val)?;
            }
            Instruction::AddSp(imm8) => {
                let sp = self.state.stack_pointer();
                let new_sp = self.alu.add_u16(sp, imm8.into());
                self.state.set_stack_pointer(new_sp);
            }
            Instruction::LdHlSpImm8(imm8) => {
                let sp = self.state.stack_pointer();
                let val = self.alu.add_u16(sp, imm8.into());
                self.update_r16(Register16::Hl, val);
            }
            Instruction::LdSpHl => {
                self.state.set_stack_pointer(self.get_r16(Register16::Hl));
            }
            Instruction::Di => self.state.set_interrupts_enabled(false),
            Instruction::Ei => self.state.set_interrupts_enabled(true),
            // Prefixed
            Instruction::Rlc(r8) => {
                self.update_r8(r8, self.alu.rotate_left_u8(self.get_r8(r8)?, true, false))?;
            }
            Instruction::Rrc(r8) => {
                self.update_r8(r8, self.alu.rotate_right_u8(self.get_r8(r8)?, true, false))?;
            }
            Instruction::Rl(r8) => {
                self.update_r8(r8, self.alu.rotate_left_u8(self.get_r8(r8)?, true, true))?;
            }
            Instruction::Rr(r8) => {
                self.update_r8(r8, self.alu.rotate_right_u8(self.get_r8(r8)?, true, true))?;
            }
            Instruction::Sla(r8) => {
                self.update_r8(r8, self.alu.shift_left_arithmetic(self.get_r8(r8)?))?;
            }
            Instruction::Sra(r8) => {
                self.update_r8(r8, self.alu.shift_right_arithmetic(self.get_r8(r8)?))?;
            }
            Instruction::Swap(r8) => {
                self.update_r8(r8, self.alu.swap_u8(self.get_r8(r8)?))?;
            }
            Instruction::Srl(r8) => {
                self.update_r8(r8, self.alu.shift_right_logical(self.get_r8(r8)?))?;
            }
            Instruction::Bit(idx, r8) => {
                let val = self.get_r8(r8)?;
                self.alu.test_bit_u8(idx.into(), val);
            }
            Instruction::Res(idx, r8) => {
                self.update_r8(r8, self.alu.reset_bit_u8(idx.into(), self.get_r8(r8)?))?;
            }
            Instruction::Set(idx, r8) => {
                self.update_r8(r8, self.alu.set_bit_u8(idx.into(), self.get_r8(r8)?))?;
            }
        }

        self.state.set_instruction_pointer(next_instruction_address);

        // println!("{}", self.state);

        Ok(())
    }

    fn push_u16(&self, value: u16) -> Result<(), Error> {
        self.push_u8((value >> 8) as u8)?;
        self.push_u8((value & 0xFF) as u8)
    }

    fn push_u8(&self, value: u8) -> Result<(), Error> {
        let new_sp = self.state.stack_pointer().wrapping_sub(1);
        self.state.set_stack_pointer(new_sp);

        self.bus.write_u8(new_sp, value)
    }

    fn pop_u16(&self) -> Result<u16, Error> {
        let lo = self.pop_u8()? as u16;
        let hi = self.pop_u8()? as u16;
        Ok((hi << 8) | lo)
    }

    fn pop_u8(&self) -> Result<u8, Error> {
        let old_sp = self.state.stack_pointer();

        let value = self.bus.read_u8(old_sp)?;

        self.state.set_stack_pointer(old_sp.wrapping_add(1));
        Ok(value)
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

    fn update_r16_stack(&mut self, r16stk: Register16Stack, value: u16) {
        match r16stk {
            Register16Stack::Af => self.state.set_reg_af(value),
            Register16Stack::Bc => self.state.set_reg_bc(value),
            Register16Stack::De => self.state.set_reg_de(value),
            Register16Stack::Hl => self.state.set_reg_hl(value),
        }
    }

    fn get_r16_stack(&self, r16stk: Register16Stack) -> u16 {
        match r16stk {
            Register16Stack::Af => self.state.reg_af(),
            Register16Stack::Bc => self.state.reg_bc(),
            Register16Stack::De => self.state.reg_de(),
            Register16Stack::Hl => self.state.reg_hl(),
        }
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
