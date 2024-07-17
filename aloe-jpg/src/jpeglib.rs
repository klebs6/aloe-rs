/*!
  | jpeglib.h
  | 
  | This file defines the application interface
  | for the JPEG library.
  | 
  | Most applications using the library
  | need only include this file, and perhaps
  | jerror.h if they want to know the exact
  | error codes.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jpeglib.h]

/*
  | First we include the configuration
  | files that record how this installation
  | of the JPEG library is set up. jconfig.h
  | can be generated automatically for
  | many systems. jmorecfg.h contains
  | manual configuration options that
  | most people need not worry about.
  |
  */

/**
  | Version ID for the JPEG library.
  | 
  | Might be useful for tests like "#if JPEG_LIB_VERSION
  | >= 60".
  |
  */
pub const JPEG_LIB_VERSION: usize = 62; //Version 6b 

/*
  | Various constants determining the
  | sizes of things.
  | 
  | All of these are specified by the JPEG
  | standard, so don't change them if you
  | want to be compatible.
  |
  */

/**
  | The basic DCT block is 8x8 samples
  |
  */
pub const DCTSIZE: usize = 8;

/**
  | DCTSIZE squared; # of elements in a block
  |
  */
pub const DCTSIZE2: usize = 64;

/**
  | Quantization tables are numbered 0..3
  |
  */
pub const NUM_QUANT_TBLS: usize = 4;

/**
  | Huffman tables are numbered 0..3
  |
  */
pub const NUM_HUFF_TBLS: usize = 4;

/**
  | Arith-coding tables are numbered 0..15
  |
  */
pub const NUM_ARITH_TBLS: usize = 16;

/**
  | JPEG limit on # of components in one scan
  |
  */
pub const MAX_COMPS_IN_SCAN: usize = 4;

/**
  | JPEG limit on sampling factors
  |
  */
pub const MAX_SAMP_FACTOR: usize = 4;

/*
  | Unfortunately, some bozo at Adobe saw
  | no reason to be bound by the standard;
  | the PostScript DCT filter can emit files
  | with many more than 10 blocks/MCU.
  | 
  | If you happen to run across such a file,
  | you can up D_MAX_BLOCKS_IN_MCU to handle
  | it. We even let you do this from the jconfig.h
  | file. However, we strongly discourage
  | changing C_MAX_BLOCKS_IN_MCU; just
  | because Adobe sometimes emits noncompliant
  | files doesn't mean you should too.
  |
  */

/**
  | compressor's limit on blocks per MCU
  |
  */
pub const C_MAX_BLOCKS_IN_MCU: usize = 10;


/**
  decompressor's limit on blocks per MCU
  */
#[cfg(not(D_MAX_BLOCKS_IN_MCU))]
pub const D_MAX_BLOCKS_IN_MCU: usize = 10;

/*
  | Data structures for images (arrays
  | of samples and of DCT coefficients).
  | 
  | On 80x86 machines, the image arrays
  | are too big for near pointers, but the
  | pointer arrays can fit in near memory.
  |
  */

/**
  | ptr to one image row of pixel samples.
  |
  */
pub type JSampRow = *mut JSample;

/**
  | ptr to some rows (a 2-D sample array)
  |
  */
pub type JSampArray = *mut JSampRow;

/**
  | a 3-D sample array: top index is color
  |
  */
pub type JSampImage = *mut JSampArray;

/**
  | one block of coefficients
  |
  */
pub type JBlock =  [JCoef; DCTSIZE2]; 

/**
  | pointer to one row of coefficient blocks
  |
  */
pub type JBlockRow = *mut JBlock;

/**
  | a 2-D array of coefficient blocks
  |
  */
pub type JBlockArray = *mut JBlockRow;

/**
  | a 3-D array of coefficient blocks
  |
  */
pub type JBlockImage = *mut JBlockArray;

/**
  | useful in a couple of places
  |
  */
pub type JCoefPtr = *mut JCoef;

/*
  | Types for JPEG compression parameters
  | and working tables.
  |
  */

/**
  | DCT coefficient quantization tables.
  |
  */
pub struct JQuantTbl {

    /*
      | This array gives the coefficient quantizers
      | in natural array order (not the zigzag
      | order in which they are stored in a JPEG
      | DQT marker).
      | 
      | CAUTION: IJG versions prior to v6a kept
      | this array in zigzag order.
      |
      */

    /**
      | quantization step for each coefficient
      |
      */
    quantval: [u16; DCTSIZE2],

    /*
      | This field is used only during compression.
      | It's initialized FALSE when the table
      | is created, and set TRUE when it's been
      | output to the file.
      | 
      | You could suppress output of a table
      | by setting this to TRUE. (See jpeg_suppress_tables
      | for an example.)
      |
      */

    /**
      | TRUE when table has been output
      |
      */
    sent_table: bool,
}


/* ------------- Huffman coding tables.  ------------- */

pub struct JHuffTbl {

    /*
      | These two fields directly represent
      | the contents of a JPEG DHT marker
      |
      */

    /**
      | bits[k] = # of symbols with codes of
      |
      */
    bits: [u8; 17],

    /*
      | length k bits; bits[0] is unused
      |
      */

    /**
      | The symbols, in order of incr code length
      |
      */
    huffval: [u8; 256],

    /*
      | This field is used only during compression.
      | It's initialized FALSE when the table
      | is created, and set TRUE when it's been
      | output to the file.
      | 
      | You could suppress output of a table
      | by setting this to TRUE. (See jpeg_suppress_tables
      | for an example.)
      |
      */

    /**
      | TRUE when table has been output
      |
      */
    sent_table: bool,

}

/**
  | Basic info about one component (color
  | channel).
  |
  */
pub struct JpegComponentInfo {

    /*
      | These values are fixed over the whole
      | image.
      | 
      | For compression, they must be supplied
      | by parameter setup; for decompression,
      | they are read from the SOF marker.
      |
      */

    /**
      | identifier for this component (0..255)
      |
      */
    component_id:    i32,

    /**
      | its index in SOF or cinfo->comp_info[]
      |
      */
    component_index: i32,

    /**
      | horizontal sampling factor (1..4)
      |
      */
    h_samp_factor:   i32,

    /**
      | vertical sampling factor (1..4)
      |
      */
    samp_factor:     i32,

    /**
      | quantization table selector (0..3)
      |
      */
    quant_tbl_no:    i32,

    /*
      | These values may vary between scans.
      | 
      | For compression, they must be supplied
      | by parameter setup; for decompression,
      | they are read from the SOS marker.
      | 
      | The decompressor output side may not
      | use these variables.
      |
      */

    /**
      | DC entropy table selector (0..3)
      |
      */
    dc_tbl_no: i32,


    /**
      | AC entropy table selector (0..3)
      |
      */
    ac_tbl_no: i32,

    /*
      | Remaining fields should be treated
      | as private by applications.
      |
      */

    /*
      | These values are computed during compression
      | or decompression startup:
      |
      */

    /**
      | Component's size in DCT blocks.
      | 
      | Any dummy blocks added to complete an
      | MCU are not counted; therefore these
      | values do not depend on whether a scan
      | is interleaved or not.
      |
      */
    width_in_blocks:  JDimension,
    height_in_blocks: JDimension,

