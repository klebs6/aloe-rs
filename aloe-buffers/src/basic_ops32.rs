crate::ix!();

#[cfg(ALOE_USE_SSE_INTRINSICS)]
pub struct BasicOps32 {

}

#[cfg(ALOE_USE_SSE_INTRINSICS)]
impl BasicOps32 {

    /*
      Integer and parallel types are the same for
      SSE. On neon they have different types
      */
    
    #[inline(always)] pub fn toint(v: basic_ops32::ParallelType) -> basic_ops32::IntegerType {
        
        todo!();
        /*
            return v;
        */
    }
    
    #[inline(always)] pub fn toflt(v: basic_ops32::IntegerType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return v;
        */
    }
    
    #[inline(always)] pub fn load1(v: Type) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return _mm_load1_ps (&v);
        */
    }
    
    #[inline(always)] pub fn loada(v: *const basic_ops32::Type) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return _mm_load_ps (v);
        */
    }
    
    #[inline(always)] pub fn loadu(v: *const basic_ops32::Type) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return _mm_loadu_ps (v);
        */
    }
    
    #[inline(always)] pub fn storea(
        dest: *mut basic_ops32::Type,
        a:    basic_ops32::ParallelType)  {
        
        todo!();
        /*
            _mm_store_ps (dest, a);
        */
    }
    
    #[inline(always)] pub fn storeu(
        dest: *mut basic_ops32::Type,
        a:    basic_ops32::ParallelType)  {
        
        todo!();
        /*
            _mm_storeu_ps (dest, a);
        */
    }
    
    #[inline(always)] pub fn add(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return _mm_add_ps (a, b);
        */
    }
    
    #[inline(always)] pub fn sub(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return _mm_sub_ps (a, b);
        */
    }
    
    #[inline(always)] pub fn mul(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return _mm_mul_ps (a, b);
        */
    }
    
    #[inline(always)] pub fn max(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return _mm_max_ps (a, b);
        */
    }
    
    #[inline(always)] pub fn min(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return _mm_min_ps (a, b);
        */
    }
    
    #[inline(always)] pub fn bit_and(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return _mm_and_ps (a, b);
        */
    }
    
    #[inline(always)] pub fn bit_not(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return _mm_andnot_ps (a, b);
        */
    }
    
    #[inline(always)] pub fn bit_or(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return _mm_or_ps (a, b);
        */
    }
    
    #[inline(always)] pub fn bit_xor(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return _mm_xor_ps (a, b);
        */
    }
    
    #[inline(always)] pub fn max(a: basic_ops32::ParallelType) -> basic_ops32::Type {
        
        todo!();
        /*
            Type v[numParallel]; storeU (v, a); return jmax (v[0], v[1], v[2], v[3]);
        */
    }
    
    #[inline(always)] pub fn min(a: basic_ops32::ParallelType) -> basic_ops32::Type {
        
        todo!();
        /*
            Type v[numParallel]; storeU (v, a); return jmin (v[0], v[1], v[2], v[3]);
        */
    }
}
