crate::ix!();

/**
  | Parses a continuous stream of U32 words
  | and emits complete MidiMessages whenever
  | a full message is received.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsU32ToBytestreamHandler<'a> {
    input:      &'a mut MidiInput,
    callback:   &'a mut dyn MidiInputCallback,
    dispatcher: UniversalMidiPacketsToBytestreamDispatcher,
}

impl<'a> UniversalMidiPacketsU32InputHandler for UniversalMidiPacketsU32ToBytestreamHandler<'a> {

    fn reset(&mut self)  {
        
        todo!();
        /*
            dispatcher.reset();
        */
    }
    
    fn push_midi_data(&mut self, 
        begin: *const u32,
        end:   *const u32,
        time:  f64)  {
        
        todo!();
        /*
            dispatcher.dispatch (begin, end, time, [this] (const MidiMessage& m)
            {
                callback.handleIncomingMidiMessage (&input, m);
            });
        */
    }
}

impl<'a> UniversalMidiPacketsU32ToBytestreamHandler<'a> {

    pub fn new(
        i: &mut MidiInput,
        c: &mut dyn MidiInputCallback

    ) -> Self {
    
        todo!();
        /*
        : input(i),
        : callback(c),
        : dispatcher(2048),
        */
    }
}
