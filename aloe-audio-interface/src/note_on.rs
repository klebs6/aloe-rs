crate::ix!();

pub trait IsNoteOn {

    /**
      | Returns true if this message is a 'key-down'
      | event. 
      |
      | @see isNoteOff, getNoteNumber, getVelocity, noteOn
      | 
      | -----------
      | @param returnTrueForVelocity0
      | 
      | if true, then if this event is a note-on
      | with velocity 0, it will still be considered
      | to be a note-on and the method will return
      | true. If returnTrueForVelocity0 is
      | false, then if this is a note-on event
      | with velocity 0, it'll be regarded as
      | a note-off, and the method will return
      | false
      |
      */
    fn is_note_on(&self, return_true_for_velocity0: Option<bool>) -> bool;
}

pub trait IsNoteOff {

    /**
      | Returns true if this message is a 'key-up'
      | event.
      | 
      | -----------
      | @note
      | 
      | If returnTrueForNoteOnVelocity0
      | is true, then his will also return true
      | 
      | for a note-on event with a velocity of 0.
      | 
      | @see isNoteOn, getNoteNumber, getVelocity,
      | noteOff
      |
      */
    fn is_note_off(&self, return_true_for_note_on_velocity0: Option<bool>) -> bool;
}

pub trait IsNoteOnOrOff {

    /**
      | Returns true if this message is a 'key-down'
      | or 'key-up' event. @see isNoteOn, isNoteOff
      |
      */
    fn is_note_on_or_off(&self) -> bool;
}

pub trait NoteOn {

    /**
      | Creates a key-down message (using an
      | integer velocity).
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param noteNumber
      | 
      | the key number, 0 to 127
      | ----------
      | @param velocity
      | 
      | in the range 0 to 127
      | 
      | @see isNoteOn
      |
      */
    fn note_on(
        &mut self, 
        channel:     i32,
        note_number: i32,
        velocity:    u8
    ) -> dyn MidiMessageInterface;
}

pub trait NoteOnWithFloatingPointVelocity {

    /**
      | Creates a key-down message (using a
      | floating-point velocity).
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param noteNumber
      | 
      | the key number, 0 to 127
      | ----------
      | @param velocity
      | 
      | in the range 0 to 1.0
      | 
      | @see isNoteOn
      |
      */
    fn note_on_with_floating_point_velocity(
        &mut self, 
        channel:     i32,
        note_number: i32,
        velocity:    f32

    ) -> dyn MidiMessageInterface;
}

pub trait NoteOffWithU8Velocity {

    /**
      | Creates a key-up message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param noteNumber
      | 
      | the key number, 0 to 127
      | ----------
      | @param velocity
      | 
      | in the range 0 to 127
      | 
      | @see isNoteOff
      |
      */
    fn note_off_with_u8_velocity(
        &mut self, 
        channel:     i32,
        note_number: i32,
        velocity:    u8
    ) -> dyn MidiMessageInterface;
}

pub trait NoteOffWithF32Velocity {

    /**
      | Creates a key-up message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param noteNumber
      | 
      | the key number, 0 to 127
      | ----------
      | @param velocity
      | 
      | in the range 0 to 1.0
      | 
      | @see isNoteOff
      |
      */
    fn note_off_with_f32_velocity(
        &mut self, 
        channel:     i32,
        note_number: i32,
        velocity:    f32

    ) -> dyn MidiMessageInterface;
}

pub trait NoteOff {

    /**
      | Creates a key-up message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param noteNumber
      | 
      | the key number, 0 to 127
      | 
      | @see isNoteOff
      |
      */
    fn note_off(
        &mut self, 
        channel:     i32,
        note_number: i32

    ) -> dyn MidiMessageInterface;
}

pub trait AllNotesOff {

    /**
      | Creates an all-notes-off message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | 
      | @see isAllNotesOff
      |
      */
    fn all_notes_off(&mut self, channel: i32) -> dyn MidiMessageInterface;
}

pub trait IsAllNotesOff {

    /**
      | Checks whether this message is an all-notes-off
      | message. @see allNotesOff
      |
      */
    fn is_all_notes_off(&self) -> bool;
}
