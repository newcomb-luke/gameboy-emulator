use super::execution_state::Flags;

pub struct Alu {}

impl Alu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn rotate_left_u8(&self, v: u8) -> (u8, Flags) {
        let carry = v >> 7;

        let result = (v << 1) | carry;

        let flags = Flags::new(carry != 0, false, false, result == 0);

        (result, flags)
    }

    pub fn add_u16(&self, v1: u16, v2: u16) -> (u16, Flags) {
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

        (real_result, flags)
    }

    pub fn add_u8(&self, v1: u8, v2: u8) -> (u8, Flags) {
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

        (real_result, flags)
    }

    pub fn xor_u8(&self, v1: u8, v2: u8) -> (u8, Flags) {
        let result = v1 ^ v2;

        let flags = Flags::zeros().with_zero(result == 0);

        (result, flags)
    }

    pub fn xor_u16(&self, v1: u16, v2: u16) -> (u16, Flags) {
        let result = v1 ^ v2;

        let flags = Flags::zeros().with_zero(result == 0);

        (result, flags)
    }

    pub fn test_bit_u8(&self, idx: u8, v: u8) -> (Flags, Flags) {
        let zero = ((v >> idx) & 1) == 0;

        let flags = Flags::new(false, true, false, zero);
        let mask = Flags::new(false, true, true, true);

        (flags, mask)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::execution_state::Flags;

    use super::Alu;

    #[test]
    fn add_u16_no_carry() {
        let alu = Alu::new();
        let (result, flags) = alu.add_u16(1, 2);

        assert_eq!(result, 3);
        assert_eq!(flags, Flags::zeros());
    }

    #[test]
    fn add_u16_full_carry() {
        let alu = Alu::new();
        let (result, flags) = alu.add_u16(0xF000, 0x1001);

        assert_eq!(result, 1);
        assert_eq!(flags, Flags::zeros().with_carry(true));
    }

    #[test]
    fn add_u16_half_carry() {
        let alu = Alu::new();
        let (result, flags) = alu.add_u16(0x0FFF, 0x0001);

        assert_eq!(result, 0x1000);
        assert_eq!(flags, Flags::zeros().with_half_carry(true));
    }

    #[test]
    fn add_u16_all_flags() {
        let alu = Alu::new();
        let (result, flags) = alu.add_u16(0xFFFF, 0x0001);

        assert_eq!(result, 0);
        assert_eq!(flags, Flags::new(true, true, false, true));
    }

    #[test]
    fn add_u8_no_carry() {
        let alu = Alu::new();
        let (result, flags) = alu.add_u8(1, 2);

        assert_eq!(result, 3);
        assert_eq!(flags, Flags::zeros());
    }

    #[test]
    fn add_u8_full_carry() {
        let alu = Alu::new();
        let (result, flags) = alu.add_u8(0xF0, 0x11);

        assert_eq!(result, 1);
        assert_eq!(flags, Flags::zeros().with_carry(true));
    }

    #[test]
    fn add_u8_half_carry() {
        let alu = Alu::new();
        let (result, flags) = alu.add_u8(0x0F, 0x01);

        assert_eq!(result, 0x10);
        assert_eq!(flags, Flags::zeros().with_half_carry(true));
    }

    #[test]
    fn add_u8_all_flags() {
        let alu = Alu::new();
        let (result, flags) = alu.add_u8(0xFF, 0x01);

        assert_eq!(result, 0);
        assert_eq!(flags, Flags::new(true, true, false, true));
    }

    #[test]
    fn xor_u16_no_flag() {
        let alu = Alu::new();
        let (result, flags) = alu.xor_u16(0x1100, 0x1000);

        assert_eq!(result, 0x0100);
        assert_eq!(flags, Flags::zeros());
    }

    #[test]
    fn xor_u16_zero() {
        let alu = Alu::new();
        let (result, flags) = alu.xor_u16(0x1100, 0x1100);

        assert_eq!(result, 0x0000);
        assert_eq!(flags, Flags::zeros().with_zero(true));
    }

    #[test]
    fn xor_u8_no_flag() {
        let alu = Alu::new();
        let (result, flags) = alu.xor_u8(0x11, 0x10);

        assert_eq!(result, 0x01);
        assert_eq!(flags, Flags::zeros());
    }

    #[test]
    fn xor_u8_zero() {
        let alu = Alu::new();
        let (result, flags) = alu.xor_u8(0x11, 0x11);

        assert_eq!(result, 0x00);
        assert_eq!(flags, Flags::zeros().with_zero(true));
    }

    #[test]
    fn rlc_u8_no_flag() {
        let alu = Alu::new();
        let (result, flags) = alu.rotate_left_u8(0b0000_1000);

        assert_eq!(result, 0b0001_0000);
        assert_eq!(flags, Flags::zeros());
    }

    #[test]
    fn rlc_u8_zero() {
        let alu = Alu::new();
        let (result, flags) = alu.rotate_left_u8(0b0000_0000);

        assert_eq!(result, 0b0000_0000);
        assert_eq!(flags, Flags::zeros().with_zero(true));
    }

    #[test]
    fn rlc_u8_carry() {
        let alu = Alu::new();
        let (result, flags) = alu.rotate_left_u8(0b1000_0000);

        assert_eq!(result, 0b0000_0001);
        assert_eq!(flags, Flags::zeros().with_carry(true));
    }

    #[test]
    fn test_bit_u8_mask() {
        let alu = Alu::new();
        let (_, mask) = alu.test_bit_u8(0, 1);

        assert_eq!(mask, Flags::new(false, true, true, true));
    }

    #[test]
    fn test_bit_u8_set_0() {
        let alu = Alu::new();
        let (flags, _) = alu.test_bit_u8(0, 1);

        assert_eq!(flags, Flags::new(false, true, false, false));
    }

    #[test]
    fn test_bit_u8_unset_0() {
        let alu = Alu::new();
        let (flags, _) = alu.test_bit_u8(0, 0);

        assert_eq!(flags, Flags::new(false, true, false, true));
    }

    #[test]
    fn test_bit_u8_set_6() {
        let alu = Alu::new();
        let (flags, _) = alu.test_bit_u8(6, 0b0100_0000);

        assert_eq!(flags, Flags::new(false, true, false, false));
    }

    #[test]
    fn test_bit_u8_unset_6() {
        let alu = Alu::new();
        let (flags, _) = alu.test_bit_u8(6, 0b1011_1111);

        assert_eq!(flags, Flags::new(false, true, false, true));
    }
}