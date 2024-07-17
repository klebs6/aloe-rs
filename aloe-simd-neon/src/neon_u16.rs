crate::ix!();

/// Digital Signal Processing (DSP) operations with NEON
/// intrinsics for 16-bit unsigned integers.
///
pub struct SimdNeonU16;

/// All-bits-set constant for bitwise operations.
const ALL_BITS_SET: uint16x8_t = uint16x8_t::new(
    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
);

impl SimdNeonU16 {
    /// Duplicates a 16-bit unsigned integer into a 128-bit vector.
    pub fn expand(s: u16) -> uint16x8_t {
        vdupq_n_u16(s)
    }

    /// Loads data from a pointer into a 128-bit vector.
    pub fn load(a: *const u16) -> uint16x8_t {
        unsafe { vld1q_u16(a) }
    }

    /// Stores a 128-bit vector into a pointer.
    pub fn store(value: uint16x8_t, a: *mut u16) {
        unsafe { vst1q_u16(a, value) }
    }

    /// Retrieves an element from a 128-bit vector.
    pub fn get(v: uint16x8_t, i: usize) -> u16 {
        v.get_lane(i as _).expect("Index out of bounds")
    }

    /// Sets an element in a 128-bit vector.
    pub fn set(v: uint16x8_t, i: usize, s: u16) -> uint16x8_t {
        v.replace_lane(i as _, s).expect("Index out of bounds")
    }

    /// Element-wise addition of two 128-bit vectors.
    pub fn add(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
        a + b
    }

    /// Element-wise subtraction of two 128-bit vectors.
    pub fn sub(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
        a - b
    }

    /// Element-wise multiplication of two 128-bit vectors.
    pub fn mul(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
        a * b
    }

    /// Element-wise bitwise AND of two 128-bit vectors.
    pub fn bit_and(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
        a & b
    }

    /// Element-wise bitwise OR of two 128-bit vectors.
    pub fn bit_or(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
        a | b
    }

    /// Element-wise bitwise XOR of two 128-bit vectors.
    pub fn bit_xor(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
        a ^ b
    }

    /// Bitwise NOT AND operation between two 128-bit vectors.
    pub fn bit_notand(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
        vbicq_u16(b, a)
    }

    /// Bitwise NOT operation for a 128-bit vector.
    pub fn bit_not(a: uint16x8_t) -> uint16x8_t {
        Self::bit_notand(a, ALL_BITS_SET)
    }

    /// Element-wise minimum between two 128-bit vectors.
    pub fn min(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
        vminq_u16(a, b)
    }

    /// Element-wise maximum between two 128-bit vectors.
    pub fn max(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
        vmaxq_u16(a, b)
    }

    /// Element-wise equality between two 128-bit vectors.
    pub fn equal(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
        unsafe { vceqq_u16(a, b) }
    }

    /// Element-wise inequality between two 128-bit vectors.
    pub fn not_equal(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
        Self::bit_not(Self::equal(a, b))
    }

    /// Checks if elements of `a` are greater than those of `b`.
    pub fn greater_than(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
        unsafe { vcgtq_u16(a, b) }
    }

    /// Checks if elements of `a` are greater than or equal to those of `b`.
    pub fn greater_than_or_equal(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
        unsafe { vcgeq_u16(a, b) }
    }

    /// Element-wise fused multiply-add.
    pub fn multiply_add(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
        unsafe { vmlaq_u16(a, b, c) }
    }

    /// Sum of all elements in a 128-bit vector.
    pub fn sum(a: uint16x8_t) -> u16 {
        let pairwise_add = vpaddq_u16(a, a);
        let quadword_add = vpaddq_u16(pairwise_add, pairwise_add);
        unsafe { vgetq_lane_u16(quadword_add, 0) + vgetq_lane_u16(quadword_add, 4) }
    }

    /// Truncate operation (NOP in this context).
    pub fn truncate(a: uint16x8_t) -> uint16x8_t {
        a
    }
}
