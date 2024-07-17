crate::ix!();

pub struct CheckAbs {

}

impl CheckAbs {
    
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
            a = SIMDRegister<type>::abs (a);

            auto calcAbs = [] (type x) -> type { return x >= type (0) ? x : type (-x); };

            for (size_t j = 0; j < SIMDRegister<type>::SIMDNumElements; ++j)
                outArray[j] = calcAbs (inArray[j]);

            u.expect (vecEqualToArray (a, outArray));
        */
    }
}
