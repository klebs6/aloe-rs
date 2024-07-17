/*!
  | jerror.h
  | 
  | This file defines the error and message
  | codes for the JPEG library.
  | 
  | Edit this file to add new codes, or to
  | translate the message strings to some
  | other language.
  | 
  | A set of error-reporting macros are
  | defined too. Some applications using
  | the JPEG library may wish to include
  | this file to get the error codes and/or
  | the macros.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jerror.h]


/**
  | Must be first entry!
  |
  */
jmessage!{
    JMSG_NOMESSAGE, 
    "Bogus message code %d"
}

/* For maintenance convenience, list is alphabetical by message code name */
jmessage!{
    JERR_ARITH_NOTIMPL, 
    "Sorry, there are legal restrictions on arithmetic coding"
}

jmessage!{
    JERR_BAD_ALIGN_TYPE, 
    "ALIGN_TYPE is wrong, please fix"
}

jmessage!{
    JERR_BAD_ALLOC_CHUNK, 
    "MAX_ALLOC_CHUNK is wrong, please fix"
}

jmessage!{
    JERR_BAD_BUFFER_MODE, 
    "Bogus buffer control mode"
}

jmessage!{
    JERR_BAD_COMPONENT_ID, 
    "Invalid component ID %d in SOS"
}

jmessage!{
    JERR_BAD_DCT_COEF, 
    "DCT coefficient out of range"
}

jmessage!{
    JERR_BAD_DCTSIZE, 
    "IDCT output block size %d not supported"
}

jmessage!{
    JERR_BAD_HUFF_TABLE, 
    "Bogus Huffman table definition"
}

jmessage!{
    JERR_BAD_IN_COLORSPACE, 
    "Bogus input colorspace"
}

jmessage!{
    JERR_BAD_J_COLORSPACE, 
    "Bogus JPEG colorspace"
}

jmessage!{
    JERR_BAD_LENGTH, 
    "Bogus marker length"
}

jmessage!{
    JERR_BAD_LIB_VERSION, 
    "Wrong JPEG library version: library is %d, caller expects %d"
}

jmessage!{
    JERR_BAD_MCU_SIZE, 
    "Sampling factors too large for interleaved scan"
}

jmessage!{
    JERR_BAD_POOL_ID, 
    "Invalid memory pool code %d"
}

jmessage!{
    JERR_BAD_PRECISION, 
    "Unsupported JPEG data precision %d"
}

jmessage!{
    JERR_BAD_PROGRESSION, 
    "Invalid progressive parameters Ss=%d Se=%d Ah=%d Al=%d"
}

jmessage!{
    JERR_BAD_PROG_SCRIPT, 
    "Invalid progressive parameters at scan script entry %d"
}

jmessage!{
    JERR_BAD_SAMPLING, 
    "Bogus sampling factors"
}

jmessage!{
    JERR_BAD_SCAN_SCRIPT, 
    "Invalid scan script at entry %d"
}

jmessage!{
    JERR_BAD_STATE, 
    "Improper call to JPEG library in state %d"
}

jmessage!{
    JERR_BAD_STRUCT_SIZE, 
    "JPEG parameter struct mismatch: library thinks size is %u, caller expects %u"
}

jmessage!{
    JERR_BAD_VIRTUAL_ACCESS, 
    "Bogus virtual array access"
}

jmessage!{
    JERR_BUFFER_SIZE, 
    "Buffer passed to JPEG library is too small"
}

jmessage!{
    JERR_CANT_SUSPEND, 
    "Suspension not allowed here"
}

jmessage!{
    JERR_CCIR601_NOTIMPL, 
    "CCIR601 sampling not implemented yet"
}

jmessage!{
    JERR_COMPONENT_COUNT, 
    "Too many color components: %d, max %d"
}

jmessage!{
    JERR_CONVERSION_NOTIMPL, 
    "Unsupported color conversion request"
}

jmessage!{
    JERR_DAC_INDEX, 
    "Bogus DAC index %d"
}

