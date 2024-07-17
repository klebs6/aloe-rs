crate::ix!();

use core::arch::x86_64::*;

/// SSE intrinsics for 16-bit unsigned integers.
/// This struct contains methods for SIMD operations.
pub struct SimdNativeU16;

impl SimdNativeU16 {

    fn high_bit() -> __m128i {
        unsafe { _mm_set1_epi16(0x8000u16 as i16) } // Cast the u16 value to i16
    }

    fn all_bits_set() -> __m128i {
        unsafe { _mm_set1_epi16(0xFFFFu16 as i16) } // Cast the u16 value to i16
    }

    /// Load data from a pointer into an SSE register
    #[inline(always)]
    pub fn vconst(a: *const u16) -> __m128i {
        Self::load(a)
    }

    /// Sign flip a vector
    #[inline(always)]
    pub fn ssign(a: __m128i) -> __m128i {
        unsafe { _mm_xor_si128(a, Self::vconst(&0x8000)) }
    }

    /// Load data from a pointer into an SSE register
    #[inline(always)]
    pub fn load(a: *const u16) -> __m128i {
        unsafe { _mm_load_si128(a as *const __m128i) }
    }

    /// Store data from SSE register to a pointer
    #[inline(always)]
    pub fn store(v: __m128i, p: *mut u16) {
        unsafe { _mm_store_si128(p as *mut __m128i, v) }
    }

    /// Expand scalar to all positions in SSE register
    #[inline(always)]
    pub fn expand(s: u16) -> __m128i {
        unsafe { _mm_set1_epi16(s as i16) }
    }

    // Arithmetic operations
    #[inline(always)]
    pub fn add(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_add_epi16(a, b) }
    }

    #[inline(always)]
    pub fn sub(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_sub_epi16(a, b) }
    }

    #[inline(always)]
    pub fn mul(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_mullo_epi16(a, b) }
    }

    // Bitwise operations
    #[inline(always)]
    pub fn bit_and(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_and_si128(a, b) }
    }

    #[inline(always)]
    pub fn bit_or(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_or_si128(a, b) }
    }

    #[inline(always)]
    pub fn bit_xor(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_xor_si128(a, b) }
    }

    #[inline(always)]
    pub fn bit_andnot(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_andnot_si128(a, b) }
    }

    #[inline(always)]
    pub fn bit_not(a: __m128i) -> __m128i {
        unsafe { _mm_andnot_si128(a, Self::all_bits_set()) }
    }

    // Comparison operations
    #[inline(always)]
    pub fn min(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_min_epu16(a, b) }
    }

    #[inline(always)]
    pub fn max(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_max_epu16(a, b) }
    }

    #[inline(always)]
    pub fn equal(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_cmpeq_epi16(a, b) }
    }

    #[inline(always)]
    pub fn greater_than(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_cmpgt_epi16(Self::ssign(a), Self::ssign(b)) }
    }

    #[inline(always)]
    pub fn greater_than_or_equal(a: __m128i, b: __m128i) -> __m128i {
        Self::bit_or(Self::greater_than(a, b), Self::equal(a, b))
    }

    // Other operations
    #[inline(always)]
    pub fn multiply_add(a: __m128i, b: __m128i, c: __m128i) -> __m128i {
        Self::add(a, Self::mul(b, c))
    }

    #[inline(always)]
    pub fn not_equal(a: __m128i, b: __m128i) -> __m128i {
        Self::bit_not(Self::equal(a, b))
    }

    #[inline(always)]
    pub fn sum(a: __m128i) -> u16 {
        unsafe {
            let tmp = _mm_hadd_epi16(a, a);
            let tmp = _mm_hadd_epi16(tmp, tmp);
            let tmp = _mm_hadd_epi16(tmp, tmp);
            _mm_cvtsi128_si32(tmp) as u16
        }
    }
}
