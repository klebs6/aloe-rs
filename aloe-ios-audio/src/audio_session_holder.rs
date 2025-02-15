crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_ios_Audio.h]

pub struct AudioSessionHolder<'a> {
    active_devices:      Vec<*mut iOSAudioIODeviceImpl<'a>>,
    active_device_types: Vec<*mut iOSAudioIODeviceType<'a>>,
    native_session:      objc_id::Id<NSObject>,
}

impl<'a> Drop for AudioSessionHolder<'a> {

    fn drop(&mut self) {
        todo!();
        /*      [nativeSession release];  */
    }
}

impl<'a> Default for AudioSessionHolder<'a> {

    fn default() -> Self {
    
        todo!();
        /*


            nativeSession = [[iOSAudioSessionNative alloc] init: this];
        */
    }
}

impl<'a> AudioSessionHolder<'a> {
    
    pub fn handle_status_change(&self, 
        enabled: bool,
        reason:  *const u8)  {
        
        todo!();
        /*
            for (auto device: activeDevices)
            device->handleStatusChange (enabled, reason);
        */
    }
    
    pub fn handle_route_change(&mut self, reason: AVAudioSessionRouteChangeReason)  {
        
        todo!();
        /*
            for (auto device: activeDevices)
            device->handleRouteChange (reason);

        for (auto deviceType: activeDeviceTypes)
            deviceType->handleRouteChange (reason);
        */
    }
}

