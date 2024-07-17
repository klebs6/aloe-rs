crate::ix!();

#[cfg(not(CA_BASIC_AU_FEATURES))]
pub struct AUMusicLookup { }

#[cfg(not(CA_BASIC_AU_FEATURES))]
impl AUMusicLookup {

    #[cfg(not(CA_BASIC_AU_FEATURES))]
    pub fn lookup(&mut self, selector: i16) -> AudioComponentMethod {
        
        todo!();
        /*
            AudioComponentMethod method = AUBaseLookup::Lookup(selector);
        if (method) return method;

        switch (selector) {
            case kMusicDeviceStartNoteSelect:   return (AudioComponentMethod)AUMethodStartNote;
            case kMusicDeviceStopNoteSelect:    return (AudioComponentMethod)AUMethodStopNote;
    #if !TARGET_OS_IPHONE
            case kMusicDevicePrepareInstrumentSelect:   return (AudioComponentMethod)AUMethodPrepareInstrument;
            case kMusicDeviceReleaseInstrumentSelect:   return (AudioComponentMethod)AUMethodReleaseInstrument;
    #endif
            default:
                break;
        }
        return MIDI_Lookup (selector);
        */
    }
}
