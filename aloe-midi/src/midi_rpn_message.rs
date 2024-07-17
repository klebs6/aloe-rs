crate::ix!();

/** 
  | Represents a MIDI RPN (registered parameter
  | number) or NRPN (non-registered parameter number)
  | message.
  |
  | @tags{Audio}
  */
pub struct MidiRPNMessage
{
    /**
      | Midi channel of the message, in the range
      | 1 to 16.
      |
      */
    channel:          i32,

    /**
      | The 14-bit parameter index, in the range
      | 0 to 16383 (0x3fff).
      |
      */
    parameter_number: i32,

    /**
      | The parameter value, in the range 0 to 16383 (0x3fff).
      | If the message contains no value LSB, the value
      | will be in the range 0 to 127 (0x7f).
      */
    value:            i32,

    /**
      | True if this message is an NRPN; false
      | if it is an RPN.
      |
      */
    isnrpn:           bool,

    /**
      | True if the value uses 14-bit resolution
      | (LSB + MSB); false if the value is 7-bit
      | (MSB only).
      |
      */
    is_14bit_value:   bool,
}
