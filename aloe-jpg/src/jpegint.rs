/*!
  | jpegint.h
  | 
  | This file provides common declarations
  | for the various JPEG modules.
  | 
  | These declarations are considered
  | internal to the JPEG library; most applications
  | using the library shouldn't need to
  | include this file.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jpegint.h]

/*
  | Declarations for both compression
  | & decompression
  |
  */

/**
  | Operating modes for buffer controllers
  |
  */
pub enum JBufMode {          

    /**
      | Plain stripwise operation
      |

      */
    JBUF_PASS_THRU,     

    /*
      | Remaining modes require a full-image
      | buffer to have been created
      |
      */

    /**
      | Run source subobject only, save output
      |
      */
    JBUF_SAVE_SOURCE,   

    /**
      | Run dest subobject only, using saved
      | data
      |
      */
    JBUF_CRANK_DEST,    

    /**
      | Run both subobjects, save output
      |
      */
    JBUF_SAVE_AND_PASS,  
}

/*
  | Values of global_state field (jdapi.c
  | has some dependencies on ordering!)
  |
  */

/**
  | after create_compress
  |
  */
pub const CSTATE_START: usize = 100;

/**
  | start_compress done, write_scanlines
  | OK
  |
  */
pub const CSTATE_SCANNING: usize = 101;

/**
  | start_compress done, write_raw_data
  | OK
  |
  */
pub const CSTATE_RAW_OK: usize = 102;

/**
  | jpeg_write_coefficients done
  |
  */
pub const CSTATE_WRCOEFS: usize = 103;

/**
  | after create_decompress
  |
  */
pub const DSTATE_START: usize = 200;

/**
  | reading header markers, no SOS yet
  |
  */
pub const DSTATE_INHEADER: usize = 201;

/**
  | found SOS, ready for start_decompress
  |
  */
pub const DSTATE_READY: usize = 202;

/**
  | reading multiscan file in start_decompress
  |
  */
pub const DSTATE_PRELOAD: usize = 203;

/**
  | performing dummy pass for 2-pass quant
  |
  */
pub const DSTATE_PRESCAN: usize = 204;

/**
  | start_decompress done, read_scanlines
  | OK
  |
  */
pub const DSTATE_SCANNING: usize = 205;

/**
  | start_decompress done, read_raw_data
  | OK
  |
  */
pub const DSTATE_RAW_OK: usize = 206;

/**
  | expecting jpeg_start_output
  |
  */
pub const DSTATE_BUFIMAGE: usize = 207;

/**
  | looking for SOS/EOI in jpeg_finish_output
  |
  */
pub const DSTATE_BUFPOST: usize = 208;

/**
  | reading file in jpeg_read_coefficients
  |
  */
pub const DSTATE_RDCOEFS: usize = 209;

/**
  | looking for EOI in jpeg_finish_decompress
  |
  */
pub const DSTATE_STOPPING: usize = 210;

/* ------ Declarations for compression modules  ------ */

/**
  | Master control module
  |
  */
pub struct JpegCompMaster {

    prepare_for_pass: fn(cinfo: JCompressPtr) -> c_void,

    pass_startup:     fn(cinfo: JCompressPtr) -> c_void,

    finish_pass:      fn(cinfo: JCompressPtr) -> c_void,

    /* -- State variables made visible to other modules  -- */

    /**
      | True if pass_startup must be called
      |
      */
    call_pass_startup: bool,

    /**
      | True during last pass
      |
      */
    is_last_pass:      bool,
}

/**
  | Main buffer control (downsampled-data
  | buffer)
  |
  */
pub struct JpegCMainController {

    start_pass:   fn(cinfo: JCompressPtr, pass_mode: JBufMode) -> c_void,

    process_data: fn(
            cinfo:         JCompressPtr,
            input_buf:     JSampArray,
            in_row_ctr:    *mut JDimension,
            in_rows_avail: JDimension
    ) -> c_void,
}

/**
  | Compression preprocessing (downsampling
  | input buffer control)
  |
  */
pub struct JpegCPrepController {
    start_pass:       fn(cinfo: JCompressPtr, pass_mode: JBufMode) -> c_void,

    pre_process_data: fn(
            cinfo:                JCompressPtr,
            input_buf:            JSampArray,
            in_row_ctr:           *mut JDimension,
            in_rows_avail:        JDimension,
            output_buf:           JSampImage,
            out_row_group_ctr:    *mut JDimension,
            out_row_groups_avail: JDimension
    ) -> c_void,
}

