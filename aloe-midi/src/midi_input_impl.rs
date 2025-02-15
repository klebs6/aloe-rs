crate::ix!();

#[cfg(target_os="android")]
pub struct MidiInputImpl {

    aloe_midi_input:   *mut MidiInput,
    callback:          *mut dyn MidiInputCallback,
    midi_concatenator: MidiDataConcatenator,
    java_midi_device:  GlobalRef,
}

#[cfg(target_os="android")]
impl Drop for MidiInputImpl {

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

impl MidiInputImpl {

    #[cfg(target_os="android")]
    pub fn new(
        midi_input:          *mut MidiInput,
        deviceid:            i32,
        midi_input_callback: *mut MidiInputCallback,
        device_manager:      jobject) -> Self {
    
        todo!();
        /*


            : aloeMidiInput (midiInput), callback (midiInputCallback), midiConcatenator (2048),
              javaMidiDevice (LocalRef<jobject>(getEnv()->CallObjectMethod (deviceManager, MidiDeviceManager.openMidiInputPortWithID,
                                                                            (jint) deviceID, (jlong) this)))
        */
    }
    
    #[cfg(target_os="android")]
    pub fn is_open(&self) -> bool {
        
        todo!();
        /*
            return javaMidiDevice != nullptr;
        */
    }
    
    #[cfg(target_os="android")]
    pub fn start(&mut self)  {
        
        todo!();
        /*
            if (jobject d = javaMidiDevice.get())
                getEnv()->CallVoidMethod (d, AloeMidiPort.start);
        */
    }
    
    #[cfg(target_os="android")]
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            if (jobject d = javaMidiDevice.get())
                getEnv()->CallVoidMethod (d, AloeMidiPort.stop);

            callback = nullptr;
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
    
    #[cfg(target_os="android")]
    pub fn handle_midi(&mut self, 
        byte_array: ByteArray,
        offset:     i64,
        len:        i32,
        timestamp:  i64)  {
        
        todo!();
        /*
            auto* env = getEnv();

            jassert (byteArray != nullptr);
            auto* data = env->GetByteArrayElements (byteArray, nullptr);

            HeapBlock<uint8> buffer (static_cast<size_t> (len));
            std::memcpy (buffer.get(), data + offset, static_cast<size_t> (len));

            midiConcatenator.pushMidiData (buffer.get(),
                                           len, static_cast<double> (timestamp) * 1.0e-9,
                                           aloeMidiInput, *callback);

            env->ReleaseByteArrayElements (byteArray, data, 0);
        */
    }
    
    #[cfg(target_os="android")]
    pub fn handle_receive(
        _0:         *mut JNIEnv,
        _1:         jobject,
        host:       i64,
        byte_array: ByteArray,
        offset:     i32,
        len:        i32,
        timestamp:  i64)  {
        
        todo!();
        /*
            auto* myself = reinterpret_cast<Impl*> (host);

            myself->handleMidi (byteArray, offset, len, timestamp);
        */
    }
}
