crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/midi_io/ump/aloe_UMPBytestreamInputHandler.h]

/**
  | A base class for classes which convert
  | bytestream midi to other formats.
  | 
  | @tags{Audio}
  |
  */
pub trait UniversalMidiPacketsBytestreamInputHandler {

    fn reset(&mut self);

    fn push_midi_data(&mut self, 
            data:  *const c_void,
            bytes: i32,
            time:  f64);
}
