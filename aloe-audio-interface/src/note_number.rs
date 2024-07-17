crate::ix!();

pub trait GetNoteNumber {

    /**
      | Returns the midi note number for note-on
      | and note-off messages. If the message
      | isn't a note-on or off, the value returned
      | is undefined. @see isNoteOff, getMidiNoteName,
      | getMidiNoteInHertz, setNoteNumber
      |
      */
    fn get_note_number(&self) -> i32;
}

pub trait SetNoteNumber {

    /**
      | Changes the midi note number of a note-on
      | or note-off message. If the message
      | isn't a note on or off, this will do nothing.
      |
      */
    fn set_note_number(&mut self, new_note_number: i32);
}
