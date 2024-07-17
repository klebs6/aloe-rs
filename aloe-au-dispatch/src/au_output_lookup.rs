crate::ix!();

pub struct AUOutputLookup { }

impl AUOutputLookup {

    pub fn lookup(&mut self, selector: i16) -> AudioComponentMethod {
        
        todo!();
        /*
            AudioComponentMethod method = AUBaseLookup::Lookup(selector);
        if (method) return method;

        switch (selector) {
            case kAudioOutputUnitStartSelect:   return (AudioComponentMethod)AUMethodStart;
            case kAudioOutputUnitStopSelect:    return (AudioComponentMethod)AUMethodStop;
            default:
                break;
        }
        return NULL;
        */
    }
}
