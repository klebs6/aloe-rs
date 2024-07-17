crate::ix!();

pub trait SetAudioProcessingEnabled {

    /**
      | On devices which support it, this allows
      | automatic gain control or other mic
      | processing to be disabled.
      | 
      | If the device doesn't support this operation,
      | it'll return false.
      |
      */
    fn set_audio_preprocessing_enabled(&mut self, should_be_enabled: bool) -> bool {
        false
    }
}
