crate::ix!();

/**
  | Parses a continuous bytestream and
  | emits complete MidiMessages whenever
  | a full message is received.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsBytestreamToBytestreamHandler<'a> {
    input:        &'a mut MidiInput,
    callback:     &'a mut dyn MidiInputCallback,
    concatenator: MidiDataConcatenator,
}

impl<'a> UniversalMidiPacketsBytestreamInputHandler 
for UniversalMidiPacketsBytestreamToBytestreamHandler<'a> {

    fn reset(&mut self)  {
        
        todo!();
        /*
            concatenator.reset();
        */
    }
    
    fn push_midi_data(&mut self, 
        data:  *const c_void,
        bytes: i32,
        time:  f64)  {
        
        todo!();
        /*
            concatenator.pushMidiData (data, bytes, time, &input, callback);
        */
    }
}

impl<'a> UniversalMidiPacketsBytestreamToBytestreamHandler<'a> {

    pub fn new(
        i: &mut MidiInput,
        c: &mut dyn MidiInputCallback) -> Self {
    
        todo!();
        /*
        : input(i),
        : callback(c),
        : concatenator(2048),

        
        */
    }
}
