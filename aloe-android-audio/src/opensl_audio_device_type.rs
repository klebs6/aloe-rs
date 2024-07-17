crate::ix!();

///--------------------
#[no_copy]
#[leak_detector]
#[cfg(target_os="android")]
pub struct OpenSLAudioDeviceType {

    base: AudioIODeviceType,
}

#[cfg(target_os="android")]
impl Default for OpenSLAudioDeviceType {
    
    fn default() -> Self {
        todo!();
        /*
        : audio_io_device_type(OpenSLAudioIODevice::openSLTypeName),

        
        */
    }
}

#[cfg(target_os="android")]
impl OpenSLAudioDeviceType {

    pub fn scan_for_devices(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_device_names(&self, _0: bool) -> Vec<String> {
        
        todo!();
        /*
            return Vec<String> (OpenSLAudioIODevice::openSLTypeName);
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
            std::unique_ptr<OpenSLAudioIODevice> dev;

            if (outputDeviceName.isNotEmpty() || inputDeviceName.isNotEmpty())
                dev.reset (new OpenSLAudioIODevice (outputDeviceName.isNotEmpty() ? outputDeviceName
                                                                                  : inputDeviceName));

            return dev.release();
        */
    }
    
    pub fn is_open_sl_available() -> bool {
        
        todo!();
        /*
            DynamicLibrary library;
            return library.open ("libOpenSLES.so");
        */
    }
}
