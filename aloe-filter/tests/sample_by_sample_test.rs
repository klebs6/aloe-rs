crate::ix!();

pub struct FIRFilterTestSampleBySampleTest { }

impl FIRFilterTestSampleBySampleTest {

    pub fn run<FloatType>(
        filter: &mut FIRFilter<FloatType>,
        src:    *mut FloatType,
        dst:    *mut FloatType,
        n:      usize)  {
    
        todo!();
        /*
            for (size_t i = 0; i < n; ++i)
                    dst[i] = filter.processSample (src[i]);
        */
    }
}
