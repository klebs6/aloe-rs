crate::ix!();

pub struct CheckMultiplyAdd {

}

impl CheckMultiplyAdd {
    
    pub fn run<type>(
        u:      &mut UnitTest,
        random: &mut Random)  {
    
        todo!();
        /*
            // set-up
                type array_a [SIMDRegister<type>::SIMDNumElements];
                type array_b [SIMDRegister<type>::SIMDNumElements];
                type array_c [SIMDRegister<type>::SIMDNumElements];
                type array_d [SIMDRegister<type>::SIMDNumElements];

                SIMDRegister_test_internal::VecFiller<type>::fill (array_a, SIMDRegister<type>::SIMDNumElements, random);
                SIMDRegister_test_internal::VecFiller<type>::fill (array_b, SIMDRegister<type>::SIMDNumElements, random);
                SIMDRegister_test_internal::VecFiller<type>::fill (array_c, SIMDRegister<type>::SIMDNumElements, random);
                SIMDRegister_test_internal::VecFiller<type>::fill (array_d, SIMDRegister<type>::SIMDNumElements, random);

                // check
                for (size_t i = 0; i < SIMDRegister<type>::SIMDNumElements; ++i)
                    array_d[i] = array_a[i] + (array_b[i] * array_c[i]);

                SIMDRegister<type> a, b, c, d;

                copy (a, array_a);
                copy (b, array_b);
                copy (c, array_c);

                d = SIMDRegister<type>::multiplyAdd (a, b, c);

                u.expect (vecEqualToArray (d, array_d));
        */
    }
}
