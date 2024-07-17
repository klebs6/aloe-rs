crate::ix!();

pub trait GetProcessor {

    /**
      | The actual processor object that this
      | node represents.
      |
      */
    fn get_processor(&self) -> *mut dyn AudioProcessorInterface;
}

pub trait GetAudioProcessor {

    /**
      | Returns a pointer to the processor that
      | this editor represents.
      | 
      | This method is here to support legacy
      | code, but it's easier to just use the
      | 
      | AudioProcessorEditor::processor
      | member variable directly to get this
      | object.
      |
      */
    fn get_audio_processor(&self) -> *mut dyn AudioProcessorInterface {

        todo!();
        /*
            return &processor;
        */
    }
}
