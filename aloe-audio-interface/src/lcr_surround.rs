crate::ix!();

pub trait CreateLCR {

    /** 
      | Creates a set containing an LCR set (left,
      | right, centre).
      |
      | Is equivalent to: k30Cine (VST),
      | AAX_eStemFormat_LCR (AAX),
      | kAudioChannelLayoutTag_MPEG_3_0_A
      | (CoreAudio)
      |
      | This format is referred to as "LRC" in
      | Cubase.
      |
      | This format is referred to as "LCR" in Pro
      | Tools.
      */
    fn createlcr(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait CreateLRS {

    /** 
      | Creates a set containing an LRS set (left,
      | right, surround).
      |
      | Is equivalent to: k30Music (VST), n/a
      | (AAX), kAudioChannelLayoutTag_ITU_2_1
      | (CoreAudio)
      |
      | This format is referred to as "LRS" in
      | Cubase.
      */
    fn createlrs(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait CreateLCRS {

    /** 
      | Creates a set containing an LCRS set (left,
      | right, centre, surround).
      |
      | Is equivalent to: k40Cine (VST),
      | AAX_eStemFormat_LCRS (AAX),
      | kAudioChannelLayoutTag_MPEG_4_0_A
      | (CoreAudio)
      |
      | This format is referred to as "LCRS (Pro
      | Logic)" in Logic Pro.
      |
      | This format is referred to as "LRCS" in
      | Cubase.
      |
      | This format is referred to as "LCRS" in
      | Pro Tools.
      */
    fn createlcrs(&mut self) -> dyn AudioChannelSetInterface;
}