    /**
      | Size of a DCT block in samples. Always
      | DCTSIZE for compression.
      | 
      | For decompression this is the size of
      | the output from one DCT block, reflecting
      | any scaling we choose to apply during
      | the IDCT step.
      | 
      | Values of 1,2,4,8 are likely to be supported.
      | Note that different components may
      | receive different IDCT scalings.
      |
      */
    dct_scaled_size: i32,

    /*
      | The downsampled dimensions are the
      | component's actual, unpadded number
      | of samples at the main buffer (preprocessing/compression
      | interface), thus downsampled_width
      | = ceil(image_width * Hi/Hmax) and similarly
      | for height. For decompression, IDCT
      | scaling is included, so downsampled_width
      | = ceil(image_width * Hi/Hmax * DCT_scaled_size/DCTSIZE)
      |
      */

    /**
      | actual width in samples
      |
      */
    downsampled_width:  JDimension,


    /**
      | actual height in samples
      |
      */
    downsampled_height: JDimension,

    /*
      | This flag is used only for decompression.
      | In cases where some of the components
      | will be ignored (eg grayscale output
      | from YCbCr image), we can skip most computations
      | for the unused components.
      |
      */

    /**
      | do we need the value of this component?
      |
      */
    component_needed: bool,

    /*
      | These values are computed before starting
      | a scan of the component.
      | 
      | The decompressor output side may not
      | use these variables.
      |
      */

    /**
      | number of blocks per MCU, horizontally
      |
      */
    mcu_width:        i32,


    /**
      | number of blocks per MCU, vertically
      |
      */
    mcu_height:       i32,


    /**
      | MCU_width * MCU_height
      |
      */
    mcu_blocks:       i32,


    /**
      | MCU width in samples, MCU_width*DCT_scaled_size
      |
      */
    mcu_sample_width: i32,


    /**
      | # of non-dummy blocks across in last
      | MCU
      |
      */
    last_col_width:   i32,


    /**
      | # of non-dummy blocks down in last MCU
      |
      */
    last_row_height:  i32,

    /**
      | Saved quantization table for component;
      | NULL if none yet saved.
      | 
      | See jdinput.c comments about the need
      | for this information.
      | 
      | This field is currently used only for
      | decompression.
      |
      */
    quant_table: *mut JQuantTbl,

    /**
      | Private per-component storage for
      | DCT or IDCT subsystem.
      |
      */
    dct_table: *mut c_void,
}

/**
  | The script for encoding a multiple-scan
  | file is an array of these:
  |
  */
pub struct JpegScanInfo {

    /**
      | number of components encoded in this
      | scan
      |
      */
    comps_in_scan:   i32,

    /**
      | their SOF/comp_info[] indexes
      |
      */
    component_index: [i32; MAX_COMPS_IN_SCAN],

    /**
      | progressive JPEG spectral selection
      | parms
      |
      */
    ss:              i32,

    /**
      | progressive JPEG spectral selection
      | parms
      |
      */
    se:              i32,

    /**
      | progressive JPEG successive approx.
      | parms
      |
      */
    ah:              i32,

    /**
      | progressive JPEG successive approx.
      | parms
      |
      */
    al:              i32,
}

/**
  | The decompressor can save APPn and COM
  | markers in a list of these:
  |
  */
pub type JpegSavedMarkerPtr = *mut JpegMarkerStruct;

pub struct JpegMarkerStruct {

    /**
      | next in list, or NULL
      |
      */
    next:            JpegSavedMarkerPtr,

    /**
      | marker code: JPEG_COM, or JPEG_APP0+n
      |
      */
    marker:          u8,

    /**
      | # bytes of data in the file
      |
      */
    original_length: u32,

    /**
      | # bytes of data saved at data[]
      |
      */
    data_length:     u32,

    /**
      | the data contained in the marker
      |
      */
    data:            *mut JOctet,

    /*
      | the marker length word is not counted
      | in data_length or original_length
      |
      */
}

/**
  | Known color spaces.
  |
  */
pub enum JColorSpace {

    /**
      | error/unspecified
      |
      */
    JCS_UNKNOWN,        

    /**
      | monochrome
      |
      */
    JCS_GRAYSCALE,      

    /**
      | red/green/blue
      |
      */
    JCS_RGB,        

    /**
      | Y/Cb/Cr (also known as YUV)
      |
      */
    JCS_YCbCr,      

    /**
      | C/M/Y/K
      |
      */
    JCS_CMYK,       

    /**
      | Y/Cb/Cr/K
      |
      */
    JCS_YCCK,        
}

/**
  | DCT/IDCT algorithm options.
  |
  */
pub enum JDctMethod {

    /**
      | slow but accurate integer algorithm
      |
      */
    JDCT_ISLOW,     

    /**
      | faster, less accurate integer method
      |
      */
    JDCT_IFAST,     

    /**
      | floating-point: accurate, fast on
      | fast HW
      |
      */
    JDCT_FLOAT,      
}

/**
  | may be overridden in jconfig.h
  |
  */
macro_rules! jdct_default {
    () => {
        /*
                JDCT_ISLOW
        */
    }
}

/**
  | may be overridden in jconfig.h
  |
  */
macro_rules! jdct_fastest {
    () => {
        /*
                JDCT_IFAST
        */
    }
}

/**
  | Dithering options for decompression.
  |
  */
pub enum JDitherMode {

    /**
      | no dithering
      |
      */
    JDITHER_NONE,       

    /**
      | simple ordered dither
      |
      */
    JDITHER_ORDERED,    

    /**
      | Floyd-Steinberg error diffusion dither
      |
      */
    JDITHER_FS,      
}

/**
  | Common fields between JPEG compression
  | and decompression master structs.
  |
  */
pub struct JpegCommonFields {

    /**
      | Error handler module
      |
      */
    err:             *mut JpegErrorMgr,

    /**
      | Memory manager module
      |
      */
    mem:             *mut JpegMemoryMgr,

    /**
      | Progress monitor, or NULL if none
      |
      */
    progress:        *mut JpegProgressMgr,

    /**
      | Available for use by application
      |
      */
    client_data:     *mut c_void,

    /**
      | So common code can tell which is which
      |
      */
    is_decompressor: bool,

    /**
      | For checking call sequence validity
      |
      */
    global_state:    i32,
}

/**
  | Routines that are to be used by both halves
  | of the library are declared to receive
  | a pointer to this structure. There are
  | no actual instances of jpeg_common_struct,
  | only of jpeg_compress_struct and jpeg_decompress_struct.
  |
  */
pub struct JpegCommonStruct {

    /**
      | Fields common to both master struct
      | types
      |
      */
    common: JpegCommonFields,

    /*
      | Additional fields follow in an actual
      | jpeg_compress_struct or jpeg_decompress_struct.
      | All three structs must agree on these
      | initial fields! (This would be a lot
      | cleaner in C++.)
      |
      */
}

pub type JCommonPtr     = *mut JpegCommonStruct;
pub type JCompressPtr   = *mut JpegCompressStruct;
pub type JDecompressPtr = *mut JpegDecompressStruct;

/**
  | Master record for a compression instance
  |
  */
pub struct JpegCompressStruct {

    /**
      | Fields shared with jpeg_decompress_struct
      |
      */
    common: JpegCommonFields,

    /**
      | Destination for compressed data
      |
      */
    dest:   *mut JpegDestinationMgr,

    /*
      | Description of source image --- these
      | fields must be filled in by outer application
      | before starting compression. in_color_space
      | must be correct before you can even call
      | jpeg_set_defaults().
      |
      */

