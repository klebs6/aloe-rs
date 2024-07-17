crate::ix!();

pub trait AddAudioProcessorParameterListener {

    /**
      | Registers a listener to receive events
      | when the parameter's state changes.
      | 
      | If the listener is already registered,
      | this will not register it again.
      | 
      | @see removeListener
      |
      */
    fn add_listener(&mut self, new_listener: *mut dyn AudioProcessorParameterListener);
}

pub trait RemoveAudioProcessorParameterListener {

    /**
      | Removes a previously registered parameter
      | listener
      | 
      | @see addListener
      |
      */
    fn remove_listener(&mut self, listener: *mut dyn AudioProcessorParameterListener);
}

pub trait SendValueChangedMessageToListeners {
    
    fn send_value_changed_message_to_listeners(&mut self, new_value: f32);
}
