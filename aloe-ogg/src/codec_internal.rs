/*!
   function: libvorbis codec headers
 */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/codec_internal.h]

pub const BLOCKTYPE_IMPULSE:    usize = 0;
pub const BLOCKTYPE_PADDING:    usize = 1;
pub const BLOCKTYPE_TRANSITION: usize = 0;
pub const BLOCKTYPE_LONG:       usize = 1;
pub const PACKETBLOBS:          usize = 15;

pub struct VorbisBlockInternal {

    /**
       this is a pointer into local storage
      */
    pcmdelay:   *mut *mut f32,

    ampmax:     f32,
    blocktype:  i32,

    /**
       initialized, must be freed; blob
       [PACKETBLOBS/2] points to the
       oggpack_buffer in the main vorbis_block
      */
    packetblob: *mut [OggPackBuffer; PACKETBLOBS],
}

pub type VorbisLookFloor     = c_void;
pub type VorbisLookResidue   = c_void;
pub type VorbisLookTransform = c_void;

/**
  | mode
  |
  */
pub struct VorbisInfoMode {
    pub blockflag:     i32,
    pub windowtype:    i32,
    pub transformtype: i32,
    pub mapping:       i32,
}

pub type VorbisInfoFloor   = c_void;
pub type VorbisInfoResidue = c_void;
pub type VorbisInfoMapping = c_void;

pub struct PrivateState<DataType> {

    /* -------------- local lookup storage  -------------- */

    /**
       envelope lookup
      */
    ve:           *mut EnvelopeLookup<DataType>,

    window:       [i32; 2],

    /**
       block, type
      */
    transform:    *mut *mut [VorbisLookTransform; 2],

    fft_look:     [DrftLookup; 2],
    modebits:     i32,
    flr:          *mut *mut VorbisLookFloor,
    residue:      *mut *mut VorbisLookResidue,
    psy:          *mut VorbisLookPsy,
    psy_g_look:   *mut VorbisLookPsyGlobal,

    /**
      | local storage, only used on the encoding side.
      | This way the application does not need to worry
      | about freeing some packets' memory and not others';
      | packet storage is always tracked. Cleared next
      | call to a _dsp_ function
      */
    header:       *mut u8,

    header1:      *mut u8,
    header2:      *mut u8,
    bms:          BitrateManagerState,
    sample_count: i64,
}

/** 
  | codec_setup_info contains all the setup
  | information specific to the specific
  | compression/decompression mode in progress (eg,
  | psychoacoustic settings, channel setup, options,
  | codebook etc).
  */
pub struct CodecSetupInfo {

    /**
       Vorbis supports only short and long blocks, but
       allows the encoder to choose the sizes
      */
    blocksizes:    [i64; 2],

    /**
      | modes are the primary means of supporting on-the-fly
      | different blocksizes, different channel mappings
      | (LR or M/A), different residue backends, etc. Each
      | mode consists of a blocksize flag and a mapping
      | (along with the mapping setup
      */
    modes:         i32,

    maps:          i32,
    floors:        i32,
    residues:      i32,
    books:         i32,

    /**
       encode only
      */
    psys:          i32,

    mode_param:    *mut [VorbisInfoMode; 64],
    map_type:      [i32; 64],
    map_param:     *mut [VorbisInfoMapping; 64],
    floor_type:    [i32; 64],
    floor_param:   *mut [VorbisInfoFloor; 64],
    residue_type:  [i32; 64],
    residue_param: *mut [VorbisInfoResidue; 64],
    book_param:    *mut [StaticCodebook; 256],
    fullbooks:     *mut Codebook,

    /**
       encode only
      */
    psy_param:     *mut [VorbisInfoPsy; 4],

    psy_g_param:   VorbisInfoPsyGlobal,
    bi:            BitrateManagerInfo,

    /**
       used only by vorbisenc.c. It's a highly redundant
       structure, but improves clarity of program flow.
      */
    hi:            HighLevelEncodeSetup,

    /**
       painless downsample for decode
      */
    halfrate_flag: i32,
}

lazy_static!{
    /*
    extern vorbis_look_psy_global *_vp_global_look(vorbis_info *vi);
    extern c_void _vp_global_free(vorbis_look_psy_global *look);
    */
}

pub struct VorbisLookFloor1 {
    sorted_index:  [i32; VIF_POSIT+2],
    forward_index: [i32; VIF_POSIT+2],
    reverse_index: [i32; VIF_POSIT+2],
    hineighbor:    [i32; VIF_POSIT],
    loneighbor:    [i32; VIF_POSIT],
    posts:         i32,
    n:             i32,
    quant_q:       i32,
    vi:            *mut VorbisInfoFloor1,
    phrasebits:    i64,
    postbits:      i64,
    frames:        i64,
}

lazy_static!{
    /*
    extern int *floor1_fit(vorbis_block *vb,vorbis_look_floor1 *look,
                              const float *logmdct,   /* in */
                              const float *logmask);
    extern int *floor1_interpolate_fit(vorbis_block *vb,vorbis_look_floor1 *look,
                              int *A,int *B,
                              int del);
    extern int floor1_encode(oggpack_buffer *opb,vorbis_block *vb,
                      vorbis_look_floor1 *look,
                      int *post,int *ilogmask);
    */
}

