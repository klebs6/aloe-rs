crate::ix!();

/**
  | FLAC STREAMINFO structure. (c.f. <A
  | HREF="../format.html#metadata_block_streaminfo">format
  | specification</A>)
  |
  */
#[derive(Copy,Clone)]
pub struct StreamMetadata_StreamInfo {
    min_blocksize:   u32,
    max_blocksize:   u32,
    min_framesize:   u32,
    max_framesize:   u32,
    sample_rate:     u32,
    channels:        u32,
    bits_per_sample: u32,
    total_samples:   u64,
    md5sum:          [u8; 16],
}

/**
  | == 16 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_STREAMINFO_MIN_BLOCK_SIZE_LEN;
    */
}

/**
  | == 16 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_STREAMINFO_MAX_BLOCK_SIZE_LEN;
    */
}

/**
  | == 24 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_STREAMINFO_MIN_FRAME_SIZE_LEN;
    */
}

/**
  | == 24 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_STREAMINFO_MAX_FRAME_SIZE_LEN;
    */
}

/**
  | == 20 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_STREAMINFO_SAMPLE_RATE_LEN;
    */
}

/**
  | == 3 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_STREAMINFO_CHANNELS_LEN;
    */
}

/**
  | == 5 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_STREAMINFO_BITS_PER_SAMPLE_LEN;
    */
}

/**
  | == 36 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_STREAMINFO_TOTAL_SAMPLES_LEN;
    */
}

/**
  | == 128 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_STREAMINFO_MD5SUM_LEN;
    */
}

/**
  | The total stream length of the STREAMINFO
  | block in bytes.
  |
  */
pub const STREAM_METADATA_STREAMINFO_LENGTH: usize = 34;

/**
  | FLAC PADDING structure. (c.f. <A HREF="../format.html#metadata_block_padding">format
  | specification</A>)
  |
  */
#[derive(Copy,Clone)]
pub struct StreamMetadata_Padding {

    /**
      | Conceptually this is an empty struct
      | since we don't store the padding bytes.
      | Empty structs are not allowed by some
      | C compilers, hence the dummy.
      |
      */
    dummy: i32,
}

/**
  | FLAC APPLICATION structure. (c.f.
  | <A HREF="../format.html#metadata_block_application">format
  | specification</A>)
  |
  */
#[derive(Copy,Clone)]
pub struct StreamMetadata_Application {
    id:   [u8; 4],
    data: *mut u8,
}

/**
  | == 32 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_APPLICATION_ID_LEN;
    */
}

/**
  | SeekPoint structure used in SEEKTABLE
  | blocks. (c.f. <A HREF="../format.html#seekpoint">format
  | specification</A>)
  |
  */
pub struct StreamMetadata_SeekPoint {

    /**
      | The sample number of the target frame.
      |
      */
    sample_number: u64,


    /**
      | The offset, in bytes, of the target frame
      | with respect to beginning of the first
      | frame.
      |
      */
    stream_offset: u64,


    /**
      | The number of samples in the target frame.
      |
      */
    frame_samples: u32,
}

/**
  | == 64 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_SEEKPOINT_SAMPLE_NUMBER_LEN;
    */
}

/**
  | == 64 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_SEEKPOINT_STREAM_OFFSET_LEN;
    */
}

/**
  | == 16 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_SEEKPOINT_FRAME_SAMPLES_LEN;
    */
}

/**
  | The total stream length of a seek point
  | in bytes.
  |
  */
pub const STREAM_METADATA_SEEKPOINT_LENGTH: usize = 18;

/**
  | The value used in the \a sample_number
  | field of
  | 
  | StreamMetadataSeekPoint used
  | to indicate a placeholder point (==
  | 0xffffffffffffffff).
  |
  */
lazy_static!{
    /*
    extern  const u64 STREAM_METADATA_SEEKPOINT_PLACEHOLDER;
    */
}

/**
  | FLAC SEEKTABLE structure. (c.f. <A
  | HREF="../format.html#metadata_block_seektable">format
  | specification</A>)
  | 
  | -----------
  | @note
  | 
  | From the format specification:
  | 
  | - The seek points must be sorted by ascending
  | sample number.
  | 
  | - Each seek point's sample number must
  | be the first sample of the target frame.
  | 
  | - Each seek point's sample number must
  | be unique within the table.
  | 
  | - Existence of a SEEKTABLE block implies
  | a correct setting of total_samples
  | in the stream_info block.
  | 
  | - Behavior is undefined when more than
  | one SEEKTABLE block is present in a stream.
  |
  */
