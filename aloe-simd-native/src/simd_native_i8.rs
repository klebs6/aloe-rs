/**
  | Signed 8-bit integer SSE intrinsics.
  | 
  | @tags{DSP}
  |
  */

crate::ix!();


pub struct SimdNativeI8;

impl SimdNativeI8 {

    fn all_bits_set() -> __m128i { 
        unsafe { _mm_set1_epi8(-1) } 
    }

    /// Load vector from memory
    #[inline(always)]
    pub fn load(a: *const i8) -> __m128i {
        unsafe { _mm_load_si128(a as *const __m128i) }
    }

    /// Store vector to memory
    #[inline(always)]
    pub fn store(v: __m128i, p: *mut i8) {
        unsafe { _mm_store_si128(p as *mut __m128i, v) }
    }

    /// Create vector with all elements set to `s`
    #[inline(always)]
    pub fn expand(s: i8) -> __m128i {
        unsafe { _mm_set1_epi8(s) }
    }

    /// Add two vectors
    #[inline(always)]
    pub fn add(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_add_epi8(a, b) }
    }

    /// Subtract two vectors
    #[inline(always)]
    pub fn sub(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_sub_epi8(a, b) }
    }

    /// Bitwise AND of two vectors
    #[inline(always)]
    pub fn bit_and(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_and_si128(a, b) }
    }

    /// Bitwise OR of two vectors
    #[inline(always)]
    pub fn bit_or(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_or_si128(a, b) }
    }

    /// Bitwise XOR of two vectors
    #[inline(always)]
    pub fn bit_xor(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_xor_si128(a, b) }
    }

    /// Bitwise AND NOT of two vectors
    #[inline(always)]
    pub fn bit_andnot(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_andnot_si128(a, b) }
    }

    /// Bitwise NOT of a vector
    #[inline(always)]
    pub fn bit_not(a: __m128i) -> __m128i {
        unsafe { _mm_andnot_si128(a, Self::all_bits_set()) }
    }

    /// Compare for equality
    #[inline(always)]
    pub fn equal(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_cmpeq_epi8(a, b) }
    }

    /// Compare for greater than
    #[inline(always)]
    pub fn greater_than(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_cmpgt_epi8(a, b) }
    }

    /// Multiply and add
    #[inline(always)]
    pub fn multiply_add(a: __m128i, b: __m128i, c: __m128i) -> __m128i {
        Self::add(a, Self::mul(b, c))
    }

    /// Compare for not equal
    #[inline(always)]
    pub fn not_equal(a: __m128i, b: __m128i) -> __m128i {
        Self::bit_not(Self::equal(a, b))
    }

    /// Multiply two vectors
    #[inline(always)]
    pub fn mul(a: __m128i, b: __m128i) -> __m128i {
        unsafe {
            let even = _mm_mullo_epi16(a, b);
            let odd = _mm_mullo_epi16(_mm_srli_epi16(a, 8), _mm_srli_epi16(b, 8));
            _mm_or_si128(_mm_slli_epi16(odd, 8), _mm_srli_epi16(_mm_slli_epi16(even, 8), 8))
        }
    }
}
