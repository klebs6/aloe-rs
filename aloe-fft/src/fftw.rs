crate::ix!();

#[cfg(any(ALOE_DSP_USE_SHARED_FFTW,ALOE_DSP_USE_STATIC_FFTW))]
pub struct FFTWImpl {
    base:         FftInstance,
    fftw_library: DynamicLibrary,
    fftw:         FftwSymbols,
    order:        usize,
    c2c_forward:  FFTWPlanRef,
    c2c_inverse:  FFTWPlanRef,
    r2c:          FFTWPlanRef,
    c2r:          FFTWPlanRef,
}

#[cfg(any(ALOE_DSP_USE_SHARED_FFTW,ALOE_DSP_USE_STATIC_FFTW))]
pub type FFTWPlanRef = *mut FFTWPlan;

#[cfg(any(ALOE_DSP_USE_SHARED_FFTW,ALOE_DSP_USE_STATIC_FFTW))]
pub mod fftw_impl {

    use super::*;

    /**
      | if the Aloe developer has gone through the
      | hassle of statically linking in fftw, they
      | probably want to use it
      */
    #[cfg(ALOE_DSP_USE_STATIC_FFTW)]
    pub const FFTW_PRIORITY: i32 = 10;

    #[cfg(not(ALOE_DSP_USE_STATIC_FFTW))]
    pub const FFTW_PRIORITY: i32 = 3;

    pub const FFTW_MEASURE:   usize = 0;
    pub const FFTW_UNALIGNED: usize = (1 << 1);
    pub const FFTW_ESTIMATE:  usize = (1 << 6);
}

#[cfg(any(ALOE_DSP_USE_SHARED_FFTW,ALOE_DSP_USE_STATIC_FFTW))]
impl Drop for FFTWImpl {

    fn drop(&mut self) {
        todo!();
        /* 
            ScopedLock lock (getFFTWPlanLock());

            fftw.destroy_fftw (c2cForward);
            fftw.destroy_fftw (c2cInverse);
            fftw.destroy_fftw (r2c);
            fftw.destroy_fftw (c2r);
         */
    }
}

#[cfg(any(ALOE_DSP_USE_SHARED_FFTW,ALOE_DSP_USE_STATIC_FFTW))]
impl FFTWImpl {

    pub fn create(order: i32) -> *mut FFTWImpl {
        
        todo!();
        /*
            DynamicLibrary lib;

          #if ! ALOE_DSP_USE_STATIC_FFTW
           #if ALOE_MAC
            auto libName = "libfftw3f.dylib";
           #elif ALOE_WINDOWS
            auto libName = "libfftw3f.dll";
           #else
            auto libName = "libfftw3f.so";
           #endif

            if (lib.open (libName))
          #endif
            {
                FftwSymbols symbols;

               #if ALOE_DSP_USE_STATIC_FFTW
                if (! FftwSymbols::symbol (symbols.plan_dft_fftw, fftwf_plan_dft_1d))     return nullptr;
                if (! FftwSymbols::symbol (symbols.plan_r2c_fftw, fftwf_plan_dft_r2c_1d)) return nullptr;
                if (! FftwSymbols::symbol (symbols.plan_c2r_fftw, fftwf_plan_dft_c2r_1d)) return nullptr;
                if (! FftwSymbols::symbol (symbols.destroy_fftw,  fftwf_destroy_plan))    return nullptr;

                if (! FftwSymbols::symbol (symbols.execute_dft_fftw, fftwf_execute_dft))     return nullptr;
                if (! FftwSymbols::symbol (symbols.execute_r2c_fftw, fftwf_execute_dft_r2c)) return nullptr;
                if (! FftwSymbols::symbol (symbols.execute_c2r_fftw, fftwf_execute_dft_c2r)) return nullptr;
               #else
                if (! FftwSymbols::symbol (lib, symbols.plan_dft_fftw, "fftwf_plan_dft_1d"))     return nullptr;
                if (! FftwSymbols::symbol (lib, symbols.plan_r2c_fftw, "fftwf_plan_dft_r2c_1d")) return nullptr;
                if (! FftwSymbols::symbol (lib, symbols.plan_c2r_fftw, "fftwf_plan_dft_c2r_1d")) return nullptr;
                if (! FftwSymbols::symbol (lib, symbols.destroy_fftw,  "fftwf_destroy_plan"))    return nullptr;

                if (! FftwSymbols::symbol (lib, symbols.execute_dft_fftw, "fftwf_execute_dft"))     return nullptr;
                if (! FftwSymbols::symbol (lib, symbols.execute_r2c_fftw, "fftwf_execute_dft_r2c")) return nullptr;
                if (! FftwSymbols::symbol (lib, symbols.execute_c2r_fftw, "fftwf_execute_dft_c2r")) return nullptr;
               #endif

                return new FFTWImpl (static_cast<size_t> (order), std::move (lib), symbols);
            }

            return nullptr;
        */
    }
    
