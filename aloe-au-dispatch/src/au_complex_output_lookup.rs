crate::ix!();

pub struct AUComplexOutputLookup { }

impl AUComplexOutputLookup {
    
    pub fn lookup(selector: i16) -> AudioComponentMethod {
        
        todo!();
        /*
            AudioComponentMethod method = AUBaseLookup::Lookup(selector);
        if (method) return method;

        method = AUOutputLookup::Lookup(selector);
        if (method) return method;

        if (selector == kAudioUnitComplexRenderSelect)
            return (AudioComponentMethod)AUMethodComplexRender;
        return NULL;
        */
    }
}