#[derive(Copy,Clone)]
pub struct StreamMetadata_SeekTable {
    num_points: u32,
    points:     *mut StreamMetadata_SeekPoint,
}

/**
  | Vorbis comment entry structure used
  | in VORBIS_COMMENT blocks. (c.f. <A
  | HREF="../format.html#metadata_block_vorbis_comment">format
  | specification</A>)
  | 
  | For convenience, the APIs maintain
  | a trailing NUL character at the end of
  | \a entry which is not counted toward
  | \a length, i.e.
  | 
  | -----------
  | @code
  | 
  | strlen(entry) == length
  |
  */
#[derive(Copy,Clone)]
pub struct StreamMetadata_VorbisComment_Entry {
    length: u32,
    entry:  *mut u8,
}

/**
  | == 32 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_VORBIS_COMMENT_ENTRY_LENGTH_LEN;
    */
}

/**
  | FLAC VORBIS_COMMENT structure. (c.f.
  | <A HREF="../format.html#metadata_block_vorbis_comment">format
  | specification</A>)
  |
  */
#[derive(Copy,Clone)]
pub struct StreamMetadata_VorbisComment {
    vendor_string: StreamMetadata_VorbisComment_Entry,
    num_comments:  u32,
    comments:      *mut StreamMetadata_VorbisComment_Entry,
}

/**
  | == 32 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_VORBIS_COMMENT_NUM_COMMENTS_LEN;
    */
}

/**
  | FLAC CUESHEET track index structure.
  | (See the <A HREF="../format.html#cuesheet_track_index">format
  | specification</A> for the full description
  | of each field.)
  |
  */
pub struct StreamMetadata_CueSheet_Index {

    /**
      | Offset in samples, relative to the track
      | offset, of the index point.
      |
      */
    offset: u64,


    /**
      | The index point number.
      |
      */
    number: u8,
}

/**
  | == 64 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_INDEX_OFFSET_LEN;
    */
}

/**
  | == 8 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_INDEX_NUMBER_LEN;
    */
}

/**
  | == 3*8 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_INDEX_RESERVED_LEN;
    */
}

/**
  | FLAC CUESHEET track structure. (See
  | the <A HREF="../format.html#cuesheet_track">format
  | specification</A> for the full description
  | of each field.)
  |
  */
//#[bitfield]
pub struct StreamMetadata_CueSheet_Track {

    /**
      | Track offset in samples, relative to
      | the beginning of the FLAC audio stream.
      |
      */
    offset:       u64,

    /**
      | The track number.
      |
      */
    number:       u8,

    /**
      | Track ISRC. This is a 12-digit alphanumeric
      | code plus a trailing \c NUL byte
      |
      */
    isrc:         [u8; 13],

    /**
      | The track type: 0 for audio, 1 for non-audio.
      |
      */
    ty:           bool,//B1,

    /**
      | The pre-emphasis flag: 0 for no pre-emphasis,
      | 1 for pre-emphasis.
      |
      */
    pre_emphasis: bool,//B1,

    /**
      | The number of track index points.
      |
      */
    num_indices:  u8,

    /**
      | NULL if num_indices == 0, else pointer
      | to array of index points.
      |
      */
    indices:      *mut StreamMetadata_CueSheet_Index,
}

/**
  | == 64 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_TRACK_OFFSET_LEN;
    */
}

/**
  | == 8 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_TRACK_NUMBER_LEN;
    */
}

/**
  | == 12*8 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_TRACK_ISRC_LEN;
    */
}

/**
  | == 1 (bit)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_TRACK_TYPE_LEN;
    */
}

/**
  | == 1 (bit)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_TRACK_PRE_EMPHASIS_LEN;
    */
}

/**
  | == 6+13*8 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_TRACK_RESERVED_LEN;
    */
}

/**
  | == 8 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_TRACK_NUM_INDICES_LEN;
    */
}

/**
  | FLAC CUESHEET structure. (See the <A
  | HREF="../format.html#metadata_block_cuesheet">format
  | specification</A> for the full description
  | of each field.)
  |
  */
#[derive(Copy,Clone)]
pub struct StreamMetadata_CueSheet {

    /**
      | Media catalog number, in ASCII printable
      | characters 0x20-0x7e. In general,
      | the media catalog number may be 0 to 128
      | bytes long; any unused characters should
      | be right-padded with NUL characters.
      |
      */
    media_catalog_number: [u8; 129],

    /**
      | The number of lead-in samples.
      |
      */
    lead_in:              u64,

