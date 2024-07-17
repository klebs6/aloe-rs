crate::ix!();

pub trait IsMetaEvent {

    /**
      | Returns true if this event is a meta-event.
      | 
      | -----------
      | @note
      | 
      | Meta-events are things like tempo changes,
      | track names, etc.
      | 
      | @see getMetaEventType, isTrackMetaEvent,
      | isEndOfTrackMetaEvent, isTextMetaEvent, 
      | isTrackNameEvent, isTempoMetaEvent, 
      | isTimeSignatureMetaEvent,
      | 
      | isKeySignatureMetaEvent, isMidiChannelMetaEvent
      |
      */
    fn is_meta_event(&self) -> bool;
}

pub trait GetMetaEventType {

    /** 
      | Returns a meta-event's type number.
      |
      | If the message isn't a meta-event, this
      | will return -1.
      |
      | @see isMetaEvent, isTrackMetaEvent,
      | isEndOfTrackMetaEvent,
      | isTextMetaEvent, isTrackNameEvent,
      | isTempoMetaEvent,
      | isTimeSignatureMetaEvent,
      | isKeySignatureMetaEvent,
      | isMidiChannelMetaEvent
      */
    fn get_meta_event_type(&self) -> i32;
}

pub trait GetMetaEventLength {

    /**
      | Returns the length of the data for a meta-event.
      | @see isMetaEvent, getMetaEventData
      |
      */
    fn get_meta_event_length(&self) -> i32;
}

pub trait GetMetaEventData {

    /**
      | Returns a pointer to the data in a meta-event.
      | @see isMetaEvent, getMetaEventLength
      */
    fn get_meta_event_data(&self) -> *const u8;
}
