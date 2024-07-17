crate::ix!();

/// Unsigned 64-bit integer AVX intrinsics.
/// Tags: DSP
pub struct SimdAvxU64;

impl SimdAvxU64 {
    /// All bits set constant for __m256i
    pub const ALL_BITS_SET: __m256i = unsafe { _mm256_set1_epi64x(-1i64) };

    /// High bit set constant for __m256i
    pub const HIGH_BIT: __m256i = unsafe { _mm256_set1_epi64x(1i64 << 63) };

    /// Expand a 64-bit unsigned integer into a __m256i
    pub fn expand(s: u64) -> __m256i {
        unsafe { _mm256_set1_epi64x(s as i64) }
    }

    /// Load a __m256i from a pointer
    pub fn load(p: *const u64) -> __m256i {
        unsafe { _mm256_load_si256(p as *const __m256i) }
    }

    /// Store a __m256i into a destination pointer
    pub fn store(value: __m256i, dest: *mut u64) {
        unsafe { _mm256_store_si256(dest as *mut __m256i, value) }
    }

    /// Sign-manipulate __m256i
    pub fn ssign(a: __m256i) -> __m256i {
        Self::bit_xor(a, Self::load(&Self::HIGH_BIT as *const __m256i as *const u64))
    }

    /// Add two __m256i values
    pub fn add(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_add_epi64(a, b) }
    }

    /// Subtract b from a
    pub fn sub(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_sub_epi64(a, b) }
    }

    /// Bitwise AND of two __m256i values
    pub fn bit_and(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_and_si256(a, b) }
    }

    /// Bitwise OR of two __m256i values
    pub fn bit_or(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_or_si256(a, b) }
    }

    /// Bitwise XOR of two __m256i values
    pub fn bit_xor(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_xor_si256(a, b) }
    }

    /// Bitwise ANDNOT operation
    pub fn bit_andnot(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_andnot_si256(a, b) }
    }

    /// Bitwise NOT operation
    pub fn bit_not(a: __m256i) -> __m256i {
        Self::bit_andnot(a, Self::ALL_BITS_SET)
    }

    /// Minimum of two __m256i values
    pub fn min(a: __m256i, b: __m256i) -> __m256i {
        let lt = Self::greater_than(b, a);
        Self::bit_or(Self::bit_and(lt, a), Self::bit_andnot(lt, b))
    }

    /// Maximum of two __m256i values
    pub fn max(a: __m256i, b: __m256i) -> __m256i {
        let gt = Self::greater_than(a, b);
        Self::bit_or(Self::bit_and(gt, a), Self::bit_andnot(gt, b))
    }

    /// Check equality of two __m256i values
    pub fn equal(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_cmpeq_epi64(a, b) }
    }

    /// Check if a > b
    pub fn greater_than(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_cmpgt_epi64(Self::ssign(a), Self::ssign(b)) }
    }

    /// Check if a >= b
    pub fn greater_than_or_equal(a: __m256i, b: __m256i) -> __m256i {
        Self::bit_or(Self::greater_than(a, b), Self::equal(a, b))
    }

    /// Multiply and add a, b, and c
    pub fn multiply_add(a: __m256i, b: __m256i, c: __m256i) -> __m256i {
        Self::add(a, Self::mul(b, c))
    }

    /// Check if a != b
    pub fn not_equal(a: __m256i, b: __m256i) -> __m256i {
        Self::bit_not(Self::equal(a, b))
    }

    /// Truncate operation (identical in this case)
    pub fn truncate(a: __m256i) -> __m256i {
        a
    }

    /// Checks if all 64-bit integers in both vectors are equal
    pub fn all_equal(a: __m256i, b: __m256i) -> bool {
        let compare_mask = unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi64(a, b)) };
        compare_mask == -1
    }

    /// Extracts a 64-bit integer at index i from a __m256i vector
    pub fn get(v: __m256i, i: usize) -> u64 {
        assert!(i < 4, "Index out of bounds for a __m256i vector");
        let mut arr = [0u64; 4];
        unsafe { _mm256_storeu_si256(arr.as_mut_ptr() as *mut __m256i, v) };
        arr[i]
    }

    /// Sets a 64-bit integer at index i within a __m256i vector
    pub fn set(mut v: __m256i, i: usize, s: u64) -> __m256i {
        assert!(i < 4, "Index out of bounds for a __m256i vector");
        let mut arr = [0u64; 4];
        unsafe { _mm256_storeu_si256(arr.as_mut_ptr() as *mut __m256i, v) };
        arr[i] = s;
        unsafe { _mm256_loadu_si256(arr.as_ptr() as *const __m256i) }
    }

    /// Computes the sum of all 64-bit integers in a __m256i vector
    pub fn sum(v: __m256i) -> u64 {
        let mut arr = [0u64; 4];
        unsafe { _mm256_storeu_si256(arr.as_mut_ptr() as *mut __m256i, v) };
        arr.iter().sum()
    }

    /// Multiplies packed 64-bit integers in vectors a and b.
    /// Note: Specialized instructions may be preferable.
    pub fn mul(a: __m256i, b: __m256i) -> __m256i {
        let mut a_arr = [0u64; 4];
        let mut b_arr = [0u64; 4];
        let mut res_arr = [0u64; 4];
        unsafe { 
            _mm256_storeu_si256(a_arr.as_mut_ptr() as *mut __m256i, a);
            _mm256_storeu_si256(b_arr.as_mut_ptr() as *mut __m256i, b);
        }
        for i in 0..4 {
            res_arr[i] = a_arr[i].wrapping_mul(b_arr[i]);
        }
        unsafe { _mm256_loadu_si256(res_arr.as_ptr() as *const __m256i) }
    }
}
