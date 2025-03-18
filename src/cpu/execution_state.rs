use std::{
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct ExecutionState {
    instruction_pointer: u16,
    stack_pointer: u16,
    reg_bc: u16,
    reg_de: u16,
    reg_hl: u16,
    reg_a: u8,
    flags: Flags,
    interrupts_enabled: bool,
}

impl ExecutionState {
    pub fn new() -> Self {
        Self {
            instruction_pointer: 0,
            stack_pointer: 0xFFFF,
            reg_bc: 0,
            reg_de: 0,
            reg_hl: 0,
            reg_a: 0,
            flags: Flags::zeros(),
            interrupts_enabled: false,
        }
    }

    pub fn reg_af(&self) -> u16 {
        ((self.reg_a as u16) << 8) | u16::from(self.flags)
    }

    pub fn set_reg_af(&mut self, value: u16) {
        self.reg_a = (value >> 8) as u8;
        self.flags = Flags::from((value & 0xFF) as u8)
    }

    pub fn instruction_pointer(&self) -> u16 {
        self.instruction_pointer
    }

    pub fn set_instruction_pointer(&mut self, value: u16) {
        self.instruction_pointer = value;
    }

    pub fn stack_pointer(&self) -> u16 {
        self.stack_pointer
    }

    pub fn set_stack_pointer(&mut self, value: u16) {
        self.stack_pointer = value;
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.flags = flags
    }

    pub fn flags(&self) -> &Flags {
        &self.flags
    }

    pub fn flags_mut(&mut self) -> &mut Flags {
        &mut self.flags
    }

    pub fn interrupts_enabled(&self) -> bool {
        self.interrupts_enabled
    }

    pub fn set_interrupts_enabled(&mut self, enabled: bool) {
        self.interrupts_enabled = enabled;
    }

    pub fn reg_a(&self) -> u8 {
        self.reg_a
    }

    pub fn set_reg_a(&mut self, value: u8) {
        self.reg_a = value;
    }

    pub fn reg_bc(&self) -> u16 {
        self.reg_bc
    }

    pub fn set_reg_bc(&mut self, value: u16) {
        self.reg_bc = value;
    }

    pub fn reg_b(&self) -> u8 {
        (self.reg_bc() >> 8) as u8
    }

    pub fn reg_c(&self) -> u8 {
        (self.reg_bc() & 0x00FF) as u8
    }

    pub fn set_reg_b(&mut self, value: u8) {
        self.set_reg_bc((self.reg_bc() & 0x00FF) | ((value as u16) << 8));
    }

    pub fn set_reg_c(&mut self, value: u8) {
        self.set_reg_bc((self.reg_bc() & 0xFF00) | value as u16)
    }

    pub fn reg_de(&self) -> u16 {
        self.reg_de
    }

    pub fn set_reg_de(&mut self, value: u16) {
        self.reg_de = value;
    }

    pub fn reg_d(&self) -> u8 {
        (self.reg_de() >> 8) as u8
    }

    pub fn reg_e(&self) -> u8 {
        (self.reg_de() & 0x00FF) as u8
    }

    pub fn set_reg_d(&mut self, value: u8) {
        self.set_reg_de((self.reg_de() & 0x00FF) | ((value as u16) << 8))
    }

    pub fn set_reg_e(&mut self, value: u8) {
        self.set_reg_de((self.reg_de() & 0xFF00) | value as u16)
    }

    pub fn reg_hl(&self) -> u16 {
        self.reg_hl
    }

    pub fn set_reg_hl(&mut self, value: u16) {
        self.reg_hl = value;
    }

    pub fn reg_h(&self) -> u8 {
        (self.reg_hl() >> 8) as u8
    }

    pub fn reg_l(&self) -> u8 {
        (self.reg_hl() & 0x00FF) as u8
    }

    pub fn set_reg_h(&mut self, value: u8) {
        self.set_reg_hl((self.reg_hl() & 0x00FF) | ((value as u16) << 8));
    }

    pub fn set_reg_l(&mut self, value: u8) {
        self.set_reg_hl((self.reg_hl() & 0xFF00) | value as u16);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags {
    pub carry: bool,
    pub half_carry: bool,
    pub subtraction: bool,
    pub zero: bool,
}

impl Flags {
    pub fn new(carry: bool, half_carry: bool, subtraction: bool, zero: bool) -> Self {
        Self {
            carry,
            half_carry,
            subtraction,
            zero,
        }
    }

    pub fn zeros() -> Self {
        Self {
            carry: false,
            half_carry: false,
            subtraction: false,
            zero: false,
        }
    }

    pub fn with_zero(mut self, zero: bool) -> Self {
        self.zero = zero;
        self
    }

    pub fn just_zero() -> Self {
        Self {
            carry: false,
            half_carry: false,
            subtraction: false,
            zero: true,
        }
    }

    pub fn with_carry(mut self, carry: bool) -> Self {
        self.carry = carry;
        self
    }

    pub fn just_carry() -> Self {
        Self {
            carry: true,
            half_carry: false,
            subtraction: false,
            zero: false,
        }
    }

    pub fn with_half_carry(mut self, half_carry: bool) -> Self {
        self.half_carry = half_carry;
        self
    }

    pub fn just_half_carry() -> Self {
        Self {
            carry: false,
            half_carry: true,
            subtraction: false,
            zero: false,
        }
    }

    pub fn with_subtraction(mut self, subtraction: bool) -> Self {
        self.subtraction = subtraction;
        self
    }

    pub fn just_subtraction() -> Self {
        Self {
            carry: false,
            half_carry: false,
            subtraction: true,
            zero: false,
        }
    }

    pub fn set_with_mask(mut self, rhs: Flags, mask: Flags) -> Self {
        if mask.carry {
            self.carry = rhs.carry;
        }
        if mask.half_carry {
            self.half_carry = rhs.half_carry;
        }
        if mask.subtraction {
            self.subtraction = rhs.subtraction;
        }
        if mask.zero {
            self.zero = rhs.zero;
        }

        self
    }
}

impl BitAnd for Flags {
    type Output = Flags;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self.carry &= rhs.carry;
        self.half_carry &= rhs.half_carry;
        self.subtraction &= rhs.subtraction;
        self.zero &= rhs.zero;

        self
    }
}

impl BitAndAssign for Flags {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for Flags {
    type Output = Flags;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self.carry |= rhs.carry;
        self.half_carry |= rhs.half_carry;
        self.subtraction |= rhs.subtraction;
        self.zero |= rhs.zero;

        self
    }
}

impl BitOrAssign for Flags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl Display for ExecutionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IP: {:04x} SP: {:04x} BC: {:04x} DE: {:04x} HL: {:04x} AF: {:04x} {}",
            self.instruction_pointer,
            self.stack_pointer,
            self.reg_bc,
            self.reg_de,
            self.reg_hl,
            self.reg_af(),
            self.flags
        )
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        let carry = ((value >> 4) & 1) != 0;
        let half_carry = ((value >> 5) & 1) != 0;
        let subtraction = ((value >> 6) & 1) != 0;
        let zero = ((value >> 7) & 1) != 0;

        Self {
            carry,
            half_carry,
            subtraction,
            zero,
        }
    }
}

impl From<Flags> for u8 {
    fn from(value: Flags) -> Self {
        let mut b = 0;

        b |= (if value.carry { 1 } else { 0 }) >> 4;
        b |= (if value.half_carry { 1 } else { 0 }) >> 5;
        b |= (if value.subtraction { 1 } else { 0 }) >> 6;
        b |= (if value.zero { 1 } else { 0 }) >> 7;

        b
    }
}

impl From<Flags> for u16 {
    fn from(value: Flags) -> Self {
        u8::from(value) as u16
    }
}

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            if self.zero { 'z' } else { '-' },
            if self.subtraction { 'n' } else { '-' },
            if self.half_carry { 'h' } else { '-' },
            if self.carry { 'c' } else { '-' },
        )
    }
}
