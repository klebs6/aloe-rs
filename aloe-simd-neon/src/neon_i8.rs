crate::ix!();

/// Digital Signal Processing (DSP) operations with NEON
/// intrinsics for 8-bit signed integers.
///
pub struct SimdNeonI8;

/// All-bits-set constant for bitwise operations.
const ALL_BITS_SET: int8x16_t = int8x16_t::new(
    -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1,
);

impl SimdNeonI8 {
    /// Duplicates an 8-bit integer into a 128-bit vector.
    pub fn expand(s: i8) -> int8x16_t {
        vdupq_n_s8(s)
    }

    /// Loads data from a pointer into a 128-bit vector.
    pub fn load(a: *const i8) -> int8x16_t {
        unsafe { vld1q_s8(a) }
    }

    /// Stores a 128-bit vector into a pointer.
    pub fn store(value: int8x16_t, a: *mut i8) {
        unsafe { vst1q_s8(a, value) }
    }

    /// Retrieves an element from a 128-bit vector.
    pub fn get(v: int8x16_t, i: usize) -> i8 {
        v.get_lane(i as _).expect("Index out of bounds")
    }

    /// Sets an element in a 128-bit vector.
    pub fn set(v: int8x16_t, i: usize, s: i8) -> int8x16_t {
        v.replace_lane(i as _, s).expect("Index out of bounds")
    }

    /// Element-wise addition of two 128-bit vectors.
    pub fn add(a: int8x16_t, b: int8x16_t) -> int8x16_t {
        a + b
    }

    /// Element-wise subtraction of two 128-bit vectors.
    pub fn sub(a: int8x16_t, b: int8x16_t) -> int8x16_t {
        a - b
    }

    /// Element-wise multiplication of two 128-bit vectors.
    pub fn mul(a: int8x16_t, b: int8x16_t) -> int8x16_t {
        a * b
    }

    /// Element-wise bitwise AND of two 128-bit vectors.
    pub fn bit_and(a: int8x16_t, b: int8x16_t) -> int8x16_t {
        a & b
    }

    /// Element-wise bitwise OR of two 128-bit vectors.
    pub fn bit_or(a: int8x16_t, b: int8x16_t) -> int8x16_t {
        a | b
    }

    /// Element-wise bitwise XOR of two 128-bit vectors.
    pub fn bit_xor(a: int8x16_t, b: int8x16_t) -> int8x16_t {
        a ^ b
    }

    /// Bitwise NOT AND operation between two 128-bit vectors.
    pub fn bit_notand(a: int8x16_t, b: int8x16_t) -> int8x16_t {
        !(a & b)
    }

    /// Bitwise NOT operation for a 128-bit vector.
    pub fn bit_not(a: int8x16_t) -> int8x16_t {
        !a
    }

    /// Element-wise minimum between two 128-bit vectors.
    pub fn min(a: int8x16_t, b: int8x16_t) -> int8x16_t {
        vminq_s8(a, b)
    }

    /// Element-wise maximum between two 128-bit vectors.
    pub fn max(a: int8x16_t, b: int8x16_t) -> int8x16_t {
        vmaxq_s8(a, b)
    }

    /// Element-wise equality between two 128-bit vectors.
    pub fn equal(a: int8x16_t, b: int8x16_t) -> int8x16_t {
        unsafe { vceqq_s8(a, b) }
    }

    /// Element-wise inequality between two 128-bit vectors.
    pub fn not_equal(a: int8x16_t, b: int8x16_t) -> int8x16_t {
        unsafe { vmvnq_s8(vceqq_s8(a, b)) }
    }

    /// Checks if elements of `a` are greater than those of `b`.
    pub fn greater_than(a: int8x16_t, b: int8x16_t) -> int8x16_t {
        unsafe { vcgtq_s8(a, b) }
    }

    /// Checks if elements of `a` are greater than or equal to those of `b`.
    pub fn greater_than_or_equal(a: int8x16_t, b: int8x16_t) -> int8x16_t {
        unsafe { vcgeq_s8(a, b) }
    }

    /// Fused multiply-add operation for three 128-bit vectors.
    pub fn multiply_add(a: int8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
        unsafe { vmlaq_s8(a, b, c) }
    }

    /// Sum of all elements in a 128-bit vector.
    pub fn sum(a: int8x16_t) -> i8 {
        let pairwise_add = vpaddq_s8(a, a);
        let quadword_add = vpaddq_s8(pairwise_add, pairwise_add);
        unsafe { vgetq_lane_s8(quadword_add, 0) + vgetq_lane_s8(quadword_add, 8) }
    }

    /// Truncate operation (NOP in this context)
    pub fn truncate(a: int8x16_t) -> int8x16_t {
        a
    }
}
