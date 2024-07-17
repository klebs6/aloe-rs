crate::ix!();

/// Represents single-precision floating point AVX intrinsics.
pub struct SimdAvxF32;

impl SimdAvxF32 {

    /// All bits set
    const ALL_BITS_SET: [i32; 8] = [-1; 8];

    /// Even high bit
    const EVEN_HIGH_BIT: [i32; 8] = [0x80000000; 8];

    /// One
    const ONE: [f32; 8] = [1.0; 8];

    /// Load float vector
    #[inline(always)]
    pub unsafe fn vconst(a: *const f32) -> __m256 {
        _mm256_loadu_ps(a)
    }

    /// Load int32 vector and cast to float
    #[inline(always)]
    pub unsafe fn vconst_i32(a: *const i32) -> __m256 {
        _mm256_castsi256_ps(_mm256_loadu_si256(a as *const __m256i))
    }

    /// Expand single float to all elements of __m256
    #[inline(always)]
    pub unsafe fn expand(s: f32) -> __m256 {
        _mm256_broadcast_ss(&s)
    }

    /// Load float vector from pointer
    #[inline(always)]
    pub unsafe fn load(a: *const f32) -> __m256 {
        _mm256_loadu_ps(a)
    }

    /// Store float vector to pointer
    #[inline(always)]
    pub unsafe fn store(value: __m256, dest: *mut f32) {
        _mm256_storeu_ps(dest, value);
    }

    /// Add two __m256 vectors
    #[inline(always)]
    pub unsafe fn add(a: __m256, b: __m256) -> __m256 {
        _mm256_add_ps(a, b)
    }

    /// Subtract two __m256 vectors
    #[inline(always)]
    pub unsafe fn sub(a: __m256, b: __m256) -> __m256 {
        _mm256_sub_ps(a, b)
    }

    /// Multiply two __m256 vectors
    #[inline(always)]
    pub unsafe fn mul(a: __m256, b: __m256) -> __m256 {
        _mm256_mul_ps(a, b)
    }

    /// Bitwise AND of two __m256 vectors
    #[inline(always)]
    pub unsafe fn bit_and(a: __m256, b: __m256) -> __m256 {
        _mm256_and_ps(a, b)
    }

    /// Bitwise OR of two __m256 vectors
    #[inline(always)]
    pub unsafe fn bit_or(a: __m256, b: __m256) -> __m256 {
        _mm256_or_ps(a, b)
    }

    /// Bitwise XOR of two __m256 vectors
    #[inline(always)]
    pub unsafe fn bit_xor(a: __m256, b: __m256) -> __m256 {
        _mm256_xor_ps(a, b)
    }

    /// Bitwise NOT AND of two __m256 vectors
    #[inline(always)]
    pub unsafe fn bit_notand(a: __m256, b: __m256) -> __m256 {
        _mm256_andnot_ps(a, b)
    }

    /// Bitwise NOT of __m256 vector
    #[inline(always)]
    pub unsafe fn bit_not(a: __m256) -> __m256 {
        _mm256_andnot_ps(a, _mm256_loadu_ps(&Self::ALL_BITS_SET as *const i32 as *const f32))
    }

    /// Minimum of two __m256 vectors
    #[inline(always)]
    pub unsafe fn min(a: __m256, b: __m256) -> __m256 {
        _mm256_min_ps(a, b)
    }

    /// Maximum of two __m256 vectors
    #[inline(always)]
    pub unsafe fn max(a: __m256, b: __m256) -> __m256 {
        _mm256_max_ps(a, b)
    }

    /// Compare __m256 vectors for equality
    #[inline(always)]
    pub unsafe fn equal(a: __m256, b: __m256) -> __m256 {
        _mm256_cmp_ps(a, b, _CMP_EQ_OQ)
    }

    /// Compare __m256 vectors for inequality
    #[inline(always)]
    pub unsafe fn not_equal(a: __m256, b: __m256) -> __m256 {
        _mm256_cmp_ps(a, b, _CMP_NEQ_OQ)
    }

    /// Compare __m256 vectors for greater than
    #[inline(always)]
    pub unsafe fn greater_than(a: __m256, b: __m256) -> __m256 {
        _mm256_cmp_ps(a, b, _CMP_GT_OQ)
    }

    /// Compare __m256 vectors for greater than or equal
    #[inline(always)]
    pub unsafe fn greater_than_or_equal(a: __m256, b: __m256) -> __m256 {
        _mm256_cmp_ps(a, b, _CMP_GE_OQ)
    }

    /// Check if all elements are equal in __m256 vectors
    #[inline(always)]
    pub unsafe fn all_equal(a: __m256, b: __m256) -> bool {
        _mm256_movemask_ps(Self::equal(a, b)) == 0xff
    }

    /// Multiply and add __m256 vectors
    #[inline(always)]
    pub unsafe fn multiply_add(a: __m256, b: __m256, c: __m256) -> __m256 {
        #[cfg(target_feature = "fma")]
        {
            _mm256_fmadd_ps(b, c, a)
        }
        #[cfg(not(target_feature = "fma"))]
        {
            Self::add(a, Self::mul(b, c))
        }
    }

    /// Complex multiplication of __m256 vectors
    #[inline(always)]
    pub unsafe fn cmplxmul(a: __m256, b: __m256) -> __m256 {
        let rr_ir = Self::mul(a, Self::dupeven(b));
        let ii_ri = Self::mul(Self::swapevenodd(a), Self::dupodd(b));
        Self::add(rr_ir, Self::bit_xor(ii_ri, _mm256_loadu_ps(&Self::EVEN_HIGH_BIT as *const i32 as *const f32)))
    }

    /// Sum of elements in __m256 vector
    #[inline(always)]
    pub unsafe fn sum(a: __m256) -> f32 {
        let one = _mm256_loadu_ps(&Self::ONE as *const f32);
        let retval = _mm256_dp_ps(a, one, 0xff);
        let tmp = _mm256_permute2f128_ps(retval, retval, 1);
        let sum_vec = _mm256_add_ps(retval, tmp);
        _mm_cvtss_f32(_mm256_castps256_ps128(sum_vec))
    }
}
