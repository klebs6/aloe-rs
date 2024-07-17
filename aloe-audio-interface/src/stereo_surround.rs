crate::ix!();

pub trait ChannelSetNull {

    /**
      | Creates a zero-channel set which can
      | be used to indicate that a bus is disabled.
      |
      */
    fn disabled(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Mono {

    /**
      | Creates a one-channel mono set (centre).
      | Is equivalent to: kMonoAAX (VST), AAX_eStemFormat_Mono
      | (AAX), kAudioChannelLayoutTag_Mono
      | (CoreAudio)
      |
      */
    fn mono(&mut self) -> dyn AudioChannelSetInterface;
}

pub trait Stereo {

    /**
      | Creates a set containing a stereo set
      | (left, right). Is equivalent to: kStereo
      | (VST), AAX_eStemFormat_Stereo (AAX),
      | kAudioChannelLayoutTag_Stereo (CoreAudio)
      |
      */
    fn stereo(&mut self) -> dyn AudioChannelSetInterface;
}
