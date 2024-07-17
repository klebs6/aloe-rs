crate::ix!();

pub trait IsMidiClock {

    /**
      | Returns true if this is a midi clock event.
      | @see midiClock, songPositionPointer
      |
      */
    fn is_midi_clock(&self) -> bool;
}

pub trait MidiClock {

    /**
      | Creates a midi clock event.
      |
      */
    fn midi_clock(&mut self) -> dyn MidiMessageInterface;
}