    /**
      | input image width
      |
      */
    image_width:      JDimension,

    /**
      | input image height
      |
      */
    image_height:     JDimension,

    /**
      | # of color components in input image
      |
      */
    input_components: i32,

    /**
      | colorspace of input image
      |
      */
    in_color_space:   JColorSpace,

    /**
      | image gamma of input image
      |
      */
    input_gamma:      f64,

    /*
      | Compression parameters --- these fields
      | must be set before calling jpeg_start_compress().
      | We recommend calling jpeg_set_defaults()
      | to initialize everything to reasonable
      | defaults, then changing anything the
      | application specifically wants to
      | change. That way you won't get burnt
      | when new parameters are added. Also
      | note that there are several helper routines
      | to simplify changing parameters.
      |
      */

    /**
      | bits of precision in image data
      |
      */
    data_precision:   i32,

    /**
      | # of color components in JPEG image
      |
      */
    num_components:   i32,

    /**
      | colorspace of JPEG image
      |
      */
    jpeg_color_space: JColorSpace,

    /**
      | comp_info[i] describes component
      | that appears i'th in SOF
      |
      */
    comp_info:        *mut JpegComponentInfo,

    /**
      | ptrs to coefficient quantization tables,
      | or NULL if not defined
      |
      */
    quant_tbl_ptrs:   [*mut JQuantTbl; NUM_QUANT_TBLS],

    /**
      | ptrs to Huffman coding tables, or NULL
      | if not defined
      |
      */
    dc_huff_tbl_ptrs: [*mut JHuffTbl; NUM_HUFF_TBLS],
    ac_huff_tbl_ptrs: [*mut JHuffTbl; NUM_HUFF_TBLS],

    /**
      | L values for DC arith-coding tables
      |
      */
    arith_dc_l:       [u8; NUM_ARITH_TBLS],

    /**
      | U values for DC arith-coding tables
      |
      */
    arith_dc_u:       [u8; NUM_ARITH_TBLS],

    /**
      | Kx values for AC arith-coding tables
      |
      */
    arith_ac_k:       [u8; NUM_ARITH_TBLS],

    /**
      | # of entries in scan_info array
      |
      */
    num_scans: i32,

    /**
      | script for multi-scan file, or NULL
      |
      */
    scan_info: *const JpegScanInfo,

    /*
      | The default value of scan_info is NULL,
      | which causes a single-scan sequential
      | JPEG file to be emitted. To create a multi-scan
      | file, set num_scans and scan_info to
      | point to an array of scan definitions.
      |
      */

    /**
      | TRUE=caller supplies downsampled
      | data
      |
      */
    raw_data_in:      bool,

    /**
      | TRUE=arithmetic coding, FALSE=Huffman
      |
      */
    arith_code:       bool,

    /**
      | TRUE=optimize entropy encoding parms
      |
      */
    optimize_coding:  bool,

    /**
      | TRUE=first samples are cosited
      |
      */
    ccir601_sampling: bool,

    /**
      | 1..100, or 0 for no input smoothing
      |
      */
    smoothing_factor: i32,

    /**
      | DCT algorithm selector
      |
      */
    dct_method:       JDctMethod,

    /*
      | The restart interval can be specified
      | in absolute MCUs by setting restart_interval,
      | or in MCU rows by setting restart_in_rows
      | (in which case the correct restart_interval
      | will be figured for each scan).
      |
      */

    /**
      | MCUs per restart, or 0 for no restart
      |
      */
    restart_interval: u32,


    /**
      | if > 0, MCU rows per restart interval
      |
      */
    restart_in_rows:  i32,

    /*
      | Parameters controlling emission of
      | special markers.
      |
      */

    /**
      | should a JFIF marker be written?
      |
      */
    write_jfif_header:  bool,

    /**
      | What to write for the JFIF version number
      |
      */
    jfif_major_version: u8,
    jfif_minor_version: u8,

    /*
      | These three values are not used by the
      | JPEG code, merely copied into the JFIF
      | APP0 marker. density_unit can be 0 for
      | unknown, 1 for dots/inch, or 2 for dots/cm.
      | Note that the pixel aspect ratio is defined
      | by X_density/Y_density even when density_unit=0.
      |
      */

    /**
      | JFIF code for pixel size units
      |
      */
    density_unit:       u8,

    /**
      | Horizontal pixel density
      |
      */
    x_density:          u16,

    /**
      | Vertical pixel density
      |
      */
    y_density:          u16,

    /**
      | should an Adobe marker be written?
      |
      */
    write_adobe_marker: bool,

    /*
      | State variable: index of next scanline
      | to be written to jpeg_write_scanlines().
      | Application may use this to control
      | its processing loop, e.g., "while (next_scanline
      | < image_height)".
      |
      */

    /**
      | 0 .. image_height-1
      |
      */
    next_scanline: JDimension,

    /*
      | Remaining fields are known throughout
      | compressor, but generally should not
      | be touched by a surrounding application.
      |
      | These fields are computed during compression
      | startup
      |
      */

    /**
      | TRUE if scan script uses progressive
      | mode
      |
      */
    progressive_mode:  bool,

    /**
      | largest h_samp_factor
      |
      */
    max_h_samp_factor: i32,

    /**
      | largest v_samp_factor
      |
      */
    max_v_samp_factor: i32,

    /**
      | # of iMCU rows to be input to coef ctlr
      |
      */
    total_imcu_rows: JDimension,

    /*
      | The coefficient controller receives
      | data in units of MCU rows as defined for
      | fully interleaved scans (whether the
      | JPEG file is interleaved or not).
      | 
      | There are v_samp_factor * DCTSIZE sample
      | rows of each component in an "iMCU" (interleaved
      | MCU) row.
      |
      | These fields are valid during any one
      | scan.
      | 
      | They describe the components and MCUs
      | actually appearing in the scan.
      |
      */

    /**
      | # of JPEG components in this scan
      |
      */
    comps_in_scan: i32,

    /**
      | cur_comp_info[i] describes component
      | that appears i'th in SOS
      |
      */
    cur_comp_info: [*mut JpegComponentInfo; MAX_COMPS_IN_SCAN],

    /**
      | # of MCUs across the image
      |
      */
    mc_us_per_row:    JDimension,

    /**
      | # of MCU rows in the image
      |
      */
    mcu_rows_in_scan: JDimension,

    /**
      | # of DCT blocks per MCU
      |
      */
    blocks_in_mcu:    i32,

    /**
      | MCU_membership[i] is index in cur_comp_info
      | of component owning i'th block in an
      | MCU
      |
      */
    mcu_membership:   [i32; C_MAX_BLOCKS_IN_MCU],

    /* ------ progressive JPEG parameters for scan  ------ */
    ss: i32,
    se: i32,
    ah: i32,
    al: i32,

    /**
      | Links to compression subobjects (methods
      | and private variables of modules)
      |
      */
    master:            *mut JpegCompMaster,
    main:              *mut JpegCMainController,
    prep:              *mut JpegCPrepController,
    coef:              *mut JpegCCoefController,
    marker:            *mut JpegMarkerWriter,
    cconvert:          *mut JpegColorConverter,
    downsample:        *mut JpegDownsampler,
    fdct:              *mut JpegForwardDct,
    entropy:           *mut JpegEntropyEncoder,

    /**
      | workspace for jpeg_simple_progression
      |
      */
    script_space:      *mut JpegScanInfo,

