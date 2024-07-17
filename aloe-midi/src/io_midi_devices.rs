crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/midi_io/aloe_MidiDevices.h]

/**
  | This struct contains information about
  | a MIDI input or output device.
  | 
  | You can get one of these structs by calling
  | the static getAvailableDevices()
  | or getDefaultDevice() methods of MidiInput
  | and MidiOutput or by calling getDeviceInfo()
  | on an instance of these classes. Devices
  | can be opened by passing the identifier
  | to the openDevice() method.
  | 
  | @tags{Audio}
  |
  */
#[derive(Default)]
pub struct MidiDeviceInfo {

    /**
      | The name of this device.
      | 
      | This will be provided by the OS unless
      | the device has been created with the
      | createNewDevice() method.
      | 
      | Note that the name is not guaranteed
      | to be unique and two devices with the
      | same name will be indistinguishable.
      | If you want to address a specific device
      | it is better to use the identifier.
      |
      */
    name: String,

    /**
      | The identifier for this device.
      | 
      | This will be provided by the OS and it's
      | format will differ on different systems
      | e.g. on macOS it will be a number whereas
      | on Windows it will be a long alphanumeric
      | string.
      |
      */
    identifier: String,
}

impl PartialEq<MidiDeviceInfo> for MidiDeviceInfo {
    
    #[inline] fn eq(&self, other: &MidiDeviceInfo) -> bool {
        todo!();
        /*
            return name == other.name && identifier == other.identifier;
        */
    }
}

impl Eq for MidiDeviceInfo {}

impl MidiDeviceInfo {

    pub fn new(
        device_name:       &String,
        device_identifier: &String) -> Self {
    
        todo!();
        /*
        : name(deviceName),
        : identifier(deviceIdentifier),

        
        */
    }
}
