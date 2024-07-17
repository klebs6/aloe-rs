/*!
  | jdct.h
  | 
  | This include file contains common declarations
  | for the forward and inverse DCT modules.
  | These declarations are private to the
  | DCT managers (jcdctmgr.c, jddctmgr.c)
  | and the individual DCT algorithms.
  | 
  | The individual DCT algorithms are kept
  | in separate files to ease machine-dependent
  | tuning (e.g., assembly coding).
  |
  -------------------------
  | A forward DCT routine is given a pointer
  | to a work area of type DCTELEM[]; the
  | DCT is to be performed in-place in that
  | buffer. Type DCTELEM is int for 8-bit
  | samples, i32 for 12-bit samples.
  | (NOTE: Floating-point DCT implementations
  | use an array of type FastFloat, instead.)
  | 
  | The DCT inputs are expected to be signed
  | (range +-CENTERJSAMPLE).
  | 
  | The DCT outputs are returned scaled
  | up by a factor of 8; they therefore have
  | a range of +-8K for 8-bit data, +-128K
  | for 12-bit data. This convention improves
  | accuracy in integer implementations
  | and saves some work in floating-point
  | ones.
  | 
  | Quantization of the output coefficients
  | is done by jcdctmgr.c.
  |
  -------------------------
  | An inverse DCT routine is given a pointer
  | to the input JBLOCK and a pointer to an
  | output sample array. The routine must
  | dequantize the input data as well as
  | perform the IDCT; for dequantization,
  | it uses the multiplier table pointed
  | to by compptr->dct_table. The output
  | data is to be placed into the sample array
  | starting at a specified column. (Any
  | row offset needed will be applied to
  | the array pointer before it is passed
  | to the IDCT code.)
  | 
  | -----------
  | @note
  | 
  | the number of samples emitted by the
  | IDCT routine is
  | 
  | DCT_scaled_size * DCT_scaled_size.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jdct.h]

/**
  | 16 or 32 bits is fine
  |
  */
#[cfg(BITS_IN_JSAMPLE_EQ_8)]
pub type DctElem = i32;

/**
  | must have 32 bits
  |
  */
#[cfg(not(BITS_IN_JSAMPLE_EQ_8))]
pub type DctElem = i32;

pub type ForwardDctMethodPtr = fn(data: *mut DctElem);
pub type FloatDctMethodPtr   = fn(data: *mut FastFloat);

/* typedef inverse_DCT_method_ptr is declared in jpegint.h */

/**
  | Each IDCT routine has its own ideas about
  | the best dct_table element type.
  |
  | short or int, whichever is faster
  |
  */
pub type ISlowMultType = Multiplier;

/**
  | 16 bits is OK, use short if faster
  |
  */
#[cfg(BITS_IN_JSAMPLE_EQ_8)]
pub type IFastMultType = Multiplier;

#[cfg(BITS_IN_JSAMPLE_EQ_8)]
pub const IFAST_SCALE_BITS: usize = 2; //fractional bits in scale factors 

/**
  | need 32 bits for scaled quantizers
  |
  */
#[cfg(not(BITS_IN_JSAMPLE_EQ_8))]
pub type IFastMultType = i32;

#[cfg(not(BITS_IN_JSAMPLE_EQ_8))]
pub const IFAST_SCALE_BITS: usize = 13; // fractional bits in scale factors 

/**
  | preferred floating type
  |
  */
pub type FloatMultType = FastFloat;


/**
  | Each IDCT routine is responsible for
  | range-limiting its results and converting
  | them to unsigned form (0..MAXJSAMPLE).
  | The raw outputs could be quite far out
  | of range if the input data is corrupt,
  | so a bulletproof range-limiting step
  | is required. We use a mask-and-table-lookup
  | method to do the combined operations
  | quickly. See the comments with prepare_range_limit_table
  | (in jdmaster.c) for more info.
  |
  */
macro_rules! idct_range_limit {
    ($cinfo:ident) => {
        /*
                ((cinfo)->sample_range_limit + CENTERJSAMPLE)
        */
    }
}

