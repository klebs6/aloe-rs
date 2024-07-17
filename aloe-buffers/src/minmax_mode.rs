crate::ix!();

///-------------------------
#[cfg(any(ALOE_USE_SSE_INTRINSICS,ALOE_USE_ARM_NEON))]
pub struct MinMax<Mode> {

}

#[cfg(any(ALOE_USE_SSE_INTRINSICS,ALOE_USE_ARM_NEON))]
impl HasType for MinMax<Mode> {
    type Type = Mode::Type;
}

#[cfg(any(ALOE_USE_SSE_INTRINSICS,ALOE_USE_ARM_NEON))]
impl HasParallelType for MinMax<Mode> {
    type ParallelType = Mode::ParallelType;
}

#[cfg(any(ALOE_USE_SSE_INTRINSICS,ALOE_USE_ARM_NEON))]
impl MinMax<Mode> {

    pub fn find_min_or_max(
        src:        *const Type,
        num:        i32,
        is_minimum: bool) -> Type {
        
        todo!();
        /*
            int numLongOps = num / Mode::numParallel;

                if (numLongOps > 1)
                {
                    ParallelType val;

                   #if ! ALOE_USE_ARM_NEON
                    if (isAligned (src))
                    {
                        val = Mode::loadA (src);

                        if (isMinimum)
                        {
                            while (--numLongOps > 0)
                            {
                                src += Mode::numParallel;
                                val = Mode::min (val, Mode::loadA (src));
                            }
                        }
                        else
                        {
                            while (--numLongOps > 0)
                            {
                                src += Mode::numParallel;
                                val = Mode::max (val, Mode::loadA (src));
                            }
                        }
                    }
                    else
                   #endif
                    {
                        val = Mode::loadU (src);

                        if (isMinimum)
                        {
                            while (--numLongOps > 0)
                            {
                                src += Mode::numParallel;
                                val = Mode::min (val, Mode::loadU (src));
                            }
                        }
                        else
                        {
                            while (--numLongOps > 0)
                            {
                                src += Mode::numParallel;
                                val = Mode::max (val, Mode::loadU (src));
                            }
                        }
                    }

                    Type result = isMinimum ? Mode::min (val)
                                            : Mode::max (val);

                    num &= (Mode::numParallel - 1);
                    src += Mode::numParallel;

                    for (int i = 0; i < num; ++i)
                        result = isMinimum ? jmin (result, src[i])
                                           : jmax (result, src[i]);

                    return result;
                }

                return isMinimum ? aloe::findMinimum (src, num)
                                 : aloe::findMaximum (src, num);
        */
    }
    
    pub fn find_min_and_max(
        src: *const Type,
        num: i32) -> Range<Type> {
        
        todo!();
        /*
            int numLongOps = num / Mode::numParallel;

                if (numLongOps > 1)
                {
                    ParallelType mn, mx;

                   #if ! ALOE_USE_ARM_NEON
                    if (isAligned (src))
                    {
                        mn = Mode::loadA (src);
                        mx = mn;

                        while (--numLongOps > 0)
                        {
                            src += Mode::numParallel;
                            const ParallelType v = Mode::loadA (src);
                            mn = Mode::min (mn, v);
                            mx = Mode::max (mx, v);
                        }
                    }
                    else
                   #endif
                    {
                        mn = Mode::loadU (src);
                        mx = mn;

                        while (--numLongOps > 0)
                        {
                            src += Mode::numParallel;
                            const ParallelType v = Mode::loadU (src);
                            mn = Mode::min (mn, v);
                            mx = Mode::max (mx, v);
                        }
                    }

                    Range<Type> result (Mode::min (mn),
                                        Mode::max (mx));

                    num &= (Mode::numParallel - 1);
                    src += Mode::numParallel;

                    for (int i = 0; i < num; ++i)
                        result = result.getUnionWith (src[i]);

                    return result;
                }

                return Range<Type>::findMinAndMax (src, num);
        */
    }
}
