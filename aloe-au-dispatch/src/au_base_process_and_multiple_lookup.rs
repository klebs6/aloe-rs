crate::ix!();

pub struct AUBaseProcessAndMultipleLookup { }

impl AUBaseProcessAndMultipleLookup {
    
    pub fn lookup(&mut self, selector: i16) -> AudioComponentMethod {
        
        todo!();
        /*
            AudioComponentMethod method = AUBaseLookup::Lookup(selector);
        if (method) return method;

        method = AUBaseProcessMultipleLookup::Lookup(selector);
        if (method) return method;

        method = AUBaseProcessLookup::Lookup(selector);
        if (method) return method;

        return NULL;
        */
    }
}
