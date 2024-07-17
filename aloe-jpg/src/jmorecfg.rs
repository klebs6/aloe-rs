/*!
  | jmorecfg.h
  | 
  | This file contains additional configuration
  | options that customize the
  | 
  | JPEG software for special applications
  | or support machine-dependent optimizations.
  | Most users will not need to touch this
  | file.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jmorecfg.h]

/**
  | Define BITS_IN_JSAMPLE as either:
  | 
  | 8 for 8-bit sample values (the usual
  | setting)
  | 
  | 12 for 12-bit sample values
  | 
  --------------
  | Only 8 and 12 are legal data precisions
  | for lossy JPEG according to the
  | 
  | JPEG standard, and the IJG code does
  | not support anything else!
  | 
  | We do not support run-time selection
  | of data precision, sorry.
  |
  */
pub const BITS_IN_JSAMPLE: usize = 8; //use 8 or 12 

/**
  | Maximum number of components (color
  | channels) allowed in JPEG image.
  | 
  | To meet the letter of the JPEG spec, set
  | this to 255. However, darn few applications
  | need more than 4 channels (maybe 5 for
  | CMYK + alpha mask). We recommend 10 as
  | a reasonable compromise; use 4 if you
  | are really short on memory. (Each allowed
  | component costs a hundred or so bytes
  | of storage, whether actually used in
  | an image or not.)
  |
  */
pub const MAX_COMPONENTS: usize = 10; //maximum number of image components 

/*
  | Basic data types.
  | 
  | You may need to change these if you have
  | a machine with unusual data type sizes;
  | for example, "char" not 8 bits, "short"
  | not 16 bits, or "long" not 32 bits. We
  | don't care whether "int" is 16 or 32 bits,
  | but it had better be at least 16.
  |
  */

/**
  | Representation of a single sample (pixel
  | element value).
  | 
  | We frequently allocate large arrays
  | of these, so it's important to keep them
  | small. But if you have memory to burn
  | and access to char or short arrays is
  | very slow on your hardware, you might
  | want to change these.
  |
  | JSAMPLE should be the smallest type
  | that will hold the values 0..255.
  | 
  | You can use a signed char by having GETJSAMPLE
  | mask it with 0xFF.
  |
  */
pub type JSample = u8;

#[cfg(BITS_IN_JSAMPLE_EQ_8)]
#[cfg(HAVE_UNSIGNED_CHAR)]
macro_rules! getjsample {
    ($value:ident) => {
        /*
                ((int) (value))
        */
    }
}

#[cfg(BITS_IN_JSAMPLE_EQ_8)]
#[cfg(not(HAVE_UNSIGNED_CHAR))]
#[cfg(CHAR_IS_UNSIGNED)]
macro_rules! getjsample {
    ($value:ident) => {
        /*
                ((int) (value))
        */
    }
}

#[cfg(BITS_IN_JSAMPLE_EQ_8)]
#[cfg(not(HAVE_UNSIGNED_CHAR))]
#[cfg(not(CHAR_IS_UNSIGNED))]
macro_rules! getjsample {
    ($value:ident) => {
        /*
                ((int) (value) & 0xFF)
        */
    }
}

#[cfg(BITS_IN_JSAMPLE_EQ_8)]
pub const MAXJSAMPLE:    usize = 255;

#[cfg(BITS_IN_JSAMPLE_EQ_8)]
pub const CENTERJSAMPLE: usize = 128;

/**
  | JSAMPLE should be the smallest type
  | that will hold the values 0..4095.
  | 
  | On nearly all machines "short" will
  | do nicely.
  |
  */
#[cfg(BITS_IN_JSAMPLE_EQ_12)]
pub type JSample = i16;

#[cfg(BITS_IN_JSAMPLE_EQ_12)]
macro_rules! getjsample {
    ($value:ident) => {
        /*
                ((int) (value))
        */
    }
}

#[cfg(BITS_IN_JSAMPLE_EQ_12)]
pub const MAXJSAMPLE: usize = 4095;

#[cfg(BITS_IN_JSAMPLE_EQ_12)]
pub const CENTERJSAMPLE: usize = 2048;

/**
  | Representation of a DCT frequency coefficient.
  | 
  | This should be a signed value of at least
  | 16 bits; "short" is usually OK.
  | 
  | Again, we allocate large arrays of these,
  | but you can change to int if you have memory
  | to burn and "short" is really slow.
  |
  */
pub type JCoef = i16;

/**
  | Compressed datastreams are represented
  | as arrays of JOctet.
  | 
  | These must be EXACTLY 8 bits wide, at
  | least once they are written to external
  | storage. Note that when using the stdio
  | data source/destination managers,
  | this is also the data type passed to fread/fwrite.
  |
  */
#[cfg(HAVE_UNSIGNED_CHAR)]
pub type JOctet = u8;

#[cfg(HAVE_UNSIGNED_CHAR)]
macro_rules! getjoctet {
    ($value:ident) => {
        /*
                (value)
        */
    }
}

