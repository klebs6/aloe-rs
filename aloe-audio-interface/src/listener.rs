crate::ix!();

pub trait AddAudioProcessorListener {

    /**
      | Adds a listener that will be called when
      | an aspect of this processor changes.
      |
      */
    fn add_listener(&mut self, new_listener: *mut dyn AudioProcessorListener);
}

pub trait RemoveAudioProcessorListener {

    /**
      | Removes a previously added listener.
      |
      */
    fn remove_listener(&mut self, listener_to_remove: *mut dyn AudioProcessorListener);
}
