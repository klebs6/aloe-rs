crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/native/aloe_linux_BluetoothMidiDevicePairingDialogue.cpp]

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl BluetoothMidiDevicePairingDialogue {

    pub fn open(
        &mut self, 
        exit_callback: *mut dyn ModalComponentManagerCallback,
        _1:            *mut Rectangle<i32>

    ) -> bool {
        
        todo!();
        /*
            std::unique_ptr<ModalComponentManager::Callback> cb (exitCallback);
        // not implemented on Linux yet!
        // You should check whether the dialogue is available on your system
        // using isAvailable() before calling open().
        jassertfalse;
        return false;
        */
    }
    
    pub fn is_available(&mut self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