#[cfg(not(HAVE_UNSIGNED_CHAR))]
pub type JOctet = u8;

#[cfg(not(HAVE_UNSIGNED_CHAR))]
#[cfg(CHAR_IS_UNSIGNED)]
macro_rules! getjoctet {
    ($value:ident) => {
        /*
                (value)
        */
    }
}

#[cfg(not(HAVE_UNSIGNED_CHAR))]
#[cfg(not(CHAR_IS_UNSIGNED))]
macro_rules! getjoctet {
    ($value:ident) => {
        /*
                ((value) & 0xFF)
        */
    }
}

/**
  | Datatype used for image dimensions.
  | The JPEG standard only supports images
  | up to 64K*64K due to 16-bit fields in
  | SOF markers. Therefore "unsigned int"
  | is sufficient on all machines. However,
  | if you need to handle larger images and
  | you don't mind deviating from the spec,
  | you can change this datatype.
  |
  */
pub type JDimension = u32;

/**
  | a tad under 64K to prevent overflows
  |
  */
pub const JPEG_MAX_DIMENSION: usize = 65500;

/*
  | These macros are used in all function
  | definitions and extern declarations.
  | 
  | You could modify them if you need to change
  | function linkage conventions; in particular,
  | you'll need to do that to make the library
  | a Windows DLL.
  | 
  | Another application is to make all functions
  | global for use with debuggers or code
  | profilers that require it.
  |
  */

/**
  | a function called through method pointers:
  |
  */
macro_rules! methoddef {
    ($type:ident) => {
        /*
                static type
        */
    }
}

/**
  | a function used only in its module:
  |
  */
macro_rules! local {
    ($type:ident) => {
        /*
                static type
        */
    }
}

/**
  | a function referenced thru EXTERNs:
  |
  */
macro_rules! global {
    ($type:ident) => {
        /*
                type
        */
    }
}

/**
  | a reference to a GLOBAL function:
  |
  */
macro_rules! extern_ {
    ($type:ident) => {
        /*
                extern type
        */
    }
}


/**
  | This macro is used to declare a "method",
  | that is, a function pointer.
  | 
  | We want to supply prototype parameters
  | if the compiler can cope.
  | 
  | -----------
  | @note
  | 
  | the arglist parameter must be parenthesized!
  | 
  | Again, you can customize this if you
  | need special linkage keywords.
  |
  */
#[cfg(HAVE_PROTOTYPES)]
macro_rules! jmethod {
    ($type:ident, $methodname:ident, $arglist:ident) => {
        /*
                type (*methodname) arglist
        */
    }
}

#[cfg(not(HAVE_PROTOTYPES))]
macro_rules! jmethod {
    ($type:ident, $methodname:ident, $arglist:ident) => {
        /*
                type (*methodname) ()
        */
    }
}

/**
  | Here is the pseudo-keyword for declaring
  | pointers that must be "far" on 80x86
  | machines. Most of the specialized coding
  | for 80x86 is handled by just saying "FAR
  | *" where such a pointer is needed. In
  | a few places explicit coding is needed;
  | see uses of the NEED_FAR_POINTERS symbol.
  |
  */
lazy_static!{
    /*
    #ifdef NEED_FAR_POINTERS
    #define FAR  far
    #else
    #define FAR
    #endif
    */
}

/**
  | in case these macros already exist
  |
  */
#[cfg(not(FALSE))]
pub const FALSE: usize = 0; //values of boolean 

#[cfg(not(TRUE))]
pub const TRUE: usize = 1;

/**
  | The remaining options affect code selection
  | within the JPEG library, but they don't
  | need to be visible to most applications
  | using the library.
  | 
  | To minimize application namespace
  | pollution, the symbols won't be defined
  | unless JPEG_INTERNALS or JPEG_INTERNAL_OPTIONS
  | has been defined.
  |
  */
#[cfg(JPEG_INTERNALS)]
pub const JPEG_INTERNAL_OPTIONS: bool = true;

#[cfg(JPEG_INTERNAL_OPTIONS)]
pub mod jpeg_internal_options {

    use super::*;

    /*
      | These defines indicate whether to include
      | various optional functions.
      | 
      | Undefining some of these symbols will
      | produce a smaller but less capable library.
      | Note that you can leave certain source
      | files out of the compilation/linking
      | process if you've #undef'd the corresponding
      | symbols. (You may HAVE to do that if your
      | compiler doesn't like null source files.)
      |
      | Arithmetic coding is unsupported for
      | legal reasons. Complaints to IBM.
      |
      */

    /**
      | Capability options common to encoder
      | and decoder:
      |
      */
    pub const DCT_ISLOW_SUPPORTED: bool = true; /* slow but accurate integer algorithm */
    pub const DCT_IFAST_SUPPORTED: bool = true; /* faster, less accurate integer method */
    pub const DCT_FLOAT_SUPPORTED: bool = true; /* floating-point: accurate, fast on fast HW */

