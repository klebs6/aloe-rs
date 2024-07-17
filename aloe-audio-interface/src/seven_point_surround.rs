crate::ix!();

pub trait Create7Point0Point2 {

    /** 
      | Creates a set for Dolby Atmos 7.0.2 surround
      | setup (left, right, centre, leftSurroundSide,
      | rightSurroundSide, leftSurroundRear,
      | rightSurroundRear, topSideLeft,
      | topSideRight).
      |
      | Is equivalent to: n/a (VST),
      | AAX_eStemFormat_7_0_2 (AAX), n/a (CoreAudio)
      */
    fn create7point0point2(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Create7Point1Point2 {

    /** 
      | Creates a set for Dolby Atmos 7.1.2 surround
      | setup (left, right, centre, leftSurroundSide,
      | rightSurroundSide, leftSurroundRear,
      | rightSurroundRear, LFE, topSideLeft,
      | topSideRight).
      |
      | Is equivalent to: k71_2 (VST),
      | AAX_eStemFormat_7_1_2 (AAX), n/a
      | (CoreAudio)
      */
    fn create7point1point2(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Create7Point0Point4 {

    /** 
      | Creates a set for Dolby Atmos 7.0.4 surround
      | setup (left, right, centre, leftSurroundSide,
      | rightSurroundSide, leftSurroundRear,
      | rightSurroundRear, topFrontLeft,
      | topFrontRight, topRearLeft, topRearRight).
      |
      | Is equivalent to: n/a (VST), n/a (AAX),
      | n/a (CoreAudio)
      */
    fn create7point0point4(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Create7Point1Point4 {

    /** 
      | Creates a set for Dolby Atmos 7.1.4 surround
      | setup (left, right, centre, leftSurroundSide,
      | rightSurroundSide, leftSurroundRear,
      | rightSurroundRear, LFE, topFrontLeft,
      | topFrontRight, topRearLeft, topRearRight).
      |
      | Is equivalent to: k71_4 (VST), n/a (AAX),
      | n/a (CoreAudio)
      */
    fn create7point1point4(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Create7Point0 {

    /** 
      | Creates a set for a DTS 7.0 surround setup
      | (left, right, centre, leftSurroundSide,
      | rightSurroundSide, leftSurroundRear,
      | rightSurroundRear).
      |
      | Is equivalent to: k70Music (VST),
      | AAX_eStemFormat_7_0_DTS (AAX),
      | kAudioChannelLayoutTag_AudioUnit_7_0
      | (CoreAudio)
      |
      | This format is referred to as "7.0" in Pro
      | Tools.
      */
    fn create7point0(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Create7Point0Sdds {

    /** 
      | Creates a set for a SDDS 7.0 surround setup
      | (left, right, centre, leftSurround,
      | rightSurround, leftCentre, rightCentre).
      |
      | Is equivalent to: k70Cine (VST),
      | AAX_eStemFormat_7_0_SDDS (AAX),
      | kAudioChannelLayoutTag_AudioUnit_7_0_Front
      | (CoreAudio)
      |
      | This format is referred to as "7.0 SDDS"
      | in Pro Tools.
      */
    fn create7point0sdds(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Create7Point1 {

    /** 
      | Creates a set for a DTS 7.1 surround setup
      | (left, right, centre, leftSurroundSide,
      | rightSurroundSide, leftSurroundRear,
      | rightSurroundRear, LFE).
      |
      | Is equivalent to: k71CineSideFill (VST),
      | AAX_eStemFormat_7_1_DTS (AAX),
      | kAudioChannelLayoutTag_MPEG_7_1_C/kAudioChannelLayoutTag_ITU_3_4_1
      | (CoreAudio)
      |
      | This format is referred to as "7.1 (3/4.1)"
      | in Logic Pro.
      |
      | This format is referred to as "7.1" in Pro
      | Tools.
      */
    fn create7point1(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Create7Point1Sdds {

    /** 
      | Creates a set for a 7.1 surround setup
      | (left, right, centre, leftSurround,
      | rightSurround, leftCentre, rightCentre,
      | LFE).
      |
      | Is equivalent to: k71Cine (VST),
      | AAX_eStemFormat_7_1_SDDS (AAX),
      | kAudioChannelLayoutTag_MPEG_7_1_A
      | (CoreAudio)
      |
      | This format is referred to as "7.1 (SDDS)"
      | in Logic Pro.
      |
      | This format is referred to as "7.1 SDDS" in
      | Pro Tools.
      */
    fn create7point1sdds(&mut self) -> dyn AudioChannelSetInterface;
}
