crate::ix!();

pub struct AUBaseProcessLookup { }

impl AUBaseProcessLookup {

    pub fn lookup(&mut self, selector: i16) -> AudioComponentMethod {
        
        todo!();
        /*
            AudioComponentMethod method = AUBaseLookup::Lookup(selector);
        if (method) return method;

        if (selector == kAudioUnitProcessSelect)
            return (AudioComponentMethod)AUMethodProcess;

        return NULL;
        */
    }
}
