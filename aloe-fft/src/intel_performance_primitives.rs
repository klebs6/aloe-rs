crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/frequency/aloe_FFT.cpp]

/**
  | Visual Studio should define no more than one of
  | these, depending on the setting at 'Project'
  | > 'Properties' > 'Configuration Properties'
  | > 'Intel Performance Libraries' > 'Use Intel(R)
  | IPP'
  */
#[cfg(any(any(_IPP_SEQUENTIAL_STATIC,_IPP_SEQUENTIAL_DYNAMIC),any(_IPP_PARALLEL_STATIC,_IPP_PARALLEL_DYNAMIC)))]
pub struct IntelPerformancePrimitivesFFT {
    base:  FFT::FftInstance,
    cplx:  Context<ComplexTraits>,
    real:  Context<RealTraits>,
    order: i32, // default = 0
}

#[cfg(any(any(_IPP_SEQUENTIAL_STATIC,_IPP_SEQUENTIAL_DYNAMIC),any(_IPP_PARALLEL_STATIC,_IPP_PARALLEL_DYNAMIC)))]
pub mod intel_performance_primitives_fft {
    use super::*;

    pub const priority: Auto = 9;
    pub const flag:     Auto = IPP_FFT_DIV_INV_BY_N;
    pub const hint:     Auto = ippAlgHintFast;

    pub struct IppFree { }

    impl IppFree {

        pub fn invoke<Ptr>(&self, ptr: *mut Ptr)  {
        
            todo!();
            /*
                ippsFree (ptr);
            */
        }
    }

    pub type IppPtr = Box<&[Ipp8u],IppFree>;

    #[derive(Default)]
    pub struct Context<Traits> {
        spec_buf: IppPtr,
        work_buf: IppPtr,
        spec_ptr: SpecPtr, // default = nullptr
    }

    impl HasSpecPtr for Context<Traits> {
        type SpecPtr = *mut Traits::Spec;
    }

    impl Context<Traits> {

        pub fn create(order: i32) -> Context {
            
            todo!();
            /*
                int specSize = 0, initSize = 0, workSize = 0;

                    if (Traits::getSize (order, flag, hint, &specSize, &initSize, &workSize) != ippStsNoErr)
                        return {};

                    const auto initBuf = IppPtr (ippsMalloc_8u (initSize));
                    auto specBuf       = IppPtr (ippsMalloc_8u (specSize));
                    SpecPtr specPtr = nullptr;

                    if (Traits::init (&specPtr, order, flag, hint, specBuf.get(), initBuf.get()) != ippStsNoErr)
                        return {};

                    if (reinterpret_cast<const Ipp8u*> (specPtr) != specBuf.get())
                        return {};

                    return { std::move (specBuf), IppPtr (ippsMalloc_8u (workSize)), specPtr };
            */
        }
        
        pub fn new(
            spec: IppPtr,
            work: IppPtr,
            ptr:  *mut Traits::Spec) -> Self {
        
            todo!();
            /*


                : specBuf (std::move (spec)), workBuf (std::move (work)), specPtr (ptr)
            */
        }
        
        pub fn is_valid(&self) -> bool {
            
            todo!();
            /*
                return specPtr != nullptr;
            */
        }
    }

    pub mod complex_traits {
        use super::*;
        pub const getSize: Auto = ippsFFTGetSize_C_32fc;
        pub const init:    Auto = ippsFFTInit_C_32fc;
        pub type Spec = IppsFFTSpec_C_32fc;
    }

    pub mod real_traits {

        use super::*;
        pub const getSize: Auto = ippsFFTGetSize_R_32f;
        pub const init:    Auto = ippsFFTInit_R_32f;
        pub type Spec = IppsFFTSpec_R_32f;
    }
}

#[cfg(any(any(_IPP_SEQUENTIAL_STATIC,_IPP_SEQUENTIAL_DYNAMIC),any(_IPP_PARALLEL_STATIC,_IPP_PARALLEL_DYNAMIC)))]
impl IntelPerformancePrimitivesFFT {

    pub fn create(order: i32) -> *mut IntelPerformancePrimitivesFFT {
        
        todo!();
        /*
            auto complexContext = Context<ComplexTraits>::create (order);
            auto realContext    = Context<RealTraits>   ::create (order);

            if (complexContext.isValid() && realContext.isValid())
                return new IntelPerformancePrimitivesFFT (std::move (complexContext), std::move (realContext), order);

            return {};
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
                ippsFFTInv_CToC_32fc (reinterpret_cast<const Ipp32fc*> (input),
                                      reinterpret_cast<Ipp32fc*> (output),
                                      cplx.specPtr,
                                      cplx.workBuf.get());
            }
            else
            {
                ippsFFTFwd_CToC_32fc (reinterpret_cast<const Ipp32fc*> (input),
                                      reinterpret_cast<Ipp32fc*> (output),
                                      cplx.specPtr,
                                      cplx.workBuf.get());
            }
        */
    }
    
    pub fn perform_real_only_forward_transform(&self, 
        inout_data:            *mut f32,
        ignore_negative_freqs: bool)  {
        
        todo!();
        /*
            ippsFFTFwd_RToCCS_32f_I (inoutData, real.specPtr, real.workBuf.get());

            if (order == 0)
                return;

            auto* out = reinterpret_cast<Complex<float>*> (inoutData);
            const auto size = (1 << order);

            if (! ignoreNegativeFreqs)
                for (auto i = size >> 1; i < size; ++i)
                    out[i] = std::conj (out[size - i]);
        */
    }
    
    pub fn perform_real_only_inverse_transform(&self, inout_data: *mut f32)  {
        
        todo!();
        /*
            ippsFFTInv_CCSToR_32f_I (inoutData, real.specPtr, real.workBuf.get());
        */
    }
    
    pub fn new(
        complex_to_use: Context<ComplexTraits>,
        real_to_use:    Context<RealTraits>,
        order_to_use:   i32) -> Self {
    
        todo!();
        /*


            : cplx (std::move (complexToUse)),
              real (std::move (realToUse)),
              order (orderToUse)
        */
    }
}

#[cfg(any(any(_IPP_SEQUENTIAL_STATIC,_IPP_SEQUENTIAL_DYNAMIC),any(_IPP_PARALLEL_STATIC,_IPP_PARALLEL_DYNAMIC)))]
lazy_static!{
    /*
    FFT::FftEngineImpl<IntelPerformancePrimitivesFFT> intelPerformancePrimitivesFFT;
    */
}
