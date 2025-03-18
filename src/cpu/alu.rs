use super::{execution_state::Flags, Cpu};

impl Cpu {
    pub fn inc_u16(&self, val: u16) -> u16 {
        val.wrapping_add(1)
    }

    pub fn dec_u16(&self, val: u16) -> u16 {
        val.wrapping_sub(1)
    }

    pub fn inc_u8(&mut self, val: u8) -> u8 {
        let flags_before = *self.state.flags();

        let result = self.add_u8(val, 1);

        self.state.flags_mut().carry = flags_before.carry;

        result
    }

    pub fn dec_u8(&mut self, val: u8) -> u8 {
        let flags_before = *self.state.flags();

        let result = self.sub_u8(1, val);

        self.state.flags_mut().carry = flags_before.carry;

        result
    }

    pub fn add_u16(&mut self, v1: u16, v2: u16) -> u16 {
        self.generic_add_u16(v1, v2, false)
    }

    pub fn adc_u16(&mut self, v1: u16, v2: u16) -> u16 {
        self.generic_add_u16(v1, v2, true)
    }

    fn generic_add_u16(&mut self, v1: u16, v2: u16, with_carry: bool) -> u16 {
        let carry = if with_carry & self.state.flags().carry {
            1
        } else {
            0
        };
        let (temp, first_carry) = v1.overflowing_add(v2);
        let (result, second_carry) = temp.overflowing_add(carry);

        let half_result = (v1 & 0x0FFF) + (v2 & 0x0FFF) + carry;
        let half_carry = half_result > 0x0FFF;

        let flags = Flags::new(first_carry | second_carry, half_carry, false, result == 0);
        self.state.set_flags(flags);

        result
    }

    pub fn add_u8(&mut self, v1: u8, v2: u8) -> u8 {
        self.generic_add_u8(v1, v2, false)
    }

    pub fn adc_u8(&mut self, v1: u8, v2: u8) -> u8 {
        self.generic_add_u8(v1, v2, true)
    }

    fn generic_add_u8(&mut self, v1: u8, v2: u8, with_carry: bool) -> u8 {
        let carry = if with_carry & self.state.flags().carry {
            1
        } else {
            0
        };
        let (temp, first_carry) = v1.overflowing_add(v2);
        let (result, second_carry) = temp.overflowing_add(carry);

        let half_result = (v1 & 0x0F) + (v2 & 0x0F) + carry;
        let half_carry = half_result > 0x0F;

        let flags = Flags::new(first_carry | second_carry, half_carry, false, result == 0);
        self.state.set_flags(flags);

        result
    }

    /// v2 - v1
    pub fn sub_u16(&mut self, v1: u16, v2: u16) -> u16 {
        self.generic_sub_u16(v1, v2, false)
    }

    /// v2 - v1
    pub fn sbc_u16(&mut self, v1: u16, v2: u16) -> u16 {
        self.generic_sub_u16(v1, v2, true)
    }

    /// v2 - v1
    fn generic_sub_u16(&mut self, v1: u16, v2: u16, with_carry: bool) -> u16 {
        let carry = if with_carry & self.state.flags().carry {
            1
        } else {
            0
        };
        let (temp, first_borrow) = v2.overflowing_sub(v1);
        let (result, second_borrow) = temp.overflowing_sub(carry);

        let half_borrow = (v2 & 0x0FFF) < ((v1 & 0x0FFF) + carry);

        self.state.set_flags(Flags::new(
            first_borrow | second_borrow,
            half_borrow,
            true,
            result == 0,
        ));

        result
    }

    /// v2 - v1
    pub fn sub_u8(&mut self, v1: u8, v2: u8) -> u8 {
        self.generic_sub_u8(v1, v2, false)
    }

    /// v2 - v1
    pub fn sbc_u8(&mut self, v1: u8, v2: u8) -> u8 {
        self.generic_sub_u8(v1, v2, true)
    }

