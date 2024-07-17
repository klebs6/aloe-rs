crate::ix!();

/**
  | The channel count of the audio stream.
  | The underlying type is `int32_t`.
  | 
  | Use of this enum is convenient to avoid
  | "magic" numbers when specifying the
  | channel count.
  | 
  | For example, you can write `builder.setChannelCount(ChannelCount::Stereo)`
  | rather than `builder.setChannelCount(2)`
  |
  */
#[repr(i32)]
#[derive(Clone)]
pub enum OboeChannelCount {

    /**
      | Audio channel count definition, use
      | Mono or Stereo
      |
      */
    Unspecified = kUnspecified,

    /**
      | Use this for mono audio
      |
      */
    Mono = 1,

    /**
      | Use this for stereo audio.
      |
      */
    Stereo = 2,
}
