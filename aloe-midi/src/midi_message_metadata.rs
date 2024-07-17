crate::ix!();

/**
  | A view of MIDI message data stored in
  | a contiguous buffer.
  |
  | Instances of this class do *not* own the midi
  | data bytes that they point to.  Instead, they
  | expect the midi data to live in a separate
  | buffer that outlives the MidiMessageMetadata
  | instance.
  |
  | @tags{Audio}
  */
pub struct MidiMessageMetadata {

    /**
      | Pointer to the first byte of a MIDI message.
      |
      */
    data:            *const u8, // default = nullptr

    /**
      | The number of bytes in the MIDI message.
      |
      */
    num_bytes:       i32, // default = 0

    /**
      | The MIDI message's timestamp.
      |
      */
    sample_position: i32, // default = 0
}

impl Default for MidiMessageMetadata {

    fn default() -> Self {
        todo!();
    }
}

impl MidiMessageMetadata {

    pub fn new(
        data_in:      *const u8,
        num_bytes_in: i32,
        position_in:  i32) -> Self {
    
        todo!();
        /*
        : data(dataIn),
        : num_bytes(numBytesIn),
        : sample_position(positionIn),

        
        */
    }

    /** 
      | Constructs a new MidiMessage instance from
      | the data that this object is viewing.
      |
      | Note that MidiMessage owns its data
      | storage, whereas MidiMessageMetadata does not.
      */
    pub fn get_message(&self) -> MidiMessage {
        
        todo!();
        /*
            return MidiMessage (data, numBytes, samplePosition);
        */
    }
}
