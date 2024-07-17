crate::ix!();

/// Struct encapsulating Signed 16-bit integer NEON intrinsics.
/// # Tags
/// DSP
pub struct SimdNeonI16;

impl SimdNeonI16 {
    // All bits set constant
    const ALL_BITS_SET: int16x8 = int16x8::splat(-1);

    /// Duplicates an int16_t value across all lanes.
    pub fn expand(s: i16) -> int16x8 {
        int16x8::splat(s)
    }

    /// Loads from a slice.
    pub fn load(a: &[i16; 8]) -> int16x8 {
        int16x8::from_slice_unaligned(a)
    }

    /// Stores into a slice.
    pub fn store(value: int16x8, a: &mut [i16; 8]) {
        value.write_to_slice_unaligned(a);
    }

    /// Gets the value from a specified lane.
    pub fn get(value: int16x8, i: usize) -> i16 {
        value.extract(i as u32)
    }

    /// Sets the value in a specified lane.
    pub fn set(value: int16x8, i: usize, s: i16) -> int16x8 {
        value.replace(i as u32, s)
    }

    /// Element-wise addition.
    pub fn add(a: int16x8, b: int16x8) -> int16x8 {
        a + b
    }

    /// Element-wise subtraction.
    pub fn sub(a: int16x8, b: int16x8) -> int16x8 {
        a - b
    }

    /// Element-wise multiplication.
    pub fn mul(a: int16x8, b: int16x8) -> int16x8 {
        a * b
    }

    /// Bitwise AND.
    pub fn bit_and(a: int16x8, b: int16x8) -> int16x8 {
        a & b
    }

    /// Bitwise OR.
    pub fn bit_or(a: int16x8, b: int16x8) -> int16x8 {
        a | b
    }

    /// Bitwise XOR.
    pub fn bit_xor(a: int16x8, b: int16x8) -> int16x8 {
        a ^ b
    }

    /// Bitwise NOT AND.
    pub fn bit_notand(a: int16x8, b: int16x8) -> int16x8 {
        !a & b
    }

    /// Bitwise NOT.
    pub fn bit_not(a: int16x8) -> int16x8 {
        !a
    }

    /// Element-wise minimum.
    pub fn min(a: int16x8, b: int16x8) -> int16x8 {
        a.min(b)
    }

    /// Element-wise maximum.
    pub fn max(a: int16x8, b: int16x8) -> int16x8 {
        a.max(b)
    }

    /// Element-wise equality.
    pub fn equal(a: int16x8, b: int16x8) -> int16x8 {
        a.eq(b)
    }

    /// Element-wise inequality.
    pub fn not_equal(a: int16x8, b: int16x8) -> int16x8 {
        a.ne(b)
    }

    /// Element-wise greater than.
    pub fn greater_than(a: int16x8, b: int16x8) -> int16x8 {
        a.gt(b)
    }

    /// Element-wise greater than or equal.
    pub fn greater_than_or_equal(a: int16x8, b: int16x8) -> int16x8 {
        a.ge(b)
    }

    /// Element-wise fused multiply-add.
    pub fn multiply_add(a: int16x8, b: int16x8, c: int16x8) -> int16x8 {
        a.mul_add(b, c)
    }

    /// Sums all the lanes.
    pub fn sum(a: int16x8) -> i16 {
        a.wrapping_sum()
    }
}
