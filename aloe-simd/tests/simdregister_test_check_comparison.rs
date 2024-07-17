crate::ix!();

pub struct CheckComparisonOps {

}

impl CheckComparisonOps {

    pub fn run<type>(
        u:      &mut UnitTest,
        random: &mut Random)  {
    
        todo!();
        /*
            typedef typename SIMDRegister<type>::vMaskType vMaskType;
                typedef typename SIMDRegister<type>::MaskType MaskType;

                for (int i = 0; i < 100; ++i)
                {
                    // set-up
                    type array_a   [SIMDRegister<type>::SIMDNumElements];
                    type array_b   [SIMDRegister<type>::SIMDNumElements];
                    MaskType array_eq  [SIMDRegister<type>::SIMDNumElements];
                    MaskType array_neq [SIMDRegister<type>::SIMDNumElements];
                    MaskType array_lt  [SIMDRegister<type>::SIMDNumElements];
                    MaskType array_le  [SIMDRegister<type>::SIMDNumElements];
                    MaskType array_gt  [SIMDRegister<type>::SIMDNumElements];
                    MaskType array_ge  [SIMDRegister<type>::SIMDNumElements];


                    SIMDRegister_test_internal::VecFiller<type>::fill (array_a, SIMDRegister<type>::SIMDNumElements, random);
                    SIMDRegister_test_internal::VecFiller<type>::fill (array_b, SIMDRegister<type>::SIMDNumElements, random);

                    // do check
                    for (size_t j = 0; j < SIMDRegister<type>::SIMDNumElements; ++j)
                    {
                        array_eq  [j] = (array_a[j] == array_b[j]) ? static_cast<MaskType> (-1) : 0;
                        array_neq [j] = (array_a[j] != array_b[j]) ? static_cast<MaskType> (-1) : 0;
                        array_lt  [j] = (array_a[j] <  array_b[j]) ? static_cast<MaskType> (-1) : 0;
                        array_le  [j] = (array_a[j] <= array_b[j]) ? static_cast<MaskType> (-1) : 0;
                        array_gt  [j] = (array_a[j] >  array_b[j]) ? static_cast<MaskType> (-1) : 0;
                        array_ge  [j] = (array_a[j] >= array_b[j]) ? static_cast<MaskType> (-1) : 0;
                    }

                    SIMDRegister<type> a (static_cast<type> (0));
                    SIMDRegister<type> b (static_cast<type> (0));

                    vMaskType eq, neq, lt, le, gt, ge;

                    copy (a, array_a);
                    copy (b, array_b);

                    eq  = SIMDRegister<type>::equal              (a, b);
                    neq = SIMDRegister<type>::notEqual           (a, b);
                    lt  = SIMDRegister<type>::lessThan           (a, b);
                    le  = SIMDRegister<type>::lessThanOrEqual    (a, b);
                    gt  = SIMDRegister<type>::greaterThan        (a, b);
                    ge  = SIMDRegister<type>::greaterThanOrEqual (a, b);

                    u.expect (vecEqualToArray (eq,  array_eq ));
                    u.expect (vecEqualToArray (neq, array_neq));
                    u.expect (vecEqualToArray (lt,  array_lt ));
                    u.expect (vecEqualToArray (le,  array_le ));
                    u.expect (vecEqualToArray (gt,  array_gt ));
                    u.expect (vecEqualToArray (ge,  array_ge ));

                    do
                    {
                        SIMDRegister_test_internal::VecFiller<type>::fill (array_a, SIMDRegister<type>::SIMDNumElements, random);
                        SIMDRegister_test_internal::VecFiller<type>::fill (array_b, SIMDRegister<type>::SIMDNumElements, random);
                    } while (std::equal (array_a, array_a + SIMDRegister<type>::SIMDNumElements, array_b));

                    copy (a, array_a);
                    copy (b, array_b);
                    u.expect (a != b);
                    u.expect (b != a);
                    u.expect (! (a == b));
                    u.expect (! (b == a));

                    SIMDRegister_test_internal::VecFiller<type>::fill (array_a, SIMDRegister<type>::SIMDNumElements, random);
                    copy (a, array_a);
                    copy (b, array_a);

                    u.expect (a == b);
                    u.expect (b == a);
                    u.expect (! (a != b));
                    u.expect (! (b != a));

                    type scalar = a[0];
                    a = SIMDRegister<type>::expand (scalar);

                    u.expect (a == scalar);
                    u.expect (! (a != scalar));

                    scalar--;

                    u.expect (a != scalar);
                    u.expect (! (a == scalar));
                }
        */
    }
}
