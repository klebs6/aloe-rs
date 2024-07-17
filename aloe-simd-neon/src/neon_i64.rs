crate::ix!();

pub struct SimdNeonI64;

impl SimdNeonI64 {

    /// All bits set constant
    const ALL_BITS_SET: i64 = !0;

    /// Duplicate `i64` across all lanes
    pub fn expand(s: i64) -> i64x2 {
        i64x2::splat(s)
    }

    /// Load from a pointer
    pub fn load(a: &[i64; 2]) -> i64x2 {
        i64x2::from_slice_unaligned(a)
    }

    /// Store to a pointer
    pub fn store(value: i64x2, a: &mut [i64; 2]) {
        value.write_to_slice_unaligned(a);
    }

    /// Get the value at a specific lane
    pub fn get(v: i64x2, i: usize) -> Option<i64> {
        if i < 2 {
            Some(v.extract(i))
        } else {
            None
        }
    }

    /// Set the value at a specific lane
    pub fn set(mut v: i64x2, i: usize, s: i64) -> Option<i64x2> {
        if i < 2 {
            v = v.replace(i, s);
            Some(v)
        } else {
            None
        }
    }

    /// Vector addition
    pub fn add(a: i64x2, b: i64x2) -> i64x2 {
        a + b
    }

    /// Vector subtraction
    pub fn sub(a: i64x2, b: i64x2) -> i64x2 {
        a - b
    }

    // Vector multiplication (fallback)
    // In this implementation, multiplication operation is not directly provided by `packed_simd`
    pub fn mul(a: i64x2, b: i64x2) -> i64x2 {
        // Fallback operation for mul
        i64x2::new(a.extract(0) * b.extract(0), a.extract(1) * b.extract(1))
    }

    /// Bitwise AND
    pub fn bit_and(a: i64x2, b: i64x2) -> i64x2 {
        a & b
    }

    /// Bitwise OR
    pub fn bit_or(a: i64x2, b: i64x2) -> i64x2 {
        a | b
    }

    /// Bitwise XOR
    pub fn bit_xor(a: i64x2, b: i64x2) -> i64x2 {
        a ^ b
    }

    /// Bitwise NOT AND
    pub fn bit_notand(a: i64x2, b: i64x2) -> i64x2 {
        (!a) & b
    }

    /// Bitwise NOT
    pub fn bit_not(a: i64x2) -> i64x2 {
        !a
    }

    // Min (fallback)
    pub fn min(a: i64x2, b: i64x2) -> i64x2 {
        i64x2::new(a.extract(0).min(b.extract(0)), a.extract(1).min(b.extract(1)))
    }

    // Max (fallback)
    pub fn max(a: i64x2, b: i64x2) -> i64x2 {
        i64x2::new(a.extract(0).max(b.extract(0)), a.extract(1).max(b.extract(1)))
    }

    /// Element-wise equality (fallback)
    pub fn equal(a: i64x2, b: i64x2) -> i64x2 {
        i64x2::new((a.extract(0) == b.extract(0)) as i64, (a.extract(1) == b.extract(1)) as i64)
    }

    /// Element-wise inequality (fallback)
    pub fn not_equal(a: i64x2, b: i64x2) -> i64x2 {
        i64x2::new((a.extract(0) != b.extract(0)) as i64, (a.extract(1) != b.extract(1)) as i64)
    }

    /// Sum all lanes (fallback)
    pub fn sum(v: i64x2) -> i64 {
        v.extract(0) + v.extract(1)
    }

    /// Element-wise greater than comparison (fallback)
    pub fn greater_than(a: i64x2, b: i64x2) -> i64x2 {
        i64x2::new((a.extract(0) > b.extract(0)) as i64, (a.extract(1) > b.extract(1)) as i64)
    }

    /// Element-wise greater than or equal comparison (fallback)
    pub fn greater_than_or_equal(a: i64x2, b: i64x2) -> i64x2 {
        i64x2::new((a.extract(0) >= b.extract(0)) as i64, (a.extract(1) >= b.extract(1)) as i64)
    }

    /// All lanes equal (fallback)
    pub fn all_equal(a: i64x2, b: i64x2) -> bool {
        a.extract(0) == b.extract(0) && a.extract(1) == b.extract(1)
    }

    /// Multiply and add operation (fused multiply-add)
    /// Compute `a * b + c` for each lane (fallback)
    pub fn multiply_add(a: i64x2, b: i64x2, c: i64x2) -> i64x2 {
        i64x2::new(a.extract(0) * b.extract(0) + c.extract(0), a.extract(1) * b.extract(1) + c.extract(1))
    }
}
