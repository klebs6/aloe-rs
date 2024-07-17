
fn main() {
    eprintln!("build.rs is running!");
}

/*
#if defined(_M_X64) || defined(__amd64__) || defined(__SSE2__) || (defined(_M_IX86_FP) && _M_IX86_FP == 2)

 #if defined(_M_X64) || defined(__amd64__)
  #ifndef __SSE2__
   #define __SSE2__
  #endif
 #endif

 #ifndef ALOE_USE_SIMD
  #define ALOE_USE_SIMD 1
 #endif

 #if ALOE_USE_SIMD
  #include <immintrin.h>
 #endif

#elif defined (__ARM_NEON__) || defined (__ARM_NEON) || defined (__arm64__) || defined (__aarch64__)

 #ifndef ALOE_USE_SIMD
  #define ALOE_USE_SIMD 1
 #endif

 #include <arm_neon.h>

#else

 // No SIMD Support
 #ifndef ALOE_USE_SIMD
  #define ALOE_USE_SIMD 0
 #endif

#endif

#ifndef ALOE_VECTOR_CALLTYPE
 // __vectorcall does not work on 64-bit due to internal compiler error in
 // release mode in both VS2015 and VS2017. Re-enable when Microsoft fixes this
 #if _MSC_VER && ALOE_USE_SIMD && ! (defined(_M_X64) || defined(__amd64__))
  #define ALOE_VECTOR_CALLTYPE __vectorcall
 #else
  #define ALOE_VECTOR_CALLTYPE
 #endif
#endif

#include <complex>


//==============================================================================
/** Config: ALOE_ASSERTION_FIRFILTER

    When this flag is enabled, an assertion will be generated during the
    execution of DEBUG configurations if you use a FIRFilter class to process
    FIRCoefficients with a size higher than 128, to tell you that's it would be
    more efficient to use the Convolution class instead. It is enabled by
    default, but you may want to disable it if you really want to process such
    a filter in the time domain.
*/
#ifndef ALOE_ASSERTION_FIRFILTER
 #define ALOE_ASSERTION_FIRFILTER 1
#endif

/** Config: ALOE_DSP_USE_INTEL_MKL

    If this flag is set, then Aloe will use Intel's MKL for Aloe's FFT and
    convolution classes.

    The folder containing the mkl_dfti.h header must be in your header
    search paths when using this flag. You also need to add all the necessary
    intel mkl libraries to the "External Libraries to Link" field in the
    Proaloer.
*/
#ifndef ALOE_DSP_USE_INTEL_MKL
 #define ALOE_DSP_USE_INTEL_MKL 0
#endif

/** Config: ALOE_DSP_USE_SHARED_FFTW

    If this flag is set, then Aloe will search for the fftw shared libraries
    at runtime and use the library for Aloe's FFT and convolution classes.

    If the library is not found, then Aloe's fallback FFT routines will be used.

    This is especially useful on linux as fftw often comes pre-installed on
    popular linux distros.

    You must respect the FFTW license when enabling this option.
*/
 #ifndef ALOE_DSP_USE_SHARED_FFTW
 #define ALOE_DSP_USE_SHARED_FFTW 0
#endif

/** Config: ALOE_DSP_USE_STATIC_FFTW

    If this flag is set, then Aloe will use the statically linked fftw libraries
    for Aloe's FFT and convolution classes.

    You must add the fftw header/library folder to the extra header/library search
    paths of your Aloe project. You also need to add the fftw library itself
    to the extra libraries supplied to your Aloe project during linking.

    You must respect the FFTW license when enabling this option.
*/
#ifndef ALOE_DSP_USE_STATIC_FFTW
 #define ALOE_DSP_USE_STATIC_FFTW 0
#endif

/** Config: ALOE_DSP_ENABLE_SNAP_TO_ZERO

    Enables code in the dsp module to avoid floating point denormals during the
    processing of some of the dsp module's filters.

    Enabling this will add a slight performance overhead to the DSP module's
    filters and algorithms. If your audio app already disables denormals altogether
    (for example, by using the ScopedNoDenormals class or the
    FloatVectorOperations::disableDenormalisedNumberSupport method), then you
    can safely disable this flag to shave off a few cpu cycles from the DSP module's
    filters and algorithms.
*/
#ifndef ALOE_DSP_ENABLE_SNAP_TO_ZERO
 #define ALOE_DSP_ENABLE_SNAP_TO_ZERO 1
#endif


//==============================================================================
#if ALOE_USE_SIMD
 #include "native/aloe_fallback_SIMDNativeOps.h"

 // include the correct native file for this build target CPU
 #if defined(__i386__) || defined(__amd64__) || defined(_M_X64) || defined(_X86_) || defined(_M_IX86)
  #ifdef __AVX2__
   #include "native/aloe_avx_SIMDNativeOps.h"
  #else
   #include "native/aloe_sse_SIMDNativeOps.h"
  #endif
 #elif defined(__arm__) || defined(_M_ARM) || defined (__arm64__) || defined (__aarch64__)
  #include "native/aloe_neon_SIMDNativeOps.h"
 #else
  #error "SIMD register support not implemented for this platform"
 #endif

 #include "containers/aloe_SIMDRegister.h"
#endif

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/aloe_dsp.cpp]

#ifdef ALOE_DSP_H_INCLUDED
 /* When you add this cpp file to your project, you mustn't include it in a file where you've
    already included any other headers - just put it inside a file on its own, possibly with your config
    flags preceding it, but don't include anything else. That also includes avoiding any automatic prefix
    header files that the compiler may be using.
 */
 #error "Incorrect use of Aloe cpp file"
#endif

#include "aloe_dsp.h"

#ifndef ALOE_USE_VDSP_FRAMEWORK
 #define ALOE_USE_VDSP_FRAMEWORK 1
#endif

#if (ALOE_MAC || ALOE_IOS) && ALOE_USE_VDSP_FRAMEWORK
 #include <Accelerate/Accelerate.h>
#else
 #undef ALOE_USE_VDSP_FRAMEWORK
#endif

#if ALOE_DSP_USE_INTEL_MKL
 #include <mkl_dfti.h>
#endif

#if _IPP_SEQUENTIAL_STATIC || _IPP_SEQUENTIAL_DYNAMIC || _IPP_PARALLEL_STATIC || _IPP_PARALLEL_DYNAMIC
 #include <ippcore.h>
 #include <ipps.h>
 #define ALOE_IPP_AVAILABLE 1
#endif

*/
