crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/midi_io/ump/aloe_UMPU32InputHandler.h]

/**
  | A base class for classes which convert
  | Universal MIDI Packets to other formats.
  | 
  | @tags{Audio}
  |
  */
pub trait UniversalMidiPacketsU32InputHandler {

    fn reset(&mut self);

    fn push_midi_data(&mut self, 
            begin: *const u32,
            end:   *const u32,
            time:  f64);

}