    /**
      | \c true if CUESHEET corresponds to a
      | Compact Disc, else \c false.
      |
      */
    is_cd:                bool,

    /**
      | The number of tracks.
      |
      */
    num_tracks:           u32,

    /**
      | NULL if num_tracks == 0, else pointer
      | to array of tracks.
      |
      */
    tracks:               *mut StreamMetadata_CueSheet_Track,
}

/**
  | == 128*8 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_MEDIA_CATALOG_NUMBER_LEN;
    */
}

/**
  | == 64 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_LEAD_IN_LEN;
    */
}

/**
  | == 1 (bit)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_IS_CD_LEN;
    */
}

/**
  | == 7+258*8 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_RESERVED_LEN;
    */
}

/**
  | == 8 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_CUESHEET_NUM_TRACKS_LEN;
    */
}

/**
  | An enumeration of the PICTURE types
  | (see StreamMetadataPicture
  | and id3 v2.4 APIC tag).
  |
  */
#[derive(Copy,Clone)]
pub enum StreamMetadata_Picture_Type {

    /**
      | Other
      |
      */
    STREAM_METADATA_PICTURE_TYPE_OTHER = 0, 

    /**
      | 32x32 pixels 'file icon' (PNG only)
      |
      */
    STREAM_METADATA_PICTURE_TYPE_FILE_ICON_STANDARD = 1, 

    /**
      | Other file icon
      |
      */
    STREAM_METADATA_PICTURE_TYPE_FILE_ICON = 2, 

    /**
      | Cover (front)
      |
      */
    STREAM_METADATA_PICTURE_TYPE_FRONT_COVER = 3, 

    /**
      | Cover (back)
      |
      */
    STREAM_METADATA_PICTURE_TYPE_BACK_COVER = 4, 

    /**
      | Leaflet page
      |
      */
    STREAM_METADATA_PICTURE_TYPE_LEAFLET_PAGE = 5, 

    /**
      | Media (e.g. label side of CD)
      |
      */
    STREAM_METADATA_PICTURE_TYPE_MEDIA = 6, 

    /**
      | Lead artist/lead performer/soloist
      |
      */
    STREAM_METADATA_PICTURE_TYPE_LEAD_ARTIST = 7, 

    /**
      | Artist/performer
      |
      */
    STREAM_METADATA_PICTURE_TYPE_ARTIST = 8, 

    /**
      | Conductor
      |
      */
    STREAM_METADATA_PICTURE_TYPE_CONDUCTOR = 9, 

    /**
      | Band/Orchestra
      |
      */
    STREAM_METADATA_PICTURE_TYPE_BAND = 10, 

    /**
      | Composer
      |
      */
    STREAM_METADATA_PICTURE_TYPE_COMPOSER = 11, 

    /**
      | Lyricist/text writer
      |
      */
    STREAM_METADATA_PICTURE_TYPE_LYRICIST = 12, 

    /**
      | Recording Location
      |
      */
    STREAM_METADATA_PICTURE_TYPE_RECORDING_LOCATION = 13, 

    /**
      | During recording
      |
      */
    STREAM_METADATA_PICTURE_TYPE_DURING_RECORDING = 14, 

    /**
      | During performance
      |
      */
    STREAM_METADATA_PICTURE_TYPE_DURING_PERFORMANCE = 15, 

    /**
      | Movie/video screen capture
      |
      */
    STREAM_METADATA_PICTURE_TYPE_VIDEO_SCREEN_CAPTURE = 16, 

    /**
      | A bright coloured fish
      |
      */
    STREAM_METADATA_PICTURE_TYPE_FISH = 17, 

    /**
      | Illustration
      |
      */
    STREAM_METADATA_PICTURE_TYPE_ILLUSTRATION = 18, 

    /**
      | Band/artist logotype
      |
      */
    STREAM_METADATA_PICTURE_TYPE_BAND_LOGOTYPE = 19, 

    /**
      | Publisher/Studio logotype
      |
      */
    STREAM_METADATA_PICTURE_TYPE_PUBLISHER_LOGOTYPE = 20, 

    STREAM_METADATA_PICTURE_TYPE_UNDEFINED
}

/**
  | Maps a StreamMetadata_Picture_Type
  | to a C string.
  | 
  | Using a StreamMetadata_Picture_Type
  | as the index to this array will give the
  | string equivalent. The contents should
  | not be modified.
  |
  */
lazy_static!{
    /*
    extern  const char * const StreamMetadata_Picture_TypeString[];
    */
}

