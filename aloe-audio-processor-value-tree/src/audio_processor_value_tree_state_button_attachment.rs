crate::ix!();

/**
  | An object of this class maintains a connection
  | between a Button and a parameter in an
  | AudioProcessorValueTreeState.
  | 
  | During the lifetime of this AudioProcessorValueTreeStateButtonAttachment
  | object, it keeps the two things in sync,
  | making it easy to connect a button to
  | a parameter. When this object is deleted,
  | the connection is broken. Make sure
  | that your AudioProcessorValueTreeState
  | and Button aren't deleted before this
  | object!
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioProcessorValueTreeStateButtonAttachment<'a> {
    attachment: Box<ButtonParameterAttachment<'a>>,
}

impl<'a> AudioProcessorValueTreeStateButtonAttachment<'a> {

    pub fn new(
        state_to_use: &mut AudioProcessorValueTreeState,
        parameterid:  &String,
        button:       &mut Button) -> Self {
    
        todo!();
        /*


            : attachment (makeAttachment<ButtonParameterAttachment> (stateToUse, parameterID, button))
        */
    }
}
