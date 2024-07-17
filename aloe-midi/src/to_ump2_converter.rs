crate::ix!();

/**
  | Allows conversion from bytestream-
  | or Universal MIDI Packet-formatted
  | messages to MIDI 2.0 messages in UMP
  | format.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsToUMP2Converter {
    translator: UniversalMidiPacketsMidi1ToMidi2DefaultTranslator,
}

impl UniversalMidiPacketsToUMP2Converter {

    pub fn convert<Fn>(&mut self, 
        m:   &MidiMessage,
        fn_: Fn)  {
    
        todo!();
        /*
            Conversion::toMidi1 (m, [&] (const UniversalMidiPacketsView& v)
                {
                    translator.dispatch (v, fn);
                });
        */
    }
    
    pub fn convert_from_universal_midi_packets_view<Fn>(
        &mut self, 
        v:   &UniversalMidiPacketsView,
        fn_: Fn

    ) {
    
        todo!();
        /*
            translator.dispatch (v, std::forward<Fn> (fn));
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            translator.reset();
        */
    }
}

