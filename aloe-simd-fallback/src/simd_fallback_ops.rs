crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/native/aloe_fallback_SIMDNativeOps.h]

pub trait SIMDOps<ScalarType: Copy, vSIMDType: Copy> {
    fn get(v: vSIMDType, i: usize) -> Result<ScalarType, SimdError>;
    fn set(v: vSIMDType, i: usize, s: ScalarType) -> Result<vSIMDType, SimdError>;
    fn bit_not(v: vSIMDType) -> vSIMDType;
    fn sum(v: vSIMDType) -> ScalarType;
    fn truncate(v: vSIMDType) -> vSIMDType;
    fn multiply_add(v: vSIMDType, b: vSIMDType, c: vSIMDType) -> vSIMDType;
    fn all_equal(a: vSIMDType, b: vSIMDType) -> Result<bool, SimdError>;
    fn cmplxmul(a: vSIMDType, b: vSIMDType) -> Result<vSIMDType, SimdError>;
    fn apply<O: Op<ScalarType>>(a: vSIMDType, b: vSIMDType) -> Result<vSIMDType, SimdError>;
    fn cmp<O: OpMask<ScalarType, i32>>(a: vSIMDType, b: vSIMDType) -> Result<vSIMDType, SimdError>;
    fn bitapply<O: Op<i32>>(a: vSIMDType, b: vSIMDType) -> Result<vSIMDType, SimdError>;
    fn expand(s: ScalarType) -> vSIMDType;
    fn load(a: *const ScalarType) -> vSIMDType;
    fn store(av:   vSIMDType, dest: *mut ScalarType);
    fn shuffle<const shuffle_idx: u32>(av: vSIMDType) -> vSIMDType;
}

pub enum SimdError {
    Default,
    IndexOOB,
}

/// UnionType for storage of vSIMDType and its scalar components
union UnionType<ScalarType: Copy, vSIMDType: Copy + Default> {
    v: vSIMDType,
    s: [ScalarType; 4], // Assuming 4 lanes; adjust as needed
}

/// UnionMaskType for storage of vSIMDType and its bitwise mask components
union UnionMaskType<vSIMDType: Copy> {
    v: vSIMDType,
    m: [i32; 4], // Assuming 4 lanes; adjust as needed
}

pub trait Op<T>: Sync {
    fn op(a: T, b: T) -> T;
}

pub trait OpMask<T, U>: Sync {
    fn op(a: T, b: T) -> U;
}

/**
  | Useful fallback routines to use if the
  | native SIMD op is not supported.
  | 
  | You should never need to use this directly.
  | Use aloe_SIMDRegister instead.
  | 
  | @tags{DSP}
  |
  */
pub struct SIMDFallbackOps;

