crate::ix!();

pub struct CheckTruncate {

}

impl CheckTruncate {
    
    pub fn run<type>(
        u:      &mut UnitTest,
        random: &mut Random)  {
    
        todo!();
        /*
            type inArray[SIMDRegister<type>::SIMDNumElements];
                type outArray[SIMDRegister<type>::SIMDNumElements];

                SIMDRegister_test_internal::VecFiller<type>::fill (inArray, SIMDRegister<type>::SIMDNumElements, random);

                SIMDRegister<type> a;
                copy (a, inArray);
                a = SIMDRegister<type>::truncate (a);

                for (size_t j = 0; j < SIMDRegister<type>::SIMDNumElements; ++j)
                    outArray[j] = (type) (int) inArray[j];

                u.expect (vecEqualToArray (a, outArray));
        */
    }
}
