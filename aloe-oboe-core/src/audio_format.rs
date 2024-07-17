crate::ix!();

/**
  | The format of audio samples.
  |
  */
#[repr(i32)]
// aaudio_format_t
#[derive(Clone)]
pub enum OboeAudioFormat { 

    /**
      | Invalid format.
      |
      */
    Invalid = -1, // AAUDIO_FORMAT_INVALID,

    /**
      | Unspecified format. Format will be
      | decided by Oboe.
      |
      */
    Unspecified = 0, // AAUDIO_FORMAT_UNSPECIFIED,

    /**
      | Signed 16-bit integers.
      |
      */
    I16 = 1, // AAUDIO_FORMAT_PCM_I16,

    /**
      | Single precision floating point.
      | 
      | This is the recommended format for most
      | applications. But note that the use
      | of Float may prevent the opening of a
      | low-latency input path on OpenSL ES
      | or Legacy AAudio streams.
      |
      */
    Float = 2, // AAUDIO_FORMAT_PCM_FLOAT,

    /**
      | Signed 24-bit integers, packed into
      | 3 bytes.
      | 
      | Note that the use of this format does
      | not guarantee that the full precision
      | will be provided. The underlying device
      | may be using I16 format.
      | 
      | Added in API 31 (S).
      |
      */
    I24 = 3, // AAUDIO_FORMAT_PCM_I24_PACKED

    /**
      | Signed 32-bit integers.
      | 
      | Note that the use of this format does
      | not guarantee that the full precision
      | will be provided. The underlying device
      | may be using I16 format.
      | 
      | Added in API 31 (S).
      |
      */
    I32 = 4, // AAUDIO_FORMAT_PCM_I32
}
