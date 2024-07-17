crate::ix!();

pub struct CheckSum {

}

impl CheckSum {
    
    pub fn run<type>(
        u:      &mut UnitTest,
        random: &mut Random)  {
    
        todo!();
        /*
            type array [SIMDRegister<type>::SIMDNumElements];
                type sumCheck = 0;

                SIMDRegister_test_internal::VecFiller<type>::fill (array, SIMDRegister<type>::SIMDNumElements, random);

                for (size_t j = 0; j < SIMDRegister<type>::SIMDNumElements; ++j)
                {
                    sumCheck += array[j];
                }

                SIMDRegister<type> a;
                copy (a, array);

                u.expect (SIMDRegister_test_internal::difference (sumCheck, a.sum()) < 1e-4);
        */
    }
}
