/*!
 function: stdio-based convenience library for
 opening/seeking/decoding
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/vorbisfile.h]

/**
  | The function prototypes for the callbacks
  | are basically the same as for the stdio
  | functions fread, fseek, fclose, ftell.
  | 
  | The one difference is that the FILE *
  | arguments have been replaced with a
  | void * - this is to be used as a pointer
  | to whatever internal data these functions
  | might need. In the stdio case, it's just
  | a FILE * cast to a void *
  | 
  | If you use other functions, check the
  | docs for these functions and return
  | the right values. For seek_func(),
  | you *MUST* return -1 if the stream is
  | unseekable
  |
  */
pub struct OvCallbacks {
    read_func:  fn(
            ptr:        *mut c_void,
            size:       usize,
            nmemb:      usize,
            datasource: *mut c_void
    ) -> usize,

    seek_func:  fn(
            datasource: *mut c_void,
            offset:     i64,
            whence:     i32
    ) -> i32,

    close_func: fn(datasource: *mut c_void) -> i32,

    tell_func:  fn(datasource: *mut c_void) -> i64,

}

/**
  | a few sets of convenient callbacks,
  | especially for use under
  | 
  | Windows where ov_open_callbacks()
  | should always be used instead of ov_open()
  | to avoid problems with incompatible
  | crt.o version linking issues.
  |
  | These structs below (OV_CALLBACKS_DEFAULT
  | etc) are defined here as static data.
  | That means that every file which includes
  | this header will get its own copy of these
  | structs whether it uses them or not unless
  | it #defines OV_EXCLUDE_STATIC_CALLBACKS.
  | 
  | These static symbols are essential
  | on platforms such as Windows on which
  | several different versions of stdio
  | support may be linked to by different
  | DLLs, and we need to be certain we know
  | which one we're using (the same one as
  | the main application).
  |
  */

pub const NOTOPEN:   usize = 0;
pub const PARTOPEN:  usize = 1;
pub const OPENED:    usize = 2;
pub const STREAMSET: usize = 3;
pub const INITSET:   usize = 4;

pub struct OggVorbis_File {

    /**
      | Pointer to a FILE *, etc.
      |
      */
    datasource:       *mut c_void,

    seekable:         i32,
    offset:           i64,
    end:              i64,
    oy:               OggSyncState,

    /**
      | If the FILE handle isn't seekable (eg,
      | a pipe), only the current stream appears
      |
      */
    links:            i32,

    offsets:          *mut i64,
    dataoffsets:      *mut i64,
    serialnos:        *mut i64,

    /**
      | overloaded to maintain binary compatibility;
      | x2 size, stores both beginning and end
      | values
      |
      */
    pcmlengths:       *mut i64,

    vi:               *mut VorbisInfo,
    vc:               *mut VorbisComment,

    /**
      | Decoding working state local storage
      |
      */
    pcm_offset:       i64,

    ready_state:      i32,
    current_serialno: i64,
    current_link:     i32,
    bittrack:         f64,
    samptrack:        f64,

    /**
      | take physical pages, weld into a logical
      | stream of packets
      |
      */
    os:               OggStreamState,


    /**
      | central working state for the packet->PCM
      | decoder
      |
      */
    vd:               VorbisDspState,


    /**
      | local working space for packet->PCM
      | decode
      |
      */
    vb:               VorbisBlock,

    callbacks:        OvCallbacks,
}

lazy_static!{
    /*
    extern int ov_clear(OggVorbis_File *vf);
    extern int ov_fopen(const char *path,OggVorbis_File *vf);
    extern int ov_open(FILE *f,OggVorbis_File *vf,const char *initial,long ibytes);
    extern int ov_open_callbacks(void *datasource, OggVorbis_File *vf, const char *initial, long ibytes, ov_callbacks callbacks);
    extern int ov_test(FILE *f,OggVorbis_File *vf,const char *initial,long ibytes);
    extern int ov_test_callbacks(void *datasource, OggVorbis_File *vf, const char *initial, long ibytes, ov_callbacks callbacks);
    extern int ov_test_open(OggVorbis_File *vf);

    extern long ov_bitrate(OggVorbis_File *vf,int i);
    extern long ov_bitrate_instant(OggVorbis_File *vf);
    extern long ov_streams(OggVorbis_File *vf);
    extern long ov_seekable(OggVorbis_File *vf);
    extern long ov_serialnumber(OggVorbis_File *vf,int i);

    extern ogg_int64_t ov_raw_total(OggVorbis_File *vf,int i);
    extern ogg_int64_t ov_pcm_total(OggVorbis_File *vf,int i);
    extern double ov_time_total(OggVorbis_File *vf,int i);

    extern int ov_raw_seek(OggVorbis_File *vf,ogg_int64_t pos);
    extern int ov_pcm_seek(OggVorbis_File *vf,ogg_int64_t pos);
    extern int ov_pcm_seek_page(OggVorbis_File *vf,ogg_int64_t pos);
    extern int ov_time_seek(OggVorbis_File *vf,double pos);
    extern int ov_time_seek_page(OggVorbis_File *vf,double pos);

    extern int ov_raw_seek_lap(OggVorbis_File *vf,ogg_int64_t pos);
    extern int ov_pcm_seek_lap(OggVorbis_File *vf,ogg_int64_t pos);
    extern int ov_pcm_seek_page_lap(OggVorbis_File *vf,ogg_int64_t pos);
    extern int ov_time_seek_lap(OggVorbis_File *vf,double pos);
    extern int ov_time_seek_page_lap(OggVorbis_File *vf,double pos);

    extern ogg_int64_t ov_raw_tell(OggVorbis_File *vf);
    extern ogg_int64_t ov_pcm_tell(OggVorbis_File *vf);
    extern double ov_time_tell(OggVorbis_File *vf);

    extern vorbis_info *ov_info(OggVorbis_File *vf,int link);
    extern vorbis_comment *ov_comment(OggVorbis_File *vf,int link);

    extern long ov_read_float(OggVorbis_File *vf,float ***pcm_channels,int samples,
                              int *bitstream);
    extern long ov_read_filter(OggVorbis_File *vf,char *buffer,int length,
                              int bigendianp,int word,int sgned,int *bitstream,
                              void (*filter)(float **pcm,long channels,long samples,void *filter_param),void *filter_param);
    extern long ov_read(OggVorbis_File *vf,char *buffer,int length,
                        int bigendianp,int word,int sgned,int *bitstream);
    extern int ov_crosslap(OggVorbis_File *vf1,OggVorbis_File *vf2);

    extern int ov_halfrate(OggVorbis_File *vf,int flag);
    extern int ov_halfrate_p(OggVorbis_File *vf);
    */
}
