crate::ix!();

pub trait GetTempoSecondsPerQuarterNote {

    /**
      | Calculates the seconds-per-quarter-note
      | from a tempo meta-event. @see isTempoMetaEvent,
      | getTempoMetaEventTickLength
      |
      */
    fn get_tempo_seconds_per_quarter_note(&self) -> f64;
}

pub trait GetTempoMetaEventTickLength {

    /**
      | Returns the tick length from a tempo
      | meta-event.
      | 
      | -----------
      | @param timeFormat
      | 
      | the 16-bit time format value from the
      | midi file's header.
      | 
      | -----------
      | @return
      | 
      | the tick length (in seconds).
      | 
      | @see isTempoMetaEvent
      |
      */
    fn get_tempo_meta_event_tick_length(&self, time_format: i16) -> f64;
}

pub trait TempoMetaEvent {

    /**
      | Creates a tempo meta-event. @see isTempoMetaEvent
      |
      */
    fn tempo_meta_event(&mut self, microseconds_per_quarter_note: i32) -> dyn MidiMessageInterface;
}

pub trait IsTempoMetaEvent {

    /**
      | Returns true if this is a 'tempo' meta-event.
      | @see getTempoMetaEventTickLength,
      | getTempoSecondsPerQuarterNote
      |
      */
    fn is_tempo_meta_event(&self) -> bool;
}


