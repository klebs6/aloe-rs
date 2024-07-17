crate::ix!();

/**
  | TODO: Add Bela MidiOutput support
  |
  */
pub struct MidiOutputPimpl {
    #[cfg(target_os="android")]
    java_midi_device: GlobalRef,
}

#[cfg(target_os="android")]
impl Drop for MidiOutputPimpl {

    fn drop(&mut self) {
        todo!();
        /* 
            if (jobject d = javaMidiDevice.get())
            {
                getEnv()->CallVoidMethod (d, AloeMidiPort.close);
                javaMidiDevice.clear();
            }
         */
    }
}

impl MidiOutputPimpl {

    #[cfg(target_os="android")]
    pub fn new(midi_device: &LocalRef<jobject>) -> Self {
    
        todo!();
        /*
        : java_midi_device(midiDevice),

        
        */
    }
    
    #[cfg(target_os="android")]
    pub fn send(&mut self, 
        byte_array: ByteArray,
        offset:     i32,
        len:        i32)  {
        
        todo!();
        /*
            if (jobject d = javaMidiDevice.get())
                getEnv()->CallVoidMethod (d,
                                          AloeMidiPort.sendMidi,
                                          byteArray, offset, len);
        */
    }
    
    #[cfg(target_os="android")]
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            if (jobject d = javaMidiDevice.get())
                return aloeString (LocalRef<jstring> ((jstring) getEnv()->CallObjectMethod (d, AloeMidiPort.getName)));

            return {};
        */
    }
}
