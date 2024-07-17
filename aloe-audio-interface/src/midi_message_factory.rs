crate::ix!();

pub trait FloatValueToMidiByte {

    /**
      | Converts a floating-point value between
      | 0 and 1 to a MIDI 7-bit value between 0
      | and 127.
      |
      */
    fn float_value_to_midi_byte(&mut self, v: f32) -> u8;
}

pub trait NewFromBlockOfData {

    /**
      | Creates a midi message from a block of
      | data.
      |
      */
    fn new_from_block_of_data(
        d:          *const c_void,
        data_size:  i32,
        time_stamp: Option<f64>

    ) -> Self where Self: Sized;
}

pub trait New1ByteShortMessage {

    /**
      | Creates a 1-byte short midi message.
      | 
      | -----------
      | @param byte1
      | 
      | message byte 1
      | ----------
      | @param timeStamp
      | 
      | the time to give the midi message - this
      | value doesn't use any particular units,
      | so will be application-specific
      |
      */
    fn new_1byte_short_message(
        byte1:      i32,
        time_stamp: Option<f64>

    ) -> Self where Self: Sized;
}

pub trait New2ByteShortMessage {

    /**
      | Creates a 2-byte short midi message.
      | 
      | -----------
      | @param byte1
      | 
      | message byte 1
      | ----------
      | @param byte2
      | 
      | message byte 2
      | ----------
      | @param timeStamp
      | 
      | the time to give the midi message - this
      | value doesn't use any particular units,
      | so will be application-specific
      |
      */
    fn new_2byte_short_message(
        byte1:      i32,
        byte2:      i32,
        time_stamp: Option<f64>

    ) -> Self where Self: Sized;
}

pub trait New3ByteShortMessage {

    /**
      | Creates a 3-byte short midi message.
      | 
      | -----------
      | @param byte1
      | 
      | message byte 1
      | ----------
      | @param byte2
      | 
      | message byte 2
      | ----------
      | @param byte3
      | 
      | message byte 3
      | ----------
      | @param timeStamp
      | 
      | the time to give the midi message - this
      | value doesn't use any particular units,
      | so will be application-specific
      |
      */
    fn new_3byte_short_message(
        byte1:      i32,
        byte2:      i32,
        byte3:      i32,
        time_stamp: Option<f64>

    ) -> Self where Self: Sized;
}


pub trait NewFromOtherMidiMessageWithDifferentTimestamp {

    /**
      | Creates a copy of another midi message,
      | with a different timestamp.
      |
      */
    fn new_from_other_with_different_timestamp(
        other:          &dyn MidiMessageInterface,
        new_time_stamp: f64
    ) -> Self where Self: Sized;
}

pub trait NewFromMidiStream {

    /**
      | Reads the next midi message from some
      | data.
      | 
      | -----------
      | @note
      | 
      | This will read as many bytes from a data
      | stream as it needs to make a
      | 
      | complete message, and will return the
      | number of bytes it used. This lets
      | 
      | you read a sequence of midi messages
      | from a file or stream.
      | 
      | -----------
      | @param data
      | 
      | the data to read from
      | ----------
      | @param maxBytesToUse
      | 
      | the maximum number of bytes it's allowed
      | to read
      | ----------
      | @param numBytesUsed
      | 
      | returns the number of bytes that were
      | actually needed
      | ----------
      | @param lastStatusByte
      | 
      | in a sequence of midi messages, the initial
      | byte can be dropped from a message if
      | it's the same as the first byte of the
      | previous message, so this lets you supply
      | the byte to use if the first byte of the
      | message has in fact been dropped.
      | ----------
      | @param timeStamp
      | 
      | the time to give the midi message - this
      | value doesn't use any particular units,
      | so will be application-specific
      | ----------
      | @param sysexHasEmbeddedLength
      | 
      | when reading sysexes, this flag indicates
      | whether to expect the data to begin with
      | a variable-length field indicating
      | its size
      |
      */
    fn new_from_midi_stream(
        src_data:                  *const c_void,
        sz:                        i32,
        num_bytes_used:            &mut i32,
        last_status_byte:          u8,
        time_stamp:                Option<f64>,
        sysex_has_embedded_length: Option<bool>

    ) -> Self where Self: Sized;
}

pub trait CreateWithTimeStamp {

    /** 
      | Return a copy of this message with a new
      | timestamp.
      |
      | The units for the timestamp will be
      | application-specific - see the notes for
      | getTimeStamp().
      */
    fn with_time_stamp(&self, new_timestamp: f64) -> dyn MidiMessageInterface;
}