    /// v2 - v1
    fn generic_sub_u8(&mut self, v1: u8, v2: u8, with_carry: bool) -> u8 {
        let carry = if with_carry & self.state.flags().carry {
            1
        } else {
            0
        };
        let (temp, first_borrow) = v2.overflowing_sub(v1);
        let (result, second_borrow) = temp.overflowing_sub(carry);

        let half_borrow = (v2 & 0x0F) < ((v1 & 0x0F) + carry);

        self.state.set_flags(Flags::new(
            first_borrow | second_borrow,
            half_borrow,
            true,
            result == 0,
        ));

        result
    }

    pub fn and_u8(&mut self, v1: u8, v2: u8) -> u8 {
        let result = v1 & v2;

        let flags = Flags::new(false, true, false, result == 0);
        self.state.set_flags(flags);

        result
    }

    pub fn xor_u8(&mut self, v1: u8, v2: u8) -> u8 {
        let result = v1 ^ v2;

        let flags = Flags::zeros().with_zero(result == 0);
        self.state.set_flags(flags);

        result
    }

    pub fn or_u8(&mut self, v1: u8, v2: u8) -> u8 {
        let result = v1 | v2;

        let flags = Flags::zeros().with_zero(result == 0);
        self.state.set_flags(flags);

        result
    }

    pub fn cp_u8(&mut self, v1: u8, v2: u8) {
        self.sub_u8(v1, v2);
    }

    pub fn decimal_adjust(&mut self, a: u8) -> u8 {
        let mut adjustment = 0;

        let flags = *self.state.flags();

        if flags.subtraction {
            if flags.half_carry {
                adjustment += 0x06;
            }
            if flags.carry {
                adjustment += 0x60;
            }

            let result = self.sub_u8(adjustment, a);

            let new_flags = self.state.flags_mut();
            new_flags.subtraction = flags.subtraction;
            new_flags.half_carry = false;

            result
        } else {
            let mut set_carry = false;
            if flags.half_carry | ((a & 0x0F) > 0x09) {
                adjustment += 0x06;
            }
            if flags.carry | (a > 0x99) {
                adjustment += 0x60;
                set_carry = true;
            }

            let result = self.add_u8(a, adjustment);

            let new_flags = self.state.flags_mut();
            new_flags.subtraction = flags.subtraction;
            new_flags.half_carry = false;
            if set_carry {
                new_flags.carry = true;
            }

            result
        }
    }

    pub fn rotate_left_u8(&mut self, v: u8, update_zero_flag: bool, through_carry: bool) -> u8 {
        let carry = v >> 7;
        let result = if through_carry {
            (v << 1) | (if self.state.flags().carry { 1 } else { 0 })
        } else {
            (v << 1) | carry
        };

        let flags = Flags::new(carry != 0, false, false, update_zero_flag && result == 0);
        self.state.set_flags(flags);

        result
    }

    pub fn rotate_right_u8(&mut self, v: u8, update_zero_flag: bool, through_carry: bool) -> u8 {
        let carry = v & 1;
        let result = if through_carry {
            (v >> 1) | ((if self.state.flags().carry { 1 } else { 0 }) << 7)
        } else {
            (v >> 1) | (carry << 7)
        };

        let flags = Flags::new(carry != 0, false, false, update_zero_flag && result == 0);
        self.state.set_flags(flags);

        result
    }

    pub fn not_u8(&mut self, v: u8) -> u8 {
        let result = !v;

        let flags = self.state.flags_mut();
        flags.subtraction = true;
        flags.half_carry = true;

        result
    }

    pub fn swap_u8(&self, v: u8) -> u8 {
        let hi = (v & 0b1111_0000) >> 4;
        let lo = (v & 0b0000_1111) << 4;

        hi | lo
    }

    pub fn shift_left_arithmetic(&mut self, v: u8) -> u8 {
        let carry = (v >> 7) != 0;
        let result = v << 1;

        self.state
            .set_flags(Flags::new(carry, false, false, result == 0));

        result
    }

    pub fn shift_right_arithmetic(&mut self, v: u8) -> u8 {
        let carry = (v & 1) != 0;
        let result = (v >> 1) | (v & 0b1000_0000);

        self.state
            .set_flags(Flags::new(carry, false, false, result == 0));

        result
    }

