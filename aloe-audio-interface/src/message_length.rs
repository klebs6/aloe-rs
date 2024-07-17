crate::ix!();

pub trait GetMessageLengthFromFirstByte {

    /**
      | Based on the first byte of a short midi
      | message, this uses a lookup table to
      | return the message length (either 1,
      | 2, or 3 bytes).
      | 
      | -----------
      | @note
      | 
      | The value passed in must be 0x80 or higher.
      |
      */
    fn get_message_length_from_first_byte(&mut self, first_byte: u8) -> i32;
}
