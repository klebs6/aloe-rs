crate::ix!();

/// All bits set constant for i32
fn set1_epi32_all_bits_set() -> __m128i { unsafe { _mm_set1_epi32(-1) } }

/// 32-bit signed integer SSE intrinsics
/// # Tags
/// DSP (Digital Signal Processing)
pub struct SimdNativeI32;

impl SimdNativeI32 {

    /// Load a SIMD vector from memory.
    pub fn vconst(a: &[i32; 4]) -> __m128i {
        Self::load(a)
    }

    /// Load a SIMD vector from memory.
    pub fn load(a: &[i32; 4]) -> __m128i {
        unsafe { _mm_loadu_si128(a.as_ptr() as *const _) }
    }

    /// Store a SIMD vector to memory.
    pub fn store(v: __m128i, p: &mut [i32; 4]) {
        unsafe { _mm_storeu_si128(p.as_mut_ptr() as *mut _, v) };
    }

    /// Set all elements to a single value `s`.
    pub fn expand(s: i32) -> __m128i {
        unsafe { _mm_set1_epi32(s) }
    }

    /// Add two SIMD vectors.
    pub fn add(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_add_epi32(a, b) }
    }

    /// Subtract two SIMD vectors.
    pub fn sub(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_sub_epi32(a, b) }
    }

    /// Bitwise AND of two SIMD vectors.
    pub fn bit_and(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_and_si128(a, b) }
    }

    /// Bitwise OR of two SIMD vectors.
    pub fn bit_or(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_or_si128(a, b) }
    }

    /// Bitwise XOR of two SIMD vectors.
    pub fn bit_xor(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_xor_si128(a, b) }
    }

    /// Bitwise AND-NOT of two SIMD vectors.
    pub fn bit_andnot(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_andnot_si128(a, b) }
    }

    /// Bitwise NOT of a SIMD vector.
    pub fn bit_not(a: __m128i) -> __m128i {
        unsafe { _mm_andnot_si128(a, set1_epi32_all_bits_set()) }
    }

    /// Element-wise equality comparison of two SIMD vectors.
    pub fn equal(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_cmpeq_epi32(a, b) }
    }

    /// Element-wise greater-than comparison of two SIMD vectors.
    pub fn greater_than(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_cmpgt_epi32(a, b) }
    }

    /// Element-wise greater-than-or-equal comparison of two SIMD vectors.
    pub fn greater_than_or_equal(a: __m128i, b: __m128i) -> __m128i {
        unsafe {
            _mm_or_si128(_mm_cmpgt_epi32(a, b), _mm_cmpeq_epi32(a, b))
        }
    }

    /// Multiply-add operation.
    pub fn multiply_add(a: __m128i, b: __m128i, c: __m128i) -> __m128i {
        Self::add(a, Self::mul(b, c))
    }

    /// Element-wise inequality comparison.
    pub fn not_equal(a: __m128i, b: __m128i) -> __m128i {
        Self::bit_not(Self::equal(a, b))
    }

    /// Checks if all elements are equal.
    pub fn all_equal(a: __m128i, b: __m128i) -> bool {
        unsafe {
            _mm_movemask_epi8(Self::equal(a, b)) == 0xffff
        }
    }

    /// Returns the sum of all elements in SIMD vector.
    pub fn sum(a: __m128i) -> i32 {
        unsafe {
            let tmp = _mm_hadd_epi32(a, a);
            _mm_cvtsi128_si32(_mm_hadd_epi32(tmp, tmp))
        }
    }

    /// Element-wise multiplication of two SIMD vectors.
    pub fn mul(a: __m128i, b: __m128i) -> __m128i {
        #[cfg(target_feature = "sse4.1")]
        {
            unsafe { _mm_mullo_epi32(a, b) }
        }
        #[cfg(not(target_feature = "sse4.1"))]
        {
            unsafe {
                let even = _mm_mul_epu32(a, b);
                let odd = _mm_mul_epu32(_mm_srli_si128(a, 4), _mm_srli_si128(b, 4));
                _mm_unpacklo_epi32(_mm_shuffle_epi32(even, 0x88), _mm_shuffle_epi32(odd, 0x88))
            }
        }
    }

    /// Element-wise minimum of two SIMD vectors.
    pub fn min(a: __m128i, b: __m128i) -> __m128i {
        #[cfg(target_feature = "sse4.1")]
        {
            unsafe { _mm_min_epi32(a, b) }
        }
        #[cfg(not(target_feature = "sse4.1"))]
        {
            let lt = Self::greater_than(b, a);
            Self::bit_or(Self::bit_and(lt, a), Self::bit_andnot(lt, b))
        }
    }

    /// Element-wise maximum of two SIMD vectors.
    pub fn max(a: __m128i, b: __m128i) -> __m128i {
        #[cfg(target_feature = "sse4.1")]
        {
            unsafe { _mm_max_epi32(a, b) }
        }
        #[cfg(not(target_feature = "sse4.1"))]
        {
            let gt = Self::greater_than(a, b);
            Self::bit_or(Self::bit_and(gt, a), Self::bit_andnot(gt, b))
        }
    }
}