jmessage!{
    JERR_DAC_VALUE, 
    "Bogus DAC value 0x%x"
}

jmessage!{
    JERR_DHT_INDEX, 
    "Bogus DHT index %d"
}

jmessage!{
    JERR_DQT_INDEX, 
    "Bogus DQT index %d"
}

jmessage!{
    JERR_EMPTY_IMAGE, 
    "Empty JPEG image (DNL not supported)"
}

jmessage!{
    JERR_EMS_READ, 
    "Read from EMS failed"
}

jmessage!{
    JERR_EMS_WRITE, 
    "Write to EMS failed"
}

jmessage!{
    JERR_EOI_EXPECTED, 
    "Didn't expect more than one scan"
}

jmessage!{
    JERR_FILE_READ, 
    "Input file read error"
}

jmessage!{
    JERR_FILE_WRITE, 
    "Output file write error --- out of disk space?"
}

jmessage!{
    JERR_FRACT_SAMPLE_NOTIMPL, 
    "Fractional sampling not implemented yet"
}

jmessage!{
    JERR_HUFF_CLEN_OVERFLOW, 
    "Huffman code size table overflow"
}

jmessage!{
    JERR_HUFF_MISSING_CODE, 
    "Missing Huffman code table entry"
}

jmessage!{
    JERR_IMAGE_TOO_BIG, 
    "Maximum supported image dimension is %u pixels"
}

jmessage!{
    JERR_INPUT_EMPTY, 
    "Empty input file"
}

jmessage!{
    JERR_INPUT_EOF, 
    "Premature end of input file"
}

jmessage!{
    JERR_MISMATCHED_QUANT_TABLE, 
    "Cannot transcode due to multiple use of quantization table %d"
}

jmessage!{
    JERR_MISSING_DATA, 
    "Scan script does not transmit all data"
}

jmessage!{
    JERR_MODE_CHANGE, 
    "Invalid color quantization mode change"
}

jmessage!{
    JERR_NOTIMPL, 
    "Not implemented yet"
}

jmessage!{
    JERR_NOT_COMPILED, 
    "Requested feature was omitted at compile time"
}

jmessage!{
    JERR_NO_BACKING_STORE, 
    "Backing store not supported"
}

jmessage!{
    JERR_NO_HUFF_TABLE, 
    "Huffman table 0x%02x was not defined"
}

jmessage!{
    JERR_NO_IMAGE, 
    "JPEG datastream contains no image"
}

jmessage!{
    JERR_NO_QUANT_TABLE, 
    "Quantization table 0x%02x was not defined"
}

jmessage!{
    JERR_NO_SOI, 
    "Not a JPEG file: starts with 0x%02x 0x%02x"
}

jmessage!{
    JERR_OUT_OF_MEMORY, 
    "Insufficient memory (case %d)"
}

jmessage!{
    JERR_QUANT_COMPONENTS, 
    "Cannot quantize more than %d color components"
}

jmessage!{
    JERR_QUANT_FEW_COLORS, 
    "Cannot quantize to fewer than %d colors"
}

jmessage!{
    JERR_QUANT_MANY_COLORS, 
    "Cannot quantize to more than %d colors"
}

jmessage!{
    JERR_SOF_DUPLICATE, 
    "Invalid JPEG file structure: two SOF markers"
}

jmessage!{
    JERR_SOF_NO_SOS, 
    "Invalid JPEG file structure: missing SOS marker"
}

jmessage!{
    JERR_SOF_UNSUPPORTED, 
    "Unsupported JPEG process: SOF type 0x%02x"
}

jmessage!{
    JERR_SOI_DUPLICATE, 
    "Invalid JPEG file structure: two SOI markers"
}

jmessage!{
    JERR_SOS_NO_SOF, 
    "Invalid JPEG file structure: SOS before SOF"
}

jmessage!{
    JERR_TFILE_CREATE, 
    "Failed to create temporary file %s"
}

jmessage!{
    JERR_TFILE_READ, 
    "Read failed on temporary file"
}

