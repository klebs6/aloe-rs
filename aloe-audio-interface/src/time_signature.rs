crate::ix!();

pub trait IsTimeSignatureMetaEvent {

    /**
      | Returns true if this is a 'time-signature'
      | meta-event. @see getTimeSignatureInfo
      |
      */
    fn is_time_signature_meta_event(&self) -> bool;
}

pub trait GetTimeSignatureInfo {

    /**
      | Returns the time-signature values
      | from a time-signature meta-event.
      | @see isTimeSignatureMetaEvent
      |
      */
    fn get_time_signature_info(
        &self, 
        numerator:   &mut i32,
        denominator: &mut i32
    );
}

pub trait TimeSignatureMetaEvent {

    /**
      | Creates a time-signature meta-event.
      | @see isTimeSignatureMetaEvent
      |
      */
    fn time_signature_meta_event(
        &mut self, 
        numerator:   i32,
        denominator: i32
    ) -> dyn MidiMessageInterface;
}
