crate::ix!();

pub trait SetValueNotifyingHost {

    /**
      | A processor should call this when it
      | needs to change one of its parameters.
      | 
      | This could happen when the editor or
      | some other internal operation changes
      | a parameter. This method will call the
      | setValue() method to change the value,
      | and will then send a message to the host
      | telling it about the change.
      | 
      | -----------
      | @note
      | 
      | to make sure the host correctly handles
      | automation, you should call the beginChangeGesture()
      | and endChangeGesture() methods to
      | tell the host when the user has started
      | and stopped changing the parameter.
      |
      */
    fn set_value_notifying_host(&mut self, new_value: f32) {}
}