/**
  | Coefficient buffer control
  |
  */
pub struct JpegCCoefController {
    start_pass:    fn(cinfo: JCompressPtr, pass_mode: JBufMode) -> c_void,

    compress_data: fn(cinfo: JCompressPtr, input_buf: JSampImage) -> bool,
}

/**
  | Colorspace conversion
  |
  */
pub struct JpegColorConverter {
    start_pass:    fn(cinfo: JCompressPtr) -> (),

    color_convert: fn(
            cinfo:      JCompressPtr,
            input_buf:  JSampArray,
            output_buf: JSampImage,
            output_row: JDimension,
            num_rows:   i32
    ) -> (),
}

/**
  | Downsampling
  |
  */
pub struct JpegDownsampler {

    start_pass:        fn(cinfo: JCompressPtr) -> (),

    downsample:        fn(
            cinfo:               JCompressPtr,
            input_buf:           JSampImage,
            in_row_index:        JDimension,
            output_buf:          JSampImage,
            out_row_group_index: JDimension
    ) -> (),


    /**
      | TRUE if need rows above & below
      |
      */
    need_context_rows: bool,
}

/**
  | Forward DCT (also controls coefficient
  | quantization)
  |
  */
pub struct JpegForwardDct {
    start_pass:  fn(cinfo: JCompressPtr) -> (),

    forward_dct: fn(
            cinfo:       JCompressPtr,
            compptr:     *mut JpegComponentInfo,
            sample_data: JSampArray,
            coef_blocks: JBlockRow,
            start_row:   JDimension,
            start_col:   JDimension,
            num_blocks:  JDimension
    ) -> (),
}

/**
  | Entropy encoding
  |
  */
pub struct JpegEntropyEncoder {
    start_pass:  fn(cinfo: JCompressPtr, gather_statistics: bool) -> c_void,
    encode_mcu:  fn(cinfo: JCompressPtr, mcu_data: *mut JBlockRow) -> bool,
    finish_pass: fn(cinfo: JCompressPtr) -> c_void,
}

/**
  | Marker writing
  |
  */
pub struct JpegMarkerWriter {

    write_file_header:   fn(cinfo: JCompressPtr) -> c_void,
    write_frame_header:  fn(cinfo: JCompressPtr) -> c_void,
    write_scan_header:   fn(cinfo: JCompressPtr) -> c_void,
    write_file_trailer:  fn(cinfo: JCompressPtr) -> c_void,
    write_tables_only:   fn(cinfo: JCompressPtr) -> c_void,

    write_marker_header: fn(
            cinfo:   JCompressPtr,
            marker:  i32,
            datalen: u32
    ) -> c_void,

    write_marker_byte:   fn(cinfo: JCompressPtr, val: i32) -> c_void,
}


/* ----- Declarations for decompression modules  ----- */

/**
  | Master control module
  |
  */
pub struct JpegDecompMaster {

    prepare_for_output_pass: fn(cinfo: JDecompressPtr) -> c_void,
    finish_output_pass:      fn(cinfo: JDecompressPtr) -> c_void,

    /* -- State variables made visible to other modules  -- */

    /**
      | True during 1st pass for 2-pass quant
      |
      */
    is_dummy_pass: bool,
}

/**
  | Input control module
  |
  */
pub struct JpegInputController {

    consume_input:          fn(cinfo: JDecompressPtr) -> i32,
    reset_input_controller: fn(cinfo: JDecompressPtr) -> c_void,
    start_input_pass:       fn(cinfo: JDecompressPtr) -> c_void,
    finish_input_pass:      fn(cinfo: JDecompressPtr) -> c_void,

    /* State variables made visible to other modules */

    /**
      | True if file has multiple scans
      |
      */
    has_multiple_scans: bool,

    /**
      | True when EOI has been consumed
      |
      */
    eoi_reached:        bool,
}

/**
  | Main buffer control (downsampled-data
  | buffer)
  |
  */
pub struct JpegDMainController {
    start_pass:   fn(cinfo: JDecompressPtr, pass_mode: JBufMode) -> c_void,

