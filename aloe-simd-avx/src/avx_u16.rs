crate::ix!();

/// Unsigned 16-bit integer AVX intrinsics.
pub struct SimdAvxU16;

impl SimdAvxU16 {

    // Commonly used constants
    pub const HIGH_BIT: u16 = 1 << 15;
    pub const ALL_BITS_SET: u16 = u16::MAX;

    /// Flip the highest bit to handle unsigned comparison as signed
    #[inline(always)]
    pub fn ssign(a: __m256i) -> __m256i {
        unsafe { _mm256_xor_si256(a, _mm256_set1_epi16(Self::HIGH_BIT as i16)) }
    }

    /// Expand scalar to vector
    #[inline(always)]
    pub fn expand(s: u16) -> __m256i {
        unsafe { _mm256_set1_epi16(s as i16) }
    }

    /// Load vector from memory
    #[inline(always)]
    pub fn load(p: *const u16) -> __m256i {
        unsafe { _mm256_load_si256(mem::transmute(p)) }
    }

    /// Store vector to memory
    #[inline(always)]
    pub fn store(value: __m256i, dest: *mut u16) {
        unsafe { _mm256_store_si256(mem::transmute(dest), value) }
    }

    /// Sum of 16-bit elements
    #[inline(always)]
    pub fn sum(a: __m256i) -> u16 {
        let tmp = unsafe { _mm256_hadd_epi16(a, a) };
        let tmp = unsafe { _mm256_hadd_epi16(tmp, tmp) };
        let tmp = unsafe { _mm256_hadd_epi16(tmp, tmp) };
        let result: [i16; 16] = unsafe { mem::transmute(tmp) };
        ((result[0] as u32 & 0xffff) + (result[2] as u32 & 0xffff)) as u16
    }

    /// Element-wise addition
    #[inline(always)]
    pub fn add(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_add_epi16(a, b) }
    }

    /// Element-wise subtraction
    #[inline(always)]
    pub fn sub(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_sub_epi16(a, b) }
    }

    /// Element-wise multiplication
    #[inline(always)]
    pub fn mul(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_mullo_epi16(a, b) }
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

    /// Bitwise ANDNOT
    #[inline(always)]
    pub fn bit_andnot(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_andnot_si256(a, b) }
    }

    /// Bitwise NOT
    #[inline(always)]
    pub fn bit_not(a: __m256i) -> __m256i {
        unsafe { _mm256_andnot_si256(a, _mm256_set1_epi16(Self::ALL_BITS_SET as i16)) }
    }

    /// Element-wise minimum
    #[inline(always)]
    pub fn min(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_min_epu16(a, b) }
    }

    /// Element-wise maximum
    #[inline(always)]
    pub fn max(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_max_epu16(a, b) }
    }

    /// Element-wise equality
    #[inline(always)]
    pub fn equal(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_cmpeq_epi16(a, b) }
    }

    /// Element-wise greater than
    #[inline(always)]
    pub fn greater_than(a: __m256i, b: __m256i) -> __m256i {
        let sa = Self::ssign(a);
        let sb = Self::ssign(b);
        unsafe { _mm256_cmpgt_epi16(sa, sb) }
    }

    /// Element-wise greater than or equal to
    #[inline(always)]
    pub fn greater_than_or_equal(a: __m256i, b: __m256i) -> __m256i {
        let gt = Self::greater_than(a, b);
        let eq = Self::equal(a, b);
        unsafe { _mm256_or_si256(gt, eq) }
    }

    /// Multiply and add
    #[inline(always)]
    pub fn multiply_add(a: __m256i, b: __m256i, c: __m256i) -> __m256i {
        let mul = Self::mul(b, c);
        Self::add(a, mul)
    }

    /// Element-wise not equal
    #[inline(always)]
    pub fn not_equal(a: __m256i, b: __m256i) -> __m256i {
        let eq = Self::equal(a, b);
        Self::bit_not(eq)
    }
}
