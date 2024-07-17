crate::ix!();

/**
  | A base class for listeners that want
  | to know about changes to an
  | 
  | AudioProcessorParameter.
  | 
  | Use AudioProcessorParameter::addListener()
  | to register your listener with an AudioProcessorParameter.
  | 
  | This AudioProcessorParameterListener replaces most of the functionality
  | in the
  | 
  | AudioProcessorListener class, which
  | will be deprecated and removed.
  |
  */
pub trait AudioProcessorParameterListener {

    /**
      | Receives a callback when a parameter
      | has been changed.
      | 
      | IMPORTANT NOTE: This will be called
      | synchronously when a parameter changes,
      | and many audio processors will change
      | their parameter during their audio
      | callback.
      | 
      | This means that not only has your handler
      | code got to be completely thread-safe,
      | but it's also got to be VERY fast, and
      | avoid blocking. If you need to handle
      | this event on your message thread, use
      | this callback to trigger an AsyncUpdater
      | or ChangeBroadcaster which you can
      | respond to on the message thread.
      |
      */
    fn parameter_value_changed(&mut self, 
        parameter_index: i32,
        new_value:       f32);

    /**
      | Indicates that a parameter change gesture
      | has started.
      | 
      | E.g. if the user is dragging a slider,
      | this would be called with gestureIsStarting
      | being true when they first press the
      | mouse button, and it will be called again
      | with gestureIsStarting being false
      | when they release it.
      | 
      | IMPORTANT NOTE: This will be called
      | synchronously, and many audio processors
      | will call it during their audio callback.
      | This means that not only has your handler
      | code got to be completely thread-safe,
      | but it's also got to be VERY fast, and
      | avoid blocking. If you need to handle
      | this event on your message thread, use
      | this callback to trigger an AsyncUpdater
      | or ChangeBroadcaster which you can
      | respond to later on the message thread.
      |
      */
    fn parameter_gesture_changed(&mut self, 
        parameter_index:     i32,
        gesture_is_starting: bool);
}
