crate::ix!();

/**
  | Helper class providing an RAII-based
  | mechanism for temporarily disabling denormals
  | on your CPU.
  |
  | @tags{Audio}
  */
pub struct ScopedNoDenormals {
    #[cfg(any(
            ALOE_USE_SSE_INTRINSICS,
            ALOE_USE_ARM_NEON,
            __arm64__,
            __aarch64__,
    ))]
    fpsr: libc::intptr_t,
}

/**
  | This casts away constness to account for
  | slightly different vDSP function signatures
  | in OSX 10.8 SDK and below. Can be safely
  | removed once those SDKs are obsolete.
  */
#[cfg(ALOE_USE_VDSP_FRAMEWORK)]
pub fn osx_108sdk_compatibility_cast<ValueType>(arg: *const ValueType) -> *mut ValueType {

    todo!();
    /*
        return const_cast<ValueType*> (arg);
    */
}


///--------------------
impl Drop for ScopedNoDenormals {
    fn drop(&mut self) {
        todo!();
        /* 
      #if ALOE_USE_SSE_INTRINSICS || (ALOE_USE_ARM_NEON || defined (__arm64__) || defined (__aarch64__))
        FloatVectorOperations::setFpStatusRegister (fpsr);
      #endif
 */
    }
}

impl ScopedNoDenormals {

    pub fn new() -> Self {
    
        todo!();
        /*


            #if ALOE_USE_SSE_INTRINSICS || (ALOE_USE_ARM_NEON || defined (__arm64__) || defined (__aarch64__))
       #if ALOE_USE_SSE_INTRINSICS
        intptr_t mask = 0x8040;
       #else /*ALOE_USE_ARM_NEON*/
        intptr_t mask = (1 << 24 /* FZ */);
       #endif

        fpsr = FloatVectorOperations::getFpStatusRegister();
        FloatVectorOperations::setFpStatusRegister (fpsr | mask);
      #endif
        */
    }
}
