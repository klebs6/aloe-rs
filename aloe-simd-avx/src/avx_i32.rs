crate::ix!();

pub struct SimdAvxI32;

impl SimdAvxI32 {

    /// Constant representing all bits set for int32_t
    pub const ALL_BITS_SET: i32 = !0;

    /// Expand scalar to all elements of the vector
    #[inline(always)]
    pub fn expand(s: i32) -> __m256i {
        unsafe { _mm256_set1_epi32(s) }
    }

    /// Load data from memory into a vector
    #[inline(always)]
    pub fn load(p: &[i32; 8]) -> __m256i {
        unsafe { _mm256_load_si256(p.as_ptr() as *const __m256i) }
    }

    /// Store vector data into memory
    #[inline(always)]
    pub fn store(value: __m256i, dest: &mut [i32; 8]) {
        unsafe { _mm256_store_si256(dest.as_mut_ptr() as *mut __m256i, value) }
    }

    /// Summation of elements in the vector
    #[inline(always)]
    pub fn sum(a: __m256i) -> i32 {
        let tmp = unsafe { _mm256_hadd_epi32(a, a) };
        let tmp = unsafe { _mm256_hadd_epi32(tmp, tmp) };
        unsafe { _mm256_cvtsi256_si32(tmp) + _mm256_cvtsi256_si32(_mm256_permute4x64_epi64(tmp, 0b01001110)) }
    }

    #[inline(always)]
    pub fn expand(s: i32) -> __m256i {
        unsafe { _mm256_set1_epi32(s) }
    }

    #[inline(always)]
    pub fn load(p: &[i32; 8]) -> __m256i {
        unsafe { _mm256_load_si256(p.as_ptr() as *const __m256i) }
    }

    #[inline(always)]
    pub fn store(value: __m256i, dest: &mut [i32; 8]) {
        unsafe { _mm256_store_si256(dest.as_mut_ptr() as *mut __m256i, value) }
    }

    #[inline(always)]
    pub fn add(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_add_epi32(a, b) }
    }

    #[inline(always)]
    pub fn sub(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_sub_epi32(a, b) }
    }

    #[inline(always)]
    pub fn mul(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_mullo_epi32(a, b) }
    }

    #[inline(always)]
    pub fn bit_and(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_and_si256(a, b) }
    }

    #[inline(always)]
    pub fn bit_or(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_or_si256(a, b) }
    }

    #[inline(always)]
    pub fn bit_xor(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_xor_si256(a, b) }
    }

    #[inline(always)]
    pub fn bit_andnot(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_andnot_si256(a, b) }
    }

    #[inline(always)]
    pub fn bit_not(a: __m256i) -> __m256i {
        unsafe { _mm256_andnot_si256(a, Self::expand(Self::ALL_BITS_SET)) }
    }

    #[inline(always)]
    pub fn min(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_min_epi32(a, b) }
    }

    #[inline(always)]
    pub fn max(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_max_epi32(a, b) }
    }

    #[inline(always)]
    pub fn equal(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_cmpeq_epi32(a, b) }
    }

    #[inline(always)]
    pub fn greater_than(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_cmpgt_epi32(a, b) }
    }

    #[inline(always)]
    pub fn greater_than_or_equal(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_or_si256(_mm256_cmpgt_epi32(a, b), _mm256_cmpeq_epi32(a, b)) }
    }

    #[inline(always)]
    pub fn multiply_add(a: __m256i, b: __m256i, c: __m256i) -> __m256i {
        unsafe { _mm256_add_epi32(a, _mm256_mullo_epi32(b, c)) }
    }

    #[inline(always)]
    pub fn not_equal(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_andnot_si256(_mm256_cmpeq_epi32(a, b), Self::expand(Self::ALL_BITS_SET)) }
    }

    #[inline(always)]
    pub fn all_equal(a: __m256i, b: __m256i) -> bool {
        unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi32(a, b)) == -1 }
    }

    #[inline(always)]
    pub fn sum(a: __m256i) -> i32 {
        let tmp = unsafe { _mm256_hadd_epi32(a, a) };
        let tmp = unsafe { _mm256_hadd_epi32(tmp, tmp) };
        unsafe { _mm256_cvtsi256_si32(tmp) + _mm256_cvtsi256_si32(_mm256_permute4x64_epi64(tmp, 0b01001110)) }
    }
}