    script_space_size: i32,
}

/**
  | Master record for a decompression instance
  |
  */
pub struct JpegDecompressStruct {

    /**
      | Fields shared with jpeg_compress_struct
      |
      */
    common: JpegCommonFields,

    /**
      | Source of compressed data
      |
      */
    src:    *mut JpegSourceMgr,

    /*
      | Basic description of image --- filled
      | in by jpeg_read_header().
      | 
      | Application may inspect these values
      | to decide how to process image.
      |
      */

    /**
      | nominal image width (from SOF marker)
      |
      */
    image_width:      JDimension,

    /**
      | nominal image height
      |
      */
    image_height:     JDimension,

    /**
      | # of color components in JPEG image
      |
      */
    num_components:   i32,

    /**
      | colorspace of JPEG image
      |
      */
    jpeg_color_space: JColorSpace,

    /*
      | Decompression processing parameters
      | --- these fields must be set before calling
      | jpeg_start_decompress(). Note that
      | jpeg_read_header() initializes them
      | to default values.
      |
      */

    /**
      | colorspace for output
      |
      */
    out_color_space:     JColorSpace,

    /**
      | fraction by which to scale image
      |
      */
    scale_num:           u32,

    /**
      | fraction by which to scale image
      |
      */
    scale_denom:         u32,

    /**
      | image gamma wanted in output
      |
      */
    output_gamma:        f64,

    /**
      | TRUE=multiple output passes
      |
      */
    buffered_image:      bool,

    /**
      | TRUE=downsampled data wanted
      |
      */
    raw_data_out:        bool,

    /**
      | IDCT algorithm selector
      |
      */
    dct_method:          JDctMethod,

    /**
      | TRUE=apply fancy upsampling
      |
      */
    do_fancy_upsampling: bool,

    /**
      | TRUE=apply interblock smoothing
      |
      */
    do_block_smoothing:  bool,

    /**
      | TRUE=colormapped output wanted
      |
      */
    quantize_colors: bool,

    /*
      | the following are ignored if not quantize_colors:
      |
      */

    /**
      | type of color dithering to use
      |
      */
    dither_mode:              JDitherMode,

    /**
      | TRUE=use two-pass color quantization
      |
      */
    two_pass_quantize:        bool,

    /**
      | max # colors to use in created colormap
      |
      */
    desired_number_of_colors: i32,

    /*
      | these are significant only in buffered-image
      | mode:
      |
      */

    /**
      | enable future use of 1-pass quantizer
      |
      */
    enable_1pass_quant:    bool,

    /**
      | enable future use of external colormap
      |
      */
    enable_external_quant: bool,

    /**
      | enable future use of 2-pass quantizer
      |
      */
    enable_2pass_quant:    bool,

    /*
      | Description of actual output image
      | that will be returned to application.
      | 
      | These fields are computed by jpeg_start_decompress().
      | 
      | You can also use jpeg_calc_output_dimensions()
      | to determine these values in advance
      | of calling jpeg_start_decompress().
      |
      */

    /**
      | scaled image width
      |
      */
    output_width:         JDimension,

    /**
      | scaled image height
      |
      */
    output_height:        JDimension,

    /**
      | # of color components in out_color_space
      |
      */
    out_color_components: i32,

    /**
      | # of color components returned
      |
      */
    output_components:    i32,

    /*
      | output_components is 1 (a colormap
      | index) when quantizing colors; otherwise
      | it equals out_color_components.
      |
      */

    /**
      | min recommended height of scanline
      | buffer
      |
      */
    rec_outbuf_height: i32,

    /*
      | If the buffer passed to jpeg_read_scanlines()
      | is less than this many rows high, space
      | and time will be wasted due to unnecessary
      | data copying.
      | 
      | Usually rec_outbuf_height will be
      | 1 or 2, at most 4.
      |
      | When quantizing colors, the output
      | colormap is described by these fields.
      | 
      | The application can supply a colormap
      | by setting colormap non-NULL before
      | calling jpeg_start_decompress; otherwise
      | a colormap is created during jpeg_start_decompress
      | or jpeg_start_output.
      | 
      | The map has out_color_components rows
      | and actual_number_of_colors columns.
      |
      */

    /**
      | number of entries in use
      |
      */
    actual_number_of_colors: i32,


    /**
      | The color map as a 2-D pixel array
      |
      */
    colormap:                JSampArray,

    /*
      | State variables: these variables indicate
      | the progress of decompression.
      | 
      | The application may examine these but
      | must not modify them.
      |
      */

    /**
      | Row index of next scanline to be read
      | from jpeg_read_scanlines().
      | 
      | Application may use this to control
      | its processing loop, e.g., "while (output_scanline
      | < output_height)".
      |
      | 0 .. output_height-1
      |
      */
    output_scanline: JDimension,

    /**
      | Current input scan number and number
      | of iMCU rows completed in scan.
      | 
      | These indicate the progress of the decompressor
      | input side.
      |
      | Number of SOS markers seen so far
      |
      */
    input_scan_number: i32,

    /**
      | Number of iMCU rows completed
      |
      */
    input_imcu_row:    JDimension,

    /**
      | The "output scan number" is the notional
      | scan being displayed by the output side.
      | The decompressor will not allow output
      | scan/row number to get ahead of input
      | scan/row, but it can fall arbitrarily
      | far behind.
      |
      | Nominal scan number being displayed
      |
      */
    output_scan_number: i32,

    /**
      | Number of iMCU rows read
      |
      */
    output_imcu_row:    JDimension,

    /**
      | Current progression status. coef_bits[c][i]
      | indicates the precision with which
      | component c's DCT coefficient i (in
      | zigzag order) is known.
      | 
      | It is -1 when no data has yet been received,
      | otherwise it is the point transform
      | (shift) value for the most recent scan
      | of the coefficient (thus, 0 at completion
      | of the progression).
      | 
      | This pointer is NULL when reading a non-progressive
      | file.
      |
      | -1 or current Al value for each coef
      |
      */
    coef_bits: [*mut i32; DCTSIZE2],

    /**
      | Internal JPEG parameters --- the application
      | usually need not look at these fields.
      | Note that the decompressor output side
      | may not use any parameters that can change
      | between scans.
      |
      | Quantization and Huffman tables are
      | carried forward across input datastreams
      | when processing abbreviated JPEG datastreams.
      |
      | ptrs to coefficient quantization tables,
      | or NULL if not defined
      |
      */
    quant_tbl_ptrs: [*mut JQuantTbl; NUM_QUANT_TBLS],

    /**
      | ptrs to Huffman coding tables, or NULL
      | if not defined
      |
      */
    dc_huff_tbl_ptrs: [*mut JHuffTbl; NUM_HUFF_TBLS],
    ac_huff_tbl_ptrs: [*mut JHuffTbl; NUM_HUFF_TBLS],

    /*
      | These parameters are never carried
      | across datastreams, since they are
      | given in SOF/SOS markers or defined
      | to be reset by SOI.
      |
      */

    /**
      | bits of precision in image data
      |
      */
    data_precision: i32,

    /**
      | comp_info[i] describes component
      | that appears i'th in SOF
      |
      */
    comp_info:        *mut JpegComponentInfo,

    /**
      | TRUE if SOFn specifies progressive
      | mode
      |
      */
    progressive_mode: bool,

    /**
      | TRUE=arithmetic coding, FALSE=Huffman
      |
      */
    arith_code:       bool,

