crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/native/aloe_avx_SIMDNativeOps.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/native/aloe_avx_SIMDNativeOps.cpp]

/**
  | Unsigned 32-bit integer AVX intrinsics.
  | 
  | @tags{DSP}
  |
  */
pub struct SimdAvxU32;

impl SimdAvxU32 {

    /// Constants representing all bits set for uint32_t and high bit set
    pub const ALL_BITS_SET: u32 = !0;
    pub const HIGH_BIT: u32 = 1 << 31;

    #[inline(always)]
    pub fn ssign(a: __m256i) -> __m256i {
        unsafe { _mm256_xor_si256(a, Self::expand(Self::HIGH_BIT)) }
    }

    #[inline(always)]
    pub fn expand(s: u32) -> __m256i {
        unsafe { _mm256_set1_epi32(s as i32) }
    }

    #[inline(always)]
    pub fn load(p: &[u32; 8]) -> __m256i {
        unsafe { _mm256_load_si256(p.as_ptr() as *const __m256i) }
    }

    #[inline(always)]
    pub fn store(value: __m256i, dest: &mut [u32; 8]) {
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
        unsafe { _mm256_min_epu32(a, b) }
    }

    #[inline(always)]
    pub fn max(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_max_epu32(a, b) }
    }

    #[inline(always)]
    pub fn equal(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_cmpeq_epi32(a, b) }
    }

    #[inline(always)]
    pub fn greater_than(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_cmpgt_epi32(Self::ssign(a), Self::ssign(b)) }
    }

    #[inline(always)]
    pub fn greater_than_or_equal(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_or_si256(Self::greater_than(a, b), Self::equal(a, b)) }
    }

    #[inline(always)]
    pub fn multiply_add(a: __m256i, b: __m256i, c: __m256i) -> __m256i {
        unsafe { _mm256_add_epi32(a, _mm256_mullo_epi32(b, c)) }
    }

    #[inline(always)]
    pub fn not_equal(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_andnot_si256(Self::equal(a, b), Self::expand(Self::ALL_BITS_SET)) }
    }

    #[inline(always)]
    pub fn all_equal(a: __m256i, b: __m256i) -> bool {
        unsafe { _mm256_movemask_epi8(Self::equal(a, b)) == -1 }
    }

    #[inline(always)]
    pub fn sum(a: __m256i) -> u32 {
        let tmp = unsafe { _mm256_hadd_epi32(a, a) };
        let tmp = unsafe { _mm256_hadd_epi32(tmp, tmp) };
        let mask = 0b01001110;

        unsafe {
            _mm256_cvtsi256_si32(tmp) as u32 + _mm256_cvtsi256_si32(_mm256_permute4x64_epi64(tmp, mask)) as u32
        }
    }
}
