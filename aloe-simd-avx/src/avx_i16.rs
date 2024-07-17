crate::ix!();

/// Signed 16-bit integer AVX intrinsics.
pub struct SimdAvxI16;

impl SimdAvxI16 {

    /// All bits set constant.
    const ALL_BITS_SET: __m256i = unsafe { _mm256_set1_epi16(-1) };

    /// Expands a 16-bit integer to a 256-bit vector.
    pub fn expand(s: i16) -> __m256i {
        unsafe { _mm256_set1_epi16(s) }
    }

    /// Loads from a pointer to a 256-bit vector.
    pub fn load(p: *const i16) -> __m256i {
        unsafe { _mm256_load_si256(p as *const __m256i) }
    }

    /// Stores a 256-bit vector to a pointer.
    pub fn store(value: __m256i, dest: *mut i16) {
        unsafe { _mm256_store_si256(dest as *mut __m256i, value) }
    }

    /// Performs addition on packed 16-bit integers.
    pub fn add(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_add_epi16(a, b) }
    }

    /// Performs subtraction on packed 16-bit integers.
    pub fn sub(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_sub_epi16(a, b) }
    }

    /// Performs multiplication on packed 16-bit integers.
    pub fn mul(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_mullo_epi16(a, b) }
    }

    /// Performs bitwise AND operation on packed 16-bit integers.
    pub fn bit_and(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_and_si256(a, b) }
    }

    /// Performs bitwise OR operation on packed 16-bit integers.
    pub fn bit_or(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_or_si256(a, b) }
    }

    /// Performs bitwise XOR operation on packed 16-bit integers.
    pub fn bit_xor(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_xor_si256(a, b) }
    }

    /// Computes the bitwise AND NOT operation on packed 16-bit integers.
    pub fn bit_andnot(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_andnot_si256(a, b) }
    }

    /// Performs bitwise NOT operation on packed 16-bit integers.
    pub fn bit_not(a: __m256i) -> __m256i {
        unsafe { _mm256_andnot_si256(a, Self::ALL_BITS_SET) }
    }

    /// Computes the minimum of packed 16-bit integers.
    pub fn min(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_min_epi16(a, b) }
    }

    /// Computes the maximum of packed 16-bit integers.
    pub fn max(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_max_epi16(a, b) }
    }

    /// Checks for equality between two vectors.
    pub fn equal(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_cmpeq_epi16(a, b) }
    }

    /// Compares packed 16-bit integers and sets if greater-than.
    pub fn greater_than(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_cmpgt_epi16(a, b) }
    }

    /// Compares packed 16-bit integers and sets if greater-than or equal.
    pub fn greater_than_or_equal(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_or_si256(Self::greater_than(a, b), Self::equal(a, b)) }
    }

    /// Performs Multiply-Add operation on packed 16-bit integers.
    pub fn multiply_add(a: __m256i, b: __m256i, c: __m256i) -> __m256i {
        Self::add(a, Self::mul(b, c))
    }

    /// Checks for inequality between two vectors.
    pub fn not_equal(a: __m256i, b: __m256i) -> __m256i {
        Self::bit_not(Self::equal(a, b))
    }

    /// Checks if all packed 16-bit integers are equal.
    pub fn all_equal(a: __m256i, b: __m256i) -> bool {
        unsafe { _mm256_movemask_epi8(Self::equal(a, b)) == -1 }
    }

    /// Truncates packed 16-bit integers (noop in this case).
    pub fn truncate(a: __m256i) -> __m256i {
        a
    }
}
