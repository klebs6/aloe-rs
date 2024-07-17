crate::ix!();

/**
  | Allows conversion from bytestream-
  | or Universal MIDI Packet-formatted
  | messages to UMP format.
  | 
  | The packet protocol can be selected
  | using the constructor parameter.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsGenericUMPConverter {
    converters: (UniversalMidiPacketsToUMP1Converter,UniversalMidiPacketsToUMP2Converter),
    mode:       UniversalMidiPacketsPacketProtocol,
}

impl UniversalMidiPacketsGenericUMPConverter {

    pub fn new(m: UniversalMidiPacketsPacketProtocol) -> Self {
    
        todo!();
        /*
        : mode(m),

        
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            std::get<1> (converters).reset();
        */
    }
    
    pub fn convert_message<Fn>(
        &mut self, 
        m:   &MidiMessage,
        fn_: Fn

    ) {
    
        todo!();
        /*
            switch (mode)
                {
                    case UniversalMidiPacketsPacketProtocol::MIDI_1_0: return std::get<0> (converters).convert (m, std::forward<Fn> (fn));
                    case UniversalMidiPacketsPacketProtocol::MIDI_2_0: return std::get<1> (converters).convert (m, std::forward<Fn> (fn));
                }
        */
    }
    
    pub fn convert<Fn>(
        &mut self, 
        v:   &UniversalMidiPacketsView,
        fn_: Fn

    ) {
    
        todo!();
        /*
            switch (mode)
                {
                    case UniversalMidiPacketsPacketProtocol::MIDI_1_0: return std::get<0> (converters).convert (v, std::forward<Fn> (fn));
                    case UniversalMidiPacketsPacketProtocol::MIDI_2_0: return std::get<1> (converters).convert (v, std::forward<Fn> (fn));
                }
        */
    }
    
    pub fn convert_range<Fn>(
        &mut self, 
        begin: UniversalMidiPacketsIterator,
        end:   UniversalMidiPacketsIterator,
        fn_:   Fn)  {
    
        todo!();
        /*
            std::for_each (begin, end, [&] (const UniversalMidiPacketsView& v)
                {
                    convert (v, fn);
                });
        */
    }
    
    pub fn get_protocol(&self) -> UniversalMidiPacketsPacketProtocol {
        
        todo!();
        /*
            return mode;
        */
    }
}
