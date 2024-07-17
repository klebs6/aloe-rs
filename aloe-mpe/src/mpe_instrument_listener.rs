crate::ix!();

/**
  | Derive from this class to be informed
  | about any changes in the expressive
  | MIDI notes played by this instrument.
  | 
  | Note: This listener type receives its
  | callbacks immediately, and not via
  | the message thread (so you might be for
  | example in the MIDI thread). Therefore
  | you should never do heavy work such as
  | graphics rendering etc. inside those
  | callbacks.
  |
  */
pub trait MpeInstrumentListener {

    /**
      | Implement this callback to be informed
      | whenever a new expressive MIDI note
      | is triggered.
      |
      */
    fn note_added(&mut self, new_note: MPENote)  {
        
        todo!();
        /*
            ignoreUnused (newNote);
        */
    }

    /**
      | Implement this callback to be informed
      | whenever a currently playing MPE note's
      | pressure value changes.
      |
      */
    fn note_pressure_changed(&mut self, changed_note: MPENote)  {
        
        todo!();
        /*
            ignoreUnused (changedNote);
        */
    }

    /**
      | Implement this callback to be informed
      | whenever a currently playing MPE note's
      | pitchbend value changes.
      | 
      | Note: This can happen if the note itself
      | is bent, if there is a master channel
      | pitchbend event, or if both occur simultaneously.
      | Call MPENote::getFrequencyInHertz
      | to get the effective note frequency.
      |
      */
    fn note_pitchbend_changed(&mut self, changed_note: MPENote)  {
        
        todo!();
        /*
            ignoreUnused (changedNote);
        */
    }

    /**
      | Implement this callback to be informed
      | whenever a currently playing MPE note's
      | timbre value changes.
      |
      */
    fn note_timbre_changed(&mut self, changed_note: MPENote)  {
        
        todo!();
        /*
            ignoreUnused (changedNote);
        */
    }

    /**
      | Implement this callback to be informed
      | whether a currently playing MPE note's
      | key state (whether the key is down and/or
      | the note is sustained) has changed.
      | 
      | Note: If the key state changes to MPENote::off,
      | noteReleased is called instead.
      |
      */
    fn note_key_state_changed(&mut self, changed_note: MPENote)  {
        
        todo!();
        /*
            ignoreUnused (changedNote);
        */
    }

    /**
      | Implement this callback to be informed
      | whenever an MPE note is released (either
      | by a note-off message, or by a sustain/sostenuto
      | pedal release for a note that already
      | received a note-off), and should therefore
      | stop playing.
      |
      */
    fn note_released(&mut self, finished_note: MPENote)  {
        
        todo!();
        /*
            ignoreUnused (finishedNote);
        */
    }
}
