crate::ix!();

#[cfg(all(any(target_os="macos",target_os="ios"),ALOE_USE_VDSP_FRAMEWORK))]
pub const APPLE_FFT_PRIORITY: i32 = 5;

#[cfg(all(any(target_os="macos",target_os="ios"),ALOE_USE_VDSP_FRAMEWORK))]
pub struct AppleFFT {
    base:                  FFT::FftInstance,
    order:                 vDSP_Length,
    fft_setup:             FFTSetup,
    forward_normalisation: f32,
    inverse_normalisation: f32,
}

#[cfg(all(any(target_os="macos",target_os="ios"),ALOE_USE_VDSP_FRAMEWORK))]
impl Drop for AppleFFT {

    fn drop(&mut self) {
        todo!();
        /* 
            if (fftSetup != nullptr)
            {
                vDSP_destroy_fftsetup (fftSetup);
                fftSetup = nullptr;
            }
         */
    }
}

#[cfg(all(any(target_os="macos",target_os="ios"),ALOE_USE_VDSP_FRAMEWORK))]
impl AppleFFT {

    pub fn create(order: i32) -> *mut AppleFFT {
        
        todo!();
        /*
            return new AppleFFT (order);
        */
    }
    
    pub fn new(order_to_use: i32) -> Self {
    
        todo!();
        /*


            : order (static_cast<vDSP_Length> (orderToUse)),
              fftSetup (vDSP_create_fftsetup (order, 2)),
              forwardNormalisation (0.5f),
              inverseNormalisation (1.0f / static_cast<float> (1 << order))
        */
    }

    pub fn perform(&self, 
        input:   *const Complex<f32>,
        output:  *mut Complex<f32>,
        inverse: bool)  {
        
        todo!();
        /*
            auto size = (1 << order);

            DSPSplitComplex splitInput  (toSplitComplex (const_cast<Complex<float>*> (input)));
            DSPSplitComplex splitOutput (toSplitComplex (output));

            vDSP_fft_zop (fftSetup, &splitInput,  2, &splitOutput, 2,
                          order, inverse ?  kFFTDirection_Inverse : kFFTDirection_Forward);

            float factor = (inverse ? inverseNormalisation : forwardNormalisation * 2.0f);
            vDSP_vsmul ((float*) output, 1, &factor, (float*) output, 1, static_cast<size_t> (size << 1));
        */
    }

    pub fn perform_real_only_forward_transform(&self, 
        inout_data:            *mut f32,
        ignore_negative_freqs: bool)  {
        
        todo!();
        /*
            auto size = (1 << order);
            auto* inout = reinterpret_cast<Complex<float>*> (inoutData);
            auto splitInOut (toSplitComplex (inout));

            inoutData[size] = 0.0f;
            vDSP_fft_zrip (fftSetup, &splitInOut, 2, order, kFFTDirection_Forward);
            vDSP_vsmul (inoutData, 1, &forwardNormalisation, inoutData, 1, static_cast<size_t> (size << 1));

            mirrorResult (inout, ignoreNegativeFreqs);
        */
    }

    pub fn perform_real_only_inverse_transform(&self, inout_data: *mut f32)  {
        
        todo!();
        /*
            auto* inout = reinterpret_cast<Complex<float>*> (inoutData);
            auto size = (1 << order);
            auto splitInOut (toSplitComplex (inout));

            // Imaginary part of nyquist and DC frequencies are always zero
            // so Apple uses the imaginary part of the DC frequency to store
            // the real part of the nyquist frequency
            if (size != 1)
                inout[0] = Complex<float> (inout[0].real(), inout[size >> 1].real());

            vDSP_fft_zrip (fftSetup, &splitInOut, 2, order, kFFTDirection_Inverse);
            vDSP_vsmul (inoutData, 1, &inverseNormalisation, inoutData, 1, static_cast<size_t> (size << 1));
            vDSP_vclr (inoutData + size, 1, static_cast<size_t> (size));
        */
    }

    pub fn mirror_result(&self, 
        out:                   *mut Complex<f32>,
        ignore_negative_freqs: bool)  {
        
        todo!();
        /*
            auto size = (1 << order);
            auto i = size >> 1;

            // Imaginary part of nyquist and DC frequencies are always zero
            // so Apple uses the imaginary part of the DC frequency to store
            // the real part of the nyquist frequency
            out[i++] = { out[0].imag(), 0.0 };
            out[0]   = { out[0].real(), 0.0 };

            if (! ignoreNegativeFreqs)
                for (; i < size; ++i)
                    out[i] = std::conj (out[size - i]);
        */
    }

    pub fn to_split_complex(data: *mut Complex<f32>) -> DSPSplitComplex {
        
        todo!();
        /*
            // this assumes that Complex interleaves real and imaginary parts
            // and is tightly packed.
            return { reinterpret_cast<float*> (data),
                     reinterpret_cast<float*> (data) + 1};
        */
    }
}
    
#[cfg(all(any(target_os="macos",target_os="ios"),ALOE_USE_VDSP_FRAMEWORK))]
lazy_static!{
    /*
    FFT::FftEngineImpl<AppleFFT> appleFFT;
    */
}
