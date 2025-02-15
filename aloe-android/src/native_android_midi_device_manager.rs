crate::ix!();

#[cfg(target_os="android")]
pub struct AndroidMidiDeviceManager {
    device_manager: GlobalRef,
}

#[cfg(target_os="android")]
impl Default for AndroidMidiDeviceManager {
    
    fn default() -> Self {
        todo!();
        /*


            : deviceManager (LocalRef<jobject>(getEnv()->CallStaticObjectMethod (AloeMidiSupport,
                                                                                 AloeMidiSupport.getAndroidMidiDeviceManager,
                                                                                 getAppContext().get()))
        */
    }
}

#[cfg(target_os="android")]
impl AndroidMidiDeviceManager {

    pub fn get_devices(&mut self, input: bool) -> Vec<MidiDeviceInfo> {
        
        todo!();
        /*
            if (jobject dm = deviceManager.get())
            {
                jobjectArray jDeviceNameAndIDs
                    = (jobjectArray) getEnv()->CallObjectMethod (dm, input ? MidiDeviceManager.getAloeAndroidMidiInputDeviceNameAndIDs
                                                                           : MidiDeviceManager.getAloeAndroidMidiOutputDeviceNameAndIDs);

                // Create a local reference as converting this to a Aloe string will call into JNI
                LocalRef<jobjectArray> localDeviceNameAndIDs (jDeviceNameAndIDs);

                auto deviceNameAndIDs = javaStringArrayToAloe (localDeviceNameAndIDs);
                deviceNameAndIDs.appendNumbersToDuplicates (false, false, CharPointer_UTF8 ("-"), CharPointer_UTF8 (""));

                Vec<MidiDeviceInfo> devices;

                for (int i = 0; i < deviceNameAndIDs.size(); i += 2)
                    devices.add ({ deviceNameAndIDs[i], deviceNameAndIDs[i + 1] });

                return devices;
            }

            return {};
        */
    }
    
    pub fn open_midi_input_port_withid(
        &mut self, 
        deviceid:        i32,
        aloe_midi_input: *mut MidiInput,
        callback:        *mut dyn MidiInputCallback

    ) -> *mut MidiInputImpl {
        
        todo!();
        /*
            if (auto dm = deviceManager.get())
            {
                auto androidMidiInput = std::make_unique<MidiInput::Impl> (aloeMidiInput, deviceID, callback, dm);

                if (androidMidiInput->isOpen())
                    return androidMidiInput.release();
            }

            return nullptr;
        */
    }
    
    pub fn open_midi_output_port_withid(&mut self, deviceid: i32) -> *mut MidiOutputImpl {
        
        todo!();
        /*
            if (auto dm = deviceManager.get())
                if (auto javaMidiPort = getEnv()->CallObjectMethod (dm, MidiDeviceManager.openMidiOutputPortWithID, (jint) deviceID))
                    return new MidiOutput::Impl (LocalRef<jobject>(javaMidiPort));

            return nullptr;
        */
    }
}
