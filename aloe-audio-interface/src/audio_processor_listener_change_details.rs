crate::ix!();

/**
  | Provides details about aspects of an
  | AudioProcessor which have changed.
  |
  */
pub struct AudioProcessorListenerChangeDetails {
    latency_changed:        bool, // default = false
    parameter_info_changed: bool, // default = false
    program_changed:        bool, // default = false
}

impl AudioProcessorListenerChangeDetails {

    pub fn with_latency_changed(&self, b: bool) -> AudioProcessorListenerChangeDetails {
        
        todo!();
        /*
            return with (&AudioProcessorListenerChangeDetails::latencyChanged,       b);
        */
    }
    
    pub fn with_parameter_info_changed(&self, b: bool) -> AudioProcessorListenerChangeDetails {
        
        todo!();
        /*
            return with (&AudioProcessorListenerChangeDetails::parameterInfoChanged, b);
        */
    }
    
    pub fn with_program_changed(&self, b: bool) -> AudioProcessorListenerChangeDetails {
        
        todo!();
        /*
            return with (&AudioProcessorListenerChangeDetails::programChanged,       b);
        */
    }
    
    pub fn get_all_changed() -> AudioProcessorListenerChangeDetails {
        
        todo!();
        /*
            return AudioProcessorListenerChangeDetails{}.withLatencyChanged (true)
                                      .withParameterInfoChanged (true)
                                      .withProgramChanged (true);
        */
    }
    
    pub fn with<Member, Value>(&self, 
        member: Member,
        value:  Value) -> AudioProcessorListenerChangeDetails {
    
        todo!();
        /*
            auto copy = *this;
                copy.*member = std::forward<Value> (value);
                return copy;
        */
    }
}
