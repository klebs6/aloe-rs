crate::ix!();

pub type AudioProcessorChangeDetails = AudioProcessorListenerChangeDetails;

pub trait AudioProcessorParameterChanged {

    /**
      | Receives a callback when a parameter
      | is changed.
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
    fn audio_processor_parameter_changed(
        &mut self, 
        processor:       *mut dyn AudioProcessorInterface,
        parameter_index: i32,
        new_value:       f32
    );
}

pub trait AudioProcessorChanged {

    /**
      | Called to indicate that something else
      | in the plugin has changed, like its program,
      | number of parameters, etc.
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
    fn audio_processor_changed(
        &mut self, 
        processor: *mut dyn AudioProcessorInterface,
        details:   &AudioProcessorListenerChangeDetails
    );

}
