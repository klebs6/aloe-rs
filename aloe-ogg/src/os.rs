/*!
 function: #ifdef jail to whip a few platforms
 into the UNIX ideal.

use crate::*;
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/os.h]

#[cfg(DJGPP)]
macro_rules! rint {
    ($x:ident) => {
        /*
                (floor((x)+0.5f))
        */
    }
}

#[cfg(all(_WIN32,not(__SYMBIAN32__)))]
macro_rules! rint {
    ($x:ident) => {
        /*
                (floor((x)+0.5f))
        */
    }
}

#[cfg(all(_WIN32,not(__SYMBIAN32__)))]
pub const NO_FLOAT_MATH_LIB: bool = true;

#[cfg(all(_WIN32,not(__SYMBIAN32__)))]
macro_rules! fast_hypot {
    ($a:ident, $b:ident) => {
        /*
                sqrt((a)*(a) + (b)*(b))
        */
    }
}

///-----------------------------
#[cfg(all(__SYMBIAN32__,__WINS__))]
pub fn alloca(size: usize)  {
    
    todo!();
        /*
        
        */
}


macro_rules! fast_hypot {
    () => {
        /*
                hypot
        */
    }
}

/**
  | Special i386 GCC implementation
  |
  */
#[cfg(all(all(__i386__,__GNUC__),not(__BEOS__)))]
pub const VORBIS_FPU_CONTROL: bool = true;

/*
  | both GCC and MSVC are kinda stupid about
  | rounding/casting to int.
  | 
  | Because of encapsulation constraints
  | (GCC can't see inside the asm block and
  | so we end up doing stupid things like
  | a store/load that is collectively a
  | noop), we do it this way
  |
  */

/**
  | we must set up the fpu before this works!!
  |
  */
#[cfg(all(all(__i386__,__GNUC__),not(__BEOS__)))]
pub type VorbisFpuControl = i16;

#[cfg(all(all(__i386__,__GNUC__),not(__BEOS__)))]
#[inline] pub fn vorbis_fpu_setround(fpu: *mut VorbisFpuControl)  {
    
    todo!();
        /*
            ogg_int16_t ret;
      ogg_int16_t temp = 0;
      __asm__ __volatile__("fnstcw %0\n\t"
              "movw %0,%%dx\n\t"
              "andw $62463,%%dx\n\t"
              "movw %%dx,%1\n\t"
              "fldcw %1\n\t":"=m"(ret):"m"(temp): "dx");
      *fpu=ret;
        */
}

#[cfg(all(all(__i386__,__GNUC__),not(__BEOS__)))]
#[inline] pub fn vorbis_fpu_restore(fpu: VorbisFpuControl)  {
    
    todo!();
        /*
            __asm__ __volatile__("fldcw %0":: "m"(fpu));
        */
}


/**
  | assumes the FPU is in round mode!
  |
  */
#[cfg(all(all(__i386__,__GNUC__),not(__BEOS__)))]
#[inline] pub fn vorbis_ftoi(f: f64) -> i32 {
    
    todo!();
        /*
    /* yes, double!  Otherwise,
    we get extra fst/fld to
    truncate precision */
    int i;
      __asm__("fistl %0": "=m"(i) : "t"(f));
      return(i);
        */
}

/**
  | MSVC inline assembly. 32 bit only; inline
  | ASM isn't implemented in the 64 bit compiler
  | and doesn't work on arm.
  |
  */
#[cfg(all(all(_MSC_VER,_M_IX86),not(_WIN32_WCE)))]
pub const VORBIS_FPU_CONTROL: bool = true;

#[cfg(all(all(_MSC_VER,_M_IX86),not(_WIN32_WCE)))]
pub type VorbisFpuControl = i16;

#[cfg(VORBIS_FPU_CONTROL)]//i think this is how we specify the gate
#[inline] pub fn vorbis_ftoi(f: f64) -> i32 {
    
    todo!();
        /*
            int i;
            __asm{
                    fld f
                    fistp i
            }
            return i;
        */
}

#[cfg(all(all(_MSC_VER,_M_IX86),not(_WIN32_WCE)))]
#[inline] pub fn vorbis_fpu_setround(fpu: *mut VorbisFpuControl)  {
    
    todo!();
        /*
            (void)fpu;
        */
}

#[cfg(all(all(_MSC_VER,_M_IX86),not(_WIN32_WCE)))]
#[inline] pub fn vorbis_fpu_restore(fpu: VorbisFpuControl)  {
    
    todo!();
        /*
            (void)fpu;
        */
}

/**
  | Optimized code path for x86_64 builds.
  | Uses SSE2 intrinsics. This can be done
  | safely because all x86_64 CPUs supports
  | SSE2.
  |
  */
#[cfg(any(all(ALOE_MSVC,ALOE_64BIT),all(ALOE_GCC,__x86_64__)))]
pub const VORBIS_FPU_CONTROL: bool = true;

#[cfg(any(all(ALOE_MSVC,ALOE_64BIT),all(ALOE_GCC,__x86_64__)))]
pub type VorbisFpuControl = i16;

#[cfg(any(all(ALOE_MSVC,ALOE_64BIT),all(ALOE_GCC,__x86_64__)))]
#[inline] pub fn vorbis_ftoi(f: f64) -> i32 {
    
    todo!();
        /*
            return _mm_cvtsd_si32(_mm_load_sd(&f));
        */
}

#[cfg(any(all(ALOE_MSVC,ALOE_64BIT),all(ALOE_GCC,__x86_64__)))]
#[inline] pub fn vorbis_fpu_setround(fpu: *mut VorbisFpuControl)  {
    
    todo!();
        /*
            (void)fpu;
        */
}


#[cfg(any(all(ALOE_MSVC,ALOE_64BIT),all(ALOE_GCC,__x86_64__)))]
#[inline] pub fn vorbis_fpu_restore(fpu: VorbisFpuControl)  {
    
    todo!();
        /*
            (void)fpu;
        */
}


/**
  | If no special implementation was found
  | for the current compiler / platform,
  | use the default implementation here:
  |
  */
#[cfg(not(VORBIS_FPU_CONTROL))]
pub type VorbisFpuControl = i32;

#[cfg(not(VORBIS_FPU_CONTROL))]
pub fn vorbis_ftoi(f: f64) -> i32 {
    
    todo!();
        /*
            /* Note: MSVC and GCC (at least on some systems) round towards zero, thus,
               the floor() call is required to ensure correct roudning of
               negative numbers */
            return (int)floor(f+.5);
        */
}


/**
  | We don't have special code for this compiler/arch,
  | so do it the slow way
  |
  */
#[cfg(not(VORBIS_FPU_CONTROL))]
macro_rules! vorbis_fpu_setround { ($vorbis_fpu_control:ident) => { /* {} */ } }

#[cfg(not(VORBIS_FPU_CONTROL))]
macro_rules! vorbis_fpu_restore { ($vorbis_fpu_control:ident) => { /* {} */ } }
