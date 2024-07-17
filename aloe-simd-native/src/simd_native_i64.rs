crate::ix!();

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// SIMD Native Operations for signed 64-bit integers
/// Tags: Digital Signal Processing (DSP)
pub struct SimdNativeI64;

impl SimdNativeI64 {

    /// All bits set constant for int64_t
    const K_ALL_BITS_SET: [i64; 2] = [i64::MAX; 2];

    /// Load a vector from a pointer
    pub unsafe fn vconst(a: *const i64) -> __m128i {
        Self::load(a)
    }

    /// Duplicate scalar to all vector elements
    pub unsafe fn expand(s: i64) -> __m128i {
        _mm_set1_epi64x(s)
    }

    /// Load a vector from a pointer
    pub unsafe fn load(a: *const i64) -> __m128i {
        _mm_load_si128(a as *const __m128i)
    }

    /// Store vector to a pointer
    pub unsafe fn store(v: __m128i, p: *mut i64) {
        _mm_store_si128(p as *mut __m128i, v)
    }

    /// Element-wise addition
    pub unsafe fn add(a: __m128i, b: __m128i) -> __m128i {
        _mm_add_epi64(a, b)
    }

    // Define a multiplication function for SimdNativeI64 vectors
    pub fn mul(a: __m128i, b: __m128i) -> __m128i {
        // Perform multiplication operation here
        // Example: Return the result of multiplying a and b
        unsafe { _mm_mul_epi32(a, b) }
    }

    /// Element-wise subtraction
    pub unsafe fn sub(a: __m128i, b: __m128i) -> __m128i {
        _mm_sub_epi64(a, b)
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
        _mm_andnot_si128(a, Self::vconst(Self::K_ALL_BITS_SET.as_ptr()))
    }

    /// Element-wise minimum
    pub unsafe fn min(a: __m128i, b: __m128i) -> __m128i {
        let lt = Self::greater_than(b, a);
        Self::bit_or(Self::bit_and(lt, a), Self::bit_andnot(lt, b))
    }

    /// Element-wise maximum
    pub unsafe fn max(a: __m128i, b: __m128i) -> __m128i {
        let gt = Self::greater_than(a, b);
        Self::bit_or(Self::bit_and(gt, a), Self::bit_andnot(gt, b))
    }

    /// Element-wise comparison (greater than or equal)
    pub unsafe fn greater_than_or_equal(a: __m128i, b: __m128i) -> __m128i {
        Self::bit_or(Self::greater_than(a, b), Self::equal(a, b))
    }

    /// Multiply and add: (a + b * c)
    pub unsafe fn multiply_add(a: __m128i, b: __m128i, c: __m128i) -> __m128i {
        Self::add(a, Self::mul(b, c))
    }

    /// Element-wise comparison (not equal)
    pub unsafe fn not_equal(a: __m128i, b: __m128i) -> __m128i {
        Self::bit_not(Self::equal(a, b))
    }

    /// Check if all elements are equal
    pub unsafe fn all_equal(a: __m128i, b: __m128i) -> bool {
        _mm_movemask_epi8(Self::equal(a, b)) == 0xffff
    }

    /// Element-wise comparison (equal)
    pub unsafe fn equal(a: __m128i, b: __m128i) -> __m128i {

        // Using SSE4.1 if available
        #[cfg(any(target_feature = "sse4.1"))]
        {
            _mm_cmpeq_epi64(a, b)
        }
        // Fallback for non-SSE4.1

        #[cfg(not(target_feature = "sse4.1"))]
        {
            let bitmask = _mm_cmpeq_epi32(a, b);
            let bitmask = _mm_and_si128(bitmask, _mm_shuffle_epi32(bitmask, 0b10110001));
            _mm_shuffle_epi32(bitmask, 0b10100000)
        }
    }

    /// Element-wise comparison (greater than)
    pub unsafe fn greater_than(a: __m128i, b: __m128i) -> __m128i {

        // Using SSE4.2 if available
        #[cfg(any(target_feature = "sse4.2"))]
        {
            _mm_cmpgt_epi64(a, b)
        }
        // Fallback for non-SSE4.2
        #[cfg(not(target_feature = "sse4.2"))]
        {
            // Custom implementation can be placed here.
            // This is a non-trivial operation without the SSE4.2 intrinsic.
            unimplemented!("Fallback for greater_than is not implemented")
        }
    }
}