    process_data: fn(
            cinfo:          JDecompressPtr,
            output_buf:     JSampArray,
            out_row_ctr:    *mut JDimension,
            out_rows_avail: JDimension
    ) -> c_void,
}

/**
  | Coefficient buffer control
  |
  */
pub struct JpegDCoefController {
    start_input_pass:  fn(cinfo: JDecompressPtr) -> c_void,
    consume_data:      fn(cinfo: JDecompressPtr) -> i32,
    start_output_pass: fn(cinfo: JDecompressPtr) -> c_void,
    decompress_data:   fn(cinfo: JDecompressPtr, output_buf: JSampImage) -> i32,

    /**
      | Pointer to array of coefficient virtual
      | arrays, or NULL if none
      |
      */
    coef_arrays:       *mut JVirtBArrayPtr,
}

/**
  | Decompression postprocessing (color
  | quantization buffer control)
  |
  */
pub struct JpegDPostController {
    start_pass:        fn(cinfo: JDecompressPtr, pass_mode: JBufMode) -> (),

    post_process_data: fn(
            cinfo:               JDecompressPtr,
            input_buf:           JSampImage,
            in_row_group_ctr:    *mut JDimension,
            in_row_groups_avail: JDimension,
            output_buf:          JSampArray,
            out_row_ctr:         *mut JDimension,
            out_rows_avail:      JDimension
    ) -> (),
}

/**
  | Marker reading & parsing
  |
  */
pub struct JpegMarkerReader {

    reset_marker_reader: fn(cinfo: JDecompressPtr) -> c_void,
    read_markers:        fn(cinfo: JDecompressPtr) -> i32,

    /**
      | Read a restart marker --- exported for
      | use by entropy decoder only
      |
      */
    read_restart_marker: JpegMarkerParserMethod,

    /*
      | State of marker reader --- nominally
      | internal, but applications supplying
      | COM or APPn handlers might like to know
      | the state.
      |
      */

    /**
      | found SOI?
      |
      */
    saw_soi:          bool,


    /**
      | found SOF?
      |
      */
    saw_sof:          bool,


    /**
      | next restart number expected (0-7)
      |
      */
    next_restart_num: i32,


    /**
      | # of bytes skipped looking for a marker
      |
      */
    discarded_bytes:  u32,
}

/**
  | Entropy decoding
  |
  */
pub struct JpegEntropyDecoder {
    start_pass: fn(cinfo: JDecompressPtr) -> c_void,
    decode_mcu: fn(cinfo: JDecompressPtr, mcu_data: *mut JBlockRow) -> bool,

    /*
      | This is here to share code between baseline
      | and progressive decoders; other modules
      | probably should not use it
      |
      */

    /**
      | set TRUE after emitting warning
      |
      */
    insufficient_data: bool,
}

/**
  | Inverse DCT (also performs dequantization)
  |
  */
pub type InverseDCTMethodPtr = fn(
        cinfo:      JDecompressPtr,
        compptr:    *mut JpegComponentInfo,
        coef_block: JCoefPtr,
        output_buf: JSampArray,
        output_col: JDimension
) -> c_void;


pub struct JpegInverseDct {
    start_pass:  fn(cinfo: JDecompressPtr) -> c_void,

    /**
      | It is useful to allow each component
      | to have a separate IDCT method.
      |
      */
    inverse_dct: [InverseDCTMethodPtr; MAX_COMPONENTS],
}

/**
  | Upsampling (note that upsampler must
  | also call color converter)
  |
  */
pub struct JpegUpsampler {

    start_pass:        fn(cinfo: JDecompressPtr) -> (),

    upsample:          fn(
            cinfo:               JDecompressPtr,
            input_buf:           JSampImage,
            in_row_group_ctr:    *mut JDimension,
            in_row_groups_avail: JDimension,
            output_buf:          JSampArray,
            out_row_ctr:         *mut JDimension,
            out_rows_avail:      JDimension
    ) -> (),

    /**
      | TRUE if need rows above & below
      |
      */
    need_context_rows: bool,
}

/**
  | Colorspace conversion
  |
  */
pub struct JpegColorDeconverter {
    start_pass:    fn(cinfo: JDecompressPtr) -> (),

    color_convert: fn(
            cinfo:      JDecompressPtr,
            input_buf:  JSampImage,
            input_row:  JDimension,
            output_buf: JSampArray,
            num_rows:   i32
    ) -> (),
}