    /**
      | L values for DC arith-coding tables
      |
      */
    arith_dc_l:       [u8; NUM_ARITH_TBLS],

    /**
      | U values for DC arith-coding tables
      |
      */
    arith_dc_u:       [u8; NUM_ARITH_TBLS],

    /**
      | Kx values for AC arith-coding tables
      |
      */
    arith_ac_k:       [u8; NUM_ARITH_TBLS],

    /**
      | MCUs per restart interval, or 0 for no
      | restart
      |
      */
    restart_interval: u32,

    /*
      | These fields record data obtained from
      | optional markers recognized by the
      | JPEG library.
      |
      */

    /**
      | TRUE iff a JFIF APP0 marker was found
      |
      */
    saw_jfif_marker: bool,

    /*
      | Data copied from JFIF marker; only valid
      | if saw_JFIF_marker is TRUE:
      |
      */

    /**
      | JFIF version number
      |
      */
    jfif_major_version: u8,
    jfif_minor_version: u8,

    /**
      | JFIF code for pixel size units
      |
      */
    density_unit:       u8,

    /**
      | Horizontal pixel density
      |
      */
    x_density:          u16,

    /**
      | Vertical pixel density
      |
      */
    y_density:          u16,

    /**
      | TRUE iff an Adobe APP14 marker was found
      |
      */
    saw_adobe_marker:   bool,

    /**
      | Color transform code from Adobe marker
      |
      */
    adobe_transform:    u8,

    /**
      | TRUE=first samples are cosited
      |
      */
    ccir601_sampling: bool,

    /*
      | Aside from the specific data retained
      | from APPn markers known to the library,
      | the uninterpreted contents of any or
      | all APPn and COM markers can be saved
      | in a list for examination by the application.
      |
      */

    /**
      | Head of list of saved markers
      |
      */
    marker_list: JpegSavedMarkerPtr,

    /*
      | Remaining fields are known throughout
      | decompressor, but generally should
      | not be touched by a surrounding application.
      |
      | These fields are computed during decompression
      | startup
      |
      */

    /**
      | largest h_samp_factor
      |
      */
    max_h_samp_factor: i32,

    /**
      | largest v_samp_factor
      |
      */
    max_v_samp_factor: i32,

    /**
      | smallest DCT_scaled_size of any component
      |
      */
    min_dct_scaled_size: i32,

    /**
      | # of iMCU rows in image
      |
      */
    total_imcu_rows: JDimension,

    /*
      | The coefficient controller's input
      | and output progress is measured in units
      | of "iMCU" (interleaved MCU) rows. These
      | are the same as MCU rows in fully interleaved
      | JPEG scans, but are used whether the
      | scan is interleaved or not. We define
      | an iMCU row as v_samp_factor DCT block
      | rows of each component. Therefore,
      | the IDCT output contains v_samp_factor*DCT_scaled_size
      | sample rows of a component per iMCU row.
      |
      */

    /**
      | table for fast range-limiting
      |
      */
    sample_range_limit: *mut JSample,

    /*
      | These fields are valid during any one
      | scan.
      | 
      | They describe the components and MCUs
      | actually appearing in the scan.
      | 
      | -----------
      | @note
      | 
      | the decompressor output side must not
      | use these fields.
      |
      */

    /**
      | # of JPEG components in this scan
      |
      */
    comps_in_scan:    i32,

    /**
      | cur_comp_info[i] describes component
      | that appears i'th in SOS
      |
      */
    cur_comp_info:    [*mut JpegComponentInfo; MAX_COMPS_IN_SCAN],

    /**
      | # of MCUs across the image
      |
      */
    mc_us_per_row:    JDimension,

    /**
      | # of MCU rows in the image
      |
      */
    mcu_rows_in_scan: JDimension,

    /**
      | # of DCT blocks per MCU
      |
      */
    blocks_in_mcu:    i32,

    /**
      | MCU_membership[i] is index in cur_comp_info
      | of component owning i'th block in an
      | MCU
      |
      */
    mcu_membership:   [i32; D_MAX_BLOCKS_IN_MCU],

    /* ------ progressive JPEG parameters for scan  ------ */
    ss: i32,
    se: i32,
    ah: i32,
    al: i32,

    /**
      | This field is shared between entropy
      | decoder and marker parser.
      | 
      | It is either zero or the code of a JPEG
      | marker that has been read from the data
      | source, but has not yet been processed.
      |
      */
    unread_marker: i32,

    /**
      | Links to decompression subobjects
      | (methods, private variables of modules)
      |
      */
    master:    *mut JpegDecompMaster,
    main:      *mut JpegDMainController,
    coef:      *mut JpegDCoefController,
    post:      *mut JpegDPostController,
    inputctl:  *mut JpegInputController,
    marker:    *mut JpegMarkerReader,
    entropy:   *mut JpegEntropyDecoder,
    idct:      *mut JpegInverseDct,
    upsample:  *mut JpegUpsampler,
    cconvert:  *mut JpegColorDeconverter,
    cquantize: *mut JpegColorQuantizer,
}

/*
  | "Object" declarations for JPEG modules
  | that may be supplied or called directly
  | by the surrounding application.
  | 
  | As with all objects in the JPEG library,
  | these structs only define the publicly
  | visible methods and state variables
  | of a module. Additional private fields
  | may exist after the public ones.
  |
  */

/**
  | recommended size of format_message
  | buffer
  |
  */
pub const JMSG_LENGTH_MAX:   usize = 200;
pub const JMSG_STR_PARM_MAX: usize = 80;

pub mod jpeg_error_mgr {
    use super::*;
    pub union U {
        i: [i32; 8],
        s: [u8; JMSG_STR_PARM_MAX],
    }
}

/**
  | Error handler object
  |
  */
pub struct JpegErrorMgr {

    /**
      | Error exit handler: does not return
      | to caller
      |
      */
    error_exit: fn(cinfo: JCommonPtr) -> c_void,

    /**
      | Conditionally emit a trace or warning
      | message
      |
      */
    emit_message: fn(cinfo: JCommonPtr, msg_level: i32) -> c_void,

    /**
      | Routine that actually outputs a trace
      | or error message
      |
      */
    output_message: fn(cinfo: JCommonPtr) -> c_void,

    /**
      | Format a message string for the most
      | recent JPEG error or message
      |
      */
    format_message: fn(cinfo: JCommonPtr, buffer: *mut u8) -> c_void,

    /**
      | Reset error state variables at start
      | of a new image
      |
      */
    reset_error_mgr: fn(cinfo: JCommonPtr) -> c_void,

    /**
      | The message ID code and any parameters
      | are saved here.
      | 
      | A message can have one string parameter
      | or up to 8 int parameters.
      |
      */
    msg_code: i32,

    msg_parm: jpeg_error_mgr::U,

    /*
      | Standard state variables for error
      | facility
      |
      */

    /**
      | max msg_level that will be displayed
      |
      */
    trace_level: i32,

    /*
      | For recoverable corrupt-data errors,
      | we emit a warning message, but keep going
      | unless emit_message chooses to abort.
      | emit_message should count warnings
      | in num_warnings. The surrounding application
      | can check for bad data by seeing if num_warnings
      | is nonzero at the end of processing.
      |
      */

    /**
      | number of corrupt-data warnings
      |
      */
    num_warnings: i64,

