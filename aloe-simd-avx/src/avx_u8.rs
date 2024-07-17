crate::ix!();

/// Unsigned 8-bit integer AVX intrinsics for Digital Signal Processing (DSP).
pub struct SimdAvxU8;

impl SimdAvxU8 {

    const HIGH_BIT:     __m256i = unsafe { _mm256_set1_epi8(0x80u8 as i8) };
    const ALL_BITS_SET: __m256i = unsafe { _mm256_set1_epi8(-1i8) };
    
    /// Sign extends the given 8-bit vector.
    pub unsafe fn ssign(a: __m256i) -> __m256i {
        _mm256_xor_si256(a, Self::HIGH_BIT)
    }

    /// Expands the scalar into a vector.
    pub unsafe fn expand(s: u8) -> __m256i {
        _mm256_set1_epi8(s as i8)
    }

    /// Loads a vector from the memory.
    pub unsafe fn load(p: *const u8) -> __m256i {
        _mm256_load_si256(p as *const __m256i)
    }

    /// Stores the vector into the memory.
    pub unsafe fn store(value: __m256i, dest: *mut u8) {
        _mm256_store_si256(dest as *mut __m256i, value)
    }

    /// Computes the sum of all packed 8-bit integers.
    pub unsafe fn sum(a: __m256i) -> u8 {

        let zero = _mm256_setzero_si256();

        let mut lo = _mm256_unpacklo_epi8(a, zero);
        let mut hi = _mm256_unpackhi_epi8(a, zero);

        for _ in 0..3 {
            lo = _mm256_hadd_epi16(lo, lo);
            hi = _mm256_hadd_epi16(hi, hi);
        }

        let sum_lo: u32 = mem::transmute(_mm256_extract_epi16(lo, 0)) & 0xff;
        let sum_hi: u32 = mem::transmute(_mm256_extract_epi16(hi, 0)) & 0xff;
        
        // summing up all the horizontal sums
        (sum_lo + sum_hi) as u8
    }

    /// Multiplication operation for 8-bit integers.
    pub unsafe fn mul(a: __m256i, b: __m256i) -> __m256i {

        let even = _mm256_mullo_epi16(a, b);
        let odd  = _mm256_mullo_epi16(_mm256_srli_epi16(a, 8), _mm256_srli_epi16(b, 8));

        _mm256_or_si256(
            _mm256_slli_epi16(odd, 8),
            _mm256_srli_epi16(_mm256_slli_epi16(even, 8), 8),
        )
    }

    /// Performs addition on packed 8-bit integers.
    pub unsafe fn add(a: __m256i, b: __m256i) -> __m256i {
         _mm256_add_epi8(a, b) 
    }

    /// Performs subtraction on packed 8-bit integers.
    pub unsafe fn sub(a: __m256i, b: __m256i) -> __m256i {
         _mm256_sub_epi8(a, b) 
    }

    /// Performs bitwise AND operation on packed 8-bit integers.
    pub unsafe fn bit_and(a: __m256i, b: __m256i) -> __m256i {
         _mm256_and_si256(a, b) 
    }

    /// Performs bitwise OR operation on packed 8-bit integers.
    pub unsafe fn bit_or(a: __m256i, b: __m256i) -> __m256i {
         _mm256_or_si256(a, b) 
    }

    /// Performs bitwise XOR operation on packed 8-bit integers.
    pub unsafe fn bit_xor(a: __m256i, b: __m256i) -> __m256i {
         _mm256_xor_si256(a, b) 
    }

    /// Computes the bitwise AND NOT operation on packed 8-bit integers.
    pub unsafe fn bit_andnot(a: __m256i, b: __m256i) -> __m256i {
         _mm256_andnot_si256(a, b) 
    }

    /// Performs bitwise NOT operation on packed 8-bit integers.
    pub unsafe fn bit_not(a: __m256i) -> __m256i {
         _mm256_andnot_si256(a, Self::ALL_BITS_SET) 
    }

    /// Computes the minimum of packed 8-bit integers.
    pub unsafe fn min(a: __m256i, b: __m256i) -> __m256i {
         _mm256_min_epu8(a, b) 
    }

    /// Computes the maximum of packed 8-bit integers.
    pub unsafe fn max(a: __m256i, b: __m256i) -> __m256i {
         _mm256_max_epu8(a, b) 
    }

    /// Checks for equality between two vectors.
    pub unsafe fn equal(a: __m256i, b: __m256i) -> __m256i {
         _mm256_cmpeq_epi8(a, b) 
    }
  
    /// Compares packed 8-bit integers and sets if greater-than.
    pub unsafe fn greater_than(a: __m256i, b: __m256i) -> __m256i {
         _mm256_cmpgt_epi8(Self::ssign(a), Self::ssign(b)) 
    }

    /// Compares packed 8-bit integers and sets if greater-than or equal.
    pub unsafe fn greater_than_or_equal(a: __m256i, b: __m256i) -> __m256i {
         _mm256_or_si256(Self::greater_than(a, b), Self::equal(a, b)) 
    }

    /// Checks if all packed 8-bit integers are equal.
    pub unsafe fn all_equal(a: __m256i, b: __m256i) -> bool {
         _mm256_movemask_epi8(Self::equal(a, b)) == -1 
    }

    /// Performs Multiply-Add operation on packed 8-bit integers.
    pub unsafe fn multiply_add(a: __m256i, b: __m256i, c: __m256i) -> __m256i {
        Self::add(a, Self::mul(b, c))
    }

    /// Checks for inequality between two vectors.
    pub unsafe fn not_equal(a: __m256i, b: __m256i) -> __m256i {
        Self::bit_not(Self::equal(a, b))
    }

    /// Truncates packed 8-bit integers (noop in this case).
    pub fn truncate(a: __m256i) -> __m256i {
        a
    }
}
