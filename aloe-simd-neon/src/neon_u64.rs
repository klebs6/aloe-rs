crate::ix!();

type vSIMDType = uint64x2_t;

type Fb = SIMDFallbackOps<u64,vSIMDType>;

/// Unsigned 64-bit integer NEON intrinsics.
pub struct SimdNeonU64;

impl SimdNeonU64 {
    /// All bits set constant for SIMD operations
    const ALL_BITS_SET: [u64; 2] = [u64::MAX; 2];

    /// Expands a scalar to all elements of a SIMD vector.
    pub fn expand(s: u64) -> uint64x2_t {
        unsafe { vdupq_n_u64(s) }
    }

    /// Loads data from memory into a SIMD register.
    pub fn load(a: &[u64; 2]) -> uint64x2_t {
        unsafe { vld1q_u64(a.as_ptr()) }
    }

    /// Stores a SIMD register value into memory.
    pub fn store(value: uint64x2_t, a: &mut [u64; 2]) {
        unsafe { vst1q_u64(a.as_mut_ptr(), value); }
    }

    /// Extracts an element from a SIMD vector.
    pub fn get(v: uint64x2_t, i: usize) -> Option<u64> {
        if i < 2 {
            unsafe { Some(vgetq_lane_u64(v, i as i32)) }
        } else {
            None
        }
    }

    /// Sets an element in a SIMD vector.
    pub fn set(v: uint64x2_t, i: usize, s: u64) -> Option<uint64x2_t> {
        if i < 2 {
            Some(unsafe { vsetq_lane_u64(s, v, i as i32) })
        } else {
            None
        }
    }

    /// Adds two SIMD vectors.
    pub fn add(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        unsafe { vaddq_u64(a, b) }
    }

    /// Subtracts the second SIMD vector from the first.
    pub fn sub(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        unsafe { vsubq_u64(a, b) }
    }

    /// Multiplies two SIMD vectors.
    pub fn mul(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        SIMDFallbackOps::mul(a, b)
    }

    /// Bitwise AND of two SIMD vectors.
    pub fn bit_and(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        unsafe { vandq_u64(a, b) }
    }

    /// Bitwise OR of two SIMD vectors.
    pub fn bit_or(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        unsafe { vorrq_u64(a, b) }
    }

    /// Bitwise XOR of two SIMD vectors.
    pub fn bit_xor(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        unsafe { veorq_u64(a, b) }
    }

    /// Bitwise NOT AND between two SIMD vectors.
    pub fn bit_notand(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        unsafe { vbicq_u64(b, a) }
    }

    /// Bitwise NOT of a SIMD vector.
    pub fn bit_not(a: uint64x2_t) -> uint64x2_t {
        Self::bit_notand(a, Self::load(&Self::ALL_BITS_SET))
    }

    /// Computes the minimum of two SIMD vectors.
    pub fn min(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        SIMDFallbackOps::min(a, b)
    }

    /// Computes the maximum of two SIMD vectors.
    pub fn max(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        SIMDFallbackOps::max(a, b)
    }

    /// Compares the elements of two SIMD vectors for equality.
    pub fn equal(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        unsafe { vceqq_u64(a, b) }
    }

    /// Compares the elements of two SIMD vectors for inequality.
    pub fn not_equal(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        unsafe { vmvnq_u64(vceqq_u64(a, b)) }
    }

    /// Checks if the elements of the first SIMD vector are greater than those of the second.
    pub fn greater_than(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        // No direct comparison intrinsic for uint64x2_t; fallback required
        SIMDFallbackOps::greaterThan(a, b)
    }

    /// Checks if the elements of the first SIMD vector are greater than or equal to those of the second.
    pub fn greater_than_or_equal(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        // No direct comparison intrinsic for uint64x2_t; fallback required
        SIMDFallbackOps::greaterThanOrEqual(a, b)
    }

    /// Checks if all elements in two SIMD vectors are equal.
    pub fn all_equal(a: uint64x2_t, b: uint64x2_t) -> bool {
        let neq = Self::not_equal(a, b);
        let sum: u64 = Self::get(neq, 0).unwrap() + Self::get(neq, 1).unwrap();
        sum == 0
    }

    /// Performs fused multiply-add, i.e., computes (a * b) + c.
    pub fn multiply_add(a: uint64x2_t, b: uint64x2_t, c: uint64x2_t) -> uint64x2_t {
        // No direct fused multiply-add intrinsic for uint64x2_t; fallback required
        SIMDFallbackOps::multiplyAdd(a, b, c)
    }

    /// Computes the sum of all elements in a SIMD vector.
    pub fn sum(a: uint64x2_t) -> u64 {
        let mut arr = [0u64; 2];
        Self::store(a, &mut arr);
        arr.iter().sum()
    }

    /// Truncates the SIMD vector (no-op for integer vectors).
    pub fn truncate(a: uint64x2_t) -> uint64x2_t {
        a
    }
}
