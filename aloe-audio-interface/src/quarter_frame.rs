crate::ix!();

pub trait IsQuarterFrame {

    /**
      | Returns true if this is a quarter-frame
      | midi timecode message. @see quarterFrame,
      | getQuarterFrameSequenceNumber,
      | getQuarterFrameValue
      |
      */
    fn is_quarter_frame(&self) -> bool;
}

pub trait GetQuarterFrameSequencerNumber {

    /**
      | Returns the sequence number of a quarter-frame
      | midi timecode message. This will be
      | a value between 0 and 7. @see isQuarterFrame,
      | getQuarterFrameValue, quarterFrame
      |
      */
    fn get_quarter_frame_sequence_number(&self) -> i32;
}

pub trait GetQuarterFrameValue {

    /**
      | Returns the value from a quarter-frame
      | message. This will be the lower nybble
      | of the message's data-byte, a value
      | between 0 and 15
      |
      */
    fn get_quarter_frame_value(&self) -> i32;
}

pub trait QuarterFrame {

    /**
      | Creates a quarter-frame MTC message.
      | 
      | -----------
      | @param sequenceNumber
      | 
      | a value 0 to 7 for the upper nybble of the
      | message's data byte
      | ----------
      | @param value
      | 
      | a value 0 to 15 for the lower nybble of
      | the message's data byte
      |
      */
    fn quarter_frame(&mut self, 
        sequence_number: i32,
        value:           i32) -> dyn MidiMessageInterface;
}
