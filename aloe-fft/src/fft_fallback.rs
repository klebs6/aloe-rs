crate::ix!();

/**
  | this should have the least priority
  | of all engines
  |
  */
pub const FFT_FALLBACK_PRIORITY: i32 = -1;

pub const FFT_FALLBACK_MAX_FFT_SCRATCH_SPACE_TO_ALLOCA: usize = 256 * 1024;

pub struct FFTFallback {
    process_lock:   SpinLock,
    config_forward: Box<FFTConfig>,
    config_inverse: Box<FFTConfig>,
    size:           i32,
}

impl FftInstance for FFTFallback {

    fn perform(&self, 
        input:   *const Complex<f32>,
        output:  *mut Complex<f32>,
        inverse: bool)  {
        
        todo!();
        /*
            if (size == 1)
            {
                *output = *input;
                return;
            }

            const SpinLock::ScopedLockType sl(processLock);

            jassert (configForward != nullptr);

            if (inverse)
            {
                configInverse->perform (input, output);

                const float scaleFactor = 1.0f / (float) size;

                for (int i = 0; i < size; ++i)
                    output[i] *= scaleFactor;
            }
            else
            {
                configForward->perform (input, output);
            }
        */
    }
    
    fn perform_real_only_forward_transform(&self, 
        d:  *mut f32,
        _1: bool)  {
        
        todo!();
        /*
            if (size == 1)
                return;

            const size_t scratchSize = 16 + (size_t) size * sizeof (Complex<float>);

            if (scratchSize < maxFFTScratchSpaceToAlloca)
            {
                ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6255)
                performRealOnlyForwardTransform (static_cast<Complex<float>*> (alloca (scratchSize)), d);
                ALOE_END_IGNORE_WARNINGS_MSVC
            }
            else
            {
                HeapBlock<char> heapSpace (scratchSize);
                performRealOnlyForwardTransform (unalignedPointerCast<Complex<float>*> (heapSpace.getData()), d);
            }
        */
    }
    
    fn perform_real_only_inverse_transform(&self, d: *mut f32)  {
        
        todo!();
        /*
            if (size == 1)
                return;

            const size_t scratchSize = 16 + (size_t) size * sizeof (Complex<float>);

            if (scratchSize < maxFFTScratchSpaceToAlloca)
            {
                ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6255)
                performRealOnlyInverseTransform (static_cast<Complex<float>*> (alloca (scratchSize)), d);
                ALOE_END_IGNORE_WARNINGS_MSVC
            }
            else
            {
                HeapBlock<char> heapSpace (scratchSize);
                performRealOnlyInverseTransform (unalignedPointerCast<Complex<float>*> (heapSpace.getData()), d);
            }
        */
    }
}

impl FFTFallback {

    pub fn create(order: i32) -> *mut FFTFallback {
        
        todo!();
        /*
            return new FFTFallback (order);
        */
    }

    pub fn new(order: i32) -> Self {
    
        todo!();
        /*


            configForward.reset (new FFTConfig (1 << order, false));
            configInverse.reset (new FFTConfig (1 << order, true));

            size = 1 << order;
        */
    }

    
    pub fn perform_real_only_forward_transform_with_scratch(
        &self, 
        scratch: *mut Complex<f32>,
        d:       *mut f32)  {
        
        todo!();
        /*
            for (int i = 0; i < size; ++i)
                scratch[i] = { d[i], 0 };

            perform (scratch, reinterpret_cast<Complex<float>*> (d), false);
        */
    }
    
    pub fn perform_real_only_inverse_transform_with_scratch(
        &self, 
        scratch: *mut Complex<f32>,
        d:       *mut f32

    ) {
        
        todo!();
        /*
            auto* input = reinterpret_cast<Complex<float>*> (d);

            for (int i = size >> 1; i < size; ++i)
                input[i] = std::conj (input[size - i]);

            perform (input, scratch, true);

            for (int i = 0; i < size; ++i)
            {
                d[i] = scratch[i].real();
                d[i + size] = scratch[i].imag();
            }
        */
    }
}

lazy_static!{
    /*
    FFT::FftEngineImpl<FFTFallback> fftFallback;
    */
}
