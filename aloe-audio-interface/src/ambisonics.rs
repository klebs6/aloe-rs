crate::ix!();

pub trait GetAmbisonicOrderForNumChannels {

    fn get_ambisonic_order_for_num_channels(&mut self, num_channels: i32) -> i32;
}

pub trait GetAmbisonicOrder {

    /** 
      | Returns the order of the ambisonic layout
      | represented by this dyn AudioChannelSetInterface. If the
      | dyn AudioChannelSetInterface is not an ambisonic layout,
      | then this method will return -1.
      */
    fn get_ambisonic_order(&self) -> i32;
}

pub trait Ambisonic {

    /** 
      | Creates a set for ACN, SN3D normalised
      | ambisonic surround setups with a given order.
      |
      | Is equivalent to: kAmbiXXXOrderACN (VST),
      | AAX_eStemFormat_Ambi_XXX_ACN (AAX),
      | kAudioChannelLayoutTag_HOA_ACN_SN3D
      | (CoreAudio)
      */
    fn ambisonic(&mut self, order: Option<i32>) -> dyn AudioChannelSetInterface;
}