    pub fn shift_right_logical(&mut self, v: u8) -> u8 {
        let carry = (v & 1) != 0;
        let result = v >> 1;

        self.state
            .set_flags(Flags::new(carry, false, false, result == 0));

        result
    }

    pub fn test_bit_u8(&mut self, idx: u8, v: u8) {
        let zero = ((v >> idx) & 1) == 0;

        let flags = self.state.flags_mut();
        flags.half_carry = true;
        flags.subtraction = false;
        flags.zero = zero;
    }

    pub fn reset_bit_u8(&self, idx: u8, v: u8) -> u8 {
        let mask = !(1 << idx);
        v & mask
    }

    pub fn set_bit_u8(&self, idx: u8, v: u8) -> u8 {
        let mask = 1 << idx;
        v | mask
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        boot::DEFAULT_BOOT_ROM,
        bus::Bus,
        cartridge::Cartridge,
        cpu::{execution_state::Flags, Cpu},
    };

    fn test_alu_operation<F>(f: F)
    where
        F: FnOnce(&mut Cpu) -> Flags,
    {
        let boot_rom = DEFAULT_BOOT_ROM;
        let cartridge = Cartridge::empty();
        let mut alu = Cpu::new(Bus::new(boot_rom, cartridge));

        let desired_flags = f(&mut alu);

        let state = alu.state;

        assert_eq!(*state.flags(), desired_flags);
    }

    #[test]
    fn add_u16_no_carry() {
        test_alu_operation(|alu| {
            let result = alu.add_u16(1, 2);
            assert_eq!(result, 3);

            Flags::zeros()
        });
    }

    #[test]
    fn add_u16_full_carry() {
        test_alu_operation(|alu| {
            let result = alu.add_u16(0xF000, 0x1001);
            assert_eq!(result, 1);

            Flags::just_carry()
        });
    }

    #[test]
    fn add_u16_half_carry() {
        test_alu_operation(|alu| {
            let result = alu.add_u16(0x0FFF, 0x0001);
            assert_eq!(result, 0x1000);

            Flags::just_half_carry()
        });
    }

    #[test]
    fn add_u16_all_flags() {
        test_alu_operation(|alu| {
            let result = alu.add_u16(0xFFFF, 0x0001);
            assert_eq!(result, 0);

            Flags::new(true, true, false, true)
        });
    }

    #[test]
    fn add_u16_negative() {
        test_alu_operation(|alu| {
            let result = alu.add_u16(4, (-2i16) as u16);
            assert_eq!(result, 2);

            Flags::new(true, true, false, false)
        });
    }

    #[test]
    fn add_u8_no_carry() {
        test_alu_operation(|alu| {
            let result = alu.add_u8(1, 2);
            assert_eq!(result, 3);

            Flags::zeros()
        });
    }

    #[test]
    fn add_u8_full_carry() {
        test_alu_operation(|alu| {
            let result = alu.add_u8(0xF0, 0x11);
            assert_eq!(result, 1);

            Flags::just_carry()
        });
    }

    #[test]
    fn add_u8_half_carry() {
        test_alu_operation(|alu| {
            let result = alu.add_u8(0x0F, 0x01);
            assert_eq!(result, 0x10);

            Flags::just_half_carry()
        });
    }

    #[test]
    fn add_u8_all_flags() {
        test_alu_operation(|alu| {
            let result = alu.add_u8(0xFF, 0x01);
            assert_eq!(result, 0);

            Flags::new(true, true, false, true)
        });
    }

    #[test]
    fn add_u8_negative() {
        test_alu_operation(|alu| {
            let result = alu.add_u8(1, (-3i8) as u8);
            assert_eq!(result, (-2i8) as u8);

            Flags::new(false, false, false, false)
        });
    }

    #[test]
    fn adc_u8_carry_not_set() {
        test_alu_operation(|alu| {
            let result = alu.adc_u8(5, 4);
            assert_eq!(result, 9);

            Flags::zeros()
        });
    }

    #[test]
    fn adc_u8_carry_set() {
        test_alu_operation(|alu| {
            alu.state.flags_mut().carry = true;
            let result = alu.adc_u8(5, 4);
            assert_eq!(result, 10);

            Flags::zeros()
        });
    }

