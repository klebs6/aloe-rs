crate::ix!();

/**
  | Allows conversion from bytestream-
  | or Universal MIDI Packet-formatted
  | messages to MIDI 1.0 messages in UMP
  | format.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsToUMP1Converter {

}

impl UniversalMidiPacketsToUMP1Converter {

    pub fn convert<Fn>(&mut self, 
        m:   &MidiMessage,
        fn_: Fn)  {
    
        todo!();
        /*
            Conversion::toMidi1 (m, std::forward<Fn> (fn));
        */
    }
    
    pub fn convert_from_universal_midi_packets_view<Fn>(
        &mut self, 
        v:   &UniversalMidiPacketsView,
        fn_: Fn

    ) {
    
        todo!();
        /*
            Conversion::midi2ToMidi1DefaultTranslation (v, std::forward<Fn> (fn));
        */
    }
}

