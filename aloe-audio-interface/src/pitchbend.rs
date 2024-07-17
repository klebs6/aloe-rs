crate::ix!();

pub trait PitchBendToPitchWheelPos {

    /**
      | Converts a pitchbend value in semitones
      | to a MIDI 14-bit pitchwheel position
      | value.
      |
      */
    fn pitchbend_to_pitchwheel_pos(
        &mut self, 
        pitchbend:       f32,
        pitchbend_range: f32
    ) -> u16;
}