    /*
      | These fields point to the table(s) of
      | error message strings.
      | 
      | An application can change the table
      | pointer to switch to a different message
      | list (typically, to change the language
      | in which errors are reported). Some
      | applications may wish to add additional
      | error codes that will be handled by the
      | JPEG library error mechanism; the second
      | table pointer is used for this purpose.
      | 
      | First table includes all errors generated
      | by JPEG library itself.
      | 
      | Error code 0 is reserved for a "no such
      | error string" message.
      |
      */

    /**
      | Library errors
      |
      */
    jpeg_message_table: *const *const u8,

    /**
      | Table contains strings 0..last_jpeg_message
      |
      */
    last_jpeg_message:  i32,

    /*
      | Second table can be added by application
      | (see cjpeg/djpeg for example).
      | 
      | It contains strings numbered first_addon_message..last_addon_message.
      |
      */

    /**
      | Non-library errors
      |
      */
    addon_message_table: *const *const u8,

    /**
      | code for first string in addon table
      |
      */
    first_addon_message: i32,

    /**
      | code for last string in addon table
      |
      */
    last_addon_message:  i32,
}

/**
  | Progress monitor object
  |
  */
pub struct JpegProgressMgr {

    progress_monitor: fn(cinfo: JCommonPtr) -> c_void,

    /**
      | work units completed in this pass
      |
      */
    pass_counter:     i64,


    /**
      | total number of work units in this pass
      |
      */
    pass_limit:       i64,


    /**
      | passes completed so far
      |
      */
    completed_passes: i32,


    /**
      | total number of passes expected
      |
      */
    total_passes:     i32,
}

/**
  | Data destination object for compression
  |
  */
pub struct JpegDestinationMgr {

    /**
      | => next byte to write in buffer
      |
      */
    next_output_byte:    *mut JOctet,


    /**
      | # of byte spaces remaining in buffer
      |
      */
    free_in_buffer:      usize,

    init_destination:    fn(cinfo: JCompressPtr) -> c_void,
    empty_output_buffer: fn(cinfo: JCompressPtr) -> bool,
    term_destination:    fn(cinfo: JCompressPtr) -> c_void,
}

/**
  | Data source object for decompression
  |
  */
pub struct JpegSourceMgr {

    /**
      | => next byte to read from buffer
      |
      */
    next_input_byte:   *const JOctet,

    /**
      | # of bytes remaining in buffer
      |
      */
    bytes_in_buffer:   usize,

    init_source:       fn(cinfo: JDecompressPtr) -> c_void,
    fill_input_buffer: fn(cinfo: JDecompressPtr) -> bool,
    skip_input_data:   fn(cinfo: JDecompressPtr, num_bytes: i64) -> c_void,
    resync_to_restart: fn(cinfo: JDecompressPtr, desired: i32) -> bool,
    term_source:       fn(cinfo: JDecompressPtr) -> c_void,
}

/*
  | Memory manager object.
  | 
  | Allocates "small" objects (a few K total),
  | "large" objects (tens of K), and "really
  | big" objects (virtual arrays with backing
  | store if needed).
  | 
  | The memory manager does not allow individual
  | objects to be freed; rather, each created
  | object is assigned to a pool, and whole
  | pools can be freed at once. This is faster
  | and more convenient than remembering
  | exactly what to free, especially where
  | malloc()/free() are not too speedy.
  | 
  | NB: alloc routines never return NULL.
  | They exit to error_exit if not successful.
  |
  */

/**
  lasts until master record is destroyed
  */
pub const JPOOL_PERMANENT: usize = 0;

/**
  lasts until done with image/datastream
  */
pub const JPOOL_IMAGE: usize = 1;

pub const JPOOL_NUMPOOLS: usize = 2;

pub type JVirtSArrayPtr = *mut JVirtSArrayControl;
pub type JVirtBArrayPtr = *mut JVirtBArrayControl;

pub struct JpegMemoryMgr {

    /* ----------------- Method pointers  ----------------- */
    alloc_small:         fn(
            cinfo:        JCommonPtr,
            pool_id:      i32,
            sizeofobject: usize
    ) -> *mut c_void,

    alloc_large:         fn(
            cinfo:        JCommonPtr,
            pool_id:      i32,
            sizeofobject: usize
    ) -> *mut c_void,

    alloc_sarray:        fn(
            cinfo:         JCommonPtr,
            pool_id:       i32,
            samplesperrow: JDimension,
            numrows:       JDimension
    ) -> JSampArray,

    alloc_barray:        fn(
            cinfo:        JCommonPtr,
            pool_id:      i32,
            blocksperrow: JDimension,
            numrows:      JDimension
    ) -> JBlockArray,

    request_virt_sarray: fn(
            cinfo:         JCommonPtr,
            pool_id:       i32,
            pre_zero:      bool,
            samplesperrow: JDimension,
            numrows:       JDimension,
            maxaccess:     JDimension
    ) -> JVirtSArrayPtr,

    request_virt_barray: fn(
            cinfo:        JCommonPtr,
            pool_id:      i32,
            pre_zero:     bool,
            blocksperrow: JDimension,
            numrows:      JDimension,
            maxaccess:    JDimension
    ) -> JVirtBArrayPtr,

    realize_virt_arrays: fn(cinfo: JCommonPtr) -> c_void,

    access_virt_sarray:  fn(
            cinfo:     JCommonPtr,
            ptr:       JVirtSArrayPtr,
            start_row: JDimension,
            num_rows:  JDimension,
            writable:  bool
    ) -> JSampArray,

    access_virt_barray:  fn(
            cinfo:     JCommonPtr,
            ptr:       JVirtBArrayPtr,
            start_row: JDimension,
            num_rows:  JDimension,
            writable:  bool
    ) -> JBlockArray,

    free_pool:           fn(cinfo: JCommonPtr, pool_id: i32) -> c_void,

    self_destruct:       fn(cinfo: JCommonPtr) -> c_void,

    /**
      | Limit on memory allocation for this
      | JPEG object. (Note that this is merely
      | advisory, not a guaranteed maximum;
      | it only affects the space used for virtual-array
      | buffers.) May be changed by outer application
      | after creating the JPEG object.
      |
      */
    max_memory_to_use: i64,

    /**
      | Maximum allocation request accepted
      | by alloc_large.
      |
      */
    max_alloc_chunk:   i64,
}

/**
  | Routine signature for application-supplied
  | marker processing methods.
  | 
  | Need not pass marker code since it is
  | stored in cinfo->unread_marker.
  |
  */
pub type JpegMarkerParserMethod = fn(cinfo: JDecompressPtr) -> bool;

/**
  | Declarations for routines called by
  | application.
  | 
  | The JPP macro hides prototype parameters
  | from compilers that can't cope.
  | 
  | Note JPP requires double parentheses.
  |
  */
#[cfg(HAVE_PROTOTYPES)]
macro_rules! jpp {
    ($arglist:ident) => {
        /*
                arglist
        */
    }
}

#[cfg(not(HAVE_PROTOTYPES))]
macro_rules! jpp {
    ($arglist:ident) => {
        /*
                ()
        */
    }
}

