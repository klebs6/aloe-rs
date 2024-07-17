crate::ix!();

pub struct OperatorTests<Operation> {

}

impl OperatorTests<Operation> {
    
    pub fn run<type>(
        u:      &mut UnitTest,
        random: &mut Random)  {
    
        todo!();
        /*
            for (int n = 0; n < 100; ++n)
                {
                    // set-up
                    SIMDRegister<type> a (static_cast<type> (0));
                    SIMDRegister<type> b (static_cast<type> (0));
                    SIMDRegister<type> c (static_cast<type> (0));

                    type array_a [SIMDRegister<type>::SIMDNumElements];
                    type array_b [SIMDRegister<type>::SIMDNumElements];
                    type array_c [SIMDRegister<type>::SIMDNumElements];

                    SIMDRegister_test_internal::VecFiller<type>::fill (array_a, SIMDRegister<type>::SIMDNumElements, random);
                    SIMDRegister_test_internal::VecFiller<type>::fill (array_b, SIMDRegister<type>::SIMDNumElements, random);
                    SIMDRegister_test_internal::VecFiller<type>::fill (array_c, SIMDRegister<type>::SIMDNumElements, random);

                    copy (a, array_a); copy (b, array_b); copy (c, array_c);

                    // test in-place with both params being vectors
                    for (size_t i = 0; i < SIMDRegister<type>::SIMDNumElements; ++i)
                        Operation::template inplace<type, type> (array_a[i], array_b[i]);

                    Operation::template inplace<SIMDRegister<type>, SIMDRegister<type>> (a, b);

                    u.expect (vecEqualToArray (a, array_a));
                    u.expect (vecEqualToArray (b, array_b));

                    SIMDRegister_test_internal::VecFiller<type>::fill (array_a, SIMDRegister<type>::SIMDNumElements, random);
                    SIMDRegister_test_internal::VecFiller<type>::fill (array_b, SIMDRegister<type>::SIMDNumElements, random);
                    SIMDRegister_test_internal::VecFiller<type>::fill (array_c, SIMDRegister<type>::SIMDNumElements, random);

                    copy (a, array_a); copy (b, array_b); copy (c, array_c);

                    // test in-place with one param being scalar
                    for (size_t i = 0; i < SIMDRegister<type>::SIMDNumElements; ++i)
                        Operation::template inplace<type, type> (array_b[i], static_cast<type> (2));

                    Operation::template inplace<SIMDRegister<type>, type> (b, 2);

                    u.expect (vecEqualToArray (a, array_a));
                    u.expect (vecEqualToArray (b, array_b));

                    // set-up again
                    SIMDRegister_test_internal::VecFiller<type>::fill (array_a, SIMDRegister<type>::SIMDNumElements, random);
                    SIMDRegister_test_internal::VecFiller<type>::fill (array_b, SIMDRegister<type>::SIMDNumElements, random);
                    SIMDRegister_test_internal::VecFiller<type>::fill (array_c, SIMDRegister<type>::SIMDNumElements, random);
                    copy (a, array_a); copy (b, array_b); copy (c, array_c);

                    // test out-of-place with both params being vectors
                    for (size_t i = 0; i < SIMDRegister<type>::SIMDNumElements; ++i)
                        array_c[i] = Operation::template outofplace<type, type> (array_a[i], array_b[i]);

                    c = Operation::template outofplace<SIMDRegister<type>, SIMDRegister<type>> (a, b);

                    u.expect (vecEqualToArray (a, array_a));
                    u.expect (vecEqualToArray (b, array_b));
                    u.expect (vecEqualToArray (c, array_c));

                    // test out-of-place with one param being scalar
                    for (size_t i = 0; i < SIMDRegister<type>::SIMDNumElements; ++i)
                        array_c[i] = Operation::template outofplace<type, type> (array_b[i], static_cast<type> (2));

                    c = Operation::template outofplace<SIMDRegister<type>, type> (b, 2);

                    u.expect (vecEqualToArray (a, array_a));
                    u.expect (vecEqualToArray (b, array_b));
                    u.expect (vecEqualToArray (c, array_c));
                }
        */
    }
}
