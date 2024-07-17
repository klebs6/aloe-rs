crate::ix!();

pub trait GetNumberOfChannels {

    /**
      | Return the number of channels of the
      | current bus.
      |
      */
    fn get_number_of_channels(&self) -> i32;
}

pub trait SetNumberOfChannels {

    /**
      | Set the number of channels of this bus.
      | This will return false if the AudioProcessor
      | does not support this layout.
      |
      */
    fn set_number_of_channels(&mut self, channels: i32) -> bool;
}

pub trait IsNumberOfChannelsSupported {

    /**
      | Checks if this bus can support a given
      | number of channels.
      |
      */
    fn is_number_of_channels_supported(&self, channels: i32) -> bool;
}

pub trait GetMaxSupportedChannels {

    /**
      | Returns the maximum number of channels
      | that this bus can support.
      | 
      | -----------
      | @param limit
      | 
      | The maximum value to return.
      |
      */
    fn get_max_supported_channels(&self, limit: i32) -> i32;
}

pub trait UpdateChannelCount {

    fn update_channel_count(&mut self);
}

pub trait GetNumChannels {

    /**
      | Get the number of channels of a particular
      | bus
      |
      */
    fn get_num_channels(
        &self, 
        is_input:  bool,
        bus_index: i32
    ) -> i32;
}

pub trait GetChannelSet {

    /**
      | Get the channel set of a particular bus
      |
      */
    fn get_channel_set(
        &mut self, 
        is_input:  bool,
        bus_index: i32
    ) -> &mut dyn AudioChannelSetInterface;
}

pub trait GetMainInputChannelSet {

    /**
      | Get the input channel layout on the main
      | bus.
      |
      */
    fn get_main_input_channel_set(&self) -> dyn AudioChannelSetInterface;
}

pub trait GetMainOutputChannelSet {

    /**
      | Get the output channel layout on the
      | main bus.
      |
      */
    fn get_main_output_channel_set(&self) -> dyn AudioChannelSetInterface;
}

pub trait GetMainInputChannels {

    /**
      | Get the number of input channels on the
      | main bus.
      |
      */
    fn get_main_input_channels(&self) -> i32;
}

pub trait GetMainOutputChannels {

    /**
      | Get the number of output channels on
      | the main bus.
      |
      */
    fn get_main_output_channels(&self) -> i32;
}
