crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/private/float.h]

/**
  | These typedefs make it easier to ensure
  | that integer versions of the library
  | really only contain integer operations.
  | All the code in libFLAC should use float
  | and double in place of float and
  | double, and be protected by checks of
  | the macro
  | 
  | INTEGER_ONLY_LIBRARY.
  | 
  | real is the basic floating point
  | type used in LPC analysis.
  |
  */
#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub type double = f64;

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub type float  = f32;

/**
  | WATCHOUT: changing real will
  | change the signatures of many functions
  | that have assembly language equivalents
  | and break them.
  |
  */
#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub type real = f32;

//------------------------------------------------------------

/**
  | The convention for fixedpoint
  | is to use the upper 16 bits for the integer
  | part and lower 16 bits for the fractional
  | part.
  |
  */
#[cfg(INTEGER_ONLY_LIBRARY)]
pub type fixedpoint = i32;

#[cfg(INTEGER_ONLY_LIBRARY)]
lazy_static!{
    /*
    extern const fixedpoint FP_ZERO;
    extern const fixedpoint FP_ONE_HALF;
    extern const fixedpoint FP_ONE;
    extern const fixedpoint FP_LN2;
    extern const fixedpoint FP_E;
    */
}

#[cfg(INTEGER_ONLY_LIBRARY)]
macro_rules! fixedpoint_trunc {
    ($x:ident) => {
        /*
                ((x)>>16)
        */
    }
}

#[cfg(INTEGER_ONLY_LIBRARY)]
macro_rules! fixedpoint_mul {
    ($x:ident, $y:ident) => {
        /*
                ( (fixedpoint) ( ((i64)(x)*(i64)(y)) >> 16 ) )
        */
    }
}

#[cfg(INTEGER_ONLY_LIBRARY)]
macro_rules! fixedpoint_div {
    ($x:ident, $y:ident) => {
        /*
                ( (fixedpoint) ( ( ((i64)(x)<<32) / (i64)(y) ) >> 16 ) )
        */
    }
}

/**
 |  fixedpoint_log2()
 |  ----------------------------------------------
 |
 |  Returns the base-2 logarithm of the
 |  fixed-point number 'x' using an algorithm by
 |  Knuth for x >= 1.0
 |
 |  'fracbits' is the number of fractional bits of
 |  'x'.  'fracbits' must be < 32 and evenly
 |  divisible by 4 (0 is OK but not very precise).
 |
 |  'precision' roughly limits the number of
 |  iterations that are done; use (unsigned)(-1)
 |  for maximum precision.
 |
 |  If 'x' is less than one -- that is,
 |  x < (1<<fracbits) -- then this function will
 |  punt and return 0.
 |
 |  The return value will also have 'fracbits'
 |  fractional bits.
 */
#[cfg(INTEGER_ONLY_LIBRARY)]
pub fn flac_fixedpoint_log2(
        x:         u32,
        fracbits:  u32,
        precision: u32) -> u32 {
    
    todo!();
        /*
        
        */
}
