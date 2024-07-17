crate::ix!();

pub trait AddParameter {

    /**
      | Adds a parameter to the AudioProcessor.
      | 
      | The parameter object will be managed
      | and deleted automatically by the
      | 
      | AudioProcessor when no longer needed.
      |
      */
    fn add_parameter(&mut self, _0: *mut dyn AudioProcessorParameterInterface);
}

pub trait AddParameterGroup {

    /**
      | Adds a group of parameters to the AudioProcessor.
      | 
      | All the parameter objects contained
      | within the group will be managed and
      | deleted automatically by the AudioProcessor
      | when no longer needed.
      | 
      | @see addParameter
      |
      */
    fn add_parameter_group(&mut self, _0: Box<dyn AudioProcessorParameterGroupInterface>);
}
