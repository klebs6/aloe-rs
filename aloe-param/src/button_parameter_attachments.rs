crate::ix!();

/**
  | An object of this class maintains a connection
  | between a Button and a plug-in parameter.
  | 
  | During the lifetime of this object it
  | keeps the two things in sync, making
  | it easy to connect a button to a parameter.
  | When this object is deleted, the connection
  | is broken. Make sure that your parameter
  | and Button are not deleted before this
  | object!
  | 
  | @tags{Audio}
  |
  */
pub struct ButtonParameterAttachment<'a> {
    button:           &'a mut Button<'a>,
    attachment:       ParameterAttachment<'a>,
    ignore_callbacks: bool, // default = false
}

impl<'a> ButtonListener for ButtonParameterAttachment<'a> {

    fn button_clicked(&mut self, _0: *mut Button)  {
        
        todo!();
        /*
            if (ignoreCallbacks)
            return;

        attachment.setValueAsCompleteGesture (button.getToggleState() ? 1.0f : 0.0f);
        */
    }
}

impl<'a> Drop for ButtonParameterAttachment<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            button.removeListener (this);
        */
    }
}

impl<'a> ButtonParameterAttachment<'a> {
    
    /**
      | Creates a connection between a plug-in
      | parameter and a Button.
      | 
      | -----------
      | @param parameter
      | 
      | The parameter to use
      | ----------
      | @param button
      | 
      | The Button to use
      | ----------
      | @param undoManager
      | 
      | An optional UndoManager
      |
      */
    pub fn new(
        param: &mut RangedAudioParameter,
        b:     &mut Button,
        um:    *mut UndoManager) -> Self {
    
        todo!();
        /*


            : button (b),
          attachment (param, [this] (float f) { setValue (f); }, um)

        sendInitialUpdate();
        button.addListener (this);
        */
    }
    
    /**
      | Call this after setting up your button
      | in the case where you need to do extra
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
            const ScopedValueSetter<bool> svs (ignoreCallbacks, true);
        button.setToggleState (newValue >= 0.5f, sendNotificationSync);
        */
    }
    
}
