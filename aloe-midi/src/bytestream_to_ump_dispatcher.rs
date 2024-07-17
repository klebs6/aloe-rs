crate::ix!();

/**
  | Parses a stream of bytes representing
  | a sequence of bytestream-encoded MIDI
  | 1.0 messages, converting the messages
  | to UMP format and passing the packets
  | to a user-provided callback as they
  | become ready.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsBytestreamToUMPDispatcher {
    concatenator: MidiDataConcatenator,
    converter:    UniversalMidiPacketsGenericUMPConverter,
}

impl UniversalMidiPacketsBytestreamToUMPDispatcher {

    /**
      | Initialises the dispatcher.
      | 
      | Channel messages will be converted
      | to the requested protocol format `pp`.
      | `storageSize` bytes will be allocated
      | to store incomplete messages.
      |
      */
    pub fn new(
        pp:           UniversalMidiPacketsPacketProtocol,
        storage_size: i32) -> Self {
    
        todo!();
        /*
        : concatenator(storageSize),
        : converter(pp),

        
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            concatenator.reset();
            converter.reset();
        */
    }

    /**
      | Calls `callback` with a View of each
      | converted packet as it becomes ready.
      | 
      | -----------
      | @param begin
      | 
      | the first byte in a range of bytes representing
      | bytestream-encoded MIDI messages.
      | ----------
      | @param end
      | 
      | one-past the last byte in a range of bytes
      | representing bytestream-encoded
      | MIDI messages.
      | ----------
      | @param timestamp
      | 
      | a timestamp to apply to the created packets.
      | ----------
      | @param callback
      | 
      | a callback which will be passed a View
      | pointing to each new packet as it becomes
      | ready.
      |
      */
    pub fn dispatch<PacketCallbackFunction>(&mut self, 
        begin:     *const u8,
        end:       *const u8,
        timestamp: f64,
        callback:  PacketCallbackFunction)  {
    
        todo!();
        /*
            using CallbackPtr = decltype (std::addressof (callback));

           #if ALOE_MINGW
            #define ALOE_MINGW_HIDDEN_VISIBILITY __attribute__ ((visibility ("hidden")))
           #else
            #define ALOE_MINGW_HIDDEN_VISIBILITY
           #endif

            struct ALOE_MINGW_HIDDEN_VISIBILITY Callback
            {
                Callback (UniversalMidiPacketsBytestreamToUMPDispatcher& d, CallbackPtr c)
                    : dispatch (d), callbackPtr (c) {}

                void handleIncomingMidiMessage (void*, const MidiMessage& msg) const
                {
                    Conversion::toMidi1 (msg, [&] (const View& view)
                    {
                        dispatch.converter.convert (view, *callbackPtr);
                    });
                }

                void handlePartialSysexMessage (void*, const uint8_t*, int, double) const {}

                UniversalMidiPacketsBytestreamToUMPDispatcher& dispatch;
                CallbackPtr callbackPtr = nullptr;
            };

           #undef ALOE_MINGW_HIDDEN_VISIBILITY

            Callback inputCallback { *this, &callback };
            concatenator.pushMidiData (begin, int (end - begin), timestamp, (void*) nullptr, inputCallback);
        */
    }
}
