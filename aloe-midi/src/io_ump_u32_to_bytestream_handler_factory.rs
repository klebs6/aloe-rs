crate::ix!();

/**
  | Provides an `operator()` which can
  | create an input handler for a given MidiInput.
  | 
  | All handler classes should have a similar
  | UniversalMidiPacketsU32ToBytestreamHandlerFactory to facilitate creation of handlers
  | in generic contexts.
  |
  */
pub struct UniversalMidiPacketsU32ToBytestreamHandlerFactory {
    callback: *mut dyn MidiInputCallback, // default = nullptr
}

impl UniversalMidiPacketsU32ToBytestreamHandlerFactory {

    pub fn new(c: *mut dyn MidiInputCallback) -> Self {
    
        todo!();
        /*
        : callback(c),
        */
    }
    
    pub fn invoke(&self, i: &mut MidiInput) -> Box<UniversalMidiPacketsU32ToBytestreamHandler> {
        
        todo!();
        /*
            if (callback != nullptr)
                    return std::make_unique<UniversalMidiPacketsU32ToBytestreamHandler> (i, *callback);

                jassertfalse;
                return {};
        */
    }
}
