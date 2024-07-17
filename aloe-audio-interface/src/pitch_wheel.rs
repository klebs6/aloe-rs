crate::ix!();

pub trait IsPitchWheel {

    /**
      | Returns true if the message is a pitch-wheel
      | move. @see getPitchWheelValue, pitchWheel
      |
      */
    fn is_pitch_wheel(&self) -> bool;
}

pub trait GetPitchWheelValue {

    /**
      | Returns the pitch wheel position from
      | a pitch-wheel move message.
      | 
      | -----------
      | @note
      | 
      | The value returned is a 14-bit number
      | from 0 to 0x3fff, indicating the wheel
      | position.
      | 
      | If called for messages which aren't
      | pitch wheel events, the number returned
      | will be
      | 
      | nonsense.
      | 
      | @see isPitchWheel
      |
      */
    fn get_pitch_wheel_value(&self) -> i32;
}

pub trait PitchWheel {

    /**
      | Creates a pitch-wheel move message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param position
      | 
      | the wheel position, in the range 0 to
      | 16383
      | 
      | @see isPitchWheel
      |
      */
    fn pitch_wheel(
        &mut self, 
        channel:  i32,
        position: i32

    ) -> dyn MidiMessageInterface;
}