    #[test]
    fn adc_u8_full_carry() {
        test_alu_operation(|alu| {
            let result = alu.adc_u8(0xF0, 0x11);
            assert_eq!(result, 1);

            Flags::just_carry()
        });
    }

    #[test]
    fn adc_u8_half_carry() {
        test_alu_operation(|alu| {
            let result = alu.adc_u8(0x0F, 0x01);
            assert_eq!(result, 0x10);

            Flags::just_half_carry()
        });
    }

    #[test]
    fn adc_u8_all_flags() {
        test_alu_operation(|alu| {
            let result = alu.add_u8(0xFF, 0x01);
            assert_eq!(result, 0);

            Flags::new(true, true, false, true)
        });
    }

    #[test]
    fn adc_u8_negative() {
        test_alu_operation(|alu| {
            let result = alu.add_u8(1, (-3i8) as u8);
            assert_eq!(result, (-2i8) as u8);

            Flags::new(false, false, false, false)
        });
    }

    #[test]
    fn adc_u16_no_carry() {
        test_alu_operation(|alu| {
            let result = alu.adc_u16(50, 40);
            assert_eq!(result, 90);

            Flags::zeros()
        });
    }

    #[test]
    fn adc_u16_carry() {
        test_alu_operation(|alu| {
            alu.state.flags_mut().carry = true;
            let result = alu.adc_u16(50, 40);
            assert_eq!(result, 91);

            Flags::zeros()
        });
    }

    #[test]
    fn sub_u8_no_borrow() {
        test_alu_operation(|alu| {
            let result = alu.sub_u8(2, 3);
            assert_eq!(result, 1);

            Flags::just_subtraction()
        });
    }

    #[test]
    fn sub_u8_borrow() {
        test_alu_operation(|alu| {
            let result = alu.sub_u8(32, 0);
            assert_eq!(result, (-32i8) as u8);

            Flags::new(true, false, true, false)
        });
    }

    #[test]
    fn sbc_u8_no_carry() {
        test_alu_operation(|alu| {
            let result = alu.sbc_u8(32, 0);
            assert_eq!(result, (-32i8) as u8);

            Flags::new(true, false, true, false)
        });
    }

    #[test]
    fn and_u8_no_flag() {
        test_alu_operation(|alu| {
            let result = alu.and_u8(0x11, 0x10);
            assert_eq!(result, 0x10);

            Flags::just_half_carry()
        });
    }

    #[test]
    fn and_u8_zero() {
        test_alu_operation(|alu| {
            let result = alu.and_u8(0x00, 0xFF);
            assert_eq!(result, 0x00);

            Flags::just_zero().with_half_carry(true)
        });
    }

    #[test]
    fn xor_u8_no_flag() {
        test_alu_operation(|alu| {
            let result = alu.xor_u8(0x11, 0x10);
            assert_eq!(result, 0x01);

            Flags::zeros()
        });
    }

    #[test]
    fn xor_u8_zero() {
        test_alu_operation(|alu| {
            let result = alu.xor_u8(0x11, 0x11);
            assert_eq!(result, 0x00);

            Flags::just_zero()
        });
    }

    #[test]
    fn or_u8_no_flag_0() {
        test_alu_operation(|alu| {
            let result = alu.or_u8(0x11, 0x10);
            assert_eq!(result, 0x11);

            Flags::zeros()
        });
    }

    #[test]
    fn or_u8_no_flag_1() {
        test_alu_operation(|alu| {
            let result = alu.or_u8(0xF1, 0x10);
            assert_eq!(result, 0xF1);

            Flags::zeros()
        });
    }

    #[test]
    fn or_u8_zero() {
        test_alu_operation(|alu| {
            let result = alu.or_u8(0x00, 0x00);
            assert_eq!(result, 0x00);

            Flags::just_zero()
        });
    }

    #[test]
    fn cp_u8_no_borrow() {
        test_alu_operation(|alu| {
            alu.cp_u8(2, 3);
            Flags::just_subtraction()
        });
    }