/**
  | Color quantization or color precision
  | reduction
  |
  */
pub struct JpegColorQuantizer {
    start_pass:     fn(cinfo: JDecompressPtr, is_pre_scan: bool) -> (),

    color_quantize: fn(
            cinfo:      JDecompressPtr,
            input_buf:  JSampArray,
            output_buf: JSampArray,
            num_rows:   i32
    ) -> (),

    finish_pass:    fn(cinfo: JDecompressPtr) -> (),

    new_color_map:  fn(cinfo: JDecompressPtr) -> (),
}

/* ----------- Miscellaneous useful macros  ----------- */

/**
  | We assume that right shift corresponds
  | to signed division by 2 with rounding
  | towards minus infinity. This is correct
  | for typical "arithmetic shift" instructions
  | that shift in copies of the sign bit.
  | But some C compilers implement >> with an 
  | unsigned shift. For these machines you must define
  | RIGHT_SHIFT_IS_UNSIGNED.
  | 
  | RIGHT_SHIFT provides a proper signed
  | right shift of an i32 quantity.
  | 
  | It is only applied with constant shift
  | counts. SHIFT_TEMPS must be included
  | in the variables of any routine using
  | RIGHT_SHIFT.
  |
  */
#[cfg(RIGHT_SHIFT_IS_UNSIGNED)]
macro_rules! shift_temps {
    () => {
        /*
                i32 shift_temp;
        */
    }
}

#[cfg(RIGHT_SHIFT_IS_UNSIGNED)]
macro_rules! right_shift {
    ($x:ident, $shft:ident) => {
        /*
        
            ((shift_temp = (x)) < 0 ? 
             (shift_temp >> (shft)) | ((~((i32) 0)) << (32-(shft))) : 
             (shift_temp >> (shft)))
        */
    }
}

#[cfg(not(RIGHT_SHIFT_IS_UNSIGNED))]
macro_rules! shift_temps { () => { } }

#[cfg(not(RIGHT_SHIFT_IS_UNSIGNED))]
macro_rules! right_shift {
    ($x:ident, $shft:ident) => {
        /*
                ((x) >> (shft))
        */
    }
}

