crate::ix!();

pub trait GetActiveOutputChannels {

    /**
      | Returns a mask showing which of the available
      | output channels are currently enabled.
      | @see getOutputChannelNames
      |
      */
    fn get_active_output_channels(&self) -> BigInteger;
}

pub trait GetActiveInputChannels {

    /**
      | Returns a mask showing which of the available
      | input channels are currently enabled.
      | @see getInputChannelNames
      |
      */
    fn get_active_input_channels(&self) -> BigInteger;
}