    /**
      | Encoder capability options:
      |
      */
    pub const  C_ARITH_CODING_SUPPORTED: bool = false;    /* Arithmetic coding back end? */

    pub const C_MULTISCAN_FILES_SUPPORTED: bool = true; /* Multiple-scan JPEG files? */
    pub const C_PROGRESSIVE_SUPPORTED:     bool = true;     /* Progressive JPEG? (Requires MULTISCAN)*/
    pub const ENTROPY_OPT_SUPPORTED:       bool = true;       /* Optimization of entropy coding parms? */

    /**
      | @note
      | 
      | if you selected 12-bit data precision,
      | it is dangerous to turn off
      | 
      | ENTROPY_OPT_SUPPORTED. The standard
      | Huffman tables are only good for 8-bit
      | precision, so jchuff.c normally uses
      | entropy optimization to compute usable
      | tables for higher precision. If you
      | don't want to do optimization, you'll
      | have to supply different default Huffman
      | tables.
      | 
      | The exact same statements apply for
      | progressive JPEG: the default tables
      | don't work for progressive mode. (This
      | may get fixed, however.)
      |
      */
    pub const INPUT_SMOOTHING_SUPPORTED: bool = true;   /* Input image smoothing option? */

    /* ---------- * Decoder capability options:  ---------- */

    /**
      | Arithmetic coding back end?
      |
      */
    pub const D_ARITH_CODING_SUPPORTED:    bool = false; 

    /**
      | Multiple-scan JPEG files?
      |
      */
    pub const D_MULTISCAN_FILES_SUPPORTED: bool = true;  

    /**
      | Progressive JPEG? (Requires MULTISCAN)
      |
      */
    pub const D_PROGRESSIVE_SUPPORTED:     bool = true;  

    /**
      | jpeg_save_markers() needed?
      |
      */
    pub const SAVE_MARKERS_SUPPORTED:      bool = true;  

    /**
      | Block smoothing? (Progressive only)
      |
      */
    pub const BLOCK_SMOOTHING_SUPPORTED:   bool = true;  

    /**
      | Output rescaling via IDCT?
      |
      */
    pub const IDCT_SCALING_SUPPORTED:      bool = true;  

    /**
      | Output rescaling at upsample stage?
      |
      */
    pub const UPSAMPLE_SCALING_SUPPORTED:  bool = false; 

    /**
      | Fast path for sloppy upsampling?
      |
      */
    pub const UPSAMPLE_MERGING_SUPPORTED:  bool = true;  

    /**
      | 1-pass color quantization?
      |
      */
    pub const QUANT_1PASS_SUPPORTED:       bool = true;  

    /**
      | 2-pass color quantization?
      |
      */
    pub const QUANT_2PASS_SUPPORTED:       bool = true;  

    /* ----- more capability options later, no doubt  ----- */

    /*
      | Ordering of RGB data in scanlines passed
      | to or from the application.
      | 
      | If your application wants to deal with
      | data in the order B,G,R, just change
      | these macros. You can also deal with
      | formats such as R,G,B,X (one extra byte
      | per pixel) by changing RGB_PIXELSIZE.
      | Note that changing the offsets will
      | also change the order in which colormap
      | data is organized.
      | 
      | RESTRICTIONS: 1. The sample applications
      | cjpeg,djpeg do NOT support modified
      | RGB formats. 2. These macros only affect
      | RGB<=>YCbCr color conversion, so they
      | are not useful if you are using JPEG color
      | spaces other than YCbCr or grayscale.
      | 3. The color quantizer modules will
      | not behave desirably if RGB_PIXELSIZE
      | is not 3 (they don't understand about
      | dummy color components!). So you can't
      | use color quantization if you change
      | that value.
      |
      */

    /**
      | Offset of Red in an RGB scanline element
      |
      */
    pub const RGB_RED: usize = 0;

    /**
      | Offset of Green
      |
      */
    pub const RGB_GREEN: usize = 1;

    /**
      | Offset of Blue
      |
      */
    pub const RGB_BLUE: usize = 2;

    /**
      | JSAMPLEs per RGB scanline element
      |
      */
    pub const RGB_PIXELSIZE: usize = 3;

    /* -- Definitions for speed-related optimizations.  -- */

}

/*
  | If your compiler supports inline functions,
  | define INLINE as the inline keyword;
  | otherwise define it as empty.
  |
  | On some machines (notably 68000 series)
  | "int" is 32 bits, but multiplying two
  | 16-bit shorts is faster than multiplying
  | two ints. Define MULTIPLIER as short
  | on such a machine. MULTIPLIER must be
  | at least 16 bits wide.
  |
  */

/**
  | type for fastest integer multiply
  |
  */
pub type Multiplier = i32;

#[cfg(feature = "have_prototypes")]
pub type FastFloat = f32;

#[cfg(not(feature = "have_prototypes"))]
pub type FastFloat = f64;
