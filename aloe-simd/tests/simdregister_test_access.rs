crate::ix!();

pub struct AccessTest {

}

impl AccessTest {
    
    pub fn run<type>(
        u:      &mut UnitTest,
        random: &mut Random)  {
    
        todo!();
        /*
            // set-up
                SIMDRegister<type> a;
                type array [SIMDRegister<type>::SIMDNumElements];

                SIMDRegister_test_internal::VecFiller<type>::fill (array, SIMDRegister<type>::SIMDNumElements, random);

                // Test non-const access operator
                for (size_t i = 0; i < SIMDRegister<type>::SIMDNumElements; ++i)
                    a[i] = array[i];

                u.expect (vecEqualToArray (a, array));

                // Test const access operator
                const SIMDRegister<type>& b = a;

                for (size_t i = 0; i < SIMDRegister<type>::SIMDNumElements; ++i)
                    u.expect (b[i] == array[i]);
        */
    }
}
