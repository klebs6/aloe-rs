crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/aloe_dsp.h]

/**
  | Use this function to prevent denormals
  | on intel CPUs.
  | 
  | This function will work with both primitives
  | and simple containers.
  |
  */
pub fn snap_to_zero<T>(x: &mut T) where T: num::Float {

    todo!();

    /*
      #if ALOE_DSP_ENABLE_SNAP_TO_ZERO
        inline void snapToZero (float&       x) noexcept            { ALOE_SNAP_TO_ZERO (x); }
       #ifndef DOXYGEN
        inline void snapToZero (double&      x) noexcept            { ALOE_SNAP_TO_ZERO (x); }
        inline void snapToZero (long double& x) noexcept            { ALOE_SNAP_TO_ZERO (x); }
       #endif
      #else
        inline void snapToZero (float&       x) noexcept            { ignoreUnused (x); }
       #ifndef DOXYGEN
        inline void snapToZero (double&      x) noexcept            { ignoreUnused (x); }
        inline void snapToZero (long double& x) noexcept            { ignoreUnused (x); }
       #endif
      #endif
    */
}

