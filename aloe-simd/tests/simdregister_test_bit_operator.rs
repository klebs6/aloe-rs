crate::ix!();

pub struct BitOperatorTests<Operation> {

}

impl BitOperatorTests<Operation> {
    
    pub fn run<type>(
        u:      &mut UnitTest,
        random: &mut Random)  {
    
        todo!();
        /*
            typedef typename SIMDRegister<type>::vMaskType vMaskType;
                typedef typename SIMDRegister<type>::MaskType MaskType;

                for (int n = 0; n < 100; ++n)
                {
                    // Check flip sign bit and using as a union
                    {
                        type array_a [SIMDRegister<type>::SIMDNumElements];

                        union ConversionUnion
                        {
                            inline ConversionUnion() : floatVersion (static_cast<type> (0)) {}
                            inline ~ConversionUnion() {}
                            SIMDRegister<type> floatVersion;
                            vMaskType intVersion;
                        } a, b;

                        vMaskType bitmask = vMaskType::expand (static_cast<MaskType> (1) << (sizeof (MaskType) - 1));
                        SIMDRegister_test_internal::VecFiller<type>::fill (array_a, SIMDRegister<type>::SIMDNumElements, random);
                        copy (a.floatVersion, array_a);
                        copy (b.floatVersion, array_a);

                        Operation::template inplace<SIMDRegister<type>, vMaskType> (a.floatVersion, bitmask);
                        Operation::template inplace<vMaskType, vMaskType> (b.intVersion, bitmask);

                       #ifdef _MSC_VER
                        __declspec(align(sizeof (SIMDRegister<type>))) type elements[SIMDRegister<type>::SIMDNumElements];
                       #else
                        type elements[SIMDRegister<type>::SIMDNumElements] __attribute__((aligned(sizeof (SIMDRegister<type>))));
                       #endif
                        b.floatVersion.copyToRawArray (elements);

                        u.expect (vecEqualToArray (a.floatVersion, elements));
                    }

                    // set-up
                    SIMDRegister<type> a, c;
                    vMaskType b;

                    MaskType array_a [SIMDRegister<MaskType>::SIMDNumElements];
                    MaskType array_b [SIMDRegister<MaskType>::SIMDNumElements];
                    MaskType array_c [SIMDRegister<MaskType>::SIMDNumElements];

                    type float_a [SIMDRegister<type>::SIMDNumElements];
                    type float_c [SIMDRegister<type>::SIMDNumElements];

                    SIMDRegister_test_internal::VecFiller<type>::fill (float_a, SIMDRegister<type>::SIMDNumElements, random);
                    SIMDRegister_test_internal::VecFiller<MaskType>::fill (array_b, SIMDRegister<MaskType>::SIMDNumElements, random);
                    SIMDRegister_test_internal::VecFiller<type>::fill (float_c, SIMDRegister<type>::SIMDNumElements, random);

                    memcpy (array_a, float_a, sizeof (type) * SIMDRegister<type>::SIMDNumElements);
                    memcpy (array_c, float_c, sizeof (type) * SIMDRegister<type>::SIMDNumElements);
                    copy (a, float_a); copy (b, array_b); copy (c, float_c);

                    // test in-place with both params being vectors
                    for (size_t i = 0; i < SIMDRegister<MaskType>::SIMDNumElements; ++i)
                        Operation::template inplace<MaskType, MaskType> (array_a[i], array_b[i]);
                    memcpy (float_a, array_a, sizeof (type) * SIMDRegister<type>::SIMDNumElements);

                    Operation::template inplace<SIMDRegister<type>, vMaskType> (a, b);

                    u.expect (vecEqualToArray (a, float_a));
                    u.expect (vecEqualToArray (b, array_b));

                    SIMDRegister_test_internal::VecFiller<type>::fill (float_a, SIMDRegister<type>::SIMDNumElements, random);
                    SIMDRegister_test_internal::VecFiller<MaskType>::fill (array_b, SIMDRegister<MaskType>::SIMDNumElements, random);
                    SIMDRegister_test_internal::VecFiller<type>::fill (float_c, SIMDRegister<type>::SIMDNumElements, random);
                    memcpy (array_a, float_a, sizeof (type) * SIMDRegister<type>::SIMDNumElements);
                    memcpy (array_c, float_c, sizeof (type) * SIMDRegister<type>::SIMDNumElements);
                    copy (a, float_a); copy (b, array_b); copy (c, float_c);

                    // test in-place with one param being scalar
                    for (size_t i = 0; i < SIMDRegister<MaskType>::SIMDNumElements; ++i)
                        Operation::template inplace<MaskType, MaskType> (array_a[i], static_cast<MaskType> (9));
                    memcpy (float_a, array_a, sizeof (type) * SIMDRegister<type>::SIMDNumElements);

                    Operation::template inplace<SIMDRegister<type>, MaskType> (a, static_cast<MaskType> (9));

                    u.expect (vecEqualToArray (a, float_a));
                    u.expect (vecEqualToArray (b, array_b));

                    // set-up again
                    SIMDRegister_test_internal::VecFiller<type>::fill (float_a, SIMDRegister<type>::SIMDNumElements, random);
                    SIMDRegister_test_internal::VecFiller<MaskType>::fill (array_b, SIMDRegister<MaskType>::SIMDNumElements, random);
                    SIMDRegister_test_internal::VecFiller<type>::fill (float_c, SIMDRegister<type>::SIMDNumElements, random);
                    memcpy (array_a, float_a, sizeof (type) * SIMDRegister<type>::SIMDNumElements);
                    memcpy (array_c, float_c, sizeof (type) * SIMDRegister<type>::SIMDNumElements);
                    copy (a, float_a); copy (b, array_b); copy (c, float_c);

                    // test out-of-place with both params being vectors
                    for (size_t i = 0; i < SIMDRegister<MaskType>::SIMDNumElements; ++i)
                    {
                        array_c[i] =
                            Operation::template outofplace<MaskType, MaskType> (array_a[i], array_b[i]);
                    }
                    memcpy (float_a, array_a, sizeof (type) * SIMDRegister<type>::SIMDNumElements);
                    memcpy (float_c, array_c, sizeof (type) * SIMDRegister<type>::SIMDNumElements);

                    c = Operation::template outofplace<SIMDRegister<type>, vMaskType> (a, b);

                    u.expect (vecEqualToArray (a, float_a));
                    u.expect (vecEqualToArray (b, array_b));
                    u.expect (vecEqualToArray (c, float_c));

                    // test out-of-place with one param being scalar
                    for (size_t i = 0; i < SIMDRegister<MaskType>::SIMDNumElements; ++i)
                        array_c[i] = Operation::template outofplace<MaskType, MaskType> (array_a[i], static_cast<MaskType> (9));
                    memcpy (float_a, array_a, sizeof (type) * SIMDRegister<type>::SIMDNumElements);
                    memcpy (float_c, array_c, sizeof (type) * SIMDRegister<type>::SIMDNumElements);

                    c = Operation::template outofplace<SIMDRegister<type>, MaskType> (a, static_cast<MaskType> (9));

                    u.expect (vecEqualToArray (a, float_a));
                    u.expect (vecEqualToArray (b, array_b));
                    u.expect (vecEqualToArray (c, float_c));
                }
        */
    }
}
