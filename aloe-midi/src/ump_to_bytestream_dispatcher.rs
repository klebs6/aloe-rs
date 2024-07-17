crate::ix!();

/**
  | Parses a stream of 32-bit words representing
  | a sequence of UMP-encoded MIDI messages,
  | converting the messages to MIDI 1.0
  | bytestream format and passing them
  | to a user-provided callback as they
  | become ready.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsToBytestreamDispatcher {
    dispatcher: UniversalMidiPacketsDispatcher,
    converter:  UniversalMidiPacketsToBytestreamConverter,
}

impl UniversalMidiPacketsToBytestreamDispatcher {

    /**
      | Initialises the dispatcher.
      | 
      | `storageSize` bytes will be allocated
      | to store incomplete messages.
      |
      */
    pub fn new(storage_size: i32) -> Self {
    
        todo!();
        /*
        : converter(storageSize),
        */
    }

    /**
      | Clears the dispatcher.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            dispatcher.reset();
            converter.reset();
        */
    }

    /**
      | Calls `callback` with converted bytestream-formatted
      | MidiMessage whenever a new message
      | becomes available.
      | 
      | -----------
      | @param begin
      | 
      | the first word in a stream of words representing
      | UMP-encoded MIDI packets.
      | ----------
      | @param end
      | 
      | one-past the last word in a stream of
      | words representing UMP-encoded MIDI
      | packets.
      | ----------
      | @param timestamp
      | 
      | a timestamp to apply to converted messages.
      | ----------
      | @param callback
      | 
      | a callback which will be passed a MidiMessage
      | each time a new message becomes ready.
      |
      */
    pub fn dispatch<BytestreamMessageCallback>(&mut self, 
        begin:     *const u32,
        end:       *const u32,
        timestamp: f64,
        callback:  BytestreamMessageCallback)  {
    
        todo!();
        /*
            dispatcher.dispatch (begin, end, timestamp, [&] (const View& view, double time)
            {
                converter.convert (view, time, callback);
            });
        */
    }
}