macro_rules! range_mask {
    () => {
        /*
                (MAXJSAMPLE * 4 + 3) /* 2 bits wider than legal samples */
        */
    }
}

//----------------------------
/* Short forms of external names for systems with brain-damaged linkers. */
#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_fdct_islow {
    () => {
        /*
                jFDislow
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_fdct_ifast {
    () => {
        /*
                jFDifast
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_fdct_float {
    () => {
        /*
                jFDfloat
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_idct_islow {
    () => {
        /*
                jRDislow
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_idct_ifast {
    () => {
        /*
                jRDifast
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_idct_float {
    () => {
        /*
                jRDfloat
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_idct_4x4 {
    () => {
        /*
                jRD4x4
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_idct_2x2 {
    () => {
        /*
                jRD2x2
        */
    }
}

#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
macro_rules! jpeg_idct_1x1 {
    () => {
        /*
                jRD1x1
        */
    }
}

/**
  | Macros for handling fixed-point arithmetic;
  | these are used by many but not all of the
  | DCT/IDCT modules.
  | 
  | All values are expected to be of type
  | i32.
  | 
  | Fractional constants are scaled left
  | by CONST_BITS bits.
  | 
  | CONST_BITS is defined within each module
  | using these macros, and may differ from
  | one module to the next.
  |
  */
macro_rules! one {
    () => {
        /*
                ((i32) 1)
        */
    }
}

macro_rules! const_scale {
    () => {
        /*
                (ONE << CONST_BITS)
        */
    }
}

/**
  | Convert a positive real constant to
  | an integer scaled by CONST_SCALE.
  | 
  | Caution: some C compilers fail to reduce
  | "FIX(constant)" at compile time, thus
  | causing a lot of useless floating-point
  | operations at run time.
  |
  */
macro_rules! fix {
    ($x:ident) => {
        /*
                ((i32) ((x) * CONST_SCALE + 0.5))
        */
    }
}

/**
  | Descale and correctly round an i32
  | value that's scaled by N bits.
  | 
  | We assume RIGHT_SHIFT rounds towards
  | minus infinity, so adding the fudge
  | factor is correct for either sign of
  | X.
  |
  */
macro_rules! descale {
    ($x:ident, $n:ident) => {
        /*
                RIGHT_SHIFT((x) + (ONE << ((n)-1)), n)
        */
    }
}

/**
  | Multiply an i32 variable by an i32
  | constant to yield an i32 result.
  | 
  | This macro is used only when the two inputs
  | will actually be no more than 16 bits
  | wide, so that a 16x16->32 bit multiply
  | can be used instead of a full 32x32 multiply.
  | This provides a useful speedup on many
  | machines.
  | 
  | Unfortunately there is no way to specify
  | a 16x16->32 multiply portably in C,
  | but some C compilers will do the right
  | thing if you provide the correct combination
  | of casts.
  |
  | may work if 'int' is 32 bits
  |
  */
#[cfg(SHORTxSHORT_32)]
macro_rules! multiply16c16 {
    ($var:ident, $const:ident) => {
        /*
                (((i16) (var)) * ((i16) (const)))
        */
    }
}

/**
  | known to work with Microsoft C 6.0
  |
  */
#[cfg(SHORTxLCONST_32)]
macro_rules! multiply16c16 {
    ($var:ident, $const:ident) => {
        /*
                (((i16) (var)) * ((i32) (const)))
        */
    }
}

/**
  | default definition
  |
  */
#[cfg(not(MULTIPLY16C16))]
macro_rules! multiply16c16 {
    ($var:ident, $const:ident) => {
        /*
                ((var) * (const))
        */
    }
}

/* ------ Same except both inputs are variables  ------ */

/**
  | may work if 'int' is 32 bits
  |
  */
#[cfg(SHORTxSHORT_32)]
macro_rules! multiply16v16 {
    ($var1:ident, $var2:ident) => {
        /*
                (((i16) (var1)) * ((i16) (var2)))
        */
    }
}

/**
  | default definition
  |
  */
#[cfg(not(MULTIPLY16V16))]
macro_rules! multiply16v16 {
    ($var1:ident, $var2:ident) => {
        /*
                ((var1) * (var2))
        */
    }
}
