crate::ix!();

#[cfg(ALOE_DSP_USE_INTEL_MKL)]
pub struct IntelFFT {
    base:  FftInstance,
    order: usize,
    c2c:   DFTI_DESCRIPTOR_HANDLE,
    c2r:   DFTI_DESCRIPTOR_HANDLE,
}

#[cfg(ALOE_DSP_USE_INTEL_MKL)]
pub const INTEL_FFT_PRIORITY: i32 = 8;

#[cfg(ALOE_DSP_USE_INTEL_MKL)]
impl Drop for IntelFFT {

    fn drop(&mut self) {
        todo!();
        /* 
            DftiFreeDescriptor (&c2c);
            DftiFreeDescriptor (&c2r);
         */
    }
}

#[cfg(ALOE_DSP_USE_INTEL_MKL)]
impl IntelFFT {

    pub fn succeeded(status: MKL_LONG) -> bool {
        
        todo!();
        /*
            return status == 0;
        */
    }
    
    pub fn create(order_to_use: i32) -> *mut IntelFFT {
        
        todo!();
        /*
            DFTI_DESCRIPTOR_HANDLE mklc2c, mklc2r;

            if (DftiCreateDescriptor (&mklc2c, DFTI_SINGLE, DFTI_COMPLEX, 1, 1 << orderToUse) == 0)
            {
                if (succeeded (DftiSetValue (mklc2c, DFTI_PLACEMENT, DFTI_NOT_INPLACE))
                     && succeeded (DftiSetValue (mklc2c, DFTI_BACKWARD_SCALE, 1.0f / static_cast<float> (1 << orderToUse)))
                     && succeeded (DftiCommitDescriptor (mklc2c)))
                {
                    if (succeeded (DftiCreateDescriptor (&mklc2r, DFTI_SINGLE, DFTI_REAL, 1, 1 << orderToUse)))
                    {
                        if (succeeded (DftiSetValue (mklc2r, DFTI_PLACEMENT, DFTI_INPLACE))
                             && succeeded (DftiSetValue (mklc2r, DFTI_BACKWARD_SCALE, 1.0f / static_cast<float> (1 << orderToUse)))
                             && succeeded (DftiCommitDescriptor (mklc2r)))
                        {
                            return new IntelFFT (static_cast<size_t> (orderToUse), mklc2c, mklc2r);
                        }

                        DftiFreeDescriptor (&mklc2r);
                    }
                }

                DftiFreeDescriptor (&mklc2c);
            }

            return {};
        */
    }
    
    pub fn new(
        order_to_use: usize,
        c2c_to_use:   DFTI_DESCRIPTOR_HANDLE,
        cr_2to_use:   DFTI_DESCRIPTOR_HANDLE) -> Self {
    
        todo!();
        /*
        : order(orderToUse),
        : c2c(c2cToUse),
        : c2r(cr2ToUse),

        
        */
    }

    pub fn perform(&self, 
        input:   *const Complex<f32>,
        output:  *mut Complex<f32>,
        inverse: bool)  {
        
        todo!();
        /*
            if (inverse)
                DftiComputeBackward (c2c, (void*) input, output);
            else
                DftiComputeForward (c2c, (void*) input, output);
        */
    }

    pub fn perform_real_only_forward_transform(&self, 
        input_output_data:     *mut f32,
        ignore_negative_freqs: bool)  {
        
        todo!();
        /*
            if (order == 0)
                return;

            DftiComputeForward (c2r, inputOutputData);

            auto* out = reinterpret_cast<Complex<float>*> (inputOutputData);
            auto size = (1 << order);

            if (! ignoreNegativeFreqs)
                for (int i = size >> 1; i < size; ++i)
                    out[i] = std::conj (out[size - i]);
        */
    }

    pub fn perform_real_only_inverse_transform(&self, input_output_data: *mut f32)  {
        
        todo!();
        /*
            DftiComputeBackward (c2r, inputOutputData);
        */
    }
}

#[cfg(ALOE_DSP_USE_INTEL_MKL)]
lazy_static!{
    /*
    FFT::FftEngineImpl<IntelFFT> fftwEngine;
    */
}
