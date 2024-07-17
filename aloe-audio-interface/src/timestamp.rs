crate::ix!();

pub trait GetTimeStamp {

    /** 
      | Returns the timestamp associated with this
      | message.
      |
      | The exact meaning of this time and its units
      | will vary, as messages are used in a variety
      | of different contexts.
      |
      | If you're getting the message from a midi
      | file, this could be a time in seconds, or
      | a number of ticks - see
      | MidiFile::convertTimestampTicksToSeconds().
      |
      | If the message is being used in a MidiBuffer,
      | it might indicate the number of audio samples
      | from the start of the buffer.
      |
      | If the message was created by a MidiInput,
      | see
      | MidiInputCallback::handleIncomingMidiMessage()
      | for details of the way that it initialises
      | this value.
      |
      | @see setTimeStamp, addToTimeStamp
      */
    fn get_time_stamp(&self) -> f64;
}

pub trait SetTimeStamp {

    /** 
      | Changes the message's associated timestamp.
      |
      | The units for the timestamp will be
      | application-specific - see the notes for
      | getTimeStamp().  @see addToTimeStamp,
      | getTimeStamp
      */
    fn set_time_stamp(&mut self, new_timestamp: f64);
}

pub trait AddToTimeStamp {

    /** 
      | Adds a value to the message's timestamp.
      |
      | The units for the timestamp will be
      | application-specific.
      */
    fn add_to_time_stamp(&mut self, delta: f64);
}
