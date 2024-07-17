crate::ix!();

/* -------------- Subframe structures  -------------- */

/**
  | An enumeration of the available subframe
  | types.
  |
  */
pub enum SubframeType {

    /**
      | constant signal
      |
      */
    SUBFRAME_TYPE_CONSTANT = 0, 

    /**
      | uncompressed signal
      |
      */
    SUBFRAME_TYPE_VERBATIM = 1, 

    /**
      | fixed polynomial prediction
      |
      */
    SUBFRAME_TYPE_FIXED    = 2, 

    /**
      | linear prediction
      |
      */
    SUBFRAME_TYPE_LPC      = 3, 
}

/**
  | Maps a SubframeType to a C string.
  | 
  | Using a SubframeType as the index
  | to this array will give the string equivalent.
  | The contents should not be modified.
  |
  */
lazy_static!{
    /*
    extern  const char * const SubframeTypeString[];
    */
}

/**
  | CONSTANT subframe. (c.f. <A HREF="../format.html#subframe_constant">format
  | specification</A>)
  |
  */
#[derive(Copy,Clone)]
pub struct Subframe_Constant {

    /**
      | The constant signal value.
      |
      */
    value: i32,
}

/**
  | VERBATIM subframe. (c.f. <A HREF="../format.html#subframe_verbatim">format
  | specification</A>)
  |
  */
#[derive(Copy,Clone)]
pub struct Subframe_Verbatim {

    /**
      | A pointer to verbatim signal.
      |
      */
    data: i32,
}

/**
  | FIXED subframe. (c.f. <A HREF="../format.html#subframe_fixed">format
  | specification</A>)
  |
  */
#[derive(Copy,Clone)]
pub struct Subframe_Fixed {

    /**
      | The residual coding method.
      |
      */
    entropy_coding_method: EntropyCodingMethod,


    /**
      | The polynomial order.
      |
      */
    order:                 u32,


    /**
      | Warmup samples to prime the predictor,
      | length == order.
      |
      */
    warmup:                [i32; MAX_FIXED_ORDER],


    /**
      | The residual signal, length == (blocksize
      | minus order) samples.
      |
      */
    residual:              *const i32,
}

/**
  | LPC subframe. (c.f. <A HREF="../format.html#subframe_lpc">format
  | specification</A>)
  |
  */
#[derive(Copy,Clone)]
pub struct Subframe_LPC {

    /**
      | The residual coding method.
      |
      */
    entropy_coding_method: EntropyCodingMethod,


    /**
      | The FIR order.
      |
      */
    order:                 u32,


    /**
      | Quantized FIR filter coefficient precision
      | in bits.
      |
      */
    qlp_coeff_precision:   u32,


    /**
      | The qlp coeff shift needed.
      |
      */
    quantization_level:    i32,


    /**
      | FIR filter coefficients.
      |
      */
    qlp_coeff:             [i32; MAX_LPC_ORDER],


    /**
      | Warmup samples to prime the predictor,
      | length == order.
      |
      */
    warmup:                [i32; MAX_LPC_ORDER],


    /**
      | The residual signal, length == (blocksize
      | minus order) samples.
      |
      */
    residual:              *const i32,
}

/**
  | == 4 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned SUBFRAME_LPC_QLP_COEFF_PRECISION_LEN;
    */
}

/**
  | == 5 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned SUBFRAME_LPC_QLP_SHIFT_LEN;
    */
}

/**
  | FLAC subframe structure. (c.f. <A HREF="../format.html#subframe">format
  | specification</A>)
  |
  */
pub struct Subframe {
    ty:          SubframeType,
    data:        FlacSubframeU,
    wasted_bits: u32,
}

pub union FlacSubframeU {
    constant: Subframe_Constant,
    fixed:    Subframe_Fixed,
    lpc:      Subframe_LPC,
    verbatim: Subframe_Verbatim,
}

/**
  | == 1 (bit)
  | 
  | This used to be a zero-padding bit (hence
  | the name
  | 
  | SUBFRAME_ZERO_PAD_LEN) but
  | is now a reserved bit. It still has a mandatory
  | value of \c 0 but in the future may take
  | on the value \c 0 or \c 1 to mean something
  | else.
  |
  */
lazy_static!{
    /*
    extern  const unsigned SUBFRAME_ZERO_PAD_LEN;
    */
}

/**
  | == 6 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned SUBFRAME_TYPE_LEN;
    */
}

/**
  | == 1 (bit)
  |
  */
lazy_static!{
    /*
    extern  const unsigned SUBFRAME_WASTED_BITS_FLAG_LEN;
    */
}

/**
  | = 0x00
  |
  */
lazy_static!{
    /*
    extern  const unsigned SUBFRAME_TYPE_CONSTANT_BYTE_ALIGNED_MASK;
    */
}

/**
  | = 0x02
  |
  */
lazy_static!{
    /*
    extern  const unsigned SUBFRAME_TYPE_VERBATIM_BYTE_ALIGNED_MASK;
    */
}

/**
  | = 0x10
  |
  */
lazy_static!{
    /*
    extern  const unsigned SUBFRAME_TYPE_FIXED_BYTE_ALIGNED_MASK;
    */
}

/**
  | = 0x40
  |
  */
lazy_static!{
    /*
    extern  const unsigned SUBFRAME_TYPE_LPC_BYTE_ALIGNED_MASK;
    */
}

