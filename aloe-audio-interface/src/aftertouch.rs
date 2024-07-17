crate::ix!();

pub trait IsAftertouch {

    /**
      | Returns true if the message is an aftertouch
      | event.
      | 
      | -----------
      | @note
      | 
      | For aftertouch events, use the getNoteNumber()
      | method to find out the key
      | 
      | that it applies to, and getAfterTouchValue()
      | to find out the amount. Use
      | 
      | getChannel() to find out the channel.
      | 
      | @see getAftertouchValue, getNoteNumber
      |
      */
    fn is_aftertouch(&self) -> bool;
}

pub trait GetAftertouchValue {

    /**
      | Returns the amount of aftertouch from
      | an aftertouch messages.
      | 
      | -----------
      | @note
      | 
      | The value returned is in the range 0 to
      | 127, and will be nonsense for messages
      | other than aftertouch messages.
      | 
      | @see isAftertouch
      |
      */
    fn get_after_touch_value(&self) -> i32;
}

pub trait AftertouchChange {

    /**
      | Creates an aftertouch message.
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
      | @param aftertouchAmount
      | 
      | the amount of aftertouch, 0 to 127
      | 
      | @see isAftertouch
      |
      */
    fn aftertouch_change(&mut self, 
        channel:          i32,
        note_num:         i32,
        aftertouch_value: i32) -> dyn MidiMessageInterface;
}
