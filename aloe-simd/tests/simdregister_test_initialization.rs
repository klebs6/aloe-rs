crate::ix!();

pub struct InitializationTest {

}

impl InitializationTest {
    
    pub fn run<type>(
        u:      &mut UnitTest,
        random: &mut Random)  {
    
        todo!();
        /*
            u.expect (allValuesEqualTo<type> (SIMDRegister<type>::expand (static_cast<type> (23)), 23));

                {
                   #ifdef _MSC_VER
                    __declspec(align(sizeof (SIMDRegister<type>))) type elements[SIMDRegister<type>::SIMDNumElements];
                   #else
                    type elements[SIMDRegister<type>::SIMDNumElements] __attribute__((aligned(sizeof (SIMDRegister<type>))));
                   #endif
                    SIMDRegister_test_internal::VecFiller<type>::fill (elements, SIMDRegister<type>::SIMDNumElements, random);
                    SIMDRegister<type> a (SIMDRegister<type>::fromRawArray (elements));

                    u.expect (vecEqualToArray (a, elements));

                    SIMDRegister<type> b (a);
                    a *= static_cast<type> (2);

                    u.expect (vecEqualToArray (b, elements));
                }
        */
    }
}
