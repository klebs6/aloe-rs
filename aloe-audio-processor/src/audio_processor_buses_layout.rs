crate::ix!();

/**
  | Represents the bus layout state of a
  | plug-in
  |
  */
pub struct AudioProcessorBusesLayout {

    /**
      | An array containing the list of input
      | buses that this processor supports.
      |
      */
    input_buses:  Vec<AudioChannelSet>,

    /**
      | An array containing the list of output
      | buses that this processor supports.
      |
      */
    output_buses: Vec<AudioChannelSet>,
}

impl PartialEq<AudioProcessorBusesLayout> for AudioProcessorBusesLayout {
    
    #[inline] fn eq(&self, other: &AudioProcessorBusesLayout) -> bool {
        todo!();
        /*
            return inputBuses == other.inputBuses && outputBuses == other.outputBuses;
        */
    }
}

impl Eq for AudioProcessorBusesLayout {}

impl AudioProcessorBusesLayout {

    /**
      | Get the number of channels of a particular
      | bus
      |
      */
    pub fn get_num_channels(&self, 
        is_input:  bool,
        bus_index: i32) -> i32 {
        
        todo!();
        /*
            auto& bus = (isInput ? inputBuses : outputBuses);
                return isPositiveAndBelow (busIndex, bus.size()) ? bus.getReference (busIndex).size() : 0;
        */
    }

    /**
      | Get the channel set of a particular bus
      |
      */
    pub fn get_channel_set_mut(
        &mut self, 
        is_input:  bool,
        bus_index: i32

    ) -> &mut AudioChannelSet {
        
        todo!();
        /*
            return (isInput ? inputBuses : outputBuses).getReference (busIndex);
        */
    }

    /**
      | Get the channel set of a particular bus
      |
      */
    pub fn get_channel_set(
        &self, 
        is_input:  bool,
        bus_index: i32

    ) -> AudioChannelSet {
        
        todo!();
        /*
            return (isInput ? inputBuses : outputBuses)[busIndex];
        */
    }

    /**
      | Get the input channel layout on the main
      | bus.
      |
      */
    pub fn get_main_input_channel_set(&self) -> AudioChannelSet {
        
        todo!();
        /*
            return getChannelSet (true,  0);
        */
    }

    /**
      | Get the output channel layout on the
      | main bus.
      |
      */
    pub fn get_main_output_channel_set(&self) -> AudioChannelSet {
        
        todo!();
        /*
            return getChannelSet (false, 0);
        */
    }

    /**
      | Get the number of input channels on the
      | main bus.
      |
      */
    pub fn get_main_input_channels(&self) -> i32 {
        
        todo!();
        /*
            return getNumChannels (true, 0);
        */
    }

    /**
      | Get the number of output channels on
      | the main bus.
      |
      */
    pub fn get_main_output_channels(&self) -> i32 {
        
        todo!();
        /*
            return getNumChannels (false, 0);
        */
    }
}
