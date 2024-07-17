crate::ix!();

pub trait GetRhythmInstrumentName {

    /**
      | Returns the standard name of a channel
      | 10 percussion sound, or nullptr if unknown
      | for this note number.
      | 
      | -----------
      | @param midiNoteNumber
      | 
      | the key number, 35 to 81
      |
      */
    fn get_rhythm_instrument_name(&mut self, n: i32) -> *const u8;
}
