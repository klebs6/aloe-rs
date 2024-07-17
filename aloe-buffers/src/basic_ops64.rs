crate::ix!();

#[cfg(ALOE_USE_SSE_INTRINSICS)]
pub struct BasicOps64 {

}

#[cfg(ALOE_USE_SSE_INTRINSICS)]
impl BasicOps64 {

    /*
       Integer and parallel types are the same for
       SSE. On neon they have different types
      */
    
    #[inline(always)] pub fn toint(v: basic_ops64::ParallelType) -> basic_ops64::IntegerType {
        
        todo!();
        /*
            return v;
        */
    }
    
    #[inline(always)] pub fn toflt(v: basic_ops64::IntegerType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return v;
        */
    }
    
    #[inline(always)] pub fn load1(v: basic_ops64::Type) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return _mm_load1_pd (&v);
        */
    }
    
    #[inline(always)] pub fn loada(v: *const basic_ops64::Type) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return _mm_load_pd (v);
        */
    }
    
    #[inline(always)] pub fn loadu(v: *const basic_ops64::Type) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return _mm_loadu_pd (v);
        */
    }
    
    #[inline(always)] pub fn storea(
        dest: *mut basic_ops64::Type,
        a:    basic_ops64::ParallelType)  {
        
        todo!();
        /*
            _mm_store_pd (dest, a);
        */
    }
    
    #[inline(always)] pub fn storeu(
        dest: *mut basic_ops64::Type,
        a:    basic_ops64::ParallelType)  {
        
        todo!();
        /*
            _mm_storeu_pd (dest, a);
        */
    }
    
    #[inline(always)] pub fn add(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return _mm_add_pd (a, b);
        */
    }
    
    #[inline(always)] pub fn sub(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return _mm_sub_pd (a, b);
        */
    }
    
    #[inline(always)] pub fn mul(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return _mm_mul_pd (a, b);
        */
    }
    
    #[inline(always)] pub fn max(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return _mm_max_pd (a, b);
        */
    }
    
    #[inline(always)] pub fn min(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return _mm_min_pd (a, b);
        */
    }
    
    #[inline(always)] pub fn bit_and(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return _mm_and_pd (a, b);
        */
    }
    
    #[inline(always)] pub fn bit_not(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return _mm_andnot_pd (a, b);
        */
    }
    
    #[inline(always)] pub fn bit_or(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return _mm_or_pd (a, b);
        */
    }
    
    #[inline(always)] pub fn bit_xor(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return _mm_xor_pd (a, b);
        */
    }
    
    #[inline(always)] pub fn max(a: basic_ops64::ParallelType) -> basic_ops64::Type {
        
        todo!();
        /*
            Type v[numParallel]; storeU (v, a); return jmax (v[0], v[1]);
        */
    }
    
    #[inline(always)] pub fn min(a: basic_ops64::ParallelType) -> basic_ops64::Type {
        
        todo!();
        /*
            Type v[numParallel]; storeU (v, a); return jmin (v[0], v[1]);
        */
    }
}
