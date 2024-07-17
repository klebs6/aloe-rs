/*!
  | \file include/FLAC/format.h
  | 
  | -----------
  | @brief
  | 
  | This module contains structure definitions
  | for the representation of FLAC format
  | components in memory. These are the
  | basic structures used by the rest of
  | the interfaces.
  | 
  | See the detailed documentation in the
  | \link flac_format format \endlink
  | module. \defgroup flac_format FLAC/format.h:
  | format components \ingroup flac
  | ----------
  | @brief
  | 
  | This module contains structure definitions
  | for the representation of FLAC format
  | components in memory. These are the
  | basic structures used by the rest of
  | the interfaces.
  | 
  | First, you should be familiar with the
  | <A HREF="../format.html">FLAC format</A>.
  | Many of the values here follow directly
  | from the specification. As a user of
  | libFLAC, the interesting parts really
  | are the structures that describe the
  | frame header and metadata blocks.
  | 
  | The format structures here are very
  | primitive, designed to store information
  | in an efficient way. Reading information
  | from the structures is easy but creating
  | or modifying them directly is more complex.
  | For the most part, as a user of a library,
  | editing is not necessary; however,
  | for metadata blocks it is, so there are
  | convenience functions provided in
  | the \link flac_metadata metadata module
  | \endlink to simplify the manipulation
  | of metadata blocks.
  | 
  | -----------
  | @note
  | 
  | It's not the best convention, but symbols
  | ending in _LEN are in bits and _LENGTH
  | are in bytes. _LENGTH symbols are \#defines
  | instead of global variables because
  | they are usually used when declaring
  | byte arrays and some compilers require
  | compile-time knowledge of array sizes
  | when declared on the stack.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/format.h]


/*
    Most of the values described in this file are
    defined by the FLAC format specification.
    There is nothing to tune here.
*/

/**
  | The largest legal metadata type code.
  |
  */
pub const MAX_METADATA_TYPE_CODE: usize = 126;

/**
  | The minimum block size, in samples,
  | permitted by the format.
  |
  */
pub const MIN_BLOCK_SIZE: usize = 16;

/**
  | The maximum block size, in samples,
  | permitted by the format.
  |
  */
pub const MAX_BLOCK_SIZE: usize = 65535;

/**
  | The maximum block size, in samples,
  | permitted by the FLAC subset for sample
  | rates up to 48kHz.
  |
  */
pub const SUBSET_MAX_BLOCK_SIZE_48000HZ: usize = 4608;

/**
  | The maximum number of channels permitted
  | by the format.
  |
  */
pub const MAX_CHANNELS: usize = 8;

/**
  | The minimum sample resolution permitted
  | by the format.
  |
  */
pub const MIN_BITS_PER_SAMPLE: usize = 4;

/**
  | The maximum sample resolution permitted
  | by the format.
  |
  */
pub const MAX_BITS_PER_SAMPLE: usize = 32;

/**
  | The maximum sample resolution permitted
  | by libFLAC.
  | 
  | -----------
  | @warning
  | 
  | MAX_BITS_PER_SAMPLE is the
  | limit of the FLAC format. However, the
  | reference encoder/decoder is currently
  | limited to 24 bits because of prevalent
  | 32-bit math, so make sure and use this
  | value when appropriate.
  |
  */
pub const REFERENCE_CODEC_MAX_BITS_PER_SAMPLE: usize = 24;

/**
  | The maximum sample rate permitted by
  | the format.
  | 
  | The value is ((2 ^ 16) - 1) * 10; see <A HREF="../format.html">FLAC
  | format</A> as to why.
  |
  */
pub const MAX_SAMPLE_RATE: usize = 655350;

/**
  | The maximum LPC order permitted by the
  | format.
  |
  */
pub const MAX_LPC_ORDER: usize = 32;

/**
  | The maximum LPC order permitted by the
  | FLAC subset for sample rates up to 48kHz.
  |
  */
pub const SUBSET_MAX_LPC_ORDER_48000HZ: usize = 12;

/**
  | The minimum quantized linear predictor
  | coefficient precision permitted by
  | the format.
  |
  */
pub const MIN_QLP_COEFF_PRECISION: usize = 5;

/**
  | The maximum quantized linear predictor
  | coefficient precision permitted by
  | the format.
  |
  */
pub const MAX_QLP_COEFF_PRECISION: usize = 15;

/**
  | The maximum order of the fixed predictors
  | permitted by the format.
  |
  */