    #[test]
    fn cp_u8_borrow() {
        test_alu_operation(|alu| {
            alu.cp_u8(32, 0);
            Flags::new(true, false, true, false)
        });
    }

    #[test]
    fn cp_u8_half_borrow() {
        test_alu_operation(|alu| {
            alu.cp_u8(8, 16);
            Flags::new(false, true, true, false)
        });
    }

    #[test]
    fn cp_u8_zero() {
        test_alu_operation(|alu| {
            alu.cp_u8(8, 8);
            Flags::new(false, false, true, true)
        });
    }

    #[test]
    fn rlc_u8_no_flag() {
        test_alu_operation(|alu| {
            let result = alu.rotate_left_u8(0b0000_1000, true, false);
            assert_eq!(result, 0b0001_0000);

            Flags::zeros()
        });
    }

    #[test]
    fn rlc_u8_zero() {
        test_alu_operation(|alu| {
            let result = alu.rotate_left_u8(0b0000_0000, true, false);
            assert_eq!(result, 0b0000_0000);

            Flags::just_zero()
        });
    }

    #[test]
    fn rlc_u8_carry() {
        test_alu_operation(|alu| {
            let result = alu.rotate_left_u8(0b1000_0000, true, false);
            assert_eq!(result, 0b0000_0001);

            Flags::just_carry()
        });
    }

    #[test]
    fn rrc_u8_no_flag() {
        test_alu_operation(|alu| {
            let result = alu.rotate_right_u8(0b0000_1000, true, false);
            assert_eq!(result, 0b0000_0100);

            Flags::zeros()
        });
    }

    #[test]
    fn rrc_u8_zero() {
        test_alu_operation(|alu| {
            let result = alu.rotate_right_u8(0b0000_0000, true, false);
            assert_eq!(result, 0b0000_0000);

            Flags::just_zero()
        });
    }

    #[test]
    fn rrc_u8_carry() {
        test_alu_operation(|alu| {
            let result = alu.rotate_right_u8(0b0000_0001, true, false);
            assert_eq!(result, 0b1000_0000);

            Flags::just_carry()
        });
    }

    #[test]
    fn test_swap_u8_0() {
        test_alu_operation(|alu| {
            let result = alu.swap_u8(0b0101_1100);
            assert_eq!(result, 0b1100_0101);

            Flags::zeros()
        });
    }

    #[test]
    fn test_swap_u8_1() {
        test_alu_operation(|alu| {
            let result = alu.swap_u8(0b0000_1111);
            assert_eq!(result, 0b1111_0000);

            Flags::zeros()
        });
    }

    #[test]
    fn test_shift_left_u8_normal() {
        test_alu_operation(|alu| {
            let result = alu.shift_left_arithmetic(0b0110_1001);
            assert_eq!(result, 0b1101_0010);

            Flags::zeros()
        });
    }

    #[test]
    fn test_shift_left_u8_carry() {
        test_alu_operation(|alu| {
            let result = alu.shift_left_arithmetic(0b1110_1001);
            assert_eq!(result, 0b1101_0010);

            Flags::just_carry()
        });
    }

    #[test]
    fn test_shift_left_u8_zero() {
        test_alu_operation(|alu| {
            let result = alu.shift_left_arithmetic(0b0000_0000);
            assert_eq!(result, 0b0000_0000);

            Flags::just_zero()
        });
    }

    #[test]
    fn test_shift_left_u8_carry_and_zero() {
        test_alu_operation(|alu| {
            let result = alu.shift_left_arithmetic(0b1000_0000);
            assert_eq!(result, 0b0000_0000);

            Flags::just_carry().with_zero(true)
        });
    }

    #[test]
    fn test_shift_right_arith_u8_bit_7_0() {
        test_alu_operation(|alu| {
            let result = alu.shift_right_arithmetic(0b0110_1010);
            assert_eq!(result, 0b0011_0101);

            Flags::zeros()
        });
    }

    #[test]
    fn test_shift_right_arith_u8_bit_7_1() {
        test_alu_operation(|alu| {
            let result = alu.shift_right_arithmetic(0b1110_1010);
            assert_eq!(result, 0b1111_0101);

            Flags::zeros()
        });
    }

