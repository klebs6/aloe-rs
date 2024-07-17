crate::ix!();

pub trait IsEndOfTrackMetaEvent {

    /**
      | Returns true if this is an 'end-of-track'
      | meta-event.
      |
      */
    fn is_end_of_track_meta_event(&self) -> bool;
}

pub trait EndOfTrack {

    /**
      | Creates an end-of-track meta-event.
      | @see isEndOfTrackMetaEvent
      |
      */
    fn end_of_track(&mut self) -> dyn MidiMessageInterface;
}
