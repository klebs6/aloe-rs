crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPConverters.h]

/**
  | Allows conversion from bytestream-
  | or Universal MIDI Packet-formatted
  | messages to bytestream format.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsToBytestreamConverter {
    translator: UniversalMidiPacketsMidi1ToBytestreamTranslator,
}

impl UniversalMidiPacketsToBytestreamConverter {

    pub fn new(storage_size: i32) -> Self {
    
        todo!();
        /*
        : translator(storageSize),
        */
    }
    
    pub fn convert<Fn>(&mut self, 
        m:   &MidiMessage,
        fn_: Fn)  {
    
        todo!();
        /*
            fn (m);
        */
    }
    
    pub fn convert_with_universal_packets_view<Fn>(
        &mut self, 
        v:    &UniversalMidiPacketsView,
        time: f64,
        fn_:  Fn

    ) {
    
        todo!();
        /*
            Conversion::midi2ToMidi1DefaultTranslation (v, [&] (const UniversalMidiPacketsView& midi1)
                {
                    translator.dispatch (midi1, time, fn);
                });
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            translator.reset();
        */
    }
}

