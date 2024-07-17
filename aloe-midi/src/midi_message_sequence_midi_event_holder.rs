crate::ix!();

/** 
  | Structure used to hold midi events in the
  | sequence.
  |
  | These structures act as 'handles' on the
  | events as they are moved about in the
  | list, and make it quick to find the
  | matching note-offs for note-on events.
  |
  | @see MidiMessageSequence::getEventPointer
  */
#[leak_detector]
pub struct MidiMessageSequenceMidiEventHolder {

    /**
      | The message itself, whose timestamp
      | is used to specify the event's time.
      |
      */
    message:         MidiMessage,

    /**
      | The matching note-off event (if this is a note-on
      | event). If this isn't a note-on, this pointer will
      | be nullptr. Use the MidiMessageSequence::updateMatchedPairs()
      | method to keep these note-offs up-to-date after
      | events have been moved around in the sequence or
      | deleted.
      */
    note_off_object: *mut MidiMessageSequenceMidiEventHolder, // default = nullptr
}

impl MidiMessageSequenceMidiEventHolder {
    
    pub fn new_from_other_ref(mm: &MidiMessage) -> Self {
    
        todo!();
        /*
        : message(mm),
        */
    }
    
    pub fn new_from_other(mm: MidiMessage) -> Self {
    
        todo!();
        /*
        : message(std::move (mm)),
        */
    }
}

