crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/native/aloe_sse_SIMDNativeOps.h]

/// Struct to encapsulate SIMD operations on u64.
pub struct SimdNativeU64;

impl SimdNativeU64 {

    /// Constants for SIMD operations on u64.
    const ALL_BITS_SET: u64 = !0;
    const HIGH_BIT: u64 = 1 << 63;

    /// Load SIMD register from a u64 slice.
    #[inline(always)]
    pub fn vconst(a: &[u64; 2]) -> __m128i {
        Self::load(a)
    }

    /// Fill SIMD register with the same u64 value.
    #[inline(always)]
    pub fn expand(s: u64) -> __m128i {
        unsafe { _mm_set1_epi64x(s as i64) }
    }

    /// Flip the highest bit of each element.
    #[inline(always)]
    pub fn ssign(a: __m128i) -> __m128i {
        unsafe { _mm_xor_si128(a, Self::vconst(&[Self::HIGH_BIT; 2])) }
    }

    /// Load SIMD register from a u64 slice.
    #[inline(always)]
    pub fn load(a: &[u64; 2]) -> __m128i {
        unsafe { _mm_load_si128(a.as_ptr() as *const __m128i) }
    }

    /// Store SIMD register values into a u64 slice.
    #[inline(always)]
    pub fn store(a: __m128i, p: &mut [u64; 2]) {
        unsafe { _mm_store_si128(p.as_mut_ptr() as *mut __m128i, a) }
    }

    /// Add corresponding elements in SIMD registers.
    #[inline(always)]
    pub fn add(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_add_epi64(a, b) }
    }

    /// Subtract corresponding elements in SIMD registers.
    #[inline(always)]
    pub fn sub(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_sub_epi64(a, b) }
    }

    /// Bitwise AND corresponding elements in SIMD registers.
    #[inline(always)]
    pub fn bit_and(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_and_si128(a, b) }
    }

    /// Bitwise OR corresponding elements in SIMD registers.
    #[inline(always)]
    pub fn bit_or(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_or_si128(a, b) }
    }

    /// Bitwise XOR corresponding elements in SIMD registers.
    #[inline(always)]
    pub fn bit_xor(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_xor_si128(a, b) }
    }

    /// Bitwise AND-NOT between SIMD registers.
    #[inline(always)]
    pub fn bit_andnot(a: __m128i, b: __m128i) -> __m128i {
        unsafe { _mm_andnot_si128(a, b) }
    }

    /// Bitwise NOT on a SIMD register.
    #[inline(always)]
    pub fn bit_not(a: __m128i) -> __m128i {
        unsafe { _mm_andnot_si128(a, Self::vconst(&[Self::ALL_BITS_SET; 2])) }
    }
    
    /// Min of each packed u64 element
    #[inline(always)]
    pub fn min(a: __m128i, b: __m128i) -> __m128i {
        let lt = Self::greater_than(b, a);
        Self::bit_or(Self::bit_and(lt, a), Self::bit_andnot(lt, b))
    }

    /// Max of each packed u64 element
    #[inline(always)]
    pub fn max(a: __m128i, b: __m128i) -> __m128i {
        let gt = Self::greater_than(a, b);
        Self::bit_or(Self::bit_and(gt, a), Self::bit_andnot(gt, b))
    }

    // Define a greater-than function for SimdNativeU64 vectors
    pub fn greater_than(a: __m128i, b: __m128i) -> __m128i {
        // Perform greater-than comparison operation here
        // Example: Return the result of comparing a > b
        unsafe { _mm_cmpgt_epi64(a, b) }
    }

    // Define an equal function for SimdNativeU64 vectors
    pub fn equal(a: __m128i, b: __m128i) -> __m128i {
        // Perform equality comparison operation here
        // Example: Return the result of comparing a == b
        unsafe { _mm_cmpeq_epi64(a, b) }
    }

    /// Greater than or equal
    #[inline(always)]
    pub fn greater_than_or_equal(a: __m128i, b: __m128i) -> __m128i {
        Self::bit_or(Self::greater_than(a, b), Self::equal(a, b))
    }

    /// Multiply-add operation: a + b * c
    #[inline(always)]
    pub fn multiply_add(a: __m128i, b: __m128i, c: __m128i) -> __m128i {
        Self::add(a, Self::mul(b, c))
    }

    /// Not equal operation
    #[inline(always)]
    pub fn not_equal(a: __m128i, b: __m128i) -> __m128i {
        Self::bit_not(Self::equal(a, b))
    }

    /// All elements equal
    #[inline(always)]
    pub fn all_equal(a: __m128i, b: __m128i) -> bool {
        let eq_mask = Self::equal(a, b);
        unsafe { _mm_movemask_epi8(eq_mask) == 0xffff }
    }

    /// Get element at index i
    #[inline(always)]
    pub fn get(a: __m128i, i: usize) -> u64 {
        if i >= 2 {
            panic!("Index out of bounds");
        }
        let arr: [u64; 2] = unsafe { mem::transmute(a) };
        arr[i]
    }

    /// Set element at index i
    #[inline(always)]
    pub fn set(a: __m128i, i: usize, s: u64) -> __m128i {
        if i >= 2 {
            panic!("Index out of bounds");
        }
        let mut arr: [u64; 2] = unsafe { mem::transmute(a) };
        arr[i] = s;
        unsafe { mem::transmute(arr) }
    }

    /// Sum of all elements
    #[inline(always)]
    pub fn sum(a: __m128i) -> u64 {
        let arr: [u64; 2] = unsafe { mem::transmute(a) };
        arr[0].wrapping_add(arr[1])
    }

    /// Multiply packed elements
    #[inline(always)]
    pub fn mul(a: __m128i, b: __m128i) -> __m128i {
        // Fallback for lack of direct u64 multiplication
        let a_arr: [u64; 2] = unsafe { mem::transmute(a) };
        let b_arr: [u64; 2] = unsafe { mem::transmute(b) };
        let res = [a_arr[0].wrapping_mul(b_arr[0]), a_arr[1].wrapping_mul(b_arr[1])];
        unsafe { mem::transmute(res) }
    }

    /// Truncate operation
    #[inline(always)]
    pub fn truncate(a: __m128i) -> __m128i {
        a
    }
}
