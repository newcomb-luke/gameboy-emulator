use super::execution_state::{Flags, SharedExecutionState};

pub struct Alu {
    state: SharedExecutionState,
}

impl Alu {
    pub fn new(state: SharedExecutionState) -> Self {
        Self { state }
    }

    pub fn rotate_left_u8(&self, v: u8) -> u8 {
        let carry = v >> 7;
        let result = (v << 1) | carry;

        let flags = Flags::new(carry != 0, false, false, result == 0);
        self.state.set_flags(flags);

        result
    }

    /// v1 - v2
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
        todo!()
    }

    pub fn dec_u8(&self, val: u8) -> u8 {
        todo!()
    }

    pub fn add_u16(&self, v1: u16, v2: u16) -> u16 {
        let half_result = (v1 & 0x0FFF) + (v2 & 0x0FFF);
        let big_result = (v1 as u32) + (v2 as u32);
        let real_result = (big_result & 0xFFFF) as u16;

        let mut carry = false;
        let mut half_carry = false;
        let zero = real_result == 0;

        if (big_result >> 16) != 0 {
            carry = true;
        }

        if (half_result >> 12) != 0 {
            half_carry = true;
        }

        let flags = Flags::new(carry, half_carry, false, zero);
        self.state.set_flags(flags);

        real_result
    }

    pub fn add_u8(&self, v1: u8, v2: u8) -> u8 {
        let half_result = (v1 & 0x0F) + (v2 & 0x0F);
        let big_result = (v1 as u16) + (v2 as u16);
        let real_result = (big_result & 0xFF) as u8;

        let mut carry = false;
        let mut half_carry = false;
        let zero = real_result == 0;

        if (big_result >> 8) != 0 {
            carry = true;
        }

        if (half_result >> 4) != 0 {
            half_carry = true;
        }

        let flags = Flags::new(carry, half_carry, false, zero);
        self.state.set_flags(flags);

        real_result
    }

    pub fn xor_u8(&self, v1: u8, v2: u8) -> u8 {
        let result = v1 ^ v2;

        let flags = Flags::zeros().with_zero(result == 0);
        self.state.set_flags(flags);

        result
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
            let result = alu.rotate_left_u8(0b0000_1000);
            assert_eq!(result, 0b0001_0000);

            Flags::zeros()
        });
    }

    #[test]
    fn rlc_u8_zero() {
        test_alu_operation(|alu| {
            let result = alu.rotate_left_u8(0b0000_0000);
            assert_eq!(result, 0b0000_0000);

            Flags::just_zero()
        });
    }

    #[test]
    fn rlc_u8_carry() {
        test_alu_operation(|alu| {
            let result = alu.rotate_left_u8(0b1000_0000);
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
