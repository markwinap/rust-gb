
use num_traits::{PrimInt, WrappingAdd, WrappingSub};


pub trait IntExt {
    /// Isolates the rightmost 1-bit leaving all other bits as 0
    /// e.g. 1010 1000 -> 0000 1000
    ///
    /// Equivalent to Intel BMI1 instruction BLSI
    fn isolate_rightmost_one(self) -> Self;

    /// Returns the specified bit as 0 or 1
    fn bit(self, bit: usize) -> Self;

    /// Returns the specified bit as boolean
    fn bit_bool(self, bit: usize) -> bool;

    /// Sets all rightmost 0-bits to 1
    /// e.g. 1010 1000 -> 1010 1111
    ///
    /// Equivalent to Intel BMI1 instruction BLSMSK
    fn activate_rightmost_zeros(self) -> Self;

    /// Tests if addition results in a carry from the specified bit.
    /// Does not support overflow, so cannot be used to check carry from the leftmost bit
    fn test_add_carry_bit(bit: usize, a: Self, b: Self) -> bool;
}

impl<T> IntExt for T
    where
        T: PrimInt + WrappingAdd + WrappingSub,
{
    /// Isolates the rightmost 1-bit leaving all other bits as 0
    /// e.g. 1010 1000 -> 0000 1000
    ///
    /// Equivalent to Intel BMI1 instruction BLSI
    #[inline(always)]
    fn isolate_rightmost_one(self) -> Self {
        let x = self;
        // Unsigned negation: -x == !x + 1
        let minus_x = (!x).wrapping_add(&Self::one());
        // Hacker's Delight 2nd ed, 2-1 Manipulating Rightmost Bits
        x & minus_x
    }

    /// Returns the specified bit as 0 or 1
    #[inline(always)]
    fn bit(self, bit: usize) -> Self {
        (self >> bit) & Self::one()
    }

    /// Returns the specified bit as boolean
    #[inline(always)]
    fn bit_bool(self, bit: usize) -> bool {
        !self.bit(bit).is_zero()
    }

    /// Sets all rightmost 0-bits to 1
    /// e.g. 1010 1000 -> 1010 1111
    ///
    /// Equivalent to Intel BMI1 instruction BLSMSK
    #[inline(always)]
    fn activate_rightmost_zeros(self) -> Self {
        let x = self;
        // Hacker's Delight 2nd ed, 2-1 Manipulating Rightmost Bits
        x | x.wrapping_sub(&Self::one())
    }

    /// Tests if addition results in a carry from the specified bit.
    /// Does not support overflow, so cannot be used to check carry from the leftmost bit
    #[inline(always)]
    fn test_add_carry_bit(bit: usize, a: Self, b: Self) -> bool {
        // Create a mask that includes the specified bit and 1-bits on the right side
        // e.g. for u8:
        //   bit=0 -> 0000 0001
        //   bit=3 -> 0000 1111
        //   bit=6 -> 0111 1111
        let mask = (Self::one() << bit).activate_rightmost_zeros();
        (a & mask) + (b & mask) > mask
    }
}

