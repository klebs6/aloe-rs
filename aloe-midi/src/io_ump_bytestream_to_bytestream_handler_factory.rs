crate::ix!();

/**
  | Provides an `operator()` which can
  | create an input handler for a given MidiInput.
  | 
  | All handler classes should have a similar
  | UniversalMidiPacketsBytestreamToBytestreamHandlerFactory to facilitate creation of handlers
  | in generic contexts.
  |
  */
pub struct UniversalMidiPacketsBytestreamToBytestreamHandlerFactory {
    callback: *mut dyn MidiInputCallback, // default = nullptr
}

impl UniversalMidiPacketsBytestreamToBytestreamHandlerFactory {

    pub fn new(c: *mut dyn MidiInputCallback) -> Self {
    
        todo!();
        /*
        : callback(c),

        
        */
    }
    
    pub fn invoke(&self, i: &mut MidiInput) -> Box<UniversalMidiPacketsBytestreamToBytestreamHandler> {
        
        todo!();
        /*
            if (callback != nullptr)
                    return std::make_unique<UniversalMidiPacketsBytestreamToBytestreamHandler> (i, *callback);

                jassertfalse;
                return {};
        */
    }
}
