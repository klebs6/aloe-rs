crate::ix!();

pub struct CheckMinMax {

}

impl CheckMinMax {
    
    pub fn run<type>(
        u:      &mut UnitTest,
        random: &mut Random)  {
    
        todo!();
        /*
            for (int i = 0; i < 100; ++i)
                {
                    type array_a [SIMDRegister<type>::SIMDNumElements];
                    type array_b [SIMDRegister<type>::SIMDNumElements];
                    type array_min [SIMDRegister<type>::SIMDNumElements];
                    type array_max [SIMDRegister<type>::SIMDNumElements];

                    for (size_t j = 0; j < SIMDRegister<type>::SIMDNumElements; ++j)
                    {
                        array_a[j] = static_cast<type> (random.nextInt (127));
                        array_b[j] = static_cast<type> (random.nextInt (127));
                    }

                    for (size_t j = 0; j < SIMDRegister<type>::SIMDNumElements; ++j)
                    {
                        array_min[j] = (array_a[j] < array_b[j]) ? array_a[j] : array_b[j];
                        array_max[j] = (array_a[j] > array_b[j]) ? array_a[j] : array_b[j];
                    }

                    SIMDRegister<type> a (static_cast<type> (0));
                    SIMDRegister<type> b (static_cast<type> (0));
                    SIMDRegister<type> vMin (static_cast<type> (0));
                    SIMDRegister<type> vMax (static_cast<type> (0));

                    copy (a, array_a);
                    copy (b, array_b);

                    vMin = jmin (a, b);
                    vMax = jmax (a, b);

                    u.expect (vecEqualToArray (vMin, array_min));
                    u.expect (vecEqualToArray (vMax, array_max));

                    copy (vMin, array_a);
                    copy (vMax, array_a);

                    vMin = SIMDRegister<type>::min (a, b);
                    vMax = SIMDRegister<type>::max (a, b);

                    u.expect (vecEqualToArray (vMin, array_min));
                    u.expect (vecEqualToArray (vMax, array_max));
                }
        */
    }
}
