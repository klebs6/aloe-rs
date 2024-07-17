crate::ix!();

#[cfg(any(ALOE_DSP_USE_SHARED_FFTW,ALOE_DSP_USE_STATIC_FFTW))]
#[cfg(ALOE_DSP_USE_STATIC_FFTW)]
extern "C"
{
    pub fn fftwf_plan_dft_1d(
            _0: i32,
            _1: *mut c_void,
            _2: *mut c_void,
            _3: i32,
            _4: i32)  {
        
        todo!();
        /*
        
        */
    }

    pub fn fftwf_plan_dft_r2c_1d(
            _0: i32,
            _1: *mut c_void,
            _2: *mut c_void,
            _3: i32)  {
        
        todo!();
        /*
        
        */
    }

    pub fn fftwf_plan_dft_c2r_1d(
            _0: i32,
            _1: *mut c_void,
            _2: *mut c_void,
            _3: i32)  {
        
        todo!();
        /*
        
        */
    }

    pub fn fftwf_destroy_plan(_0: *mut c_void)  {
        
        todo!();
        /*
        
        */
    }

    pub fn fftwf_execute_dft(
            _0: *mut c_void,
            _1: *mut c_void,
            _2: *mut c_void)  {
        
        todo!();
        /*
        
        */
    }

    pub fn fftwf_execute_dft_r2c(
            _0: *mut c_void,
            _1: *mut c_void,
            _2: *mut c_void)  {
        
        todo!();
        /*
        
        */
    }

    pub fn fftwf_execute_dft_c2r(
            _0: *mut c_void,
            _1: *mut c_void,
            _2: *mut c_void)  {
        
        todo!();
        /*
        
        */
    }
}
