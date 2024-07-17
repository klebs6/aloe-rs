crate::ix!();

/// Struct for SIMD operations on 64-bit integers utilizing AVX intrinsics
/// Tags: DSP
pub struct SimdAvxI64;

impl SimdAvxI64 {

    /// All bits set constant for 64-bit integers
    const ALL_BITS_SET: i64x4 = i64x4::splat(-1);

    /// Expands a scalar to all SIMD lanes.
    pub fn expand(s: i64) -> i64x4 {
        i64x4::splat(s)
    }

    /// Loads data from a pointer into a SIMD register.
    pub fn load(p: *const i64) -> i64x4 {
        unsafe { intrinsics::simd_load(p) }
    }

    /// Stores the contents of the SIMD register into a pointer.
    pub fn store(value: i64x4, dest: *mut i64) {
        unsafe { intrinsics::simd_store(dest, value) }
    }

    /// Element-wise addition of two SIMD vectors.
    pub fn add(a: i64x4, b: i64x4) -> i64x4 {
        a + b
    }

    /// Element-wise subtraction of two SIMD vectors.
    pub fn sub(a: i64x4, b: i64x4) -> i64x4 {
        a - b
    }

    /// Bitwise AND of two SIMD vectors.
    pub fn bit_and(a: i64x4, b: i64x4) -> i64x4 {
        a & b
    }

    /// Bitwise OR of two SIMD vectors.
    pub fn bit_or(a: i64x4, b: i64x4) -> i64x4 {
        a | b
    }

    /// Bitwise XOR of two SIMD vectors.
    pub fn bit_xor(a: i64x4, b: i64x4) -> i64x4 {
        a ^ b
    }

    /// Bitwise AND-NOT of two SIMD vectors.
    pub fn bit_andnot(a: i64x4, b: i64x4) -> i64x4 {
        (!a) & b
    }

    /// Bitwise NOT of a SIMD vector.
    pub fn bit_not(a: i64x4) -> i64x4 {
        a ^ Self::ALL_BITS_SET
    }

    /// Element-wise minimum of two SIMD vectors.
    pub fn min(a: i64x4, b: i64x4) -> i64x4 {
        a.min(b)
    }

    /// Element-wise maximum of two SIMD vectors.
    pub fn max(a: i64x4, b: i64x4) -> i64x4 {
        a.max(b)
    }

    /// Element-wise equality of two SIMD vectors.
    pub fn equal(a: i64x4, b: i64x4) -> i64x4 {
        a.eq(b)
    }

    /// Checks if elements in 'a' are greater than corresponding elements in 'b'.
    pub fn greater_than(a: i64x4, b: i64x4) -> i64x4 {
        a.gt(b)
    }

    /// Checks if elements in 'a' are greater than or equal to corresponding elements in 'b'.
    pub fn greater_than_or_equal(a: i64x4, b: i64x4) -> i64x4 {
        a.ge(b)
    }

    /// Multiply and add: a + (b * c)
    pub fn multiply_add(a: i64x4, b: i64x4, c: i64x4) -> i64x4 {
        a + (b * c)
    }

    /// Element-wise inequality of two SIMD vectors.
    pub fn not_equal(a: i64x4, b: i64x4) -> i64x4 {
        a.ne(b)
    }

    /// Checks if all elements in 'a' are equal to corresponding elements in 'b'.
    pub fn all_equal(a: i64x4, b: i64x4) -> bool {
        // TODO: Replace with an optimized routine for AVX-based comparison
        a.eq(b).all()
    }

    /// Element-wise multiplication of two SIMD vectors.
    pub fn mul(a: i64x4, b: i64x4) -> i64x4 {
        a * b
    }
}
