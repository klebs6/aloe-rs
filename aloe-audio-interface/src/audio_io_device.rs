crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/audio_io/aloe_AudioIODevice.h]

/**
  | Base class for an audio device with synchronised
  | input and output channels.
  | 
  | Subclasses of this are used to implement
  | different protocols such as DirectSound,
  | ASIO, CoreAudio, etc.
  | 
  | To create one of these, you'll need to
  | use the AudioIODeviceType class - see
  | the documentation for that class for
  | more info.
  | 
  | For an easier way of managing audio devices
  | and their settings, have a look at the
  | AudioDeviceManager class.
  | 
  | @see AudioIODeviceType, AudioDeviceManager
  | 
  | @tags{Audio}
  |
  */
pub struct AudioIODevice {
    name:      String,
    type_name: String,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/audio_io/aloe_AudioIODevice.cpp]
impl AudioIODevice {
    
    /**
      | Creates a device, setting its name and
      | type member variables.
      |
      */
    pub fn new(
        device_name:      &String,
        device_type_name: &String) -> Self {
    
        todo!();
        /*
            : name (deviceName), typeName (deviceTypeName)
        */
    }

    /**
      | Returns the device's name, (as set in
      | the constructor).
      |
      */
    pub fn get_name(&self) -> &String {
        
        todo!();
        /*
            return name;
        */
    }

    /**
      | Returns the type of the device.
      | 
      | E.g. "CoreAudio", "ASIO", etc. - this
      | comes from the AudioIODeviceType that
      | created it.
      |
      */
    pub fn get_type_name(&self) -> &String {
        
        todo!();
        /*
            return typeName;
        */
    }
}
