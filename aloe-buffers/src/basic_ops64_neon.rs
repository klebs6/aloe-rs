crate::ix!();

#[cfg(ALOE_USE_ARM_NEON)]
pub struct BasicOps64 {

}

#[cfg(ALOE_USE_ARM_NEON)]
pub mod basic_ops64 {
    use super::*;

    pub type Type         = f64;
    pub type ParallelType = f64;
    pub type IntegerType  = u64;

    pub union signMaskUnion { 
        f: ParallelType,
        i: IntegerType,
    }

    pub const numParallel: usize = 1;
}

#[cfg(ALOE_USE_ARM_NEON)]
pub mod basic_ops64 {
    use super::*;

    pub type Type         = f64;
    pub type ParallelType = __m128d;
    pub type IntegerType  = __m128d;

    pub const numParallel: usize = 2;
}

#[cfg(ALOE_USE_ARM_NEON)]
impl BasicOps64 {
    
    #[inline(always)] pub fn toint(v: basic_ops64::ParallelType) -> basic_ops64::IntegerType {
        
        todo!();
        /*
            signMaskUnion u; u.f = v; return u.i;
        */
    }
    
    #[inline(always)] pub fn toflt(v: basic_ops64::IntegerType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            signMaskUnion u; u.i = v; return u.f;
        */
    }
    
    #[inline(always)] pub fn load1(v: basic_ops64::Type) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return v;
        */
    }
    
    #[inline(always)] pub fn loada(v: *const basic_ops64::Type) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return *v;
        */
    }
    
    #[inline(always)] pub fn loadu(v: *const basic_ops64::Type) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return *v;
        */
    }
    
    #[inline(always)] pub fn storea(
        dest: *mut basic_ops64::Type,
        a:    basic_ops64::ParallelType)  {
        
        todo!();
        /*
            *dest = a;
        */
    }
    
    #[inline(always)] pub fn storeu(
        dest: *mut basic_ops64::Type,
        a:    basic_ops64::ParallelType)  {
        
        todo!();
        /*
            *dest = a;
        */
    }
    
    #[inline(always)] pub fn add(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return a + b;
        */
    }
    
    #[inline(always)] pub fn sub(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return a - b;
        */
    }
    
    #[inline(always)] pub fn mul(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return a * b;
        */
    }
    
    #[inline(always)] pub fn max(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return jmax (a, b);
        */
    }
    
    #[inline(always)] pub fn min(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return jmin (a, b);
        */
    }
    
    #[inline(always)] pub fn bit_and(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return toflt (toint (a) & toint (b));
        */
    }
    
    #[inline(always)] pub fn bit_not(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return toflt ((~toint (a)) & toint (b));
        */
    }
    
    #[inline(always)] pub fn bit_or(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return toflt (toint (a) | toint (b));
        */
    }
    
    #[inline(always)] pub fn bit_xor(
        a: basic_ops64::ParallelType,
        b: basic_ops64::ParallelType) -> basic_ops64::ParallelType {
        
        todo!();
        /*
            return toflt (toint (a) ^ toint (b));
        */
    }
    
    #[inline(always)] pub fn max(a: basic_ops64::ParallelType) -> basic_ops64::Type {
        
        todo!();
        /*
            return a;
        */
    }
    
    #[inline(always)] pub fn min(a: basic_ops64::ParallelType) -> basic_ops64::Type {
        
        todo!();
        /*
            return a;
        */
    }
}

