crate::ix!();

#[no_copy]
#[leak_detector]
pub struct BelaAudioIODeviceType {
    base: AudioIODeviceType,
}

impl Default for BelaAudioIODeviceType {
    
    fn default() -> Self {
        todo!();
        /*
        : audio_io_device_type("Bela"),

        
        */
    }
}

impl BelaAudioIODeviceType {

    pub fn get_device_names(&self, _0: bool) -> Vec<String> {
        
        todo!();
        /*
            return Vec<String> (BelaAudioIODevice::belaTypeName);
        */
    }
    
    pub fn scan_for_devices(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_default_device_index(&self, _0: bool) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn get_index_of_device(&self, 
        device: *mut AudioIODevice,
        _1:     bool) -> i32 {
        
        todo!();
        /*
            return device != nullptr ? 0 : -1;
        */
    }
    
    pub fn has_separate_inputs_and_outputs(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn create_device(&mut self, 
        output_name: &String,
        input_name:  &String) -> *mut AudioIODevice {
        
        todo!();
        /*
            // TODO: switching whether to support analog/digital with possible multiple Bela device types?
            if (outputName == BelaAudioIODevice::belaTypeName || inputName == BelaAudioIODevice::belaTypeName)
                return new BelaAudioIODevice();

            return nullptr;
        */
    }
}
