crate::ix!();

pub struct SimdAvxF64;

impl SimdAvxF64 {

    const ALL_BITS_SET: i64 = -1;
    const EVEN_HIGH_BIT: i64 = 0x8000000000000000;
    const ONE: f64 = 1.0;

    /// Load vector from `double` array
    #[inline(always)]
    pub unsafe fn vconst(a: *const f64) -> __m256d {
        Self::load(a)
    }

    /// Load vector from `int64_t` array
    #[inline(always)]
    pub unsafe fn vconst_i64(a: *const i64) -> __m256d {
        _mm256_castsi256_pd(_mm256_load_si256(a as *const __m256i))
    }

    /// Expand a single double to a vector
    #[inline(always)]
    pub unsafe fn expand(s: f64) -> __m256d {
        _mm256_broadcast_sd(&s)
    }

    /// Load vector from memory
    #[inline(always)]
    pub unsafe fn load(a: *const f64) -> __m256d {
        _mm256_load_pd(a)
    }

    /// Store vector to memory
    #[inline(always)]
    pub unsafe fn store(value: __m256d, dest: *mut f64) {
        _mm256_store_pd(dest, value);
    }

    /// Add vectors
    #[inline(always)]
    pub unsafe fn add(a: __m256d, b: __m256d) -> __m256d {
        _mm256_add_pd(a, b)
    }

    /// Subtract vectors
    #[inline(always)]
    pub unsafe fn sub(a: __m256d, b: __m256d) -> __m256d {
        _mm256_sub_pd(a, b)
    }

    /// Multiply vectors
    #[inline(always)]
    pub unsafe fn mul(a: __m256d, b: __m256d) -> __m256d {
        _mm256_mul_pd(a, b)
    }

    /// Bitwise AND
    #[inline(always)]
    pub unsafe fn bit_and(a: __m256d, b: __m256d) -> __m256d {
        _mm256_and_pd(a, b)
    }

    /// Bitwise OR
    #[inline(always)]
    pub unsafe fn bit_or(a: __m256d, b: __m256d) -> __m256d {
        _mm256_or_pd(a, b)
    }

    /// Bitwise XOR
    #[inline(always)]
    pub unsafe fn bit_xor(a: __m256d, b: __m256d) -> __m256d {
        _mm256_xor_pd(a, b)
    }

    /// Bitwise NOT AND
    #[inline(always)]
    pub unsafe fn bit_notand(a: __m256d, b: __m256d) -> __m256d {
        _mm256_andnot_pd(a, b)
    }

    /// Bitwise NOT
    #[inline(always)]
    pub unsafe fn bit_not(a: __m256d) -> __m256d {
        let all_bits_set = _mm256_broadcast_sd(&Self::ALL_BITS_SET as *const i64 as *const f64);
        _mm256_andnot_pd(a, all_bits_set)
    }

    /// Minimum of vectors
    #[inline(always)]
    pub unsafe fn min(a: __m256d, b: __m256d) -> __m256d {
        _mm256_min_pd(a, b)
    }

    /// Maximum of vectors
    #[inline(always)]
    pub unsafe fn max(a: __m256d, b: __m256d) -> __m256d {
        _mm256_max_pd(a, b)
    }

    /// Equality comparison
    #[inline(always)]
    pub unsafe fn equal(a: __m256d, b: __m256d) -> __m256d {
        _mm256_cmp_pd(a, b, _CMP_EQ_OQ)
    }

    /// Inequality comparison
    #[inline(always)]
    pub unsafe fn not_equal(a: __m256d, b: __m256d) -> __m256d {
        _mm256_cmp_pd(a, b, _CMP_NEQ_OQ)
    }

    /// Greater than comparison
    #[inline(always)]
    pub unsafe fn greater_than(a: __m256d, b: __m256d) -> __m256d {
        _mm256_cmp_pd(a, b, _CMP_GT_OQ)
    }

    /// Greater than or equal comparison
    #[inline(always)]
    pub unsafe fn greater_than_or_equal(a: __m256d, b: __m256d) -> __m256d {
        _mm256_cmp_pd(a, b, _CMP_GE_OQ)
    }

    /// Check if all elements are equal
    #[inline(always)]
    pub unsafe fn all_equal(a: __m256d, b: __m256d) -> bool {
        _mm256_movemask_pd(Self::equal(a, b)) == 0xf
    }

    /// Multiply-add vectors
    #[inline(always)]
    pub unsafe fn multiply_add(a: __m256d, b: __m256d, c: __m256d) -> __m256d {
        _mm256_add_pd(a, _mm256_mul_pd(b, c))
    }

    /// Dupeven elements in a vector
    #[inline(always)]
    pub unsafe fn dupeven(a: __m256d) -> __m256d {
        _mm256_shuffle_pd(a, a, 0)
    }

    /// Dupodd elements in a vector
    #[inline(always)]
    pub unsafe fn dupodd(a: __m256d) -> __m256d {
        _mm256_shuffle_pd(a, a, 0b1111)
    }

    /// Swap even and odd elements
    #[inline(always)]
    pub unsafe fn swapevenodd(a: __m256d) -> __m256d {
        _mm256_shuffle_pd(a, a, 0b0101)
    }

    /// Sum of odd and even elements
    #[inline(always)]
    pub unsafe fn oddevensum(a: __m256d) -> __m256d {
        _mm256_add_pd(_mm256_permute2f128_pd(a, a, 1), a)
    }

    /// Complex multiplication of __m256d vectors
    #[inline(always)]
    pub unsafe fn cmplxmul(a: __m256d, b: __m256d) -> __m256d {
        let rr_ir = _mm256_mul_pd(a, Self::dupeven(b));
        let ii_ri = _mm256_mul_pd(Self::swapevenodd(a), Self::dupodd(b));
        _mm256_add_pd(rr_ir, _mm256_xor_pd(ii_ri, _mm256_broadcast_sd(&Self::EVEN_HIGH_BIT as *const i64 as *const f64)))
    }

    /// Sum of elements in __m256d vector
    #[inline(always)]
    pub unsafe fn sum(a: __m256d) -> f64 {
        let mut retval = _mm256_hadd_pd(a, a);
        let tmp = _mm256_permute2f128_pd(retval, retval, 1);
        retval = _mm256_add_pd(retval, tmp);
        
        let mut dest = [0.0; 4];
        _mm256_storeu_pd(dest.as_mut_ptr(), retval);
        dest[0]
    }
}
