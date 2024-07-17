crate::ix!();

pub trait GetVelocity {

    /**
      | Returns the velocity of a note-on or
      | note-off message.
      | 
      | -----------
      | @note
      | 
      | The value returned will be in the range
      | 0 to 127.
      | 
      | If the message isn't a note-on or off
      | event, it will return 0.
      | 
      | @see getFloatVelocity
      |
      */
    fn get_velocity(&self) -> u8;
}

pub trait GetFloatVelocity {

    /**
      | Returns the velocity of a note-on or
      | note-off message.
      | 
      | -----------
      | @note
      | 
      | The value returned will be in the range
      | 0 to 1.0
      | 
      | If the message isn't a note-on or off
      | event, it will return 0.
      | 
      | @see getVelocity, setVelocity
      |
      */
    fn get_float_velocity(&self) -> f32;
}

pub trait SetVelocity {

    /**
      | Changes the velocity of a note-on or
      | note-off message.
      | 
      | -----------
      | @note
      | 
      | If the message isn't a note on or off,
      | this will do nothing.
      | 
      | -----------
      | @param newVelocity
      | 
      | the new velocity, in the range 0 to 1.0
      | 
      | @see getFloatVelocity, multiplyVelocity
      |
      */
    fn set_velocity(&mut self, new_velocity: f32);
}

pub trait MultiplyVelocity {

    /**
      | Multiplies the velocity of a note-on
      | or note-off message by a given amount.
      | 
      | -----------
      | @note
      | 
      | If the message isn't a note on or off,
      | this will do nothing.
      | 
      | -----------
      | @param scaleFactor
      | 
      | the value by which to multiply the velocity
      | 
      | @see setVelocity
      |
      */
    fn multiply_velocity(&mut self, scale_factor: f32);
}
