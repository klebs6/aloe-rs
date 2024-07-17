crate::ix!();

/**
  | An object of this class maintains a connection
  | between a ComboBox and a plug-in parameter.
  | 
  | ComboBox items will be spaced linearly
  | across the range of the parameter. For
  | example if the range is specified by
  | NormalisableRange<float> (-0.5f,
  | 0.5f, 0.5f) and you add three items then
  | the first will be mapped to a value of
  | -0.5, the second to 0, and the third to
  | 0.5.
  | 
  | During the lifetime of this object it
  | keeps the two things in sync, making
  | it easy to connect a combo box to a parameter.
  | When this object is deleted, the connection
  | is broken. Make sure that your parameter
  | and ComboBox are not deleted before
  | this object!
  | 
  | @tags{Audio}
  |
  */
pub struct ComboBoxParameterAttachment<'a> {
    combo_box:        &'a mut ComboBox<'a>,
    stored_parameter: &'a mut RangedAudioParameter,
    attachment:       ParameterAttachment<'a>,
    ignore_callbacks: bool, // default = false
}

impl<'a> ComboBoxListener for ComboBoxParameterAttachment<'a> {

    fn combo_box_changed(&mut self, _0: *mut ComboBox)  {
        
        todo!();
        /*
            if (ignoreCallbacks)
            return;

        const auto numItems = comboBox.getNumItems();
        const auto selected = (float) comboBox.getSelectedItemIndex();
        const auto newValue = numItems > 1 ? selected / (float) (numItems - 1)
                                           : 0.0f;

        attachment.setValueAsCompleteGesture (storedParameter.convertFrom0to1 (newValue));
        */
    }
}

impl<'a> Drop for ComboBoxParameterAttachment<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            comboBox.removeListener (this);
        */
    }
}

impl<'a> ComboBoxParameterAttachment<'a> {

    /**
      | Creates a connection between a plug-in
      | parameter and a ComboBox.
      | 
      | -----------
      | @param parameter
      | 
      | The parameter to use
      | ----------
      | @param combo
      | 
      | The ComboBox to use
      | ----------
      | @param undoManager
      | 
      | An optional UndoManager
      |
      */
    pub fn new(
        param: &mut RangedAudioParameter,
        c:     &mut ComboBox,
        um:    *mut UndoManager) -> Self {
    
        todo!();
        /*


            : comboBox (c),
          storedParameter (param),
          attachment (param, [this] (float f) { setValue (f); }, um)

        sendInitialUpdate();
        comboBox.addListener (this);
        */
    }
    
    /**
      | Call this after setting up your combo
      | box in the case where you need to do extra
      | setup after constructing this attachment.
      |
      */
    pub fn send_initial_update(&mut self)  {
        
        todo!();
        /*
            attachment.sendInitialUpdate();
        */
    }
    
    pub fn set_value(&mut self, new_value: f32)  {
        
        todo!();
        /*
            const auto normValue = storedParameter.convertTo0to1 (newValue);
        const auto index = roundToInt (normValue * (float) (comboBox.getNumItems() - 1));

        if (index == comboBox.getSelectedItemIndex())
            return;

        const ScopedValueSetter<bool> svs (ignoreCallbacks, true);
        comboBox.setSelectedItemIndex (index, sendNotificationSync);
        */
    }
    
}
