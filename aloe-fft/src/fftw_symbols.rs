crate::ix!();

#[cfg(any(ALOE_DSP_USE_SHARED_FFTW,ALOE_DSP_USE_STATIC_FFTW))]
pub struct FftwSymbols {

    plan_dft_fftw:    fn(_0: u32, 
                         _1: *mut Complex<f32>, 
                         _2: *mut Complex<f32>, 
                         _3: i32, 
                         _4: u32) -> FFTWPlanRef,

    plan_r2c_fftw:    fn(_0: u32, 
                         _1: *mut f32, 
                         _2: *mut Complex<f32>, 
                         _3: u32) -> FFTWPlanRef,

    plan_c2r_fftw:    fn(_0: u32, 
                         _1: *mut Complex<f32>, 
                         _2: *mut f32, 
                         _3: u32) -> FFTWPlanRef,

    destroy_fftw:     fn(_0: FFTWPlanRef) -> c_void,

    execute_dft_fftw: fn(_0: FFTWPlanRef, 
                         _1: *const Complex<f32>, 
                         _2: *mut Complex<f32>) -> c_void,

    execute_r2c_fftw: fn(_0: FFTWPlanRef, 
                         _1: *mut f32, 
                         _2: *mut Complex<f32>) -> c_void,

    execute_c2r_fftw: fn(_0: FFTWPlanRef, 
                         _1: *mut Complex<f32>, 
                         _2: *mut f32) -> c_void,
}

#[cfg(any(ALOE_DSP_USE_SHARED_FFTW,ALOE_DSP_USE_STATIC_FFTW))]
impl FftwSymbols {

    #[cfg(ALOE_DSP_USE_STATIC_FFTW)]
    pub fn symbol<FuncPtr, ActualSymbolType>(
        dst: &mut FuncPtr,
        sym: ActualSymbolType) -> bool {
    
        todo!();
        /*
            dst = reinterpret_cast<FuncPtr> (sym);
                return true;
        */
    }

    #[cfg(not(ALOE_DSP_USE_STATIC_FFTW))]
    pub fn symbol<FuncPtr>(
        lib:  &mut DynamicLibrary,
        dst:  &mut FuncPtr,
        name: *const u8) -> bool {
    
        todo!();
        /*
            dst = reinterpret_cast<FuncPtr> (lib.getFunction (name));
                return (dst != nullptr);
        */
    }
}
