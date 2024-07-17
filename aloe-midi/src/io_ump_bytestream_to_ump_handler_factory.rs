crate::ix!();

/**
  | Provides an `operator()` which can
  | create an input handler for a given MidiInput.
  | 
  | All handler classes should have a similar
  | UniversalMidiPacketsBytestreamToUMPHandlerFactory to facilitate creation of handlers
  | in generic contexts.
  |
  */
pub struct UniversalMidiPacketsBytestreamToUMPHandlerFactory<'a> {
    protocol: UniversalMidiPacketsPacketProtocol,
    callback: &'a mut dyn UniversalMidiPacketsReceiver,
}

impl<'a> UniversalMidiPacketsBytestreamToUMPHandlerFactory<'a> {

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
    
    pub fn invoke(&self, _0: &mut MidiInput) -> Box<BytestreamToUMPHandler> {
        
        todo!();
        /*
            return std::make_unique<BytestreamToUMPHandler> (protocol, callback);
        */
    }
}
