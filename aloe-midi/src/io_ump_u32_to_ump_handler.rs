crate::ix!();

/**
  | Parses a continuous stream of U32 words
  | and emits full messages in the requested
  | UMP format.
  | 
  | @tags{Audio}
  |
  */
pub struct U32ToUMPHandler<'a> {
    recipient:  &'a mut dyn UniversalMidiPacketsReceiver,
    dispatcher: UniversalMidiPacketsDispatcher,
    converter:  UniversalMidiPacketsGenericUMPConverter,
}

impl<'a> UniversalMidiPacketsU32InputHandler for U32ToUMPHandler<'a> {

    fn reset(&mut self)  {
        
        todo!();
        /*
            dispatcher.reset();
            converter.reset();
        */
    }
    
    fn push_midi_data(
        &mut self, 
        begin: *const u32,
        end:   *const u32,
        time:  f64

    ) {
        
        todo!();
        /*
            dispatcher.dispatch (begin, end, time, [this] (const View& view, double thisTime)
            {
                converter.convert (view, [&] (const View& converted)
                {
                    recipient.packetReceived (converted, thisTime);
                });
            });
        */
    }
}

impl<'a> U32ToUMPHandler<'a> {

    pub fn new(
        protocol: UniversalMidiPacketsPacketProtocol,
        c:        &mut dyn UniversalMidiPacketsReceiver

    ) -> Self {
    
        todo!();
        /*
        : recipient(c),
        : converter(protocol),

        
        */
    }
}
