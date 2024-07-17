crate::ix!();

pub type AudioComponentMethod = MissingType; //TODO

#[cfg(not(CA_BASIC_AU_FEATURES))]
pub struct AUAuxBaseLookup { }

#[cfg(not(CA_BASIC_AU_FEATURES))]
impl AUAuxBaseLookup {
    
    #[cfg(not(CA_BASIC_AU_FEATURES))]
    pub fn lookup(&mut self, selector: i16) -> AudioComponentMethod {
        
        todo!();
        /*
            switch (selector) {
            case kAudioUnitGetPropertyInfoSelect:   return (AudioComponentMethod)AUMethodGetPropertyInfo;
            case kAudioUnitGetPropertySelect:       return (AudioComponentMethod)AUMethodGetProperty;
            case kAudioUnitSetPropertySelect:       return (AudioComponentMethod)AUMethodSetProperty;

            case kAudioUnitGetParameterSelect:      return (AudioComponentMethod)AUMethodGetParameter;
            case kAudioUnitSetParameterSelect:      return (AudioComponentMethod)AUMethodSetParameter;

            default:
                break;
        }
        return NULL;
        */
    }
}