    #[test]
    fn test_shift_right_arith_u8_carry() {
        test_alu_operation(|alu| {
            let result = alu.shift_right_arithmetic(0b0110_1011);
            assert_eq!(result, 0b0011_0101);

            Flags::just_carry()
        });
    }

    #[test]
    fn test_shift_right_arith_u8_zero() {
        test_alu_operation(|alu| {
            let result = alu.shift_right_arithmetic(0b0000_0000);
            assert_eq!(result, 0b0000_0000);

            Flags::just_zero()
        });
    }

    #[test]
    fn test_shift_right_arith_u8_carry_and_zero() {
        test_alu_operation(|alu| {
            let result = alu.shift_right_arithmetic(0b0000_0001);
            assert_eq!(result, 0b0000_0000);

            Flags::just_carry().with_zero(true)
        });
    }

    #[test]
    fn test_shift_right_logical_u8_bit_7_0() {
        test_alu_operation(|alu| {
            let result = alu.shift_right_logical(0b0110_1010);
            assert_eq!(result, 0b0011_0101);

            Flags::zeros()
        });
    }

    #[test]
    fn test_shift_right_logical_u8_bit_7_1() {
        test_alu_operation(|alu| {
            let result = alu.shift_right_logical(0b1110_1010);
            assert_eq!(result, 0b0111_0101);

            Flags::zeros()
        });
    }

    #[test]
    fn test_shift_right_logical_u8_carry() {
        test_alu_operation(|alu| {
            let result = alu.shift_right_logical(0b0110_1011);
            assert_eq!(result, 0b0011_0101);

            Flags::just_carry()
        });
    }

    #[test]
    fn test_shift_right_logical_u8_zero() {
        test_alu_operation(|alu| {
            let result = alu.shift_right_logical(0b0000_0000);
            assert_eq!(result, 0b0000_0000);

            Flags::just_zero()
        });
    }

    #[test]
    fn test_shift_right_logical_u8_carry_and_zero() {
        test_alu_operation(|alu| {
            let result = alu.shift_right_logical(0b0000_0001);
            assert_eq!(result, 0b0000_0000);

            Flags::just_carry().with_zero(true)
        });
    }

    #[test]
    fn test_test_bit_u8_set_0() {
        test_alu_operation(|alu| {
            alu.test_bit_u8(0, 1);
            Flags::just_half_carry()
        });
    }

    #[test]
    fn test_test_bit_u8_unset_0() {
        test_alu_operation(|alu| {
            alu.test_bit_u8(0, 0);
            Flags::new(false, true, false, true)
        });
    }

    #[test]
    fn test_test_bit_u8_set_6() {
        test_alu_operation(|alu| {
            alu.test_bit_u8(6, 0b0100_0000);
            Flags::just_half_carry()
        });
    }

    #[test]
    fn test_test_bit_u8_unset_6() {
        test_alu_operation(|alu| {
            alu.test_bit_u8(6, 0b1011_1111);
            Flags::new(false, true, false, true)
        });
    }

    #[test]
    fn test_reset_bit_u8_set_0() {
        test_alu_operation(|alu| {
            let result = alu.reset_bit_u8(0, 0b1001_0011);
            assert_eq!(result, 0b1001_0010);
            Flags::zeros()
        });
    }

    #[test]
    fn test_reset_bit_u8_set_4() {
        test_alu_operation(|alu| {
            let result = alu.reset_bit_u8(4, 0b1001_1011);
            assert_eq!(result, 0b1000_1011);
            Flags::zeros()
        });
    }

    #[test]
    fn test_set_bit_u8_unset_0() {
        test_alu_operation(|alu| {
            let result = alu.set_bit_u8(0, 0b1001_0010);
            assert_eq!(result, 0b1001_0011);
            Flags::zeros()
        });
    }

    #[test]
    fn test_set_bit_u8_unset_4() {
        test_alu_operation(|alu| {
            let result = alu.set_bit_u8(4, 0b1000_1011);
            assert_eq!(result, 0b1001_1011);
            Flags::zeros()
        });
    }
}
