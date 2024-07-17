crate::ix!();

// ------------------------------------------------------------------------------------------------

#[cfg(not(CA_BASIC_AU_FEATURES))]
#[inline] pub fn midi_lookup(selector: i16) -> AudioComponentMethod {
    
    todo!();
        /*
            switch (selector) {
            case kMusicDeviceMIDIEventSelect:   return (AudioComponentMethod)AUMethodMIDIEvent;
            case kMusicDeviceSysExSelect:       return (AudioComponentMethod)AUMethodSysEx;
            default:
                break;
        }
        return NULL;
        */
}
