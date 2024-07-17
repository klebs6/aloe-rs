crate::ix!();

#[cfg(not(CA_BASIC_AU_FEATURES))]
pub struct AUMIDILookup { }

#[cfg(not(CA_BASIC_AU_FEATURES))]
impl AUMIDILookup {
    
    #[cfg(not(CA_BASIC_AU_FEATURES))]
    pub fn lookup(&mut self, selector: i16) -> AudioComponentMethod {
        
        todo!();
        /*
            AudioComponentMethod method = AUBaseLookup::Lookup(selector);
        if (method) return method;

        return MIDI_Lookup(selector);
        */
    }
}
