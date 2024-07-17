crate::ix!();

/// Struct encapsulating SIMD operations for i16 vectors using SSE intrinsics.
#[cfg(target_arch = "x86_64")]
pub struct SimdNativeI16;

#[cfg(target_arch = "x86_64")]
impl SimdNativeI16 {

    /// 128-bit vector type for i16
    pub type vSIMDType = __m128i;
    
    /// A constant representing all bits set.
    fn all_bits_set() -> i16 { -1 }

    /// Load a SIMD vector from a pointer.
    pub unsafe fn load(a: *const i16) -> Self::vSIMDType {
        _mm_load_si128(a as *const Self::vSIMDType)
    }

    /// Store a SIMD vector to a pointer.
    pub unsafe fn store(a: Self::vSIMDType, p: *mut i16) {
        _mm_store_si128(p as *mut Self::vSIMDType, a)
    }

    /// Create a SIMD vector with identical elements.
    pub unsafe fn expand(s: i16) -> Self::vSIMDType {
        _mm_set1_epi16(s)
    }

    /// Add two SIMD vectors.
    pub unsafe fn add(a: Self::vSIMDType, b: Self::vSIMDType) -> Self::vSIMDType {
        _mm_add_epi16(a, b)
    }

        /// Subtract two SIMD vectors.
    pub unsafe fn sub(a: Self::vSIMDType, b: Self::vSIMDType) -> Self::vSIMDType {
        _mm_sub_epi16(a, b)
    }

    /// Multiply two SIMD vectors.
    pub unsafe fn mul(a: Self::vSIMDType, b: Self::vSIMDType) -> Self::vSIMDType {
        _mm_mullo_epi16(a, b)
    }

    /// Bitwise AND of two SIMD vectors.
    pub unsafe fn bit_and(a: Self::vSIMDType, b: Self::vSIMDType) -> Self::vSIMDType {
        _mm_and_si128(a, b)
    }

    /// Bitwise OR of two SIMD vectors.
    pub unsafe fn bit_or(a: Self::vSIMDType, b: Self::vSIMDType) -> Self::vSIMDType {
        _mm_or_si128(a, b)
    }

    /// Bitwise XOR of two SIMD vectors.
    pub unsafe fn bit_xor(a: Self::vSIMDType, b: Self::vSIMDType) -> Self::vSIMDType {
        _mm_xor_si128(a, b)
    }

    /// Bitwise AND-NOT of two SIMD vectors.
    pub unsafe fn bit_andnot(a: Self::vSIMDType, b: Self::vSIMDType) -> Self::vSIMDType {
        _mm_andnot_si128(a, b)
    }

    /// Bitwise NOT of a SIMD vector.
    pub unsafe fn bit_not(a: Self::vSIMDType) -> Self::vSIMDType {
        _mm_andnot_si128(a, _mm_set1_epi16(Self::all_bits_set()))
    }

    /// Element-wise minimum of two SIMD vectors.
    pub unsafe fn min(a: Self::vSIMDType, b: Self::vSIMDType) -> Self::vSIMDType {
        _mm_min_epi16(a, b)
    }

    /// Element-wise maximum of two SIMD vectors.
    pub unsafe fn max(a: Self::vSIMDType, b: Self::vSIMDType) -> Self::vSIMDType {
        _mm_max_epi16(a, b)
    }

    /// Element-wise equality check of two SIMD vectors.
    pub unsafe fn equal(a: Self::vSIMDType, b: Self::vSIMDType) -> Self::vSIMDType {
        _mm_cmpeq_epi16(a, b)
    }

    /// Element-wise greater-than check of two SIMD vectors.
    pub unsafe fn greater_than(a: Self::vSIMDType, b: Self::vSIMDType) -> Self::vSIMDType {
        _mm_cmpgt_epi16(a, b)
    }

    /// Element-wise greater-than-or-equal-to check of two SIMD vectors.
    pub unsafe fn greater_than_or_equal(a: Self::vSIMDType, b: Self::vSIMDType) -> Self::vSIMDType {
        _mm_or_si128(_mm_cmpgt_epi16(a, b), _mm_cmpeq_epi16(a, b))
    }

    /// Multiply and add operation on SIMD vectors.
    pub unsafe fn multiply_add(a: Self::vSIMDType, b: Self::vSIMDType, c: Self::vSIMDType) -> Self::vSIMDType {
        _mm_add_epi16(a, _mm_mullo_epi16(b, c))
    }

    /// Element-wise not-equal-to check of two SIMD vectors.
    pub unsafe fn not_equal(a: Self::vSIMDType, b: Self::vSIMDType) -> Self::vSIMDType {
        _mm_andnot_si128(_mm_cmpeq_epi16(a, b), _mm_set1_epi16(Self::all_bits_set()))
    }

    /// Check if all elements are equal between two SIMD vectors.
    pub unsafe fn all_equal(a: Self::vSIMDType, b: Self::vSIMDType) -> bool {
        _mm_movemask_epi8(_mm_cmpeq_epi16(a, b)) == 0xffff
    }

    /// Sum elements in a SIMD vector.
    pub unsafe fn sum(a: Self::vSIMDType) -> i16 {
        let tmp1 = _mm_hadd_epi16(a, a);
        let tmp2 = _mm_hadd_epi16(tmp1, tmp1);
        let tmp3 = _mm_hadd_epi16(tmp2, tmp2);
        (_mm_cvtsi128_si32(tmp3) & 0xffff) as i16
    }

    /// Get a value at index i from a SIMD vector.
    pub unsafe fn get(v: Self::vSIMDType, i: usize) -> i16 {
        let mut arr = [0i16; 8];
        _mm_store_si128(arr.as_mut_ptr() as *mut __m128i, v);
        arr[i]
    }

    /// Set a value at index i in a SIMD vector.
    pub unsafe fn set(v: Self::vSIMDType, i: usize, s: i16) -> Self::vSIMDType {
        let mut arr = [0i16; 8];
        _mm_store_si128(arr.as_mut_ptr() as *mut __m128i, v);
        arr[i] = s;
        _mm_load_si128(arr.as_ptr() as *const __m128i)
    }
}
