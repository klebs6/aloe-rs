crate::ix!();

pub trait GetChannelLayout {

    /**
      | Get the channel layout of the audio stream.
      |
      */
    fn get_channel_layout(&mut self) -> AudioChannelSet;
}

pub trait IsChannelLayoutSupported {

    fn is_channel_layout_supported(
        &mut self, 
        channel_set: &AudioChannelSet
    ) -> bool;
}

pub trait GetNumChannels {

    /**
      | Returns the number of channels being
      | written.
      |
      */
    fn get_num_channels(&self) -> i32;
}
