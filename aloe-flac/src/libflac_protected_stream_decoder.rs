crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/protected/stream_decoder.h]

pub struct StreamDecoderProtected {
    state:              StreamDecoderState,
    initstate:          StreamDecoderInitStatus,
    channels:           u32,
    channel_assignment: ChannelAssignment,
    bits_per_sample:    u32,

    /**
      | in Hz
      |
      */
    sample_rate:        u32,


    /**
      | in samples (per channel)
      |
      */
    blocksize:          u32,


    /**
      | if true, generate MD5 signature of decoded
      | data and compare against signature
      | in the STREAMINFO metadata block
      |
      */
    md5_checking:       bool,

    #[cfg(HAS_OGG)]
    ogg_decoder_aspect: OggDecoderAspect,
}

/**
  | return the number of input bytes consumed
  |
  */
pub fn flac_stream_decoder_get_input_bytes_unconsumed(decoder: *const StreamDecoder) -> u32 {
    
    todo!();
        /*
        
        */
}
