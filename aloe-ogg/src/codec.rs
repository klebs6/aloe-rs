/*!
  | function: libvorbis codec headers
  |
  | libvorbis encodes in two abstraction
  | layers; first we perform DSP and produce
  | a packet (see docs/analysis.txt).
  | The packet is then coded into a framed
  | OggSquish bitstream by the second layer
  | (see docs/framing.txt). Decode is
  | the reverse process; we sync/frame
  | the bitstream and extract individual
  | packets, then decode the packet back
  | into PCM audio.
  | 
  | The extra framing/packetizing is used
  | in streaming formats, such as files.
  | Over the net (such as with UDP), the framing
  | and packetization aren't necessary
  | as they're provided by the transport
  | and the streaming layer is not used
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/codec.h]

pub struct VorbisInfo{
    version:         i32,
    channels:        i32,
    rate:            i64,

    /** 
      | The below bitrate declarations are *hints*.
      |   Combinations of the three values carry the
      |   following implications:
      |
      |   all three set to the same value:
      |     implies a fixed rate bitstream
      |
      |   only nominal set:
      |     implies a VBR stream that averages the
      |     nominal bitrate.  No hard upper/lower limit
      |
      |   upper and or lower set:
      |     implies a VBR bitstream that obeys the
      |     bitrate limits. nominal may also be set to
      |     give a nominal rate.
      |
      |   none set:
      |     the coder does not care to speculate.
      */

    bitrate_upper:   i64,
    bitrate_nominal: i64,
    bitrate_lower:   i64,
    bitrate_window:  i64,
    codec_setup:     *mut c_void,
}

/**
  | vorbis_dsp_state buffers the current
  | vorbis audio analysis/synthesis state.
  | The DSP state belongs to a specific logical
  | bitstream 
  |
  */
pub struct VorbisDspState{
    analysisp:      i32,
    vi:             *mut VorbisInfo,
    pcm:            *mut *mut f32,
    pcmret:         *mut *mut f32,
    pcm_storage:    i32,
    pcm_current:    i32,
    pcm_returned:   i32,
    preextrapolate: i32,
    eofflag:        i32,
    lw:             i64,
    w:              i64,
    nw:             i64,
    centerw:        i64,
    granulepos:     i64,
    sequence:       i64,
    glue_bits:      i64,
    time_bits:      i64,
    floor_bits:     i64,
    res_bits:       i64,
    backend_state:  *mut c_void,
}

pub struct VorbisBlock {

    /* 
       necessary stream state for linking to the
       framing abstraction 
       */

    /**
       this is a pointer into local storage
      */
    pcm:        *mut *mut f32,

    opb:        OggPackBuffer,
    lw:         i64,
    w:          i64,
    nw:         i64,
    pcmend:     i32,
    mode:       i32,
    eofflag:    i32,
    granulepos: i64,
    sequence:   i64,

    /**
       For read-only access of configuration
      */
    vd:         *mut VorbisDspState,

    /**
       local storage to avoid remallocing; it's up
       to the mapping to structure it
      */
    localstore: *mut c_void,

    localtop:   i64,
    localalloc: i64,
    totaluse:   i64,
    reap:       *mut AllocChain,

    /**
       bitmetrics for the frame
      */
    glue_bits:  i64,

    time_bits:  i64,
    floor_bits: i64,
    res_bits:   i64,
    internal:   *mut c_void,
}

/**
  | vorbis_block is a single block of data
  | to be processed as part of the analysis/synthesis
  | stream; it belongs to a specific logical
  | bitstream, but is independent from
  | other vorbis_blocks belonging to that
  | logical bitstream
  |
  */
pub struct AllocChain {
    ptr:  *mut c_void,
    next: *mut AllocChain,
}

/*
  | vorbis_info contains all the setup
  | information specific to the specific
  | compression/decompression mode in
  | progress (eg, psychoacoustic settings,
  | channel setup, options, codebook etc).
  | vorbis_info and substructures are
  | in backends.h
  */

/**
  | the comments are not part of vorbis_info
  | so that vorbis_info can be static storage
  */
pub struct VorbisComment {

    /**
       unlimited user comment fields. libvorbis
       writes 'libvorbis' whatever vendor is set
       to in encode
      */
    user_comments:   *mut *mut u8,

    comment_lengths: *mut i32,
    comments:        i32,
    vendor:          *mut u8,
}

/**
  | Vorbis PRIMITIVES: general
  */