impl<ScalarType, vSIMDType> SIMDOps<ScalarType, vSIMDType> for SIMDFallbackOps
where
    ScalarType: Copy + From<i32> + Default + std::ops::Add<Output = ScalarType> + std::ops::Mul<Output = ScalarType> + Into<i32> + std::cmp::PartialEq,
    vSIMDType: Copy + Default + std::ops::Not<Output = vSIMDType>,
{
    #[inline(always)]
    fn get(v: vSIMDType, i: usize) -> Result<ScalarType, SimdError> {
        let mut u = UnionType::<ScalarType,vSIMDType> { v };
        if i >= 4 {
            return Err(SimdError::IndexOOB);
        }
        unsafe { Ok(u.s[i]) }
    }

    #[inline(always)]
    fn set(v: vSIMDType, i: usize, s: ScalarType) -> Result<vSIMDType, SimdError> {
        let mut u = UnionType::<ScalarType,vSIMDType> { v };
        if i >= 4 {
            return Err(SimdError::IndexOOB);
        }
        unsafe {
            u.s[i] = s;
            Ok(u.v)
        }
    }

    #[inline(always)]
    fn bit_not(v: vSIMDType) -> vSIMDType {
        let mut a = UnionMaskType { v };
        unsafe {
            for i in 0..4 {
                let mut val = a.m[i];
                val = !val;
                a.m[i] = val;
            }
        }
        unsafe { a.v }
    }

    #[inline(always)]
    fn sum(v: vSIMDType) -> ScalarType {
        let mut a = UnionType::<ScalarType,vSIMDType> { v };
        let mut sum = ScalarType::default();
        unsafe {
            for i in 0..4 {
                sum = sum + a.s[i];
            }
        }
        sum
    }

    #[inline(always)]
    fn truncate(v: vSIMDType) -> vSIMDType {
        let mut a = UnionType::<ScalarType,vSIMDType> { v };
        unsafe {
            for i in 0..4 {
                let truncated = a.s[i];
                a.s[i] = truncated;
            }
        }
        unsafe { a.v }
    }

    #[inline(always)]
    fn multiply_add(a: vSIMDType, b: vSIMDType, c: vSIMDType) -> vSIMDType {

        let mut va = UnionType::<ScalarType,vSIMDType> { v: a };
        let vb     = UnionType::<ScalarType,vSIMDType> { v: b };
        let vc     = UnionType::<ScalarType,vSIMDType> { v: c };

        unsafe {
            for i in 0..4 {
                va.s[i] = va.s[i] + vb.s[i] * vc.s[i];
            }
        }
        unsafe { va.v }
    }

    #[inline(always)]
    fn all_equal(a: vSIMDType, b: vSIMDType) -> Result<bool, SimdError> {
        let ua = UnionType::<ScalarType,vSIMDType> { v: a };
        let ub = UnionType::<ScalarType,vSIMDType> { v: b };

        for i in 0..4 {
            let sa: ScalarType;
            let sb: ScalarType;
            unsafe {
                sa = ua.s[i];
                sb = ub.s[i];
            }
            if sa != sb {
                return Ok(false);
            }
        }
        Ok(true)
    }

    #[inline(always)]
    fn cmplxmul(a: vSIMDType, b: vSIMDType) -> Result<vSIMDType, SimdError> {
        let ua = UnionType::<ScalarType,vSIMDType> { v: a };
        let ub = UnionType::<ScalarType,vSIMDType> { v: b };

        let mut ur = UnionType::<ScalarType,vSIMDType> {
            s: Default::default(),
        };

        let m = 2;  // Assuming 4 lanes, hence m = n / 2
        for i in 0..m {
            let real_a: ScalarType;
            let imag_a: ScalarType;
            let real_b: ScalarType;
            let imag_b: ScalarType;

            unsafe {
                real_a = ua.s[i << 1];
                imag_a = ua.s[(i << 1) | 1];
                real_b = ub.s[i << 1];
                imag_b = ub.s[(i << 1) | 1];
            }

            let cmplx_a = Complex::new(real_a.into(), imag_a.into());
            let cmplx_b = Complex::new(real_b.into(), imag_b.into());

            let result = cmplx_a * cmplx_b;

            unsafe {
                ur.s[i << 1]       = result.re.into();
                ur.s[(i << 1) | 1] = result.im.into();
            }
        }

        Ok(unsafe { ur.v })
    }

    #[inline(always)]
    fn apply<O: Op<ScalarType>>(a: vSIMDType, b: vSIMDType) -> Result<vSIMDType, SimdError> {
        let mut ua = UnionType::<ScalarType,vSIMDType> { v: a };
        let ub = UnionType::<ScalarType,vSIMDType> { v: b };

        for i in 0..4 {
            let mut sa: ScalarType;
            let sb: ScalarType;
            unsafe {
                sa = ua.s[i];
                sb = ub.s[i];
            }
            sa = O::op(sa, sb);
            unsafe {
                ua.s[i] = sa;
            }
        }
        Ok(unsafe { ua.v })
    }

    #[inline(always)]
    fn cmp<O: OpMask<ScalarType, i32>>(a: vSIMDType, b: vSIMDType) -> Result<vSIMDType, SimdError> {

        let ua = UnionType::<ScalarType,vSIMDType> { v: a };
        let ub = UnionType::<ScalarType,vSIMDType> { v: b };

        let mut ur = UnionType::<ScalarType,vSIMDType> {
            s: Default::default(),
        };

        for i in 0..4 {
            let sa: ScalarType;
            let sb: ScalarType;
            unsafe {
                sa = ua.s[i];
                sb = ub.s[i];
            }
            let mask = O::op(sa, sb);
            unsafe {
                ur.s[i] = mask.into();
            }
        }
        Ok(unsafe { ur.v })
    }

    #[inline(always)]
    fn bitapply<O: Op<i32>>(a: vSIMDType, b: vSIMDType) -> Result<vSIMDType, SimdError> {
        let mut ua = UnionMaskType { v: a };
        let ub = UnionMaskType { v: b };

        for i in 0..4 {
            let mut ma: i32;
            let mb: i32;
            unsafe {
                ma = ua.m[i];
                mb = ub.m[i];
            }
            ma = O::op(ma, mb);
            unsafe {
                ua.m[i] = ma;
            }
        }
        Ok(unsafe { ua.v })
    }
    
    #[inline(always)] fn expand(s: ScalarType) -> vSIMDType {
        
        todo!();
        /*
            UnionType r;

            for (size_t i = 0; i < n; ++i)
                r.s[i] = s;

            return r.v;
        */
    }
    
    #[inline(always)] fn load(a: *const ScalarType) -> vSIMDType {
        
        todo!();
        /*
            UnionType r;

            for (size_t i = 0; i < n; ++i)
                r.s[i] = a[i];

            return r.v;
        */
    }
    
    #[inline(always)] fn store(
        av:   vSIMDType,
        dest: *mut ScalarType)  {
        
        todo!();
        /*
            UnionType a {av};

            for (size_t i = 0; i < n; ++i)
                dest[i] = a.s[i];
        */
    }
    
    #[inline(always)] fn shuffle<const shuffle_idx: u32>(av: vSIMDType) -> vSIMDType {
    
        todo!();
        /*
            UnionType a {av}, r;

            // the compiler will unroll this loop and the index can
            // be computed at compile-time, so this will be super fast
            for (size_t i = 0; i < n; ++i)
                r.s[i] = a.s[(shuffle_idx >> (bits * i)) & mask];

            return r.v;
        */
    }
}
