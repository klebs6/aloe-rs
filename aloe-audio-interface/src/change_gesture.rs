crate::ix!();

pub trait BeginChangeGesture {

    /**
      | Sends a signal to the host to tell it that
      | the user is about to start changing this
      | parameter.
      | 
      | This allows the host to know when a parameter
      | is actively being held by the user, and
      | it may use this information to help it
      | record automation.
      | 
      | If you call this, it must be matched by
      | a later call to endChangeGesture().
      |
      */
    fn begin_change_gesture(&mut self) {}
}

pub trait EndChangeGesture {

    /**
      | Tells the host that the user has finished
      | changing this parameter.
      | 
      | This allows the host to know when a parameter
      | is actively being held by the user, and
      | it may use this information to help it
      | record automation.
      | 
      | A call to this method must follow a call
      | to beginChangeGesture().
      |
      */
    fn end_change_gesture(&mut self) {}
}

pub trait AudioProcessorParameterChangeGestureBegin {

    /**
      | Indicates that a parameter change gesture
      | has started.
      | 
      | E.g. if the user is dragging a slider,
      | this would be called when they first
      | press the mouse button, and audioProcessorParameterChangeGestureEnd
      | would be called when they release it.
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
      | @see audioProcessorParameterChangeGestureEnd
      |
      */
    fn audio_processor_parameter_change_gesture_begin(
        &mut self, 
        processor:       *mut dyn AudioProcessorInterface,
        parameter_index: i32
    ) {}
}

pub trait AudioProcessorParameterChangeGestureEnd {

    /**
      | Indicates that a parameter change gesture
      | has finished.
      | 
      | E.g. if the user is dragging a slider,
      | this would be called when they release
      | the mouse button.
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
      | @see audioProcessorParameterChangeGestureBegin
      |
      */
    fn audio_processor_parameter_change_gesture_end(
        &mut self, 
        processor:       *mut dyn AudioProcessorInterface,
        parameter_index: i32
    ) {}
}
