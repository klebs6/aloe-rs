crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPReceiver.h]

/**
  | A base class for classes which receive
  | Universal MIDI Packets from an input.
  | 
  | @tags{Audio}
  |
  */
pub trait UniversalMidiPacketsReceiver {

    /**
      | This will be called each time a new packet
      | is ready for processing.
      |
      */
    fn packet_received(&mut self, 
            packet: &UniversalMidiPacketsView,
            time:   f64);
}
