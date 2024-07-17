crate::ix!();

#[cfg(ALOE_USE_ARM_NEON)]
pub struct BasicOps32 {

}

#[cfg(ALOE_USE_ARM_NEON)]
pub mod basic_ops32 {
    use super::*;

    pub type Type         = f32;
    pub type ParallelType = float32x4_t;
    pub type IntegerType  = uint32x4_t;

    pub union signMaskUnion { 
        f: ParallelType,
        i: IntegerType,
    }

    pub const numParallel: usize = 4;
}

#[cfg(ALOE_USE_ARM_NEON)]
pub mod basic_ops32 {
    use super::*;

    pub type Type         = f32;
    pub type ParallelType = __m128;
    pub type IntegerType  = __m128;

    pub const numParallel: usize = 4;
}

#[cfg(ALOE_USE_ARM_NEON)]
impl BasicOps32 {

    #[inline(always)] pub fn toint(v: basic_ops32::ParallelType) -> basic_ops32::IntegerType {
        
        todo!();
        /*
            signMaskUnion u; u.f = v; return u.i;
        */
    }
    
    #[inline(always)] pub fn toflt(v: basic_ops32::IntegerType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            signMaskUnion u; u.i = v; return u.f;
        */
    }
    
    #[inline(always)] pub fn load1(v: basic_ops64::Type) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return vld1q_dup_f32 (&v);
        */
    }
    
    #[inline(always)] pub fn loada(v: *const basic_ops64::Type) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return vld1q_f32 (v);
        */
    }
    
    #[inline(always)] pub fn loadu(v: *const basic_ops64::Type) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return vld1q_f32 (v);
        */
    }
    
    #[inline(always)] pub fn storea(
        dest: *mut basic_ops64::Type,
        a:    basic_ops32::ParallelType)  {
        
        todo!();
        /*
            vst1q_f32 (dest, a);
        */
    }
    
    #[inline(always)] pub fn storeu(
        dest: *mut basic_ops64::Type,
        a:    basic_ops32::ParallelType)  {
        
        todo!();
        /*
            vst1q_f32 (dest, a);
        */
    }
    
    #[inline(always)] pub fn add(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return vaddq_f32 (a, b);
        */
    }
    
    #[inline(always)] pub fn sub(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return vsubq_f32 (a, b);
        */
    }
    
    #[inline(always)] pub fn mul(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return vmulq_f32 (a, b);
        */
    }
    
    #[inline(always)] pub fn max(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return vmaxq_f32 (a, b);
        */
    }
    
    #[inline(always)] pub fn min(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return vminq_f32 (a, b);
        */
    }
    
    #[inline(always)] pub fn bit_and(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return toflt (vandq_u32 (toint (a), toint (b)));
        */
    }
    
    #[inline(always)] pub fn bit_not(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return toflt (vbicq_u32 (toint (a), toint (b)));
        */
    }
    
    #[inline(always)] pub fn bit_or(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return toflt (vorrq_u32 (toint (a), toint (b)));
        */
    }
    
    #[inline(always)] pub fn bit_xor(
        a: basic_ops32::ParallelType,
        b: basic_ops32::ParallelType) -> basic_ops32::ParallelType {
        
        todo!();
        /*
            return toflt (veorq_u32 (toint (a), toint (b)));
        */
    }
    
    #[inline(always)] pub fn max(a: basic_ops32::ParallelType) -> basic_ops64::Type {
        
        todo!();
        /*
            Type v[numParallel]; storeU (v, a); return jmax (v[0], v[1], v[2], v[3]);
        */
    }
    
    #[inline(always)] pub fn min(a: basic_ops32::ParallelType) -> basic_ops64::Type {
        
        todo!();
        /*
            Type v[numParallel]; storeU (v, a); return jmin (v[0], v[1], v[2], v[3]);
        */
    }
}

