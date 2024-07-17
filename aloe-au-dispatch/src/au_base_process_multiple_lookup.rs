crate::ix!();

pub struct AUBaseProcessMultipleLookup { }

impl AUBaseProcessMultipleLookup {

    pub fn lookup(&mut self, selector: i16) -> AudioComponentMethod {
        
        todo!();
        /*
            AudioComponentMethod method = AUBaseLookup::Lookup(selector);
        if (method) return method;

        if (selector == kAudioUnitProcessMultipleSelect)
            return (AudioComponentMethod)AUMethodProcessMultiple;

        return NULL;
        */
    }
}
