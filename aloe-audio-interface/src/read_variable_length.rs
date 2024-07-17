crate::ix!();

pub trait ReadVariableLengthValue {

    /**
      | Reads a midi variable-length integer,
      | with protection against buffer overflow.
      | 
      | -----------
      | @param data
      | 
      | the data to read the number from
      | ----------
      | @param maxBytesToUse
      | 
      | the number of bytes in the region following
      | `data`
      | 
      | -----------
      | @return
      | 
      | a struct containing the parsed value,
      | and the number of bytes that were read.
      | If parsing fails, both the `value` and
      | `bytesUsed` fields will be set to 0 and
      | `isValid()` will return false
      |
      */
    fn read_variable_length_value(&mut self, 
        data:             *const u8,
        max_bytes_to_use: i32) -> MidiMessageVariableLengthValue;
}

pub trait ReadVariableLengthVal {

    /**
      | Reads a midi variable-length integer.
      | 
      | -----------
      | @note
      | 
      | This signature has been deprecated
      | in favour of the safer
      | 
      | readVariableLengthValue.
      | ----------
      | @note
      | 
      | The `data` argument indicates the data
      | to read the number from, and `numBytesUsed` 
      | is used as an out-parameter to indicate the 
      | number of bytes that were read.
      |
      */
    #[deprecated]
    fn read_variable_length_val(
        &mut self, 
        data:           *const u8,
        num_bytes_used: &mut i32
    ) -> i32;
}
