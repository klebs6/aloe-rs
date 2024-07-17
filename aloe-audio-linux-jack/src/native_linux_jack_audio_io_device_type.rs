crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_linux_JackAudio.cpp]

#[no_copy]
#[leak_detector]
pub struct JackAudioIODeviceType {
    base:         AudioIODeviceType,
    input_names:  Vec<String>,
    output_names: Vec<String>,
    has_scanned:  bool, // default = false
}

impl Default for JackAudioIODeviceType {
    
    fn default() -> Self {
        todo!();
        /*


            : AudioIODeviceType ("JACK"
        */
    }
}

impl JackAudioIODeviceType {

    pub fn scan_for_devices(&mut self)  {
        
        todo!();
        /*
            hasScanned = true;
            inputNames.clear();
            outputNames.clear();

            if (aloe_libjackHandle == nullptr)  aloe_libjackHandle = dlopen ("libjack.so.0", RTLD_LAZY);
            if (aloe_libjackHandle == nullptr)  aloe_libjackHandle = dlopen ("libjack.so",   RTLD_LAZY);
            if (aloe_libjackHandle == nullptr)  return;

            jack_status_t status = {};

            // open a dummy client
            if (auto* const client = jack_client_open ("AloeJackDummy", JackNoStartServer, &status))
            {
                // scan for output devices
                for (JackPortIterator i (client, false); i.next();)
                    if (i.getClientName() != (ALOE_JACK_CLIENT_NAME) && ! inputNames.contains (i.getClientName()))
                        inputNames.add (i.getClientName());

                // scan for input devices
                for (JackPortIterator i (client, true); i.next();)
                    if (i.getClientName() != (ALOE_JACK_CLIENT_NAME) && ! outputNames.contains (i.getClientName()))
                        outputNames.add (i.getClientName());

                jack_client_close (client);
            }
            else
            {
                ALOE_JACK_LOG_STATUS (status);
            }
        */
    }
    
    pub fn get_device_names(&self, want_input_names: bool) -> Vec<String> {
        
        todo!();
        /*
            jassert (hasScanned); // need to call scanForDevices() before doing this
            return wantInputNames ? inputNames : outputNames;
        */
    }
    
    pub fn get_default_device_index(&self, for_input: bool) -> i32 {
        
        todo!();
        /*
            jassert (hasScanned); // need to call scanForDevices() before doing this
            return 0;
        */
    }
    
    pub fn has_separate_inputs_and_outputs(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_index_of_device(&self, 
        device:   *mut AudioIODevice,
        as_input: bool) -> i32 {
        
        todo!();
        /*
            jassert (hasScanned); // need to call scanForDevices() before doing this

            if (JackAudioIODevice* d = dynamic_cast<JackAudioIODevice*> (device))
                return asInput ? inputNames.indexOf (d->inputName)
                               : outputNames.indexOf (d->outputName);

            return -1;
        */
    }
    
    pub fn create_device(&mut self, 
        output_device_name: &String,
        input_device_name:  &String) -> *mut AudioIODevice {
        
        todo!();
        /*
            jassert (hasScanned); // need to call scanForDevices() before doing this

            const int inputIndex = inputNames.indexOf (inputDeviceName);
            const int outputIndex = outputNames.indexOf (outputDeviceName);

            if (inputIndex >= 0 || outputIndex >= 0)
                return new JackAudioIODevice (inputDeviceName, outputDeviceName,
                                              [this] { callDeviceChangeListeners(); });

            return nullptr;
        */
    }
}
