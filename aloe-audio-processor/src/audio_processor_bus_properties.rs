crate::ix!();

/**
  | Structure used for AudioProcessor
  | Callbacks
  |
  */
pub struct AudioProcessorBusesProperties {

    /**
      | The layouts of the input buses
      |
      */
    input_layouts:  Vec<AudioProcessorBusProperties>,

    /**
      | The layouts of the output buses
      |
      */
    output_layouts: Vec<AudioProcessorBusProperties>,
}

impl AudioProcessorBusesProperties {
    
    pub fn add_bus(
        &mut self, 
        is_input:                bool,
        name:                    &String,
        dflt_layout:             &AudioChannelSet,
        is_activated_by_default: Option<bool>

    )  {

        let is_activated_by_default: bool = is_activated_by_default.unwrap_or(true);
        
        todo!();
        /*
            jassert (dfltLayout.size() != 0);

        AudioProcessorBusProperties props;

        props.busName = name;
        props.defaultLayout = dfltLayout;
        props.isActivatedByDefault = isActivatedByDefault;

        (isInput ? inputLayouts : outputLayouts).add (props);
        */
    }
    
    pub fn with_input(
        &self, 
        name:                    &String,
        dflt_layout:             &AudioChannelSet,
        is_activated_by_default: Option<bool>

    ) -> AudioProcessorBusesProperties {

        let is_activated_by_default: bool = is_activated_by_default.unwrap_or(true);
        
        todo!();
        /*
            auto retval = *this;
        retval.addBus (true, name, dfltLayout, isActivatedByDefault);
        return retval;
        */
    }
    
    pub fn with_output(
        &self, 
        name:                    &String,
        dflt_layout:             &AudioChannelSet,
        is_activated_by_default: Option<bool>

    ) -> AudioProcessorBusesProperties {

        let is_activated_by_default: bool = is_activated_by_default.unwrap_or(true);
        
        todo!();
        /*
            auto retval = *this;
        retval.addBus (false, name, dfltLayout, isActivatedByDefault);
        return retval;
        */
    }
}