jmessage!{
    JERR_TFILE_SEEK, 
    "Seek failed on temporary file"
}

jmessage!{
    JERR_TFILE_WRITE, 
    "Write failed on temporary file --- out of disk space?"
}

jmessage!{
    JERR_TOO_LITTLE_DATA, 
    "Application transferred too few scanlines"
}

jmessage!{
    JERR_UNKNOWN_MARKER, 
    "Unsupported marker type 0x%02x"
}

jmessage!{
    JERR_VIRTUAL_BUG, 
    "Virtual array controller messed up"
}

jmessage!{
    JERR_WIDTH_OVERFLOW, 
    "Image too wide for this implementation"
}

jmessage!{
    JERR_XMS_READ, 
    "Read from XMS failed"
}

jmessage!{
    JERR_XMS_WRITE, 
    "Write to XMS failed"
}

jmessage!{
    JMSG_COPYRIGHT, 
    JCOPYRIGHT
}

jmessage!{
    JMSG_VERSION, 
    JVERSION
}

jmessage!{
    JTRC_16BIT_TABLES, 
    "Caution: quantization tables are too coarse for baseline JPEG"
}

jmessage!{
    JTRC_ADOBE, 
    "Adobe APP14 marker: version %d, flags 0x%04x 0x%04x, transform %d"
}

jmessage!{
    JTRC_APP0, 
    "Unknown APP0 marker (not JFIF), length %u"
}

jmessage!{
    JTRC_APP14, 
    "Unknown APP14 marker (not Adobe), length %u"
}

jmessage!{
    JTRC_DAC, 
    "Define Arithmetic Table 0x%02x: 0x%02x"
}

jmessage!{
    JTRC_DHT, 
    "Define Huffman Table 0x%02x"
}

jmessage!{
    JTRC_DQT, 
    "Define Quantization Table %d  precision %d"
}

jmessage!{
    JTRC_DRI, 
    "Define Restart Interval %u"
}

jmessage!{
    JTRC_EMS_CLOSE, 
    "Freed EMS handle %u"
}

jmessage!{
    JTRC_EMS_OPEN, 
    "Obtained EMS handle %u"
}

jmessage!{
    JTRC_EOI, 
    "End Of Image"
}

jmessage!{
    JTRC_HUFFBITS, 
    "        %3d %3d %3d %3d %3d %3d %3d %3d"
}

jmessage!{
    JTRC_JFIF, 
    "JFIF APP0 marker: version %d.%02d, density %dx%d  %d"
}

jmessage!{
    JTRC_JFIF_BADTHUMBNAILSIZE, 
    "Warning: thumbnail image size does not match data length %u"
}

jmessage!{
    JTRC_JFIF_EXTENSION, 
    "JFIF extension marker: type 0x%02x, length %u"
}

jmessage!{
    JTRC_JFIF_THUMBNAIL, 
    "    with %d x %d thumbnail image"
}

jmessage!{
    JTRC_MISC_MARKER, 
    "Miscellaneous marker 0x%02x, length %u"
}

jmessage!{
    JTRC_PARMLESS_MARKER, 
    "Unexpected marker 0x%02x"
}

jmessage!{
    JTRC_QUANTVALS, 
    "        %4u %4u %4u %4u %4u %4u %4u %4u"
}

jmessage!{
    JTRC_QUANT_3_NCOLORS, 
    "Quantizing to %d = %d*%d*%d colors"
}

jmessage!{
    JTRC_QUANT_NCOLORS, 
    "Quantizing to %d colors"
}

jmessage!{
    JTRC_QUANT_SELECTED, 
    "Selected %d colors for quantization"
}

jmessage!{
    JTRC_RECOVERY_ACTION, 
    "At marker 0x%02x, recovery action %d"
}

jmessage!{
    JTRC_RST, 
    "RST%d"
}

jmessage!{
    JTRC_SMOOTH_NOTIMPL, 
    "Smoothing not supported with nonstandard sampling ratios"
}

