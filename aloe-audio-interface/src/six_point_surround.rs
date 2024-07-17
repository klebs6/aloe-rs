crate::ix!();

pub trait Create6Point0 {

    /** 
      | Creates a set for a 6.0 Cine surround setup
      | (left, right, centre, leftSurround,
      | rightSurround, centreSurround).
      |
      | Is equivalent to: k60Cine (VST),
      | AAX_eStemFormat_6_0 (AAX),
      | kAudioChannelLayoutTag_AudioUnit_6_0
      | (CoreAudio)
      |
      | Logic Pro incorrectly uses this for the
      | surround format labeled "6.1 (ES/EX)".
      |
      | This format is referred to as "6.0 Cine"
      | in Cubase.
      |
      | This format is referred to as "6.0" in Pro
      | Tools.
      */
    fn create6point0(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Create6Point1 {

    /** 
      | Creates a set for a 6.1 Cine surround setup
      | (left, right, centre, leftSurround,
      | rightSurround, centreSurround, LFE).
      |
      | Is equivalent to: k61Cine (VST),
      | AAX_eStemFormat_6_1 (AAX),
      | kAudioChannelLayoutTag_MPEG_6_1_A
      | (CoreAudio)
      |
      | This format is referred to as "6.1" in
      | Pro Tools.
     */
    fn create6point1(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Create6Point0Music {

    /** 
      | Creates a set for a 6.0 Music surround setup
      | (left, right, leftSurround, rightSurround,
      | leftSurroundSide, rightSurroundSide).
      |
      | Is equivalent to: k60Music (VST), n/a
      | (AAX), kAudioChannelLayoutTag_DTS_6_0_A
      | (CoreAudio)
      |
      | This format is referred to as "6.0 Music"
      | in Cubase.
      */
    fn create6point_0music(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Create6Point1Music {

    /** 
      | Creates a set for a 6.0 Music surround setup
      | (left, right, leftSurround, rightSurround,
      | leftSurroundSide, rightSurroundSide, LFE).
      |
      | Is equivalent to: k61Music (VST), n/a
      | (AAX), kAudioChannelLayoutTag_DTS_6_1_A
      | (CoreAudio)
      */
    fn create6point_1music(&mut self) -> dyn AudioChannelSetInterface;
}
