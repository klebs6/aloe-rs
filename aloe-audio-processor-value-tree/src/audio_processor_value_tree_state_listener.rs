crate::ix!();

/**
  | A listener class that can be attached
  | to an AudioProcessorValueTreeState.
  | 
  | Use AudioProcessorValueTreeState::addParameterListener()
  | to register a callback.
  |
  */
pub trait AudioProcessorValueTreeStateListener
{
    /**
      | This callback method is called by the
      | AudioProcessorValueTreeState when
      | a parameter changes.
      | 
      | Within this call, retrieving the value
      | of the parameter that has changed via
      | the getRawParameterValue() or getParameter()
      | methods is not guaranteed to return
      | the up-to-date value. If you need this
      | you should instead use the newValue
      | parameter.
      |
      */
    fn parameter_changed(&mut self, 
        parameterid: &String,
        new_value:   f32);
}

