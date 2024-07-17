crate::ix!();

#[no_copy]
#[leak_detector]
#[weak_referenceable]
pub struct iOSAudioIODeviceType<'a> {
    base:           AudioIODeviceType,
    base2:          AsyncUpdater<'a>,
    session_holder: SharedResourcePointer<AudioSessionHolder<'a>>,
}

impl<'a> Drop for iOSAudioIODeviceType<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        sessionHolder->activeDeviceTypes.removeFirstMatchingValue (this);
 */
    }
}

/**
  | The list of devices is updated automatically
  |
  */
impl<'a> iOSAudioIODeviceType<'a> {

    pub fn get_device_names(&self, _0: bool) -> Vec<String> {
        
        todo!();
        /*
            return { iOSAudioDeviceName };
        */
    }
    
    pub fn get_default_device_index(&self, _0: bool) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn get_index_of_device(&self, 
        _0: *mut AudioIODevice,
        _1: bool) -> i32 {
        
        todo!();
        /*
            return 0;
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
            return new iOSAudioIODevice (this, outputDeviceName, inputDeviceName);
        */
    }
    
    pub fn handle_route_change(&mut self, _0: AVAudioSessionRouteChangeReason)  {
        
        todo!();
        /*
            triggerAsyncUpdate();
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            callDeviceChangeListeners();
        */
    }
}

