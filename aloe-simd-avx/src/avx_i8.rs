crate::ix!();

/// SIMD Native Operations for AVX with 8-bit signed integers.
/// DSP (Digital Signal Processing) related operations
pub struct SimdAvxI8;

impl SimdAvxI8 {

    /// All bits set constant for int8
    const ALL_BITS_SET: i8 = -1;

    /// Expands an 8-bit integer into a 256-bit vector
    #[inline(always)]
    pub fn expand(s: i8) -> __m256i {
        unsafe { _mm256_set1_epi8(s) }
    }

    /// Loads a 256-bit vector from memory
    #[inline(always)]
    pub fn load(p: *const i8) -> __m256i {
        unsafe { _mm256_load_si256(p as *const __m256i) }
    }

    /// Stores a 256-bit vector to memory
    #[inline(always)]
    pub fn store(value: __m256i, dest: *mut i8) {
        unsafe { _mm256_store_si256(dest as *mut __m256i, value) };
    }

    /// Element-wise addition
    #[inline(always)]
    pub fn add(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_add_epi8(a, b) }
    }

    /// Element-wise subtraction
    #[inline(always)]
    pub fn sub(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_sub_epi8(a, b) }
    }

    /// Bitwise AND
    #[inline(always)]
    pub fn bit_and(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_and_si256(a, b) }
    }

    /// Bitwise OR
    #[inline(always)]
    pub fn bit_or(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_or_si256(a, b) }
    }

    /// Bitwise XOR
    #[inline(always)]
    pub fn bit_xor(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_xor_si256(a, b) }
    }

    /// Bitwise AND-NOT
    #[inline(always)]
    pub fn bit_andnot(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_andnot_si256(a, b) }
    }

    /// Bitwise NOT
    #[inline(always)]
    pub fn bit_not(a: __m256i) -> __m256i {
        unsafe { _mm256_andnot_si256(a, Self::expand(Self::ALL_BITS_SET)) }
    }

    /// Element-wise minimum
    #[inline(always)]
    pub fn min(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_min_epi8(a, b) }
    }

    /// Element-wise maximum
    #[inline(always)]
    pub fn max(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_max_epi8(a, b) }
    }

    /// Element-wise equality check
    #[inline(always)]
    pub fn equal(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_cmpeq_epi8(a, b) }
    }

    /// Element-wise greater-than check
    #[inline(always)]
    pub fn greater_than(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_cmpgt_epi8(a, b) }
    }

    /// Element-wise greater-than-or-equal check
    #[inline(always)]
    pub fn greater_than_or_equal(a: __m256i, b: __m256i) -> __m256i {
        unsafe {
            _mm256_or_si256(Self::greater_than(a, b), Self::equal(a, b))
        }
    }

    /// Check if all elements are equal
    #[inline(always)]
    pub fn all_equal(a: __m256i, b: __m256i) -> bool {
        unsafe {
            _mm256_movemask_epi8(Self::equal(a, b)) == -1
        }
    }

    /// Multiply-add operation
    #[inline(always)]
    pub fn multiply_add(a: __m256i, b: __m256i, c: __m256i) -> __m256i {
        Self::add(a, Self::mul(b, c))
    }

    /// Element-wise inequality check
    #[inline(always)]
    pub fn not_equal(a: __m256i, b: __m256i) -> __m256i {
        Self::bit_not(Self::equal(a, b))
    }

    /// Truncation (identity operation for 8-bit integers)
    #[inline(always)]
    pub fn truncate(a: __m256i) -> __m256i {
        a
    }

    /// Multiply operation
    #[inline(always)]
    pub fn mul(a: __m256i, b: __m256i) -> __m256i {
        unsafe {
            let even = _mm256_mullo_epi16(a, b);
            let odd = _mm256_mullo_epi16(_mm256_srli_epi16(a, 8), _mm256_srli_epi16(b, 8));
            _mm256_or_si256(_mm256_slli_epi16(odd, 8), _mm256_srli_epi16(_mm256_slli_epi16(even, 8), 8))
        }
    }

    /// Sum of all elements
    #[inline(always)]
    pub fn sum(a: __m256i) -> i8 {
        unsafe {
            let lo = _mm256_unpacklo_epi8(a, _mm256_setzero_si256());
            let hi = _mm256_unpackhi_epi8(a, _mm256_setzero_si256());
            let mut sum_lo = lo;
            let mut sum_hi = hi;
            
            for _ in 0..3 {
                sum_lo = _mm256_hadd_epi16(sum_lo, sum_lo);
                sum_hi = _mm256_hadd_epi16(sum_hi, sum_hi);
            }

            let mask = 0b00100001;

            ((_mm256_cvtsi256_si32(sum_lo) & 0xff) +
             (_mm256_cvtsi256_si32(sum_hi) & 0xff) +
             (_mm256_cvtsi256_si32(_mm256_permute4x64_epi64(sum_lo, mask)) & 0xff) +
             (_mm256_cvtsi256_si32(_mm256_permute4x64_epi64(sum_hi, mask)) & 0xff)) as i8
        }
    }
}
