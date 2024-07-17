crate::ix!();

pub enum RuntimePermissionsPermissionID
{
    /**
      | Permission to access the microphone
      | (required on Android).
      | 
      | You need to request this, for example,
      | to initialise an AudioDeviceManager
      | with a non-zero number of input channels,
      | and to open the default audio input device.
      |
      */
    recordAudio = 1,

    /**
      | Permission to scan for and pair to Bluetooth
      | MIDI devices (required on Android).
      | 
      | You need to request this before calling
      | BluetoothMidiDevicePairingDialogue::open(),
      | otherwise no devices will be found.
      |
      */
    bluetoothMidi = 2,

    /**
      | Permission to read from external storage
      | such as SD cards
      |
      */
    readExternalStorage = 3,

    /**
      | Permission to write to external storage
      | such as SD cards
      |
      */
    writeExternalStorage = 4,

    /**
      | Permission to use camera
      |
      */
    camera = 5
}
