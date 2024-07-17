crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_android_Midi.cpp]

impl Drop for MidiOutput {

    fn drop(&mut self) {
        todo!();
        /* 
        stopBackgroundThread();
 */
    }
}

impl MidiOutput {

    pub fn get_available_devices(&mut self) -> Vec<MidiDeviceInfo> {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23)
            return {};

        AndroidMidiDeviceManager manager;
        return manager.getDevices (false);
        */
    }
    
    pub fn get_default_device(&mut self) -> MidiDeviceInfo {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23)
            return {};

        return getAvailableDevices().getFirst();
        */
    }
    
    pub fn open_device(&mut self, device_identifier: &String) -> Box<MidiOutput> {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23 || deviceIdentifier.isEmpty())
            return {};

        AndroidMidiDeviceManager manager;

        if (auto* port = manager.openMidiOutputPortWithID (deviceIdentifier.getIntValue()))
        {
            std::unique_ptr<MidiOutput> midiOutput (new MidiOutput ({}, deviceIdentifier));
            midiOutput->internal.reset (port);
            midiOutput->setName (port->getName());

            return midiOutput;
        }

        return {};
        */
    }
    
    pub fn get_devices(&mut self) -> Vec<String> {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23)
            return {};

        StringArray deviceNames;

        for (auto& d : getAvailableDevices())
            deviceNames.add (d.name);

        return deviceNames;
        */
    }
    
    pub fn get_default_device_index(&mut self) -> i32 {
        
        todo!();
        /*
            return (getAndroidSDKVersion() < 23 ? -1 : 0);
        */
    }
    
    pub fn open_device(&mut self, index: i32) -> Box<MidiOutput> {
        
        todo!();
        /*
            return openDevice (getAvailableDevices()[index].identifier);
        */
    }
    
    pub fn send_message_now(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            if (auto* androidMidi = internal.get())
        {
            auto* env = getEnv();
            auto messageSize = message.getRawDataSize();

            LocalRef<jbyteArray> messageContent (env->NewByteArray (messageSize));
            auto content = messageContent.get();

            auto* rawBytes = env->GetByteArrayElements (content, nullptr);
            std::memcpy (rawBytes, message.getRawData(), static_cast<size_t> (messageSize));
            env->ReleaseByteArrayElements (content, rawBytes, 0);

            androidMidi->send (content, (jint) 0, (jint) messageSize);
        }
        */
    }
}