pub const MAX_FIXED_ORDER: usize = 4;

/**
  | The maximum Rice partition order permitted
  | by the format.
  |
  */
pub const MAX_RICE_PARTITION_ORDER: usize = 15;

/**
  | The maximum Rice partition order permitted
  | by the FLAC Subset.
  |
  */
pub const SUBSET_MAX_RICE_PARTITION_ORDER: usize = 8;

/**
  | The version string of the release, stamped
  | onto the libraries and binaries.
  | 
  | -----------
  | @note
  | 
  | This does not correspond to the shared
  | library version number, which is used
  | to determine binary compatibility.
  |
  */
lazy_static!{
    /*
    extern  const char *VERSION_STRING;
    */
}

/**
  | The vendor string inserted by the encoder
  | into the VORBIS_COMMENT block.
  | 
  | This is a NUL-terminated ASCII string;
  | when inserted into the
  | 
  | VORBIS_COMMENT the trailing null is
  | stripped.
  |
  */
lazy_static!{
    /*
    extern  const char *VENDOR_STRING;
    */
}

/**
  | The byte string representation of the
  | beginning of a FLAC stream.
  |
  */
lazy_static!{
    /*
    extern  const byte STREAM_SYNC_STRING[4]; /* = "fLaC" */
    */
}

/**
  | The 32-bit integer big-endian representation
  | of the beginning of a FLAC stream.
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_SYNC; /* = 0x664C6143 */
    */
}

/**
  | The length of the FLAC signature in bits.
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_SYNC_LEN; /* = 32 bits */
    */
}

/**
  | The length of the FLAC signature in bytes.
  |
  */
pub const STREAM_SYNC_LENGTH: usize = 4;

/* ---------------- Frame structures  ---------------- */

/**
  | An enumeration of the available channel
  | assignments.
  |
  */
pub enum ChannelAssignment {

    /**
      | independent channels
      |
      */
    CHANNEL_ASSIGNMENT_INDEPENDENT = 0, 

    /**
      | left+side stereo
      |
      */
    CHANNEL_ASSIGNMENT_LEFT_SIDE   = 1, 

    /**
      | right+side stereo
      |
      */
    CHANNEL_ASSIGNMENT_RIGHT_SIDE  = 2, 

    /**
      | mid+side stereo
      |
      */
    CHANNEL_ASSIGNMENT_MID_SIDE    = 3, 
}

/**
  | Maps a ChannelAssignment to
  | a C string.
  | 
  | Using a ChannelAssignment as
  | the index to this array will give the
  | string equivalent. The contents should
  | not be modified.
  |
  */
lazy_static!{
    /*
    extern  const char * const ChannelAssignmentString[];
    */
}

/**
  | An enumeration of the possible frame
  | numbering methods.
  |
  */
pub enum FrameNumberType {

    /**
      | number contains the frame number
      |
      */
    FRAME_NUMBER_TYPE_FRAME_NUMBER, 

    /**
      | number contains the sample number of
      | first sample in frame
      |
      */
    FRAME_NUMBER_TYPE_SAMPLE_NUMBER, 
}

/**
  | Maps a FrameNumberType to a C
  | string.
  | 
  | Using a FrameNumberType as the
  | index to this array will give the string
  | equivalent. The contents should not
  | be modified.
  |
  */
lazy_static!{
    /*
    extern  const char * const FrameNumberTypeString[];
    */
}

/**
  | FLAC frame structure. (c.f. <A HREF="../format.html#frame">format
  | specification</A>)
  |
  */
pub struct Frame {
    header:    FrameHeader,
    subframes: [Subframe; MAX_CHANNELS],
    footer:    FrameFooter,
}

/* -------------- Meta-data structures  -------------- */

/**
  | An enumeration of the available metadata
  | block types.
  |
  */
pub enum MetadataType {

    /**
      | <A HREF="../format.html#metadata_block_streaminfo">STREAMINFO</A>
      | block
      |
      */
    METADATA_TYPE_STREAMINFO,

    /**
      | <A HREF="../format.html#metadata_block_padding">PADDING</A>
      | block
      |
      */
    METADATA_TYPE_PADDING,

    /**
      | <A HREF="../format.html#metadata_block_application">APPLICATION</A>
      | block
      |
      */
    METADATA_TYPE_APPLICATION,

    /**
      | <A HREF="../format.html#metadata_block_seektable">SEEKTABLE</A>
      | block
      |
      */
    METADATA_TYPE_SEEKTABLE,

