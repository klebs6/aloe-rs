crate::ix!();

pub trait GetTotalNumInputChannels {

    /**
      | Returns the total number of input channels.
      | 
      | This method will return the total number
      | of input channels by accumulating the
      | number of channels on each input bus.
      | The number of channels of the buffer
      | passed to your processBlock callback
      | will be equivalent to either getTotalNumInputChannels
      | or getTotalNumOutputChannels - which
      | ever is greater.
      | 
      | -----------
      | @note
      | 
      | getTotalNumInputChannels is equivalent
      | to getMainBusNumInputChannels if
      | your processor does not have any sidechains
      | or aux buses.
      |
      */
    fn get_total_num_input_channels(&self) -> i32;
}

pub trait GetTotalNumOutputChannels {

    /**
      | Returns the total number of output channels.
      | 
      | This method will return the total number
      | of output channels by accumulating
      | the number of channels on each output
      | bus. The number of channels of the buffer
      | passed to your processBlock callback
      | will be equivalent to either getTotalNumInputChannels
      | or getTotalNumOutputChannels - which
      | ever is greater.
      | 
      | -----------
      | @note
      | 
      | getTotalNumOutputChannels is equivalent
      | to getMainBusNumOutputChannels if
      | your processor does not have any sidechains
      | or aux buses.
      |
      */
    fn get_total_num_output_channels(&self) -> i32;
}

pub trait GetMainBusNumInputChannels {

    /**
      | Returns the number of input channels
      | on the main bus.
      |
      */
    fn get_main_bus_num_input_channels(&self) -> i32;
}

pub trait GetMainBusNumOutputChannels {

    /**
      | Returns the number of output channels
      | on the main bus.
      |
      */
    fn get_main_bus_num_output_channels(&self) -> i32;
}