/**
  | Short forms of external names for systems
  | with brain-damaged linkers.
  |
  */
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_compress_master   { () => { /* jICompress */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_c_master_control  { () => { /* jICMaster */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_c_main_controller { () => { /* jICMainC */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_c_prep_controller { () => { /* jICPrepC */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_c_coef_controller { () => { /* jICCoefC */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_color_converter   { () => { /* jICColor */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_downsampler       { () => { /* jIDownsampler */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_forward_dct       { () => { /* jIFDCT */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_huff_encoder      { () => { /* jIHEncoder */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_phuff_encoder     { () => { /* jIPHEncoder */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_marker_writer     { () => { /* jIMWriter */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_master_decompress { () => { /* jIDMaster */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_d_main_controller { () => { /* jIDMainC */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_d_coef_controller { () => { /* jIDCoefC */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_d_post_controller { () => { /* jIDPostC */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_input_controller  { () => { /* jIInCtlr */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_marker_reader     { () => { /* jIMReader */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_huff_decoder      { () => { /* jIHDecoder */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_phuff_decoder     { () => { /* jIPHDecoder */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_inverse_dct       { () => { /* jIIDCT */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_upsampler         { () => { /* jIUpsampler */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_color_deconverter { () => { /* jIDColor */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_1pass_quantizer   { () => { /* jI1Quant */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_2pass_quantizer   { () => { /* jI2Quant */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_merged_upsampler  { () => { /* jIMUpsampler */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jinit_memory_mgr        { () => { /* jIMemMgr */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jdiv_round_up           { () => { /* jDivRound */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jround_up               { () => { /* jRound */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jcopy_sample_rows       { () => { /* jCopySamples */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jcopy_block_row         { () => { /* jCopyBlocks */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jzero_far               { () => { /* jZeroFar */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_zigzag_order       { () => { /* jZIGTable */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_natural_order      { () => { /* jZAGTable */ } }

/**
  | Compression module initialization
  | routines
  |
  */
lazy_static!{
    /*
    EXTERN(c_void) jinit_compress_master JPP((JCompressPtr cinfo));
    EXTERN(c_void) jinit_c_master_control JPP((JCompressPtr cinfo,
                         boolean transcode_only));
    EXTERN(c_void) jinit_c_main_controller JPP((JCompressPtr cinfo,
                          boolean need_full_buffer));
    EXTERN(c_void) jinit_c_prep_controller JPP((JCompressPtr cinfo,
                          boolean need_full_buffer));
    EXTERN(c_void) jinit_c_coef_controller JPP((JCompressPtr cinfo,
                          boolean need_full_buffer));
    EXTERN(c_void) jinit_color_converter JPP((JCompressPtr cinfo));
    EXTERN(c_void) jinit_downsampler JPP((JCompressPtr cinfo));
    EXTERN(c_void) jinit_forward_dct JPP((JCompressPtr cinfo));
    EXTERN(c_void) jinit_huff_encoder JPP((JCompressPtr cinfo));
    EXTERN(c_void) jinit_phuff_encoder JPP((JCompressPtr cinfo));
    EXTERN(c_void) jinit_marker_writer JPP((JCompressPtr cinfo));
    */
}

/**
  | Decompression module initialization
  | routines
  |
  */
lazy_static!{
    /*
    EXTERN(c_void) jinit_master_decompress JPP((JDecompressPtr cinfo));
    EXTERN(c_void) jinit_d_main_controller JPP((JDecompressPtr cinfo,
                          boolean need_full_buffer));
    EXTERN(c_void) jinit_d_coef_controller JPP((JDecompressPtr cinfo,
                          boolean need_full_buffer));
    EXTERN(c_void) jinit_d_post_controller JPP((JDecompressPtr cinfo,
                          boolean need_full_buffer));
    EXTERN(c_void) jinit_input_controller JPP((JDecompressPtr cinfo));
    EXTERN(c_void) jinit_marker_reader JPP((JDecompressPtr cinfo));
    EXTERN(c_void) jinit_huff_decoder JPP((JDecompressPtr cinfo));
    EXTERN(c_void) jinit_phuff_decoder JPP((JDecompressPtr cinfo));
    EXTERN(c_void) jinit_inverse_dct JPP((JDecompressPtr cinfo));
    EXTERN(c_void) jinit_upsampler JPP((JDecompressPtr cinfo));
    EXTERN(c_void) jinit_color_deconverter JPP((JDecompressPtr cinfo));
    EXTERN(c_void) jinit_1pass_quantizer JPP((JDecompressPtr cinfo));
    EXTERN(c_void) jinit_2pass_quantizer JPP((JDecompressPtr cinfo));
    EXTERN(c_void) jinit_merged_upsampler JPP((JDecompressPtr cinfo));
    */
}

/**
  | Memory manager initialization
  |
  */
lazy_static!{
    /*
    EXTERN(c_void) jinit_memory_mgr JPP((JCommonPtr cinfo));
    */
}

/**
  | Utility routines in jutils.c
  |
  */
lazy_static!{
    /*
    EXTERN(long) jdiv_round_up JPP((long a, long b));
    EXTERN(long) jround_up JPP((long a, long b));
    EXTERN(c_void) jcopy_sample_rows JPP((JSAMPARRAY input_array, int source_row, JSAMPARRAY output_array, int dest_row, int num_rows, JDimension num_cols));
    EXTERN(c_void) jcopy_block_row JPP((JBLOCKROW input_row, JBLOCKROW output_row, JDimension num_blocks));
    EXTERN(c_void) jzero_far JPP((c_void FAR * target, size_t bytestozero));
    */
}

/**
  | Constant tables in jutils.c
  |
  */
lazy_static!{
    /*
    #if 0               /* This table is not actually needed in v6a */
    extern const int jpeg_zigzag_order[]; /* natural coef order to zigzag order */
    #endif
    extern const int jpeg_natural_order[]; /* zigzag coef order to natural order */
    */
}

/**
  | Suppress undefined-structure complaints
  | if necessary.
  |
  | only jmemmgr.c defines these
  |
  */
#[cfg(INCOMPLETE_TYPES_BROKEN)]
#[cfg(not(AM_MEMORY_MANAGER))]
pub struct JVirtSArrayControl { dummy: i64, }

#[cfg(INCOMPLETE_TYPES_BROKEN)]
#[cfg(not(AM_MEMORY_MANAGER))]
pub struct JVirtBArrayControl { dummy: i64, }
