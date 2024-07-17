crate::ix!();

pub struct AUBaseLookup { }

impl AUBaseLookup {

    pub fn lookup(&mut self, selector: i16) -> AudioComponentMethod {
        
        todo!();
        /*
            switch (selector) {
            case kAudioUnitInitializeSelect:        return (AudioComponentMethod)AUMethodInitialize;
            case kAudioUnitUninitializeSelect:      return (AudioComponentMethod)AUMethodUninitialize;
            case kAudioUnitGetPropertyInfoSelect:   return (AudioComponentMethod)AUMethodGetPropertyInfo;
            case kAudioUnitGetPropertySelect:       return (AudioComponentMethod)AUMethodGetProperty;
            case kAudioUnitSetPropertySelect:       return (AudioComponentMethod)AUMethodSetProperty;
            case kAudioUnitAddPropertyListenerSelect:return (AudioComponentMethod)AUMethodAddPropertyListener;
            case kAudioUnitRemovePropertyListenerSelect:
                                                    return (AudioComponentMethod)AUMethodRemovePropertyListener;
            case kAudioUnitRemovePropertyListenerWithUserDataSelect:
                                                    return (AudioComponentMethod)AUMethodRemovePropertyListenerWithUserData;
            case kAudioUnitAddRenderNotifySelect:   return (AudioComponentMethod)AUMethodAddRenderNotify;
            case kAudioUnitRemoveRenderNotifySelect:return (AudioComponentMethod)AUMethodRemoveRenderNotify;
            case kAudioUnitGetParameterSelect:      return (AudioComponentMethod)AUMethodGetParameter;
            case kAudioUnitSetParameterSelect:      return (AudioComponentMethod)AUMethodSetParameter;
            case kAudioUnitScheduleParametersSelect:return (AudioComponentMethod)AUMethodScheduleParameters;
            case kAudioUnitRenderSelect:            return (AudioComponentMethod)AUMethodRender;
            case kAudioUnitResetSelect:             return (AudioComponentMethod)AUMethodReset;
            default:
                break;
        }
        return NULL;
        */
    }
}
