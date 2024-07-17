crate::ix!();

pub trait Create5Point0 {

    /** 
      | Creates a set for a 5.0 surround setup (left,
      | right, centre, leftSurround, rightSurround).
      |
      | Is equivalent to: k50 (VST),
      | AAX_eStemFormat_5_0 (AAX),
      | kAudioChannelLayoutTag_MPEG_5_0_A
      | (CoreAudio)
      |
      | This format is referred to as "5.0" in
      | Cubase.
      |
      | This format is referred to as "5.0" in Pro
      | Tools.
      */
    fn create5point0(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Create5Point1 {

    /** 
      | Creates a set for a 5.1 surround setup (left,
      | right, centre, leftSurround, rightSurround,
      | LFE).
      |
      | Is equivalent to: k51 (VST),
      | AAX_eStemFormat_5_1 (AAX),
      | kAudioChannelLayoutTag_MPEG_5_1_A
      | (CoreAudio)
      |
      | This format is referred to as "5.1 (ITU
      | 775)" in Logic Pro.
      |
      | This format is referred to as "5.1" in
      | Cubase.
      |
      | This format is referred to as "5.1" in Pro
      | Tools.
      */
    fn create5point1(&mut self) -> dyn AudioChannelSetInterface;
}