    /**
      | <A HREF="../format.html#metadata_block_vorbis_comment">VORBISCOMMENT</A>
      | block (a.k.a. FLAC tags)
      |
      */
    METADATA_TYPE_VORBIS_COMMENT,

    /**
      | <A HREF="../format.html#metadata_block_cuesheet">CUESHEET</A>
      | block
      |
      */
    METADATA_TYPE_CUESHEET,

    /**
      | <A HREF="../format.html#metadata_block_picture">PICTURE</A>
      | block
      |
      */
    METADATA_TYPE_PICTURE,

    /**
      | marker to denote beginning of undefined
      | type range; this number will increase
      | as new metadata types are added
      |
      */
    METADATA_TYPE_UNDEFINED,

    /**
      | No type will ever be greater than this.
      | There is not enough room in the protocol
      | block.
      |
      */
    MAX_METADATA_TYPE,
}

impl MetadataType {

    pub fn value(&self) -> usize {
        match self {
            MetadataType::METADATA_TYPE_STREAMINFO     => 0,
            MetadataType::METADATA_TYPE_PADDING        => 1,
            MetadataType::METADATA_TYPE_APPLICATION    => 2,
            MetadataType::METADATA_TYPE_SEEKTABLE      => 3,
            MetadataType::METADATA_TYPE_VORBIS_COMMENT => 4,
            MetadataType::METADATA_TYPE_CUESHEET       => 5,
            MetadataType::METADATA_TYPE_PICTURE        => 6,
            MetadataType::METADATA_TYPE_UNDEFINED      => 7,
            MetadataType::MAX_METADATA_TYPE            => MAX_METADATA_TYPE_CODE,
        }
    }
}

/**
  | Maps a MetadataType to a C string.
  | 
  | Using a MetadataType as the index
  | to this array will give the string equivalent.
  | The contents should not be modified.
  |
  */
lazy_static!{
    /*
    extern  const char * const MetadataTypeString[];
    */
}

/**
  | Tests that a sample rate is valid for
  | FLAC.
  | 
  | -----------
  | @param sample_rate
  | 
  | The sample rate to test for compliance.
  | 
  | -----------
  | @return
  | 
  | bool \c true if the given sample
  | rate conforms to the specification,
  | else \c false.
  |
  */