/**
  | FLAC PICTURE structure. (See the <A
  | HREF="../format.html#metadata_block_picture">format
  | specification</A> for the full description
  | of each field.)
  |
  */
#[derive(Copy,Clone)]
pub struct StreamMetadata_Picture {

    /**
      | The kind of picture stored.
      |
      */
    ty:          StreamMetadata_Picture_Type,

    /**
      | Picture data's MIME type, in ASCII printable
      | characters 0x20-0x7e, NUL terminated.
      | For best compatibility with players,
      | use picture data of MIME type \c image/jpeg
      | or \c image/png. A
      | 
      | MIME type of '-->' is also allowed, in
      | which case the picture data should be
      | a complete Url. In file storage, the
      | MIME type is stored as a 32-bit length
      | followed by the ASCII string with no
      | NUL terminator, but is converted to
      | a plain C string in this structure for
      | convenience.
      |
      */
    mime_type:   *mut u8,

    /**
      | Picture's description in UTF-8, NUL
      | terminated. In file storage, the description
      | is stored as a 32-bit length followed
      | by the UTF-8 string with no NUL terminator,
      | but is converted to a plain C string in
      | this structure for convenience.
      |
      */
    description: *mut u8,

    /**
      | Picture's width in pixels.
      |
      */
    width:       u32,

    /**
      | Picture's height in pixels.
      |
      */
    height:      u32,

    /**
      | Picture's color depth in bits-per-pixel.
      |
      */
    depth:       u32,

    /**
      | For indexed palettes (like GIF), picture's
      | number of colors (the number of palette
      | entries), or \c 0 for non-indexed (i.e.
      | 2^depth).
      |
      */
    colors:      u32,

    /**
      | Length of binary picture data in bytes.
      |
      */
    data_length: u32,

    /**
      | Binary picture data.
      |
      */
    data:        *mut u8,
}

/**
  | == 32 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_PICTURE_TYPE_LEN;
    */
}

/**
  | == 32 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_PICTURE_MIME_TYPE_LENGTH_LEN;
    */
}

/**
  | == 32 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_PICTURE_DESCRIPTION_LENGTH_LEN;
    */
}

/**
  | == 32 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_PICTURE_WIDTH_LEN;
    */
}

/**
  | == 32 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_PICTURE_HEIGHT_LEN;
    */
}

/**
  | == 32 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_PICTURE_DEPTH_LEN;
    */
}

/**
  | == 32 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_PICTURE_COLORS_LEN;
    */
}

/**
  | == 32 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_PICTURE_DATA_LENGTH_LEN;
    */
}

/**
  | Structure that is used when a metadata
  | block of unknown type is loaded.
  | 
  | The contents are opaque. The structure
  | is used only internally to correctly
  | handle unknown metadata.
  |
  */
#[derive(Copy,Clone)]
pub struct StreamMetadata_Unknown {
    data: *mut u8,
}

/**
  | FLAC metadata block structure. (c.f.
  | <A HREF="../format.html#metadata_block">format
  | specification</A>)
  |
  */
pub struct StreamMetadata {

    /**
      | The type of the metadata block; used
      | determine which member of the \a data
      | union to dereference. If type >= METADATA_TYPE_UNDEFINED
      | then \a data.unknown must be used.
      |
      */
    ty:      MetadataType,

    /**
      | \c true if this metadata block is the
      | last, else \a false
      |
      */
    is_last: bool,

    /**
      | Length, in bytes, of the block data as
      | it appears in the stream.
      |
      */
    length:  u32,

    /**
      | Polymorphic block data; use the \a type
      | value to determine which to use.
      |
      */
    data:    FlacStreamMetadataU,
}

pub union FlacStreamMetadataU {
    stream_info:    StreamMetadata_StreamInfo,
    padding:        StreamMetadata_Padding,
    application:    StreamMetadata_Application,
    seek_table:     StreamMetadata_SeekTable,
    vorbis_comment: StreamMetadata_VorbisComment,
    cue_sheet:      StreamMetadata_CueSheet,
    picture:        StreamMetadata_Picture,
    unknown:        StreamMetadata_Unknown,
}

/**
  | == 1 (bit)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_IS_LAST_LEN;
    */
}

/**
  | == 7 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_TYPE_LEN;
    */
}

/**
  | == 24 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned STREAM_METADATA_LENGTH_LEN;
    */
}

/**
  | The total stream length of a metadata
  | block header in bytes.
  |
  */
pub const STREAM_METADATA_HEADER_LENGTH: usize = 4;