    pub fn new(
        order_to_use:   usize,
        library_to_use: DynamicLibrary,
        symbols:        &FftwSymbols) -> Self {
    
        todo!();
        /*


            : fftwLibrary (std::move (libraryToUse)), fftw (symbols), order (static_cast<size_t> (orderToUse))

            ScopedLock lock (getFFTWPlanLock());

            auto n = (1u << order);
            HeapBlock<Complex<float>> in (n), out (n);

            c2cForward = fftw.plan_dft_fftw (n, in.getData(), out.getData(), -1, unaligned | estimate);
            c2cInverse = fftw.plan_dft_fftw (n, in.getData(), out.getData(), +1, unaligned | estimate);

            r2c = fftw.plan_r2c_fftw (n, (float*) in.getData(), in.getData(), unaligned | estimate);
            c2r = fftw.plan_c2r_fftw (n, in.getData(), (float*) in.getData(), unaligned | estimate);
        */
    }
    
    pub fn perform(&self, 
        input:   *const Complex<f32>,
        output:  *mut Complex<f32>,
        inverse: bool)  {
        
        todo!();
        /*
            if (inverse)
            {
                auto n = (1u << order);
                fftw.execute_dft_fftw (c2cInverse, input, output);
                FloatVectorOperations::multiply ((float*) output, 1.0f / static_cast<float> (n), (int) n << 1);
            }
            else
            {
                fftw.execute_dft_fftw (c2cForward, input, output);
            }
        */
    }
    
    pub fn perform_real_only_forward_transform(&self, 
        input_output_data:     *mut f32,
        ignore_negative_freqs: bool)  {
        
        todo!();
        /*
            if (order == 0)
                return;

            auto* out = reinterpret_cast<Complex<float>*> (inputOutputData);

            fftw.execute_r2c_fftw (r2c, inputOutputData, out);

            auto size = (1 << order);

            if (! ignoreNegativeFreqs)
                for (int i = size >> 1; i < size; ++i)
                    out[i] = std::conj (out[size - i]);
        */
    }
    
    pub fn perform_real_only_inverse_transform(&self, input_output_data: *mut f32)  {
        
        todo!();
        /*
            auto n = (1u << order);

            fftw.execute_c2r_fftw (c2r, (Complex<float>*) inputOutputData, inputOutputData);
            FloatVectorOperations::multiply ((float*) inputOutputData, 1.0f / static_cast<float> (n), (int) n);
        */
    }

    /**
       fftw's plan_* and destroy_* methods are NOT
       thread safe. So we need to share a lock
       between all instances of FFTWImpl
      */
    pub fn get_fftw_plan_lock() -> &mut CriticalSection {
        
        todo!();
        /*
            static CriticalSection cs;
            return cs;
        */
    }
}

#[cfg(any(ALOE_DSP_USE_SHARED_FFTW,ALOE_DSP_USE_STATIC_FFTW))]
lazy_static!{
    /*
    FFT::FftEngineImpl<FFTWImpl> fftwEngine;
    */
}