pub fn flac_format_sample_rate_is_valid(sample_rate: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Tests that a blocksize at the given sample
  | rate is valid for the FLAC subset.
  | 
  | -----------
  | @param blocksize
  | 
  | The blocksize to test for compliance.
  | ----------
  | @param sample_rate
  | 
  | The sample rate is needed, since the
  | valid subset blocksize depends on the
  | sample rate.
  | 
  | -----------
  | @return
  | 
  | bool \c true if the given blocksize
  | conforms to the specification for the
  | subset at the given sample rate, else
  | \c false.
  |
  */
pub fn flac_format_blocksize_is_subset(
        blocksize:   u32,
        sample_rate: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Tests that a sample rate is valid for
  | the FLAC subset. The subset rules for
  | valid sample rates are slightly more
  | complex since the rate has to be expressible
  | completely in the frame header.
  | 
  | -----------
  | @param sample_rate
  | 
  | The sample rate to test for compliance.
  | 
  | -----------
  | @return
  | 
  | bool \c true if the given sample
  | rate conforms to the specification
  | for the subset, else \c false.
  |
  */
pub fn flac_format_sample_rate_is_subset(sample_rate: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Check a Vorbis comment entry name to
  | see if it conforms to the Vorbis comment
  | specification.
  | 
  | Vorbis comment names must be composed
  | only of characters from [0x20-0x3C,0x3E-0x7D].
  | 
  | -----------
  | @param name
  | 
  | A NUL-terminated string to be checked.
  | \assert
  | 
  | -----------
  | @code
  | 
  | name != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c false if entry name is
  | illegal, else \c true.
  |
  */
pub fn flac_format_vorbiscomment_entry_name_is_legal(name: *const u8) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Check a Vorbis comment entry value to
  | see if it conforms to the Vorbis comment
  | specification.
  | 
  | Vorbis comment values must be valid
  | UTF-8 sequences.
  | 
  | -----------
  | @param value
  | 
  | A string to be checked.
  | ----------
  | @param length
  | 
  | A the length of \a value in bytes. May
  | be \c (unsigned)(-1) to indicate that
  | \a value is a plain
  | 
  | UTF-8 NUL-terminated string. \assert
  | 
  | -----------
  | @code
  | 
  | value != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c false if entry name is
  | illegal, else \c true.
  |
  */
pub fn flac_format_vorbiscomment_entry_value_is_legal(
        value:  *const u8,
        length: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Check a Vorbis comment entry to see if
  | it conforms to the Vorbis comment specification.
  | 
  | Vorbis comment entries must be of the
  | form 'name=value', and 'name' and 'value'
  | must be legal according to
  | 
  | format_vorbiscomment_entry_name_is_legal()
  | and
  | 
  | format_vorbiscomment_entry_value_is_legal()
  | respectively.
  | 
  | -----------
  | @param entry
  | 
  | An entry to be checked.
  | ----------
  | @param length
  | 
  | The length of \a entry in bytes. \assert
  | 
  | -----------
  | @code
  | 
  | value != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c false if entry name is
  | illegal, else \c true.
  |
  */
pub fn flac_format_vorbiscomment_entry_is_legal(
        entry:  *const u8,
        length: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Check a seek table to see if it conforms
  | to the FLAC specification.
  | 
  | See the format specification for limits
  | on the contents of the seek table.
  | 
  | -----------
  | @param seek_table
  | 
  | A pointer to a seek table to be checked.
  | \assert
  | 
  | -----------
  | @code
  | 
  | seek_table != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c false if seek table is
  | illegal, else \c true.
  |
  */
pub fn flac_format_seektable_is_legal(seek_table: *const StreamMetadata_SeekTable) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Sort a seek table's seek points according
  | to the format specification.
  | 
  | This includes a "unique-ification"
  | step to remove duplicates, i.e. seek
  | points with identical \a sample_number
  | values. Duplicate seek points are converted
  | into placeholder points and sorted
  | to the end of the table.
  | 
  | -----------
  | @param seek_table
  | 
  | A pointer to a seek table to be sorted.
  | \assert
  | 
  | -----------
  | @code
  | 
  | seek_table != NULL
  | 
  | -----------
  | @return
  | 
  | unsigned
  | 
  | The number of duplicate seek points
  | converted into placeholders.
  |
  */
pub fn flac_format_seektable_sort(seek_table: *mut StreamMetadata_SeekTable) -> u32 {
    
    todo!();
        /*
        
        */
}

/**
  | Check a cue sheet to see if it conforms
  | to the FLAC specification.
  | 
  | See the format specification for limits
  | on the contents of the cue sheet.
  | 
  | -----------
  | @param cue_sheet
  | 
  | A pointer to an existing cue sheet to
  | be checked.
  | ----------
  | @param check_cd_da_subset
  | 
  | If \c true, check CUESHEET against more
  | stringent requirements for a CD-DA
  | (audio) disc.
  | ----------
  | @param violation
  | 
  | Address of a pointer to a string. If there
  | is a violation, a pointer to a string
  | explanation of the violation will be
  | returned here. \a violation may be \c
  | NULL if you don't need the returned string.
  | Do not free the returned string; it will
  | always point to static data. \assert
  | 
  | -----------
  | @code
  | 
  | cue_sheet != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c false if cue sheet is illegal,
  | else \c true.
  |
  */
pub fn flac_format_cuesheet_is_legal(
        cue_sheet:          *const StreamMetadata_CueSheet,
        check_cd_da_subset: bool,
        violation:          *const *const u8) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Check picture data to see if it conforms
  | to the FLAC specification.
  | 
  | See the format specification for limits
  | on the contents of the
  | 
  | PICTURE block.
  | 
  | -----------
  | @param picture
  | 
  | A pointer to existing picture data to
  | be checked.
  | ----------
  | @param violation
  | 
  | Address of a pointer to a string. If there
  | is a violation, a pointer to a string
  | explanation of the violation will be
  | returned here. \a violation may be \c
  | NULL if you don't need the returned string.
  | Do not free the returned string; it will
  | always point to static data. \assert
  | 
  | -----------
  | @code
  | 
  | picture != NULL
  | 
  | -----------
  | @return
  | 
  | bool \c false if picture data
  | is illegal, else \c true.
  |
  */
pub fn flac_format_picture_is_legal(
        picture:   *const StreamMetadata_Picture,
        violation: *const *const u8) -> bool {
    
    todo!();
        /*
        
        */
}