jmessage!{
    JTRC_SOF, 
    "Start Of Frame 0x%02x: width=%u, height=%u, components=%d"
}

jmessage!{
    JTRC_SOF_COMPONENT, 
    "    Component %d: %dhx%dv q=%d"
}

jmessage!{
    JTRC_SOI, 
    "Start of Image"
}

jmessage!{
    JTRC_SOS, 
    "Start Of Scan: %d components"
}

jmessage!{
    JTRC_SOS_COMPONENT, 
    "    Component %d: dc=%d ac=%d"
}

jmessage!{
    JTRC_SOS_PARAMS, 
    "  Ss=%d, Se=%d, Ah=%d, Al=%d"
}

jmessage!{
    JTRC_TFILE_CLOSE, 
    "Closed temporary file %s"
}

jmessage!{
    JTRC_TFILE_OPEN, 
    "Opened temporary file %s"
}

jmessage!{
    JTRC_THUMB_JPEG, 
    "JFIF extension marker: JPEG-compressed thumbnail image, length %u"
}

jmessage!{
    JTRC_THUMB_PALETTE, 
    "JFIF extension marker: palette thumbnail image, length %u"
}

jmessage!{
    JTRC_THUMB_RGB, 
    "JFIF extension marker: RGB thumbnail image, length %u"
}

jmessage!{
    JTRC_UNKNOWN_IDS, 
    "Unrecognized component IDs %d %d %d, assuming YCbCr"
}

jmessage!{
    JTRC_XMS_CLOSE, 
    "Freed XMS handle %u"
}

jmessage!{
    JTRC_XMS_OPEN, 
    "Obtained XMS handle %u"
}

jmessage!{
    JWRN_ADOBE_XFORM, 
    "Unknown Adobe color transform code %d"
}

jmessage!{
    JWRN_BOGUS_PROGRESSION, 
    "Inconsistent progression sequence for component %d coefficient %d"
}

jmessage!{
    JWRN_EXTRANEOUS_DATA, 
    "Corrupt JPEG data: %u extraneous bytes before marker 0x%02x"
}

jmessage!{
    JWRN_HIT_MARKER, 
    "Corrupt JPEG data: premature end of data segment"
}

jmessage!{
    JWRN_HUFF_BAD_CODE, 
    "Corrupt JPEG data: bad Huffman code"
}

jmessage!{
    JWRN_JFIF_MAJOR, 
    "Warning: unknown JFIF revision number %d.%02d"
}

jmessage!{
    JWRN_JPEG_EOF, 
    "Premature end of JPEG file"
}

jmessage!{
    JWRN_MUST_RESYNC, 
    "Corrupt JPEG data: found marker 0x%02x instead of RST%d"
}

jmessage!{
    JWRN_NOT_SEQUENTIAL, 
    "Invalid SOS parameters for sequential JPEG"
}

jmessage!{
    JWRN_TOO_MUCH_DATA, 
    "Application transferred too many scanlines"
}

/*
  | Macros to simplify using the error and
  | trace message stuff
  | 
  | The first parameter is either type of
  | cinfo pointer
  |
  */

/**
  | Fatal errors (print message and exit)
  |
  */
macro_rules! errexit {
    ($cinfo:ident, $code:ident) => {
        /*
        
          ((cinfo)->err->msg_code = (code), 
           (*(cinfo)->err->error_exit) ((JCommonPtr) (cinfo)))
        */
    }
}

macro_rules! errexit1 {
    ($cinfo:ident, $code:ident, $p1:ident) => {
        /*
        
          ((cinfo)->err->msg_code = (code), 
           (cinfo)->err->msg_parm.i[0] = (p1), 
           (*(cinfo)->err->error_exit) ((JCommonPtr) (cinfo)))
        */
    }
}

macro_rules! errexit2 {
    ($cinfo:ident, $code:ident, $p1:ident, $p2:ident) => {
        /*
        
          ((cinfo)->err->msg_code = (code), 
           (cinfo)->err->msg_parm.i[0] = (p1), 
           (cinfo)->err->msg_parm.i[1] = (p2), 
           (*(cinfo)->err->error_exit) ((JCommonPtr) (cinfo)))
        */
    }
}

