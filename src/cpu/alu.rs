use super::execution_state::{Flags, SharedExecutionState};

pub struct Alu {
    state: SharedExecutionState,
}

impl Alu {
    pub fn new(state: SharedExecutionState) -> Self {
        Self { state }
    }

    pub fn rotate_left_u8(&self, v: u8, update_zero_flag: bool, through_carry: bool) -> u8 {
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

    /// v2 - v1
    pub fn sub_u16(&self, v1: u16, v2: u16) -> u16 {
        todo!()
    }

    pub fn inc_u16(&self, val: u16) -> u16 {
        val.wrapping_add(1)
    }

    pub fn dec_u16(&self, val: u16) -> u16 {
        val.wrapping_sub(1)
    }

    pub fn inc_u8(&self, val: u8) -> u8 {
        let flags_before = self.state.flags();

        let result = self.add_u8(val, 1);

        self.state.modify_flags(|f| {
            f.carry = flags_before.carry;
        });

        result
    }

    pub fn dec_u8(&self, val: u8) -> u8 {
        let flags_before = self.state.flags();

        let result = self.sub_u8(1, val);

        self.state.modify_flags(|f| {
            f.carry = flags_before.carry;
        });

        result
    }

    pub fn add_u16(&self, v1: u16, v2: u16) -> u16 {
        let (result, carry) = v1.overflowing_add(v2);
        let half_result = (v1 & 0x0FFF) + (v2 & 0x0FFF);
        let half_carry = half_result > 0x0FFF;

        let flags = Flags::new(carry, half_carry, false, result == 0);
        self.state.set_flags(flags);

        result
    }

    pub fn add_u8(&self, v1: u8, v2: u8) -> u8 {
        let (result, carry) = v1.overflowing_add(v2);

        let half_result = (v1 & 0x0F) + (v2 & 0x0F);
        let half_carry = half_result > 0x0F;

        let flags = Flags::new(carry, half_carry, false, result == 0);
        self.state.set_flags(flags);

        result
    }

    pub fn adc_u8(&self, v1: u8, v2: u8) -> u8 {
        todo!()
    }

    /// v2 - v1
    pub fn sub_u8(&self, v1: u8, v2: u8) -> u8 {
        let (result, carry) = v2.overflowing_sub(v1);
        let half_carry = (v2 & 0x0F) < (v1 & 0x0F);

        self.state.set_flags(Flags::new(carry, half_carry, true, result == 0));

        result
    }

    pub fn sbc_u8(&self, v1: u8, v2: u8) -> u8 {
        todo!()
    }

    pub fn and_u8(&self, v1: u8, v2: u8) -> u8 {
        todo!()
    }

    pub fn xor_u8(&self, v1: u8, v2: u8) -> u8 {
        let result = v1 ^ v2;

        let flags = Flags::zeros().with_zero(result == 0);
        self.state.set_flags(flags);

        result
    }

    pub fn or_u8(&self, v1: u8, v2: u8) -> u8 {
        todo!()
    }

    pub fn cp_u8(&self, v1: u8, v2: u8) {
        self.sub_u8(v1, v2);
    }

    pub fn xor_u16(&self, v1: u16, v2: u16) -> u16 {
        let result = v1 ^ v2;

        let flags = Flags::zeros().with_zero(result == 0);
        self.state.set_flags(flags);

        result
    }

    pub fn test_bit_u8(&self, idx: u8, v: u8) {
        let zero = ((v >> idx) & 1) == 0;

        self.state.modify_flags(|f| {
            f.half_carry = true;
            f.subtraction = false;
            f.zero = zero;
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::execution_state::{Flags, SharedExecutionState};

    use super::Alu;

    fn test_alu_operation<F>(f: F)
    where
        F: FnOnce(&mut Alu) -> Flags,
    {
        let state = SharedExecutionState::new();
        let mut alu = Alu::new(state.clone());

        let desired_flags = f(&mut alu);

        assert_eq!(state.flags(), desired_flags);
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
    fn xor_u16_no_flag() {
        test_alu_operation(|alu| {
            let result = alu.xor_u16(0x1100, 0x1000);
            assert_eq!(result, 0x0100);

            Flags::zeros()
        });
    }

    #[test]
    fn xor_u16_zero() {
        test_alu_operation(|alu| {
            let result = alu.xor_u16(0x1100, 0x1100);
            assert_eq!(result, 0);

            Flags::just_zero()
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
    fn test_bit_u8_set_0() {
        test_alu_operation(|alu| {
            alu.test_bit_u8(0, 1);
            Flags::just_half_carry()
        });
    }

    #[test]
    fn test_bit_u8_unset_0() {
        test_alu_operation(|alu| {
            alu.test_bit_u8(0, 0);
            Flags::new(false, true, false, true)
        });
    }

    #[test]
    fn test_bit_u8_set_6() {
        test_alu_operation(|alu| {
            alu.test_bit_u8(6, 0b0100_0000);
            Flags::just_half_carry()
        });
    }

    #[test]
    fn test_bit_u8_unset_6() {
        test_alu_operation(|alu| {
            alu.test_bit_u8(6, 0b1011_1111);
            Flags::new(false, true, false, true)
        });
    }
}
