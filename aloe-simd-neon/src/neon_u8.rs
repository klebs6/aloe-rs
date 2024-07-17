crate::ix!();

/// Unsigned 8-bit integer NEON intrinsics.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct SimdNeonU8(pub uint8x16_t);

impl SimdNeonU8 {
    // All bits set constant
    const ALL_BITS_SET: u8 = 0xFF;

    /// Duplicate scalar across all lanes
    #[inline(always)]
    pub fn expand(s: u8) -> Self {
        Self(vdupq_n_u8(s))
    }

    /// Load from a pointer
    #[inline(always)]
    pub fn load(ptr: *const u8) -> Self {
        Self(unsafe { vld1q_u8(ptr) })
    }

    /// Store to a pointer
    #[inline(always)]
    pub fn store(self, ptr: *mut u8) {
        unsafe { vst1q_u8(ptr, self.0) };
    }

    /// Get element at index `i`
    #[inline(always)]
    pub fn get(self, i: usize) -> u8 {
        let array: [u8; 16] = unsafe { mem::transmute(self.0) };
        array[i]
    }

    /// Set element at index `i`
    #[inline(always)]
    pub fn set(self, i: usize, s: u8) -> Self {
        let mut array: [u8; 16] = unsafe { mem::transmute(self.0) };
        array[i] = s;
        Self(unsafe { mem::transmute(array) })
    }

    /// Add two vectors
    #[inline(always)]
    pub fn add(self, other: Self) -> Self {
        Self(unsafe { vaddq_u8(self.0, other.0) })
    }

    /// Subtract two vectors
    #[inline(always)]
    pub fn sub(self, other: Self) -> Self {
        Self(unsafe { vsubq_u8(self.0, other.0) })
    }

    /// Multiply two vectors
    #[inline(always)]
    pub fn mul(self, other: Self) -> Self {
        Self(unsafe { vmulq_u8(self.0, other.0) })
    }

    /// Bitwise AND
    #[inline(always)]
    pub fn bit_and(self, other: Self) -> Self {
        Self(unsafe { vandq_u8(self.0, other.0) })
    }

    /// Bitwise OR
    #[inline(always)]
    pub fn bit_or(self, other: Self) -> Self {
        Self(unsafe { vorrq_u8(self.0, other.0) })
    }

    /// Bitwise XOR
    #[inline(always)]
    pub fn bit_xor(self, other: Self) -> Self {
        Self(unsafe { veorq_u8(self.0, other.0) })
    }

    /// Bitwise NOT AND
    #[inline(always)]
    pub fn bit_notand(self, other: Self) -> Self {
        Self(unsafe { vbicq_u8(other.0, self.0) })
    }

    /// Bitwise NOT
    #[inline(always)]
    pub fn bit_not(self) -> Self {
        Self(unsafe { vbicq_u8(self.0, vdupq_n_u8(Self::ALL_BITS_SET)) })
    }

    /// Minimum
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        Self(unsafe { vminq_u8(self.0, other.0) })
    }

    /// Maximum
    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        Self(unsafe { vmaxq_u8(self.0, other.0) })
    }

    /// Equality
    #[inline(always)]
    pub fn equal(self, other: Self) -> Self {
        Self(unsafe { vceqq_u8(self.0, other.0) })
    }

    /// Not equal
    #[inline(always)]
    pub fn not_equal(self, other: Self) -> Self {
        Self(self.equal(other).bit_not().0)
    }

    /// Greater than
    #[inline(always)]
    pub fn greater_than(self, other: Self) -> Self {
        Self(unsafe { vcgtq_u8(self.0, other.0) })
    }

    /// Greater than or equal
    #[inline(always)]
    pub fn greater_than_or_equal(self, other: Self) -> Self {
        Self(unsafe { vcgeq_u8(self.0, other.0) })
    }

    /// Multiply and add
    #[inline(always)]
    pub fn multiply_add(self, mul: Self, add: Self) -> Self {
        Self(unsafe { vmlaq_u8(add.0, self.0, mul.0) })
    }

    /// Sum of all lanes
    #[inline(always)]
    pub fn sum(self) -> u8 {
        let mut sum = 0u8;
        let array: [u8; 16] = unsafe { mem::transmute(self.0) };
        for &elem in array.iter() {
            sum = sum.wrapping_add(elem);
        }
        sum
    }
}
