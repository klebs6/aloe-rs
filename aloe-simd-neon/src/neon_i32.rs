crate::ix!();

pub struct SimdNeonI32;

impl SimdNeonI32 {

    /// All bits set constant
    const ALL_BITS_SET: i32 = !0;

    /// Duplicate `i32` across all lanes
    pub fn expand(s: i32) -> i32x4 {
        i32x4::splat(s)
    }

    /// Load from a pointer
    pub fn load(a: &[i32; 4]) -> i32x4 {
        i32x4::from_slice_unaligned(a)
    }

    /// Store to a pointer
    pub fn store(value: i32x4, a: &mut [i32; 4]) {
        value.write_to_slice_unaligned(a);
    }

    /// Get the value at a specific lane
    pub fn get(v: i32x4, i: usize) -> Option<i32> {
        if i < 4 {
            Some(v.extract(i))
        } else {
            None
        }
    }

    /// Set the value at a specific lane
    pub fn set(mut v: i32x4, i: usize, s: i32) -> Option<i32x4> {
        if i < 4 {
            v = v.replace(i, s);
            Some(v)
        } else {
            None
        }
    }

    /// Vector addition
    pub fn add(a: i32x4, b: i32x4) -> i32x4 {
        a + b
    }

    /// Vector subtraction
    pub fn sub(a: i32x4, b: i32x4) -> i32x4 {
        a - b
    }

    /// Vector multiplication
    pub fn mul(a: i32x4, b: i32x4) -> i32x4 {
        a * b
    }

    /// Bitwise AND
    pub fn bit_and(a: i32x4, b: i32x4) -> i32x4 {
        a & b
    }

    /// Bitwise OR
    pub fn bit_or(a: i32x4, b: i32x4) -> i32x4 {
        a | b
    }

    /// Bitwise XOR
    pub fn bit_xor(a: i32x4, b: i32x4) -> i32x4 {
        a ^ b
    }

    /// Bitwise NOT AND
    pub fn bit_notand(a: i32x4, b: i32x4) -> i32x4 {
        (!a) & b
    }

    /// Bitwise NOT
    pub fn bit_not(a: i32x4) -> i32x4 {
        !a
    }

    /// Min of two vectors
    pub fn min(a: i32x4, b: i32x4) -> i32x4 {
        a.min(b)
    }

    /// Max of two vectors
    pub fn max(a: i32x4, b: i32x4) -> i32x4 {
        a.max(b)
    }

    /// Element-wise equality
    pub fn equal(a: i32x4, b: i32x4) -> i32x4 {
        let eq_mask: i32x4 = a.eq(b);
        eq_mask & i32x4::splat(Self::ALL_BITS_SET)
    }

    /// Element-wise inequality
    pub fn not_equal(a: i32x4, b: i32x4) -> i32x4 {
        Self::bit_not(Self::equal(a, b))
    }

    /// Sum all lanes
    pub fn sum(v: i32x4) -> i32 {
        v.sum()
    }
}
