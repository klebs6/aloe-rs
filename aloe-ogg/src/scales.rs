/*!
 function: linear scale -> dB, Bark and Mel scales
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/scales.h]

/**
  | 20log10(x)
  |
  */
pub const VORBIS_IEEE_FLOAT32: usize = 1;

#[cfg(VORBIS_IEEE_FLOAT32)]
#[inline] pub fn unitnorm(x: f32) -> f32 {
    
    todo!();
        /*
            union {
        ogg_uint32_t i;
        float f;
      } ix;
      ix.f = x;
      ix.i = (ix.i & 0x80000000U) | (0x3f800000U);
      return ix.f;
        */
}

/**
  | Segher was off (too high) by ~ .3 decibel.
  | Center the conversion correctly.
  |
  */
#[cfg(VORBIS_IEEE_FLOAT32)]
#[inline] pub fn todb(x: *const f32) -> f32 {
    
    todo!();
        /*
            union {
        ogg_uint32_t i;
        float f;
      } ix;
      ix.f = *x;
      ix.i = ix.i&0x7fffffff;
      return (float)(ix.i * 7.17711438e-7f -764.6161886f);
        */
}

#[cfg(VORBIS_IEEE_FLOAT32)]
macro_rules! todb_nn {
    ($x:ident) => {
        /*
                todB(x)
        */
    }
}

#[cfg(not(VORBIS_IEEE_FLOAT32))]
pub fn unitnorm(x: f32) -> f32 {
    
    todo!();
        /*
            if(x<0)return(-1.f);
      return(1.f);
        */
}

#[cfg(not(VORBIS_IEEE_FLOAT32))]
macro_rules! todb {
    ($x:ident) => {
        /*
                (*(x)==0?-400.f:log(*(x)**(x))*4.34294480f)
        */
    }
}

#[cfg(not(VORBIS_IEEE_FLOAT32))]
macro_rules! todb_nn {
    ($x:ident) => {
        /*
                (*(x)==0.f?-400.f:log(*(x))*8.6858896f)
        */
    }
}

macro_rules! fromdb {
    ($x:ident) => {
        /*
                (exp((x)*.11512925f))
        */
    }
}

/**
  | The bark scale equations are approximations,
  | since the original table was somewhat
  | hand rolled. The below are chosen to
  | have the best possible fit to the rolled
  | tables, thus their somewhat odd appearance
  | (these are more accurate and over a longer
  | range than the oft-quoted bark equations
  | found in the texts I have). The approximations
  | are valid from 0 - 30kHz (nyquist) or
  | so.
  | 
  | all f in Hz, z in Bark
  |
  */
macro_rules! tobark {
    ($n:ident) => {
        /*
                (13.1f*atan(.00074f*(n))+2.24f*atan((n)*(n)*1.85e-8f)+1e-4f*(n))
        */
    }
}

macro_rules! frombark {
    ($z:ident) => {
        /*
                (102.f*(z)-2.f*pow(z,2.f)+.4f*pow(z,3.f)+pow(1.46f,z)-1.f)
        */
    }
}

macro_rules! tomel {
    ($n:ident) => {
        /*
                (log(1.f+(n)*.001f)*1442.695f)
        */
    }
}

macro_rules! frommel {
    ($m:ident) => {
        /*
                (1000.f*exp((m)/1442.695f)-1000.f)
        */
    }
}

/**
  | Frequency to octave. We arbitrarily
  | declare 63.5 Hz to be octave 0.0
  |
  */
macro_rules! tooc {
    ($n:ident) => {
        /*
                (log(n)*1.442695f-5.965784f)
        */
    }
}

macro_rules! fromoc {
    ($o:ident) => {
        /*
                (exp(((o)+5.965784f)*.693147f))
        */
    }
}
