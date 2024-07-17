crate::ix!();

/**
  | An object of this class maintains a connection
  | between a Slider and a parameter in an
  | AudioProcessorValueTreeState.
  | 
  | During the lifetime of this AudioProcessorValueTreeStateSliderAttachment
  | object, it keeps the two things in sync,
  | making it easy to connect a slider to
  | a parameter. When this object is deleted,
  | the connection is broken. Make sure
  | that your AudioProcessorValueTreeState
  | and Slider aren't deleted before this
  | object!
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioProcessorValueTreeStateSliderAttachment<'a> {
    attachment: Box<SliderParameterAttachment<'a>>,
}

impl<'a> AudioProcessorValueTreeStateSliderAttachment<'a> {
    
    pub fn new(
        state_to_use: &mut AudioProcessorValueTreeState,
        parameterid:  &String,
        slider:       &mut Slider) -> Self {
    
        todo!();
        /*


            : attachment (makeAttachment<SliderParameterAttachment> (stateToUse, parameterID, slider))
        */
    }
}
