crate::ix!();

/**
  | A class for receiving events when audio
  | devices are inserted or removed.
  | 
  | You can register an AudioIODeviceType::AudioIODeviceTypeListener
  | with an~AudioIODeviceType object
  | using the AudioIODeviceType::addListener()
  | method, and it will be called when devices
  | of that type are added or removed.
  | 
  | @see AudioIODeviceType::addListener,
  | AudioIODeviceType::removeListener
  |
  */
pub trait AudioIODeviceTypeListener {

    /**
      | Called when the list of available audio
      | devices changes.
      |
      */
    fn audio_device_list_changed(&mut self);
}