macro_rules! errexit3 {
    ($cinfo:ident, 
     $code:ident, 
     $p1:ident, 
     $p2:ident, 
     $p3:ident) => {
        /*
        
          ((cinfo)->err->msg_code = (code), 
           (cinfo)->err->msg_parm.i[0] = (p1), 
           (cinfo)->err->msg_parm.i[1] = (p2), 
           (cinfo)->err->msg_parm.i[2] = (p3), 
           (*(cinfo)->err->error_exit) ((JCommonPtr) (cinfo)))
        */
    }
}

macro_rules! errexit4 {
    ($cinfo:ident, 
     $code:ident, 
     $p1:ident, 
     $p2:ident, 
     $p3:ident, 
     $p4:ident) => {
        /*
        
          ((cinfo)->err->msg_code = (code), 
           (cinfo)->err->msg_parm.i[0] = (p1), 
           (cinfo)->err->msg_parm.i[1] = (p2), 
           (cinfo)->err->msg_parm.i[2] = (p3), 
           (cinfo)->err->msg_parm.i[3] = (p4), 
           (*(cinfo)->err->error_exit) ((JCommonPtr) (cinfo)))
        */
    }
}

macro_rules! errexits {
    ($cinfo:ident, 
     $code:ident, 
     $str:ident) => {
        /*
        
          ((cinfo)->err->msg_code = (code), 
           strncpy((cinfo)->err->msg_parm.s, (str), JMSG_STR_PARM_MAX), 
           (*(cinfo)->err->error_exit) ((JCommonPtr) (cinfo)))
        */
    }
}

macro_rules! makestmt {
    ($stuff:ident) => {
        /*
                do { stuff } while (0)
        */
    }
}

/**
  | Nonfatal errors (we can keep going,
  | but the data is probably corrupt)
  |
  */
macro_rules! warnms {
    ($cinfo:ident, $code:ident) => {
        /*
        
          ((cinfo)->err->msg_code = (code), 
           (*(cinfo)->err->emit_message) ((JCommonPtr) (cinfo), -1))
        */
    }
}

macro_rules! warnms1 {
    ($cinfo:ident, 
     $code:ident, 
     $p1:ident) => {
        /*
        
          ((cinfo)->err->msg_code = (code), 
           (cinfo)->err->msg_parm.i[0] = (p1), 
           (*(cinfo)->err->emit_message) ((JCommonPtr) (cinfo), -1))
        */
    }
}

macro_rules! warnms2 {
    ($cinfo:ident, 
     $code:ident, 
     $p1:ident, 
     $p2:ident) => {
        /*
        
          ((cinfo)->err->msg_code = (code), 
           (cinfo)->err->msg_parm.i[0] = (p1), 
           (cinfo)->err->msg_parm.i[1] = (p2), 
           (*(cinfo)->err->emit_message) ((JCommonPtr) (cinfo), -1))
        */
    }
}

/**
  | Informational/debugging messages
  |
  */
macro_rules! tracems {
    ($cinfo:ident, 
     $lvl:ident, 
     $code:ident) => {
        /*
        
          ((cinfo)->err->msg_code = (code), 
           (*(cinfo)->err->emit_message) ((JCommonPtr) (cinfo), (lvl)))
        */
    }
}

macro_rules! tracems1 {
    ($cinfo:ident, 
     $lvl:ident, 
     $code:ident, 
     $p1:ident) => {
        /*
        
          ((cinfo)->err->msg_code = (code), 
           (cinfo)->err->msg_parm.i[0] = (p1), 
           (*(cinfo)->err->emit_message) ((JCommonPtr) (cinfo), (lvl)))
        */
    }
}

