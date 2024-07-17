crate::ix!();

pub trait Quadraphonic {

    /** 
      | Creates a set for quadraphonic surround setup
      | (left, right, leftSurround, rightSurround)
      |
      | Is equivalent to: k40Music (VST),
      | AAX_eStemFormat_Quad (AAX),
      | kAudioChannelLayoutTag_Quadraphonic
      | (CoreAudio)
      |
      | This format is referred to as
      | "Quadraphonic" in Logic Pro.
      |
      | This format is referred to as "Quadro" in
      | Cubase.
      |
      | This format is referred to as "Quad" in
      | Pro Tools.
      |
      */
    fn quadraphonic(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Pentagonal {

    /** 
      | Creates a set for pentagonal surround setup
      | (left, right, centre, leftSurroundRear,
      | rightSurroundRear).
      |
      | Is equivalent to: n/a (VST), n/a (AAX),
      | kAudioChannelLayoutTag_Pentagonal
      | (CoreAudio)
      */
    fn pentagonal(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Hexagonal {

    /** 
      | Creates a set for hexagonal surround setup
      | (left, right, leftSurroundRear,
      | rightSurroundRear, centre, surroundCentre).
      |
      | Is equivalent to: n/a (VST), n/a (AAX),
      | kAudioChannelLayoutTag_Hexagonal (CoreAudio)
      */
    fn hexagonal(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Octagonal {

    /** 
      | Creates a set for octagonal surround setup
      | (left, right, leftSurround, rightSurround,
      | centre, centreSurround, wideLeft, wideRight).
      |
      | Is equivalent to: n/a (VST), n/a (AAX),
      | kAudioChannelLayoutTag_Octagonal (CoreAudio)
      */
    fn octagonal(&mut self) -> dyn AudioChannelSetInterface;
}
