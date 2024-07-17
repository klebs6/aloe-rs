crate::ix!();

/**
  | Provides an `operator()` which can
  | create an input handler for a given MidiInput.
  | 
  | All handler classes should have a similar
  | UniversalMidiPacketsU32ToUMPHandlerFactory to facilitate creation of handlers
  | in generic contexts.
  |
  */
pub struct UniversalMidiPacketsU32ToUMPHandlerFactory<'a> {
    protocol: UniversalMidiPacketsPacketProtocol,
    callback: &'a mut dyn UniversalMidiPacketsReceiver,
}

impl<'a> UniversalMidiPacketsU32ToUMPHandlerFactory<'a> {

    pub fn new(
        p: UniversalMidiPacketsPacketProtocol,
        c: &mut dyn UniversalMidiPacketsReceiver

    ) -> Self {
    
        todo!();
        /*
        : protocol(p),
        : callback(c),

        
        */
    }
    
    pub fn invoke(&self, _0: &mut MidiInput) -> Box<U32ToUMPHandler> {
        
        todo!();
        /*
            return std::make_unique<U32ToUMPHandler> (protocol, callback);
        */
    }
}