macro_rules! tracems2 {
    ($cinfo:ident, 
     $lvl:ident, 
     $code:ident, 
     $p1:ident, 
     $p2:ident) => {
        /*
        
          ((cinfo)->err->msg_code = (code), 
           (cinfo)->err->msg_parm.i[0] = (p1), 
           (cinfo)->err->msg_parm.i[1] = (p2), 
           (*(cinfo)->err->emit_message) ((JCommonPtr) (cinfo), (lvl)))
        */
    }
}

macro_rules! tracems3 {
    ($cinfo:ident, 
     $lvl:ident, 
     $code:ident, 
     $p1:ident, 
     $p2:ident, 
     $p3:ident) => {
        /*
        
          MAKESTMT(int * _mp = (cinfo)->err->msg_parm.i; 
               _mp[0] = (p1); _mp[1] = (p2); _mp[2] = (p3); 
               (cinfo)->err->msg_code = (code); 
               (*(cinfo)->err->emit_message) ((JCommonPtr) (cinfo), (lvl)); )
        */
    }
}

macro_rules! tracems4 {
    ($cinfo:ident, 
     $lvl:ident, 
     $code:ident, 
     $p1:ident, 
     $p2:ident, 
     $p3:ident, 
     $p4:ident) => {
        /*
        
          MAKESTMT(int * _mp = (cinfo)->err->msg_parm.i; 
               _mp[0] = (p1); _mp[1] = (p2); _mp[2] = (p3); _mp[3] = (p4); 
               (cinfo)->err->msg_code = (code); 
               (*(cinfo)->err->emit_message) ((JCommonPtr) (cinfo), (lvl)); )
        */
    }
}

macro_rules! tracems5 {
    ($cinfo:ident, 
     $lvl:ident, 
     $code:ident, 
     $p1:ident, 
     $p2:ident, 
     $p3:ident, 
     $p4:ident, 
     $p5:ident) => {
        /*
        
          MAKESTMT(int * _mp = (cinfo)->err->msg_parm.i; 
               _mp[0] = (p1); _mp[1] = (p2); _mp[2] = (p3); _mp[3] = (p4); 
               _mp[4] = (p5); 
               (cinfo)->err->msg_code = (code); 
               (*(cinfo)->err->emit_message) ((JCommonPtr) (cinfo), (lvl)); )
        */
    }
}

macro_rules! tracems8 {
    ($cinfo:ident, 
     $lvl:ident, 
     $code:ident, 
     $p1:ident, 
     $p2:ident, 
     $p3:ident, 
     $p4:ident, 
     $p5:ident, 
     $p6:ident, 
     $p7:ident, 
     $p8:ident) => {
        /*
        
          MAKESTMT(int * _mp = (cinfo)->err->msg_parm.i; 
               _mp[0] = (p1); _mp[1] = (p2); _mp[2] = (p3); _mp[3] = (p4); 
               _mp[4] = (p5); _mp[5] = (p6); _mp[6] = (p7); _mp[7] = (p8); 
               (cinfo)->err->msg_code = (code); 
               (*(cinfo)->err->emit_message) ((JCommonPtr) (cinfo), (lvl)); )
        */
    }
}

