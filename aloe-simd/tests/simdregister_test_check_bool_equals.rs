crate::ix!();

pub struct CheckBoolEquals {

}

impl CheckBoolEquals {
    
    pub fn run<type>(
        u:      &mut UnitTest,
        random: &mut Random)  {
    
        todo!();
        /*
            bool is_signed = std::is_signed<type>::value;
            type array [SIMDRegister<type>::SIMDNumElements];

            auto value = is_signed ? static_cast<type> ((random.nextFloat() * 16.0) - 8.0)
                                   : static_cast<type> (random.nextFloat() * 8.0);

            std::fill (array, array + SIMDRegister<type>::SIMDNumElements, value);
            SIMDRegister<type> a, b;
            copy (a, array);

            u.expect (a == value);
            u.expect (! (a != value));
            value += 1;

            u.expect (a != value);
            u.expect (! (a == value));

            SIMDRegister_test_internal::VecFiller<type>::fill (array, SIMDRegister<type>::SIMDNumElements, random);
            copy (a, array);
            copy (b, array);

            u.expect (a == b);
            u.expect (! (a != b));

            SIMDRegister_test_internal::VecFiller<type>::fill (array, SIMDRegister<type>::SIMDNumElements, random);
            copy (b, array);

            u.expect (a != b);
            u.expect (! (a == b));
        */
    }
}
