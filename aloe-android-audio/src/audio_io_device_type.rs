crate::ix!();

#[cfg(target_os="android")]
#[no_copy]
#[leak_detector]
pub struct AndroidAudioIODeviceType {
    base: AudioIODeviceType,
}

#[cfg(target_os="android")]
impl Default for AndroidAudioIODeviceType {
    
    fn default() -> Self {
        todo!();
        /*
        : audio_io_device_type(javaAudioTypeName),

        
        */
    }
}

#[cfg(target_os="android")]
impl AndroidAudioIODeviceType {

    pub fn scan_for_devices(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_device_names(&self, _0: bool) -> Vec<String> {
        
        todo!();
        /*
            return Vec<String> (javaAudioTypeName);
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
        output_device_name: &String,
        input_device_name:  &String) -> *mut AudioIODevice {
        
        todo!();
        /*
            std::unique_ptr<AndroidAudioIODevice> dev;

            if (outputDeviceName.isNotEmpty() || inputDeviceName.isNotEmpty())
            {
                dev.reset (new AndroidAudioIODevice (outputDeviceName.isNotEmpty() ? outputDeviceName
                                                                                   : inputDeviceName));

                if (dev->getCurrentSampleRate() <= 0 || dev->getDefaultBufferSize() <= 0)
                    dev = nullptr;
            }

            return dev.release();
        */
    }
}
