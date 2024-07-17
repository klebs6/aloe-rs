crate::ix!();

/// Unsigned 32-bit integer NEON intrinsics
pub struct SimdNeonU32;

impl SimdNeonU32 {

    /// All bits set constant
    const ALL_BITS_SET: uint32x4_t = uint32x4_t::new(u32::MAX, u32::MAX, u32::MAX, u32::MAX);

    /// Duplicate scalar to all vector lanes
    pub fn expand(s: u32) -> uint32x4_t {
        unsafe { vdupq_n_u32(s) }
    }

    /// Load from memory to vector
    pub fn load(a: *const u32) -> uint32x4_t {
        unsafe { vld1q_u32(a) }
    }

    /// Store vector to memory
    pub fn store(value: uint32x4_t, a: *mut u32) {
        unsafe { vst1q_u32(a, value) }
    }

    /// Get scalar from vector at index
    pub fn get(v: uint32x4_t, i: usize) -> u32 {
        unsafe { mem::transmute::<uint32x4_t, [u32; 4]>(v)[i] }
    }

    /// Set vector lane value at index
    pub fn set(mut v: uint32x4_t, i: usize, s: u32) -> uint32x4_t {
        let arr = unsafe { mem::transmute::<uint32x4_t, [u32; 4]>(v) };
        arr[i] = s;
        v = unsafe { mem::transmute::<[u32; 4], uint32x4_t>(arr) };
        v
    }

    /// Add vectors
    pub fn add(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
        unsafe { vaddq_u32(a, b) }
    }

    /// Subtract vectors
    pub fn sub(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
        unsafe { vsubq_u32(a, b) }
    }

    /// Multiply vectors
    pub fn mul(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
        unsafe { vmulq_u32(a, b) }
    }

    /// Bitwise AND
    pub fn bit_and(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
        unsafe { vandq_u32(a, b) }
    }

    /// Bitwise OR
    pub fn bit_or(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
        unsafe { vorrq_u32(a, b) }
    }

    /// Bitwise XOR
    pub fn bit_xor(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
        unsafe { veorq_u32(a, b) }
    }

    /// Bitwise NOT AND
    pub fn bit_notand(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
        unsafe { vbicq_u32(b, a) }
    }

    /// Bitwise NOT
    pub fn bit_not(a: uint32x4_t) -> uint32x4_t {
        Self::bit_notand(a, Self::ALL_BITS_SET)
    }

    /// Minimum
    pub fn min(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
        unsafe { vminq_u32(a, b) }
    }

    /// Maximum
    pub fn max(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
        unsafe { vmaxq_u32(a, b) }
    }

    /// Equal
    pub fn equal(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
        unsafe { vceqq_u32(a, b) }
    }

    /// Not equal
    pub fn not_equal(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
        Self::bit_not(Self::equal(a, b))
    }

    /// All lanes equal
    pub fn all_equal(a: uint32x4_t, b: uint32x4_t) -> bool {
        Self::sum(Self::not_equal(a, b)) == 0
    }

    /// Greater than
    pub fn greater_than(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
        unsafe { vcgtq_u32(a, b) }
    }

    /// Greater than or equal
    pub fn greater_than_or_equal(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
        unsafe { vcgeq_u32(a, b) }
    }

    /// Multiply and add
    pub fn multiply_add(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
        unsafe { vmlaq_u32(a, b, c) }
    }

    /// Truncate
    pub fn truncate(a: uint32x4_t) -> uint32x4_t {
        a
    }

    /// Sum of all vector lanes
    pub fn sum(a: uint32x4_t) -> u32 {
        let sum_lanes = unsafe { vadd_u32(vget_high_u32(a), vget_low_u32(a)) };
        unsafe { vget_lane_u32(vpadd_u32(sum_lanes, sum_lanes), 0) }
    }
}
