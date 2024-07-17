crate::ix!();

/**
  | An object of this class maintains a connection
  | between a ComboBox and a parameter in
  | an AudioProcessorValueTreeState.
  | 
  | Combobox items will be spaced linearly
  | across the range of the parameter. For
  | example if the range is specified by
  | NormalisableRange<float> (-0.5f,
  | 0.5f, 0.5f) and you add three items then
  | the first will be mapped to a value of
  | -0.5, the second to 0, and the third to
  | 0.5.
  | 
  | During the lifetime of this AudioProcessorValueTreeStateComboBoxAttachment
  | object, it keeps the two things in sync,
  | making it easy to connect a combo box
  | to a parameter. When this object is deleted,
  | the connection is broken. Make sure
  | that your AudioProcessorValueTreeState
  | and ComboBox aren't deleted before
  | this object!
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioProcessorValueTreeStateComboBoxAttachment<'a> {
    attachment: Box<ComboBoxParameterAttachment<'a>>,
}

impl<'a> AudioProcessorValueTreeStateComboBoxAttachment<'a> {
    
    pub fn new(
        state_to_use: &mut AudioProcessorValueTreeState,
        parameterid:  &String,
        combo:        &mut ComboBox) -> Self {
    
        todo!();
        /*


            : attachment (makeAttachment<ComboBoxParameterAttachment> (stateToUse, parameterID, combo))
        */
    }
}
