crate::ix!();

fn all_bits_set() -> __m128i {
    unsafe { _mm_set1_epi32(-1) }
}

fn even_high_bit() -> __m128 {
    unsafe { _mm_set1_ps(std::mem::transmute(0x80000000u32)) }
}

fn one() -> __m128 {
    unsafe { _mm_set1_ps(1.0) }
}

/// SIMD Native Operations for single-precision floating
/// points
///
pub struct SimdNativeF32;

impl SimdNativeF32 {

    pub fn expand(s: f32) 
        -> __m128 
    {
        unsafe { 
            _mm_load1_ps(&s) 
        }
    }

    pub fn load(a: &[f32; 4]) 
        -> __m128 
    { 
        unsafe { 
            _mm_load_ps(a.as_ptr()) 
        }
    }

    pub fn store(value: __m128, dest: &mut [f32; 4]) 
    { 
        unsafe { 
            _mm_store_ps(dest.as_mut_ptr(), value) 
        }
    }

    #[inline] pub fn add(a: __m128, b: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_add_ps(a, b) 
        } 
    }

    #[inline] pub fn sub(a: __m128, b: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_sub_ps(a, b) 
        } 
    }

    #[inline] pub fn mul(a: __m128, b: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_mul_ps(a, b) 
        } 
    }

    #[inline] pub fn bit_and(a: __m128, b: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_and_ps(a, b) 
        } 
    }

    #[inline] pub fn bit_or(a: __m128, b: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_or_ps(a, b) 
        } 
    }

    #[inline] pub fn bit_xor(a: __m128, b: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_xor_ps(a, b) 
        } 
    }

    #[inline] pub fn bit_notand(a: __m128, b: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_andnot_ps(a, b) 
        } 
    }

    #[inline]
    pub fn bit_not(a: __m128) -> __m128 {
        unsafe {
            let all_bits_set = _mm_set1_ps(-1.0); // Represent all bits set for __m128
            _mm_xor_ps(a, all_bits_set)
        }
    }

    #[inline] pub fn min(a: __m128, b: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_min_ps(a, b) 
        } 
    }

    #[inline] pub fn max(a: __m128, b: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_max_ps(a, b) 
        } 
    }

    #[inline] pub fn equal(a: __m128, b: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_cmpeq_ps(a, b) 
        } 
    }

    #[inline] pub fn not_equal(a: __m128, b: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_cmpneq_ps(a, b) 
        } 
    }

    #[inline] pub fn greater_than(a: __m128, b: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_cmpgt_ps(a, b) 
        } 
    }

    #[inline] pub fn greater_than_or_equal(a: __m128, b: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_cmpge_ps(a, b) 
        } 
    }

    #[inline] pub fn all_equal(a: __m128, b: __m128) 
    -> bool 
    { 
        unsafe { 
            _mm_movemask_ps(Self::equal(a, b)) == 0xf 
        } 
    }

    #[inline] pub fn multiply_add(a: __m128, b: __m128, c: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_add_ps(a, _mm_mul_ps(b, c)) 
        } 
    }

    #[inline] pub fn dupeven(a: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_shuffle_ps(a, a, 0x00) 
        } 
    }

    #[inline] pub fn dupodd(a: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_shuffle_ps(a, a, 0x55) 
        } 
    }

    #[inline] pub fn swapevenodd(a: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_shuffle_ps(a, a, 0x4E) 
        } 
    }

    #[inline] pub fn oddevensum(a: __m128) 
    -> __m128 
    { 
        unsafe { 
            _mm_add_ps(_mm_shuffle_ps(a, a, 0xB1), a) 
        } 
    }

    /// Complex number multiplication
    #[inline] pub fn cmplxmul(a: __m128, b: __m128) -> __m128 {
        let rr_ir = Self::mul(a, Self::dupeven(b));
        let ii_ri = Self::mul(Self::swapevenodd(a), Self::dupodd(b));
        Self::add(rr_ir, Self::bit_xor(ii_ri, even_high_bit()))
    }

    /// Sum of all elements
    #[inline] pub fn sum(a: __m128) -> f32 {
        unsafe {
            #[cfg(target_feature = "sse4.1")]
            {
                let retval = _mm_dp_ps(a, Self::K_ONE, 0xff);
                _mm_cvtss_f32(retval)
            }
            #[cfg(target_feature = "sse3")]
            {
                let retval = _mm_hadd_ps(_mm_hadd_ps(a, a), a);
                _mm_cvtss_f32(retval)
            }
            #[cfg(not(any(target_feature = "sse3", target_feature = "sse4.1")))]
            {
                let mut retval = _mm_add_ps(_mm_shuffle_ps(a, a, 0x4E), a);
                retval = _mm_add_ps(retval, _mm_shuffle_ps(retval, retval, 0xB1));
                _mm_cvtss_f32(retval)
            }
        }
    }
}
