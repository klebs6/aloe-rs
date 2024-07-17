crate::ix!();

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// SIMD Native Operations for unsigned 32-bit integers.
/// Tags: Digital Signal Processing (DSP)
pub struct SimdNativeU32;

impl SimdNativeU32 {

    /// Constants
    const ALL_BITS_SET: [u32; 4] = [u32::MAX; 4];
    const HIGH_BIT: [u32; 4] = [0x80000000; 4];

    /// Load vector from pointer
    pub unsafe fn vconst(a: *const u32) -> __m128i {
        Self::load(a)
    }
    
    /// Sign change
    pub unsafe fn ssign(a: __m128i) -> __m128i {
        _mm_xor_si128(a, Self::vconst(Self::HIGH_BIT.as_ptr()))
    }

    /// Load a vector from a pointer
    pub unsafe fn load(a: *const u32) -> __m128i {
        _mm_load_si128(a as *const __m128i)
    }

    /// Store vector to pointer
    pub unsafe fn store(v: __m128i, p: *mut u32) {
        _mm_store_si128(p as *mut __m128i, v)
    }

    /// Duplicate scalar to all vector elements
    pub unsafe fn expand(s: u32) -> __m128i {
        _mm_set1_epi32(s as i32)
    }

    /// Element-wise addition
    pub unsafe fn add(a: __m128i, b: __m128i) -> __m128i {
        _mm_add_epi32(a, b)
    }

    /// Element-wise subtraction
    pub unsafe fn sub(a: __m128i, b: __m128i) -> __m128i {
        _mm_sub_epi32(a, b)
    }

    /// Element-wise bitwise AND
    pub unsafe fn bit_and(a: __m128i, b: __m128i) -> __m128i {
        _mm_and_si128(a, b)
    }

    /// Element-wise bitwise OR
    pub unsafe fn bit_or(a: __m128i, b: __m128i) -> __m128i {
        _mm_or_si128(a, b)
    }

    /// Element-wise bitwise XOR
    pub unsafe fn bit_xor(a: __m128i, b: __m128i) -> __m128i {
        _mm_xor_si128(a, b)
    }

    /// Element-wise bitwise AND-NOT
    pub unsafe fn bit_andnot(a: __m128i, b: __m128i) -> __m128i {
        _mm_andnot_si128(a, b)
    }

    /// Element-wise bitwise NOT
    pub unsafe fn bit_not(a: __m128i) -> __m128i {
        _mm_andnot_si128(a, Self::vconst(Self::ALL_BITS_SET.as_ptr()))
    }

    /// Element-wise comparison (equal)
    pub unsafe fn equal(a: __m128i, b: __m128i) -> __m128i {
        _mm_cmpeq_epi32(a, b)
    }

    /// Element-wise comparison (greater than)
    pub unsafe fn greater_than(a: __m128i, b: __m128i) -> __m128i {
        _mm_cmpgt_epi32(Self::ssign(a), Self::ssign(b))
    }

    /// Element-wise comparison (greater than or equal)
    pub unsafe fn greater_than_or_equal(a: __m128i, b: __m128i) -> __m128i {
        _mm_or_si128(Self::greater_than(a, b), Self::equal(a, b))
    }

    /// Multiply and add: (a + b * c)
    pub unsafe fn multiply_add(a: __m128i, b: __m128i, c: __m128i) -> __m128i {
        Self::add(a, Self::mul(b, c))
    }

    /// Element-wise comparison (not equal)
    pub unsafe fn not_equal(a: __m128i, b: __m128i) -> __m128i {
        _mm_andnot_si128(Self::equal(a, b), Self::vconst(Self::ALL_BITS_SET.as_ptr()))
    }

    /// Check if all elements are equal
    pub unsafe fn all_equal(a: __m128i, b: __m128i) -> bool {
        _mm_movemask_epi8(Self::equal(a, b)) == 0xffff
    }

    /// Retrieve vector element by index
    pub unsafe fn get(v: __m128i, i: usize) -> u32 {
        let mut arr = [0u32; 4];
        Self::store(v, arr.as_mut_ptr());
        arr[i]
    }

    /// Set vector element by index
    pub unsafe fn set(v: __m128i, i: usize, s: u32) -> __m128i {
        let mut arr = [0u32; 4];
        Self::store(v, arr.as_mut_ptr());
        arr[i] = s;
        Self::load(arr.as_ptr())
    }

    /// No-op for truncation in this case
    pub unsafe fn truncate(a: __m128i) -> __m128i {
        a
    }

    /// Sum of vector elements
    pub unsafe fn sum(a: __m128i) -> u32 {
        let tmp = _mm_hadd_epi32(a, a);
        _mm_cvtsi128_si32(_mm_hadd_epi32(tmp, tmp)) as u32
    }

    /// Element-wise multiplication
    pub unsafe fn mul(a: __m128i, b: __m128i) -> __m128i {
        _mm_mullo_epi32(a, b)
    }

    /// Element-wise minimum
    pub unsafe fn min(a: __m128i, b: __m128i) -> __m128i {
        _mm_min_epi32(a, b)
    }

    /// Element-wise maximum
    pub unsafe fn max(a: __m128i, b: __m128i) -> __m128i {
        _mm_max_epi32(a, b)
    }
}