/**
  | Short forms of external names for systems
  | with brain-damaged linkers.
  | 
  | We shorten external names to be unique
  | in the first six letters, which is good
  | enough for all known systems. (If your
  | compiler itself needs names to be unique
  | in less than 15 characters, you are out
  | of luck. Get a better compiler.)
  |
  */
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_std_error                { () => { /* jStdError */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_create_compress          { () => { /* jCreaCompress */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_create_decompress        { () => { /* jCreaDecompress */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_destroy_compress         { () => { /* jDestCompress */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_destroy_decompress       { () => { /* jDestDecompress */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_stdio_dest               { () => { /* jStdDest */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_stdio_src                { () => { /* jStdSrc */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_set_defaults             { () => { /* jSetDefaults */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_set_colorspace           { () => { /* jSetColorspace */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_default_colorspace       { () => { /* jDefColorspace */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_set_quality              { () => { /* jSetQuality */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_set_linear_quality       { () => { /* jSetLQuality */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_add_quant_table          { () => { /* jAddQuantTable */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_quality_scaling          { () => { /* jQualityScaling */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_simple_progression       { () => { /* jSimProgress */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_suppress_tables          { () => { /* jSuppressTables */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_alloc_quant_table        { () => { /* jAlcQTable */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_alloc_huff_table         { () => { /* jAlcHTable */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_start_compress           { () => { /* jStrtCompress */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_write_scanlines          { () => { /* jWrtScanlines */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_finish_compress          { () => { /* jFinCompress */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_write_raw_data           { () => { /* jWrtRawData */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_write_marker             { () => { /* jWrtMarker */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_write_m_header           { () => { /* jWrtMHeader */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_write_m_byte             { () => { /* jWrtMByte */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_write_tables             { () => { /* jWrtTables */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_read_header              { () => { /* jReadHeader */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_start_decompress         { () => { /* jStrtDecompress */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_read_scanlines           { () => { /* jReadScanlines */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_finish_decompress        { () => { /* jFinDecompress */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_read_raw_data            { () => { /* jReadRawData */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_has_multiple_scans       { () => { /* jHasMultScn */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_start_output             { () => { /* jStrtOutput */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_finish_output            { () => { /* jFinOutput */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_input_complete           { () => { /* jInComplete */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_new_colormap             { () => { /* jNewCMap */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_consume_input            { () => { /* jConsumeInput */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_calc_output_dimensions   { () => { /* jCalcDimensions */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_save_markers             { () => { /* jSaveMarkers */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_set_marker_processor     { () => { /* jSetMarker */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_read_coefficients        { () => { /* jReadCoefs */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_write_coefficients       { () => { /* jWrtCoefs */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_copy_critical_parameters { () => { /* jCopyCrit */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_abort_compress           { () => { /* jAbrtCompress */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_abort_decompress         { () => { /* jAbrtDecompress */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_abort                    { () => { /* jAbort */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_destroy                  { () => { /* jDestroy */ } }
#[cfg(NEED_SHORT_EXTERNAL_NAMES)] macro_rules! jpeg_resync_to_restart        { () => { /* jResyncRestart */ } }

/**
  | Default error-management setup
  |
  */
lazy_static!{
    /*
    EXTERN(struct jpeg_error_mgr *) jpeg_std_error
        JPP((struct jpeg_error_mgr * err));
    */
}

/**
  | Initialization of JPEG compression
  | objects. jpeg_create_compress()
  | and jpeg_create_decompress() are
  | the exported names that applications
  | should call. These expand to calls on
  | jpeg_CreateCompress and jpeg_CreateDecompress
  | with additional information passed
  | for version mismatch checking.
  | 
  | NB: you must set up the error-manager
  | BEFORE calling jpeg_create_xxx.
  |
  */
macro_rules! jpeg_create_compress {
    ($cinfo:ident) => {
        /*
        
            jpeg_CreateCompress((cinfo), JPEG_LIB_VERSION, 
                    (size_t) sizeof(struct jpeg_compress_struct))
        */
    }
}

macro_rules! jpeg_create_decompress {
    ($cinfo:ident) => {
        /*
        
            jpeg_CreateDecompress((cinfo), JPEG_LIB_VERSION, 
                      (size_t) sizeof(struct jpeg_decompress_struct))
        */
    }
}

lazy_static!{
    /*
    EXTERN(c_void) jpeg_CreateCompress JPP((JCompressPtr cinfo,
                          int version, size_t structsize));
    EXTERN(c_void) jpeg_CreateDecompress JPP((JDecompressPtr cinfo,
                        int version, size_t structsize));
    /* Destruction of JPEG compression objects */
    EXTERN(c_void) jpeg_destroy_compress JPP((JCompressPtr cinfo));
    EXTERN(c_void) jpeg_destroy_decompress JPP((JDecompressPtr cinfo));

    /* Standard data source and destination managers: stdio streams. */
    /* Caller is responsible for opening the file before and closing after. */
    EXTERN(c_void) jpeg_stdio_dest JPP((JCompressPtr cinfo, FILE * outfile));
    EXTERN(c_void) jpeg_stdio_src JPP((JDecompressPtr cinfo, FILE * infile));

    /* Default parameter setup for compression */
    EXTERN(c_void) jpeg_set_defaults JPP((JCompressPtr cinfo));
    /* Compression parameter setup aids */
    EXTERN(c_void) jpeg_set_colorspace JPP((JCompressPtr cinfo,
                          J_COLOR_SPACE colorspace));
    EXTERN(c_void) jpeg_default_colorspace JPP((JCompressPtr cinfo));
    EXTERN(c_void) jpeg_set_quality JPP((JCompressPtr cinfo, int quality,
                       boolean force_baseline));
    EXTERN(c_void) jpeg_set_linear_quality JPP((JCompressPtr cinfo,
                          int scale_factor,
                          boolean force_baseline));
    EXTERN(c_void) jpeg_add_quant_table JPP((JCompressPtr cinfo, int which_tbl,
                           const unsigned int *basic_table,
                           int scale_factor,
                           boolean force_baseline));
    EXTERN(int) jpeg_quality_scaling JPP((int quality));
    EXTERN(c_void) jpeg_simple_progression JPP((JCompressPtr cinfo));
    EXTERN(c_void) jpeg_suppress_tables JPP((JCompressPtr cinfo,
                           boolean suppress));
    EXTERN(JQUANT_TBL *) jpeg_alloc_quant_table JPP((JCommonPtr cinfo));
    EXTERN(JHUFF_TBL *) jpeg_alloc_huff_table JPP((JCommonPtr cinfo));

    /* Main entry points for compression */
    EXTERN(c_void) jpeg_start_compress JPP((JCompressPtr cinfo,
                          boolean write_all_tables));
    EXTERN(JDimension) jpeg_write_scanlines JPP((JCompressPtr cinfo,
                             JSAMPARRAY scanlines,
                             JDimension num_lines));
    EXTERN(c_void) jpeg_finish_compress JPP((JCompressPtr cinfo));

    /* Replaces jpeg_write_scanlines when writing raw downsampled data. */
    EXTERN(JDimension) jpeg_write_raw_data JPP((JCompressPtr cinfo,
                            JSAMPIMAGE data,
                            JDimension num_lines));

    /* Write a special marker.  See libjpeg.doc concerning safe usage. */
    EXTERN(c_void) jpeg_write_marker
        JPP((JCompressPtr cinfo, int marker,
             const JOctet * dataptr, unsigned int datalen));
    /* Same, but piecemeal. */
    EXTERN(c_void) jpeg_write_m_header
        JPP((JCompressPtr cinfo, int marker, unsigned int datalen));
    EXTERN(c_void) jpeg_write_m_byte
        JPP((JCompressPtr cinfo, int val));

    /* Alternate compression function: just write an abbreviated table file */
    EXTERN(c_void) jpeg_write_tables JPP((JCompressPtr cinfo));

    /* Decompression startup: read start of JPEG datastream to see what's there */
    EXTERN(int) jpeg_read_header JPP((JDecompressPtr cinfo,
                      boolean require_image));
    */
}

/*
  | Return value is one of:
  |
  */

/**
  Suspended due to lack of input data
  */
pub const JPEG_SUSPENDED: usize = 0;

/**
  Found valid image datastream
  */
pub const JPEG_HEADER_OK: usize = 1;

/**
  Found valid table-specs-only datastream
  */
pub const JPEG_HEADER_TABLES_ONLY: usize = 2;

/*
  | If you pass require_image = TRUE (normal
  | case), you need not check for a TABLES_ONLY
  | return code; an abbreviated file will
  | cause an error exit.
  | 
  | JPEG_SUSPENDED is only possible if
  | you use a data source module that can
  | give a suspension return (the stdio
  | source module doesn't).
  |
  */

/**
  | Main entry points for decompression
  |
  */
lazy_static!{
    /*
    EXTERN(boolean) jpeg_start_decompress JPP((JDecompressPtr cinfo));
    EXTERN(JDimension) jpeg_read_scanlines JPP((JDecompressPtr cinfo,
                            JSAMPARRAY scanlines,
                            JDimension max_lines));
    EXTERN(boolean) jpeg_finish_decompress JPP((JDecompressPtr cinfo));
    */
}

/**
  | Replaces jpeg_read_scanlines when
  | reading raw downsampled data.
  |
  */
lazy_static!{
    /*
    EXTERN(JDimension) jpeg_read_raw_data JPP((JDecompressPtr cinfo,
                           JSAMPIMAGE data,
                           JDimension max_lines));
    */
}

/**
  | Additional entry points for buffered-image
  | mode.
  |
  */
lazy_static!{
    /*
    EXTERN(boolean) jpeg_has_multiple_scans JPP((JDecompressPtr cinfo));
    EXTERN(boolean) jpeg_start_output JPP((JDecompressPtr cinfo, int scan_number));
    EXTERN(boolean) jpeg_finish_output JPP((JDecompressPtr cinfo));
    EXTERN(boolean) jpeg_input_complete JPP((JDecompressPtr cinfo));
    EXTERN(c_void) jpeg_new_colormap JPP((JDecompressPtr cinfo));
    EXTERN(int) jpeg_consume_input JPP((JDecompressPtr cinfo));
    */
}

/**
  | Return value is one of:
  |
  */

/* #define JPEG_SUSPENDED   0    Suspended due to lack of input data */

/**
  | Reached start of new scan
  |
  */
pub const JPEG_REACHED_SOS:    usize = 1; 

/**
  | Reached end of image
  |
  */
pub const JPEG_REACHED_EOI:    usize = 2; 

/**
  | Completed one iMCU row
  |
  */
pub const JPEG_ROW_COMPLETED:  usize = 3; 

/**
  | Completed last iMCU row of a scan
  |
  */
pub const JPEG_SCAN_COMPLETED: usize = 4; 

/**
  | Precalculate output dimensions for
  | current decompression parameters.
  |
  */
lazy_static!{
    /*
    EXTERN(c_void) jpeg_calc_output_dimensions JPP((JDecompressPtr cinfo));

    /* Control saving of COM and APPn markers into marker_list. */
    EXTERN(c_void) jpeg_save_markers
        JPP((JDecompressPtr cinfo, int marker_code,
             unsigned int length_limit));

    /* Install a special processing method for COM or APPn markers. */
    EXTERN(c_void) jpeg_set_marker_processor
        JPP((JDecompressPtr cinfo, int marker_code,
             jpeg_marker_parser_method routine));

    /* Read or write raw DCT coefficients --- useful for lossless transcoding. */
    EXTERN(jvirt_barray_ptr *) jpeg_read_coefficients JPP((JDecompressPtr cinfo));
    EXTERN(c_void) jpeg_write_coefficients JPP((JCompressPtr cinfo,
                          jvirt_barray_ptr * coef_arrays));
    EXTERN(c_void) jpeg_copy_critical_parameters JPP((JDecompressPtr srcinfo,
                            JCompressPtr dstinfo));

    /* If you choose to abort compression or decompression before completing
     * jpeg_finish_(de)compress, then you need to clean up to release memory,
     * temporary files, etc.  You can just call jpeg_destroy_(de)compress
     * if you're done with the JPEG object, but if you want to clean it up and
     * reuse it, call this:
     */
    EXTERN(c_void) jpeg_abort_compress JPP((JCompressPtr cinfo));
    EXTERN(c_void) jpeg_abort_decompress JPP((JDecompressPtr cinfo));

    /* Generic versions of jpeg_abort and jpeg_destroy that work on either
     * flavor of JPEG object.  These may be more convenient in some places.
     */
    EXTERN(c_void) jpeg_abort JPP((JCommonPtr cinfo));
    EXTERN(c_void) jpeg_destroy JPP((JCommonPtr cinfo));

    /* Default restart-marker-resync procedure for use by data source modules */
    EXTERN(boolean) jpeg_resync_to_restart JPP((JDecompressPtr cinfo,
                            int desired));
    */
}

/*
  | These marker codes are exported since
  | applications and data source modules
  | are likely to want to use them.
  |
  */

/**
  RST0 marker code
  */
pub const JPEG_RST0: usize = 0xD0;

/**
  EOI marker code
  */
pub const JPEG_EOI: usize = 0xD9;

/**
  APP0 marker code
  */
pub const JPEG_APP0: usize = 0xE0;

/**
  COM marker code
  */
pub const JPEG_COM: usize = 0xFE;

/**
  | If we have a brain-damaged compiler
  | that emits warnings (or worse, errors)
  | for structure definitions that are
  | never filled in, keep it quiet by supplying
  | dummy definitions for the various substructures.
  |
  */
lazy_static!{
    /*
    #ifdef INCOMPLETE_TYPES_BROKEN
    #ifndef JPEG_INTERNALS      /* will be defined in jpegint.h */
    struct jvirt_sarray_control { long dummy; };
    struct jvirt_barray_control { long dummy; };
    struct jpeg_comp_master { long dummy; };
    struct jpeg_c_main_controller { long dummy; };
    struct jpeg_c_prep_controller { long dummy; };
    struct jpeg_c_coef_controller { long dummy; };
    struct jpeg_marker_writer { long dummy; };
    struct jpeg_color_converter { long dummy; };
    struct jpeg_downsampler { long dummy; };
    struct jpeg_forward_dct { long dummy; };
    struct jpeg_entropy_encoder { long dummy; };
    struct jpeg_decomp_master { long dummy; };
    struct jpeg_d_main_controller { long dummy; };
    struct jpeg_d_coef_controller { long dummy; };
    struct jpeg_d_post_controller { long dummy; };
    struct jpeg_input_controller { long dummy; };
    struct jpeg_marker_reader { long dummy; };
    struct jpeg_entropy_decoder { long dummy; };
    struct jpeg_inverse_dct { long dummy; };
    struct jpeg_upsampler { long dummy; };
    struct jpeg_color_deconverter { long dummy; };
    struct jpeg_color_quantizer { long dummy; };
    #endif /* JPEG_INTERNALS */
    #endif /* INCOMPLETE_TYPES_BROKEN */
    */
}
