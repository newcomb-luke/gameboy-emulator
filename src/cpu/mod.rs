use decoder::Decoder;
use error::Error;
use execution_state::ExecutionState;
use instruction::{
    Condition, Instruction, Register16, Register16Memory, Register16Stack, Register8,
};

use crate::{
    bus::Bus,
    io::{dma::DMA_TRANSFER_CYCLES_LENGTH, interrupts::Interrupt},
};

pub mod alu;
pub mod decoder;
pub mod error;
pub mod execution_state;
pub mod instruction;

pub struct Cpu {
    state: ExecutionState,
    bus: Bus,
    decoder: Decoder,
    interrupt_enable_next: bool,
    halted: bool
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        Self {
            state: ExecutionState::new(),
            bus,
            decoder: Decoder::new(),
            interrupt_enable_next: false,
            halted: false
        }
    }

    pub fn execution_state(&self) -> &ExecutionState {
        &self.state
    }

    pub fn step(&mut self) -> Result<usize, Error> {
        let mut cycles = 0;

        if self.halted {
            if self.detect_interrupt().is_some() {
                self.halted = false;
            } else {
                return Ok(1);
            }
        }

        if self.state.interrupts_enabled() {
            if let Some(interrupt) = self.detect_interrupt() {
                // Wait
                cycles += 5;
                // Clear the bit in the IF register
                self.clear_requested_interrupt(interrupt);
                // Disable interrupts
                self.state.set_interrupts_enabled(false);
                // Call the interrupt handler
                self.call_interrupt_handler(interrupt)?;
            }
        }

        let current_instruction = self.decoder.decode_one(&self.state, &self.bus)?;
        let mut next_instruction_address = self
            .state
            .instruction_pointer()
            .wrapping_add(current_instruction.length());
        cycles += current_instruction.base_num_cycles();

        if self.interrupt_enable_next & self.interrupt_enable_next
            != self.state.interrupts_enabled()
        {
            self.state.set_interrupts_enabled(true);
            self.interrupt_enable_next = false;
        }

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
                self.update_r16(r16, self.inc_u16(self.get_r16(r16)));
            }
            Instruction::Dec16(r16) => {
                self.update_r16(r16, self.dec_u16(self.get_r16(r16)));
            }
            Instruction::AddHl(r16) => {
                let val1 = self.get_r16(Register16::Hl);
                let val2 = self.get_r16(r16);
                let result = self.add_u16(val1, val2);
                self.update_r16(r16, result);
            }
            Instruction::Inc8(r8) | Instruction::Dec8(r8) => {
                let val = self.get_r8(r8)?;
                let result = match current_instruction {
                    Instruction::Inc8(_) => self.inc_u8(val),
                    Instruction::Dec8(_) => self.dec_u8(val),
                    _ => panic!(),
                };
                self.update_r8(r8, result)?;
            }
            Instruction::LdReg8Imm(r8, imm8) => {
                self.update_r8(r8, imm8.into())?;
            }
            Instruction::Rlca
            | Instruction::Rrca
            | Instruction::Rla
            | Instruction::Rra
            | Instruction::Daa
            | Instruction::Cpl => {
                let a = self.get_r8(Register8::A)?;
                let result = match current_instruction {
                    Instruction::Rlca => self.rotate_left_u8(a, false, false),
                    Instruction::Rrca => self.rotate_right_u8(a, false, false),
                    Instruction::Rla => self.rotate_left_u8(a, false, true),
                    Instruction::Rra => self.rotate_right_u8(a, false, true),
                    Instruction::Daa => self.decimal_adjust(a),
                    Instruction::Cpl => self.not_u8(a),
                    _ => panic!(),
                };
                self.update_r8(Register8::A, result)?;
            }
            Instruction::Scf => {
                let flags = self.state.flags_mut();
                flags.subtraction = false;
                flags.half_carry = false;
                flags.carry = true;
            }
            Instruction::Ccf => {
                let flags = self.state.flags_mut();
                flags.subtraction = false;
                flags.half_carry = false;
                flags.carry = !flags.carry;
            }
            Instruction::JrImm(imm8) => {
                next_instruction_address =
                    self.rel_jump_dest(imm8.into(), current_instruction.length());
            }
            Instruction::JrCond(cond, imm8) => {
                let dest = self.rel_jump_dest(imm8.into(), current_instruction.length());

                if self.is_condition_met(cond) {
                    next_instruction_address = dest;
                    cycles += 1;
                }
            }
            Instruction::Stop => {
                self.bus_mut().io_mut().timer_mut().set_divider(0);
                self.halted = true;
                todo!()
            }
            Instruction::LdReg8Reg8(dest, src) => {
                let val = self.get_r8(src)?;
                self.update_r8(dest, val)?;
            }
            Instruction::Halt => {
                self.halted = true;
            },
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
                    Instruction::AddReg8(_) => self.add_u8(val, a),
                    Instruction::AdcReg8(_) => self.adc_u8(val, a),
                    Instruction::SubReg8(_) => self.sub_u8(val, a),
                    Instruction::SbcReg8(_) => self.sbc_u8(val, a),
                    Instruction::AndReg8(_) => self.and_u8(val, a),
                    Instruction::XorReg8(_) => self.xor_u8(val, a),
                    Instruction::OrReg8(_) => self.or_u8(val, a),
                    _ => panic!(),
                };

                self.update_r8(Register8::A, result)?;
            }
            Instruction::CpReg8(r8) => {
                let val = self.get_r8(r8)?;
                let a = self.get_r8(Register8::A)?;
                self.cp_u8(val, a);
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
                    Instruction::AddImm8(_) => self.add_u8(val, a),
                    Instruction::AdcImm8(_) => self.adc_u8(val, a),
                    Instruction::SubImm8(_) => self.sub_u8(val, a),
                    Instruction::SbcImm8(_) => self.sbc_u8(val, a),
                    Instruction::AndImm8(_) => self.and_u8(val, a),
                    Instruction::XorImm8(_) => self.xor_u8(val, a),
                    Instruction::OrImm8(_) => self.or_u8(val, a),
                    _ => panic!(),
                };

                self.update_r8(Register8::A, result)?;
            }
            Instruction::CpImm8(imm8) => {
                let val = imm8.into();
                let a = self.get_r8(Register8::A)?;
                self.cp_u8(val, a);
            }
            Instruction::RetCond(cond) => {
                if self.is_condition_met(cond) {
                    next_instruction_address = self.pop_u16()?;
                    cycles += 3;
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
                    cycles += 1;
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
                    cycles += 3;
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
                let new_sp = self.add_u16(sp, imm8.into());
                self.state.set_stack_pointer(new_sp);
            }
            Instruction::LdHlSpImm8(imm8) => {
                let sp = self.state.stack_pointer();
                let val = self.add_u16(sp, imm8.into());
                self.update_r16(Register16::Hl, val);
            }
            Instruction::LdSpHl => {
                self.state.set_stack_pointer(self.get_r16(Register16::Hl));
            }
            Instruction::Di => {
                self.state.set_interrupts_enabled(false);
                self.interrupt_enable_next = false;
            }
            Instruction::Ei => self.interrupt_enable_next = true,
            // Prefixed
            Instruction::Rlc(r8)
            | Instruction::Rrc(r8)
            | Instruction::Rl(r8)
            | Instruction::Rr(r8)
            | Instruction::Sla(r8)
            | Instruction::Sra(r8)
            | Instruction::Swap(r8)
            | Instruction::Srl(r8)
            | Instruction::Res(_, r8)
            | Instruction::Set(_, r8) => {
                let val = self.get_r8(r8)?;
                let result = match current_instruction {
                    Instruction::Rlc(_) => self.rotate_left_u8(val, true, false),
                    Instruction::Rrc(_) => self.rotate_right_u8(val, true, false),
                    Instruction::Rl(_) => self.rotate_left_u8(val, true, true),
                    Instruction::Rr(_) => self.rotate_right_u8(val, true, true),
                    Instruction::Sla(_) => self.shift_left_arithmetic(val),
                    Instruction::Sra(_) => self.shift_right_arithmetic(val),
                    Instruction::Swap(_) => self.swap_u8(val),
                    Instruction::Srl(_) => self.shift_right_logical(val),
                    Instruction::Res(idx, _) => self.reset_bit_u8(idx.into(), val),
                    Instruction::Set(idx, _) => self.set_bit_u8(idx.into(), val),
                    _ => panic!(),
                };
                self.update_r8(r8, result)?;
            }
            Instruction::Bit(idx, r8) => {
                let val = self.get_r8(r8)?;
                self.test_bit_u8(idx.into(), val);
            }
        }

        self.state.set_instruction_pointer(next_instruction_address);

        if self.step_dma(cycles) {
            self.do_dma_transfer()?;
        }

        Ok(cycles)
    }

    fn do_dma_transfer(&mut self) -> Result<(), Error> {
        let source_address = self.bus().io().dma().full_source_address();

        // Do the entire DMA transfer all at once, for simplicity
        // The number of cycles is also the number of bytes
        for i in 0..DMA_TRANSFER_CYCLES_LENGTH {
            let source_addr = source_address + i;
            let dest_addr = 0xFE00 + i;

            let byte = self.bus.read_u8(source_addr)?;
            self.bus.write_u8(dest_addr, byte)?;
        }

        Ok(())
    }

    fn step_dma(&mut self, cycles: usize) -> bool {
        self.bus.io_mut().dma_mut().step(cycles)
    }

    fn clear_requested_interrupt(&mut self, interrupt: Interrupt) {
        self.bus_mut()
            .io_mut()
            .interrupts_mut()
            .clear_requested_interrupt(interrupt);
    }

    fn detect_interrupt(&self) -> Option<Interrupt> {
        self.bus()
            .io()
            .interrupts()
            .highest_priority_triggered_interrupt()
    }

    fn call_interrupt_handler(&mut self, interrupt: Interrupt) -> Result<(), Error> {
        let vector = match interrupt {
            Interrupt::VBlank => 0x40,
            Interrupt::Lcd => 0x48,
            Interrupt::Timer => 0x50,
            Interrupt::Serial => 0x58,
            Interrupt::Joypad => 0x60,
        };

        self.push_u16(self.state.instruction_pointer())?;
        self.state.set_instruction_pointer(vector);

        Ok(())
    }

    fn push_u16(&mut self, value: u16) -> Result<(), Error> {
        self.push_u8((value >> 8) as u8)?;
        self.push_u8((value & 0xFF) as u8)
    }

    fn push_u8(&mut self, value: u8) -> Result<(), Error> {
        let new_sp = self.state.stack_pointer().wrapping_sub(1);
        self.state.set_stack_pointer(new_sp);

        self.bus.write_u8(new_sp, value)
    }

    fn pop_u16(&mut self) -> Result<u16, Error> {
        let lo = self.pop_u8()? as u16;
        let hi = self.pop_u8()? as u16;
        Ok((hi << 8) | lo)
    }

    fn pop_u8(&mut self) -> Result<u8, Error> {
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

    pub fn bus(&self) -> &Bus {
        &self.bus
    }

    pub fn bus_mut(&mut self) -> &mut Bus {
        &mut self.bus
    }
}
