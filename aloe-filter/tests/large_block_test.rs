crate::ix!();

pub struct FIRFilterTestLargeBlockTest { }

impl FIRFilterTestLargeBlockTest {

    pub fn run<FloatType>(
        filter: &mut FIRFilter<FloatType>,
        src:    *mut FloatType,
        dst:    *mut FloatType,
        n:      usize

    ) {
    
        todo!();
        /*
            AudioBlock<const FloatType> input (&src, 1, n);
                AudioBlock<FloatType> output (&dst, 1, n);
                ProcessContextNonReplacing<FloatType> context (input, output);

                filter.process (context);
        */
    }
}
