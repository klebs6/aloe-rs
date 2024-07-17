crate::ix!();

/**
  | Parses a continuous MIDI 1.0 bytestream,
  | and emits full messages in the requested
  | UMP format.
  | 
  | @tags{Audio}
  |
  */
pub struct BytestreamToUMPHandler<'a> {
    recipient:  &'a mut dyn UniversalMidiPacketsReceiver,
    dispatcher: UniversalMidiPacketsBytestreamToUMPDispatcher,
}

impl<'a> UniversalMidiPacketsBytestreamInputHandler for BytestreamToUMPHandler<'a> {

    fn reset(&mut self)  {
        
        todo!();
        /*
            dispatcher.reset();
        */
    }
    
    fn push_midi_data(&mut self, 
        data:  *const c_void,
        bytes: i32,
        time:  f64)  {
        
        todo!();
        /*
            const auto* ptr = static_cast<const uint8_t*> (data);
            dispatcher.dispatch (ptr, ptr + bytes, time, [&] (const View& v)
            {
                recipient.packetReceived (v, time);
            });
        */
    }
}

impl<'a> BytestreamToUMPHandler<'a> {

    pub fn new(
        protocol: UniversalMidiPacketsPacketProtocol,
        c:        &mut dyn UniversalMidiPacketsReceiver) -> Self {
    
        todo!();
        /*
        : recipient(c),
        : dispatcher(protocol, 2048),
        */
    }
}
