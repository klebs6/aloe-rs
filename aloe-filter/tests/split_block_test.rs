crate::ix!();

pub struct FIRFilterTestSplitBlockTest { }

impl FIRFilterTestSplitBlockTest {

    pub fn run<FloatType>(
        filter: &mut FIRFilter<FloatType>,
        input:  *mut FloatType,
        output: *mut FloatType,
        n:      usize)  {
    
        todo!();
        /*
            size_t len = 0;
                for (size_t i = 0; i < n; i += len)
                {
                    len = jmin (n - i, n / 3);
                    auto* src = input + i;
                    auto* dst = output + i;

                    AudioBlock<const FloatType> inBlock (&src, 1, len);
                    AudioBlock<FloatType> outBlock (&dst, 1, len);
                    ProcessContextNonReplacing<FloatType> context (inBlock, outBlock);

                    filter.process (context);
                }
        */
    }
}
