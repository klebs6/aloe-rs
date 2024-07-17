crate::ix!();

pub struct FIRFilterTestHelpers<Type> { }

impl FIRFilterTestHelpers<Type> {

    pub fn fill_random(
        random: &mut Random,
        buffer: *mut Type,
        n:      usize)  {
        
        todo!();
        /*
            for (size_t i = 0; i < n; ++i)
                    buffer[i] = (2.0f * random.nextFloat()) - 1.0f;
        */
    }
    
    pub fn check_array_is_similar(
        a: *mut Type,
        b: *mut Type,
        n: usize) -> bool {
        
        todo!();
        /*
            for (size_t i = 0; i < n; ++i)
                    if (std::abs (a[i] - b[i]) > 1e-6f)
                        return false;

                return true;
        */
    }
}

#[cfg(ALOE_USE_SIMD)]
pub struct FIRFilterTestHelpers<T: SIMDRegister> { }

impl<T: SIMDRegister> FIRFilterTestHelpers<T> {

    pub fn fill_random(
        random: &mut Random,
        buffer: *mut T,
        n:      usize)  {
        
        todo!();
        /*
            FIRFilterTestHelpers<Type>::fillRandom (random, reinterpret_cast<Type*> (buffer), n * SIMDRegister<Type>::size());
        */
    }
    
    pub fn check_array_is_similar(
        a: *mut T,
        b: *mut T,
        n: usize) -> bool {
        
        todo!();
        /*
            return FIRFilterTestHelpers<Type>::checkArrayIsSimilar (reinterpret_cast<Type*> (a),
                                                           reinterpret_cast<Type*> (b),
                                                           n * SIMDRegister<Type>::size());
        */
    }
}

