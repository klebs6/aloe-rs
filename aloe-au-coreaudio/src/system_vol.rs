crate::ix!();

pub struct SystemVol {
    output_deviceid: AudioDeviceID,
    addr:            AudioObjectPropertyAddress,
}

impl SystemVol {

    pub fn new(selector: AudioObjectPropertySelector) -> Self {
    
        todo!();
        /*
        : output_deviceid(kAudioObjectUnknown),

            addr.mScope    = kAudioObjectPropertyScopeGlobal;
            addr.mElement  = aloeAudioObjectPropertyElementMain;
            addr.mSelector = kAudioHardwarePropertyDefaultOutputDevice;

            if (AudioObjectHasProperty (kAudioObjectSystemObject, &addr))
            {
                UInt32 deviceIDSize = sizeof (outputDeviceID);
                OSStatus status = AudioObjectGetPropertyData (kAudioObjectSystemObject, &addr, 0, nullptr, &deviceIDSize, &outputDeviceID);

                if (status == noErr)
                {
                    addr.mElement  = aloeAudioObjectPropertyElementMain;
                    addr.mSelector = selector;
                    addr.mScope    = kAudioDevicePropertyScopeOutput;

                    if (! AudioObjectHasProperty (outputDeviceID, &addr))
                        outputDeviceID = kAudioObjectUnknown;
                }
            }
        */
    }
    
    pub fn get_gain(&self) -> f32 {
        
        todo!();
        /*
            Float32 gain = 0;

            if (outputDeviceID != kAudioObjectUnknown)
            {
                UInt32 size = sizeof (gain);
                AudioObjectGetPropertyData (outputDeviceID, &addr,  0, nullptr, &size, &gain);
            }

            return (float) gain;
        */
    }
    
    pub fn set_gain(&self, gain: f32) -> bool {
        
        todo!();
        /*
            if (outputDeviceID != kAudioObjectUnknown && canSetVolume())
            {
                Float32 newVolume = gain;
                UInt32 size = sizeof (newVolume);

                return AudioObjectSetPropertyData (outputDeviceID, &addr, 0, nullptr, size, &newVolume) == noErr;
            }

            return false;
        */
    }
    
    pub fn is_muted(&self) -> bool {
        
        todo!();
        /*
            UInt32 muted = 0;

            if (outputDeviceID != kAudioObjectUnknown)
            {
                UInt32 size = sizeof (muted);
                AudioObjectGetPropertyData (outputDeviceID, &addr, 0, nullptr, &size, &muted);
            }

            return muted != 0;
        */
    }
    
    pub fn set_muted(&self, mute: bool) -> bool {
        
        todo!();
        /*
            if (outputDeviceID != kAudioObjectUnknown && canSetVolume())
            {
                UInt32 newMute = mute ? 1 : 0;
                UInt32 size = sizeof (newMute);

                return AudioObjectSetPropertyData (outputDeviceID, &addr, 0, nullptr, size, &newMute) == noErr;
            }

            return false;
        */
    }
    
    pub fn can_set_volume(&self) -> bool {
        
        todo!();
        /*
            Boolean isSettable = NO;
            return AudioObjectIsPropertySettable (outputDeviceID, &addr, &isSettable) == noErr && isSettable;
        */
    }
}