lazy_static!{
    /*
    extern void     vorbis_info_init(vorbis_info *vi);
    extern void     vorbis_info_clear(vorbis_info *vi);
    extern int      vorbis_info_blocksize(vorbis_info *vi,int zo);
    extern void     vorbis_comment_init(vorbis_comment *vc);
    extern void     vorbis_comment_add(vorbis_comment *vc, const char *comment);
    extern void     vorbis_comment_add_tag(vorbis_comment *vc,
                                           const char *tag, const char *contents);
    extern char    *vorbis_comment_query(vorbis_comment *vc, const char *tag, int count);
    extern int      vorbis_comment_query_count(vorbis_comment *vc, const char *tag);
    extern void     vorbis_comment_clear(vorbis_comment *vc);

    extern int      vorbis_block_init(vorbis_dsp_state *v, vorbis_block *vb);
    extern int      vorbis_block_clear(vorbis_block *vb);
    extern void     vorbis_dsp_clear(vorbis_dsp_state *v);
    extern double   vorbis_granule_time(vorbis_dsp_state *v,
                                        ogg_int64_t granulepos);

    extern const char *vorbis_version_string(void);
    */
}

/**
  | Vorbis PRIMITIVES: analysis/DSP layer
  |
  */
lazy_static!{
    /*
    extern int      vorbis_analysis_init(vorbis_dsp_state *v,vorbis_info *vi);
    extern int      vorbis_commentheader_out(vorbis_comment *vc, ogg_packet *op);
    extern int      vorbis_analysis_headerout(vorbis_dsp_state *v,
                                              vorbis_comment *vc,
                                              ogg_packet *op,
                                              ogg_packet *op_comm,
                                              ogg_packet *op_code);
    extern float  **vorbis_analysis_buffer(vorbis_dsp_state *v,int vals);
    extern int      vorbis_analysis_wrote(vorbis_dsp_state *v,int vals);
    extern int      vorbis_analysis_blockout(vorbis_dsp_state *v,vorbis_block *vb);
    extern int      vorbis_analysis(vorbis_block *vb,ogg_packet *op);

    extern int      vorbis_bitrate_addblock(vorbis_block *vb);
    extern int      vorbis_bitrate_flushpacket(vorbis_dsp_state *vd,
                                               ogg_packet *op);
    */
}

/**
  | Vorbis PRIMITIVES: synthesis layer
  |
  */
lazy_static!{
    /*
    extern int      vorbis_synthesis_idheader(ogg_packet *op);
    extern int      vorbis_synthesis_headerin(vorbis_info *vi,vorbis_comment *vc,
                                              ogg_packet *op);

    extern int      vorbis_synthesis_init(vorbis_dsp_state *v,vorbis_info *vi);
    extern int      vorbis_synthesis_restart(vorbis_dsp_state *v);
    extern int      vorbis_synthesis(vorbis_block *vb,ogg_packet *op);
    extern int      vorbis_synthesis_trackonly(vorbis_block *vb,ogg_packet *op);
    extern int      vorbis_synthesis_blockin(vorbis_dsp_state *v,vorbis_block *vb);
    extern int      vorbis_synthesis_pcmout(vorbis_dsp_state *v,float ***pcm);
    extern int      vorbis_synthesis_lapout(vorbis_dsp_state *v,float ***pcm);
    extern int      vorbis_synthesis_read(vorbis_dsp_state *v,int samples);
    extern long     vorbis_packet_blocksize(vorbis_info *vi,ogg_packet *op);

    extern int      vorbis_synthesis_halfrate(vorbis_info *v,int flag);
    extern int      vorbis_synthesis_halfrate_p(vorbis_info *v);
    */
}

/**
  | Vorbis ERRORS and return codes
  |
  */
pub const OV_FALSE:      isize = -1;
pub const OV_EOF:        isize = -2;
pub const OV_HOLE:       isize = -3;
pub const OV_EREAD:      isize = -128;
pub const OV_EFAULT:     isize = -129;
pub const OV_EIMPL:      isize = -130;
pub const OV_EINVAL:     isize = -131;
pub const OV_ENOTVORBIS: isize = -132;
pub const OV_EBADHEADER: isize = -133;
pub const OV_EVERSION:   isize = -134;
pub const OV_ENOTAUDIO:  isize = -135;
pub const OV_EBADPACKET: isize = -136;
pub const OV_EBADLINK:   isize = -137;
pub const OV_ENOSEEK:    isize = -138;
