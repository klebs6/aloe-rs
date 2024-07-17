crate::ix!();

/// Double-precision floating point SSE intrinsics for DSP operations.
pub struct SimdNativeF64;

impl SimdNativeF64 {

    /// Load from pointer of type `f64`
    pub fn vconst_f64(a: *const f64) -> Result<__m128d, &'static str> {
        if a.is_null() {
            return Err("Null pointer encountered.");
        }
        unsafe { Ok(_mm_loadu_pd(a)) }
    }

    /// Load from pointer of type `i64`
    pub fn vconst_i64(a: *const i64) -> Result<__m128d, &'static str> {
        if a.is_null() {
            return Err("Null pointer encountered.");
        }
        unsafe { Ok(_mm_castsi128_pd(_mm_loadu_si128(mem::transmute(a)))) }
    }

    /// Expand a scalar to a vector
    pub fn expand(s: f64) -> __m128d {
        unsafe { _mm_set1_pd(s) }
    }

    /// Load data from array
    pub fn load(a: &[f64; 2]) -> __m128d {
        unsafe { _mm_loadu_pd(a.as_ptr()) }
    }

    /// Store data to destination
    pub fn store(value: __m128d, dest: &mut [f64; 2]) {
        unsafe { _mm_storeu_pd(dest.as_mut_ptr(), value) }
    }

    /// Add two vectors
    #[inline(always)] pub fn add(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_add_pd(a, b) } }

    /// Subtract two vectors
    #[inline(always)] pub fn sub(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_sub_pd(a, b) } }

    /// Multiply two vectors
    #[inline(always)] pub fn mul(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_mul_pd(a, b) } }

    /// Bitwise AND two vectors
    #[inline(always)] pub fn bit_and(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_and_pd(a, b) } }

    /// Bitwise OR two vectors
    #[inline(always)] pub fn bit_or(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_or_pd(a, b) } }

    /// Bitwise XOR two vectors
    #[inline(always)] pub fn bit_xor(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_xor_pd(a, b) } }

    /// Bitwise NOT AND two vectors
    #[inline(always)] pub fn bit_notand(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_andnot_pd(a, b) } }

    /// Bitwise NOT a vector
    #[inline(always)] pub fn bit_not(a: __m128d) -> __m128d { unsafe { _mm_andnot_pd(a, _mm_set1_pd(-1.0)) } }

    /// Min of two vectors
    #[inline(always)] pub fn min(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_min_pd(a, b) } }

    /// Max of two vectors
    #[inline(always)] pub fn max(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_max_pd(a, b) } }

    /// Compare Equal
    #[inline(always)] pub fn equal(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_cmpeq_pd(a, b) } }

    /// Compare Not Equal
    #[inline(always)] pub fn not_equal(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_cmpneq_pd(a, b) } }

    /// Greater Than
    #[inline(always)] pub fn greater_than(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_cmpgt_pd(a, b) } }

    /// Greater Than Or Equal
    #[inline(always)] pub fn greater_than_or_equal(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_cmpge_pd(a, b) } }
    
    /// Divide two vectors
    #[inline(always)] pub fn div(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_div_pd(a, b) } }

    /// Square root of a vector
    #[inline(always)] pub fn sqrt(a: __m128d) -> __m128d { unsafe { _mm_sqrt_pd(a) } }

    /// Compute absolute value of each element
    #[inline(always)] pub fn abs(a: __m128d) -> __m128d { 
        unsafe { _mm_andnot_pd(_mm_set1_pd(-0.0), a) }
    }

    /// Compare Less Than
    #[inline(always)] pub fn less_than(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_cmplt_pd(a, b) } }

    /// Compare Less Than Or Equal
    #[inline(always)] pub fn less_than_or_equal(a: __m128d, b: __m128d) -> __m128d { unsafe { _mm_cmple_pd(a, b) } }

    /// Compute the floor of each element
    #[inline(always)] pub fn floor(a: __m128d) -> __m128d { unsafe { _mm_floor_pd(a) } }

    /// Compute the ceiling of each element
    #[inline(always)] pub fn ceil(a: __m128d) -> __m128d { unsafe { _mm_ceil_pd(a) } }

    /// Round to nearest integer, away from zero
    #[inline(always)] pub fn round(a: __m128d) -> __m128d { unsafe { _mm_round_pd(a, _MM_FROUND_TO_NEAREST_INT) } }

    /// Round to integer toward zero (truncate)
    #[inline(always)] pub fn trunc(a: __m128d) -> __m128d { unsafe { _mm_round_pd(a, _MM_FROUND_TO_ZERO) } }

    /// Negate each element
    #[inline(always)] pub fn negate(a: __m128d) -> __m128d { unsafe { _mm_xor_pd(a, _mm_set1_pd(-0.0)) } }

    /// Compute the reciprocal of each element
    #[inline(always)] pub fn reciprocal(a: __m128d) -> __m128d { unsafe { _mm_div_pd(_mm_set1_pd(1.0), a) } }

    /// Compute the reciprocal square root of each element
    #[inline(always)] pub fn rsqrt(a: __m128d) -> __m128d { unsafe { _mm_div_pd(_mm_set1_pd(1.0), _mm_sqrt_pd(a)) } }
}
