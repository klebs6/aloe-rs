crate::ix!();

pub trait IsMidiStart {

    /**
      | Returns true if this is a midi start event.
      | @see midiStart
      |
      */
    fn is_midi_start(&self) -> bool;
}

pub trait MidiStart {

    /**
      | Creates a midi start event.
      |
      */
    fn midi_start(&mut self) -> dyn MidiMessageInterface;
}

pub trait IsMidiContinue {

    /**
      | Returns true if this is a midi continue
      | event. @see midiContinue
      |
      */
    fn is_midi_continue(&self) -> bool;
}

pub trait MidiContinue {

    /**
      | Creates a midi continue event.
      |
      */
    fn midi_continue(&mut self) -> dyn MidiMessageInterface;
}

pub trait IsMidiStop {

    /**
      | Returns true if this is a midi stop event.
      | @see midiStop
      |
      */
    fn is_midi_stop(&self) -> bool;
}

pub trait MidiStop {

    /**
      | Creates a midi stop event.
      |
      */
    fn midi_stop(&mut self) -> dyn MidiMessageInterface;
}
