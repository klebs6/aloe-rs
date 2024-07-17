crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_BluetoothMidiDevicePairingDialogue.h]

/**
  | Opens a Bluetooth MIDI pairing dialogue
  | that allows the user to view and connect
  | to Bluetooth MIDI devices that are currently
  | found nearby.
  | 
  | The dialogue will ignore non-MIDI Bluetooth
  | devices.
  | 
  | Only after a Bluetooth MIDI device has
  | been paired will its MIDI ports be available
  | through Aloe's MidiInput and MidiOutput
  | classes.
  | 
  | This dialogue is currently only available
  | on macOS targetting versions 10.11+,
  | iOS and Android. When targeting older
  | versions of macOS you should instead
  | pair Bluetooth MIDI devices using the
  | "Audio MIDI Setup" app (located in /Applications/Utilities).
  | On Windows, you should use the system
  | settings. On Linux, Bluetooth MIDI
  | devices are currently not supported.
  | 
  | @tags{Audio}
  |
  */
pub struct BluetoothMidiDevicePairingDialogue {

}

impl BluetoothMidiDevicePairingDialogue {

    #[cfg(target_os="android")]
    pub fn open(
        &mut self, 
        exit_callback_ptr: *mut dyn ModalComponentManagerCallback,
        bt_bounds:         *mut Rectangle<i32>

    ) -> bool {
        
        todo!();
        /*
            std::unique_ptr<ModalComponentManager::Callback> exitCallback (exitCallbackPtr);

        if (getAndroidSDKVersion() < 23)
            return false;

        auto boundsToUse = (btBounds != nullptr ? *btBounds : Rectangle<int> {});

        if (! RuntimePermissions::isGranted (RuntimePermissions::bluetoothMidi))
        {
            // If you hit this assert, you probably forgot to get RuntimePermissions::bluetoothMidi.
            // This is not going to work, boo! The pairing dialogue won't be able to scan for or
            // find any devices, it will just display an empty list, so don't bother opening it.
            jassertfalse;
            return false;
        }

        new BluetoothMidiSelectorOverlay (exitCallback.release(), boundsToUse);
        return true;
        */
    }
    
    #[cfg(target_os="android")]
    pub fn is_available(&mut self) -> bool {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23)
            return false;

        auto* env = getEnv();

        LocalRef<jobject> btManager (env->CallStaticObjectMethod (AndroidAloeMidiSupport, AndroidAloeMidiSupport.getAndroidBluetoothManager, getAppContext().get()));
        return btManager != nullptr;
        */
    }

    /**
      | Opens the Bluetooth MIDI pairing dialogue,
      | if it is available.
      | 
      | -----------
      | @param exitCallback
      | 
      | A callback which will be called when
      | the modal bluetooth dialog is closed.
      | ----------
      | @param btWindowBounds
      | 
      | The bounds of the bluetooth window that
      | will be opened. The dialog itself is
      | opened by the OS so cannot be customised
      | by Aloe.
      | 
      | -----------
      | @return
      | 
      | true if the dialogue was opened, false
      | on error.
      | 
      | @see ModalComponentManager::Callback
      |
      */
    pub fn open(
        exit_callback:    *mut dyn ModalComponentManagerCallback,
        bt_window_bounds: *mut Rectangle<i32>

    ) -> bool {

        todo!();
        /*
        
        */
    }

    /**
      | Checks if a Bluetooth MIDI pairing dialogue
      | is available on this platform.
      | 
      | On iOS, this will be true for iOS versions
      | 8.0 and higher.
      | 
      | On Android, this will be true only for
      | Android SDK versions 23 and higher,
      | and additionally only if the device
      | itself supports MIDI over Bluetooth.
      | 
      | On desktop platforms, this will typically
      | be false as the bluetooth pairing is
      | not done inside the app but by other means.
      | 
      | -----------
      | @return
      | 
      | true if the Bluetooth MIDI pairing dialogue
      | is available, false otherwise.
      |
      */
    pub fn is_available() -> bool {
        
        todo!();
        /*
        
        */
    }
}