macro_rules! tracemss {
    ($cinfo:ident, 
     $lvl:ident, 
     $code:ident, 
     $str:ident) => {
        /*
        
          ((cinfo)->err->msg_code = (code), 
           strncpy((cinfo)->err->msg_parm.s, (str), JMSG_STR_PARM_MAX), 
           (*(cinfo)->err->emit_message) ((JCommonPtr) (cinfo), (lvl)))
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jerror.c]

/*
  | jerror.c
  | 
  | This file contains simple error-reporting
  | and trace-message routines.
  | 
  | These are suitable for Unix-like systems
  | and others where writing to stderr is
  | the right thing to do. Many applications
  | will want to replace some or all of these
  | routines.
  | 
  | If you define USE_WINDOWS_MESSAGEBOX
  | in jconfig.h or in the makefile, you
  | get a Windows-specific hack to display
  | error messages in a dialog box.
  | 
  | It ain't much, but it beats dropping
  | error messages into the bit bucket,
  | which is what happens to output to stderr
  | under most Windows C compilers.
  | 
  | These routines are used by both the compression
  | and decompression code.
  |
  */

pub const EXIT_FAILURE: usize = 1;

/**
  | Create the message string table.
  | 
  | We do this from the master message list
  | in jerror.h by re-reading jerror.h
  | with a suitable definition for macro
  | JMESSAGE.
  | 
  | The message table is made an external
  | symbol just in case any applications
  | want to refer to it directly.
  |
  */
#[cfg(NEED_SHORT_EXTERNAL_NAMES)]
#[macro_export] 
macro_rules! jpeg_std_message_table {
    () => {
        /*
                jMsgTable
        */
    }
}

#[macro_export] 
macro_rules! jmessage {
    ($code:expr, $string:expr) => {
        /*
                string ,
        */
    }
}

lazy_static!{
    /*
    const char * const jpeg_std_message_table[] = {
    #include "jerror.h"
      NULL
    };
    */
}

/**
  | Error exit handler: must not return
  | to caller.
  | 
  | Applications may override this if they
  | want to get control back after an error.
  | Typically one would longjmp somewhere
  | instead of exiting.
  | 
  | The setjmp buffer can be made a private
  | field within an expanded error handler
  | object. Note that the info needed to
  | generate an error message is stored
  | in the error object, so you can generate
  | the message now or later, at your convenience.
  | 
  | You should make sure that the JPEG object
  | is cleaned up (with jpeg_abort or jpeg_destroy)
  | at some point.
  |
  */
pub fn error_exit(cinfo: JCommonPtr)  {
    
    todo!();
        /*
            /* Always display the message */
      (*cinfo->err->output_message) (cinfo);

      /* Let the memory manager delete any temp files before we die */
      jpeg_destroy(cinfo);

      exit(EXIT_FAILURE);
        */
}

/**
  | Actual output of an error or trace message.
  | 
  | Applications may override this method
  | to send JPEG messages somewhere other
  | than stderr.
  | 
  | On Windows, printing to stderr is generally
  | completely useless, so we provide optional
  | code to produce an error-dialog popup.
  | 
  | Most Windows applications will still
  | prefer to override this routine, but
  | if they don't, it'll do something at
  | least marginally useful.
  | 
  | -----------
  | @note
  | 
  | to use the library in an environment
  | that doesn't support the
  | 
  | C stdio library, you may have to delete
  | the call to fprintf() entirely, not
  | just not use this routine.
  |
  */
pub fn output_message(cinfo: JCommonPtr)  {
    
    todo!();
        /*
            char buffer[JMSG_LENGTH_MAX];

      /* Create the message */
      (*cinfo->err->format_message) (cinfo, buffer);

    #ifdef USE_WINDOWS_MESSAGEBOX
      /* Display it in a message dialog box */
      MessageBox(GetActiveWindow(), buffer, "JPEG Library Error",
             MB_OK | MB_ICONERROR);
    #else
      /* Send it to stderr, adding a newline */
      fprintf(stderr, "%s\n", buffer);
    #endif
        */
}


/**
  | Decide whether to emit a trace or warning
  | message. msg_level is one of:
  | 
  | -1: recoverable corrupt-data warning,
  | may want to abort.
  | 
  | 0: important advisory messages (always
  | display to user).
  | 
  | 1: first level of tracing detail.
  | 
  | 2,3,...: successively more detailed
  | tracing messages.
  | 
  | An application might override this
  | method if it wanted to abort on warnings
  | or change the policy about which messages
  | to display.
  |
  */
pub fn emit_message(
        cinfo:     JCommonPtr,
        msg_level: i32)  {
    
    todo!();
        /*
            struct jpeg_error_mgr * err = cinfo->err;

      if (msg_level < 0) {
        /* It's a warning message.  Since corrupt files may generate many warnings,
         * the policy implemented here is to show only the first warning,
         * unless trace_level >= 3.
         */
        if (err->num_warnings == 0 || err->trace_level >= 3)
          (*err->output_message) (cinfo);
        /* Always count warnings in num_warnings. */
        err->num_warnings++;
      } else {
        /* It's a trace message.  Show it if trace_level >= msg_level. */
        if (err->trace_level >= msg_level)
          (*err->output_message) (cinfo);
      }
        */
}

/**
  | Format a message string for the most
  | recent JPEG error or message.
  | 
  | The message is stored into buffer, which
  | should be at least JMSG_LENGTH_MAX
  | characters. Note that no '\n' character
  | is added to the string.
  | 
  | Few applications should need to override
  | this method.
  |
  */
pub fn format_message(
        cinfo:  JCommonPtr,
        buffer: *mut u8)  {
    
    todo!();
        /*
            struct jpeg_error_mgr * err = cinfo->err;
      int msg_code = err->msg_code;
      const char * msgtext = NULL;
      const char * msgptr;
      char ch;
      boolean isstring;

      /* Look up message string in proper table */
      if (msg_code > 0 && msg_code <= err->last_jpeg_message) {
        msgtext = err->jpeg_message_table[msg_code];
      } else if (err->addon_message_table != NULL &&
             msg_code >= err->first_addon_message &&
             msg_code <= err->last_addon_message) {
        msgtext = err->addon_message_table[msg_code - err->first_addon_message];
      }

      /* Defend against bogus message number */
      if (msgtext == NULL) {
        err->msg_parm.i[0] = msg_code;
        msgtext = err->jpeg_message_table[0];
      }

      /* Check for string parameter, as indicated by %s in the message text */
      isstring = FALSE;
      msgptr = msgtext;
      while ((ch = *msgptr++) != '\0') {
        if (ch == '%') {
          if (*msgptr == 's') isstring = TRUE;
          break;
        }
      }

      /* Format the message into the passed buffer */
      if (isstring)
        sprintf(buffer, msgtext, err->msg_parm.s);
      else
        sprintf(buffer, msgtext,
            err->msg_parm.i[0], err->msg_parm.i[1],
            err->msg_parm.i[2], err->msg_parm.i[3],
            err->msg_parm.i[4], err->msg_parm.i[5],
            err->msg_parm.i[6], err->msg_parm.i[7]);
        */
}

/**
  | Reset error state variables at start
  | of a new image.
  | 
  | This is called during compression startup
  | to reset trace/error processing to
  | default state, without losing any application-specific
  | method pointers. An application might
  | possibly want to override this method
  | if it has additional error processing
  | state.
  |
  */
pub fn reset_error_mgr(cinfo: JCommonPtr)  {
    
    todo!();
        /*
            cinfo->err->num_warnings = 0;
      /* trace_level is not reset since it is an application-supplied parameter */
      cinfo->err->msg_code = 0; /* may be useful as a flag for "no error" */
        */
}

/**
  | Fill in the standard error-handling
  | methods in a jpeg_error_mgr object.
  | 
  | Typical call is: struct jpeg_compress_struct
  | cinfo; struct jpeg_error_mgr err;
  | cinfo.err = jpeg_std_error(&err);
  | after which the application may override
  | some of the methods.
  |
  */
pub fn jpeg_std_error(err: *mut JpegErrorMgr) -> *mut JpegErrorMgr {
    
    todo!();
        /*
            err->error_exit = error_exit;
      err->emit_message = emit_message;
      err->output_message = output_message;
      err->format_message = format_message;
      err->reset_error_mgr = reset_error_mgr;

      err->trace_level = 0;     /* default = no tracing */
      err->num_warnings = 0;    /* no warnings emitted yet */
      err->msg_code = 0;        /* may be useful as a flag for "no error" */

      /* Initialize message table pointers */
      err->jpeg_message_table = jpeg_std_message_table;
      err->last_jpeg_message = (int) JMSG_LASTMSGCODE - 1;

      err->addon_message_table = NULL;
      err->first_addon_message = 0; /* for safety */
      err->last_addon_message = 0;

      return err;
        */
}
