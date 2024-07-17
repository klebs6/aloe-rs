crate::ix!();

/** 
  | Holds information about a variable-length
  | value which was parsed from a stream of
  | bytes.
  |
  | A valid value requires that `bytesUsed` is
  | greater than 0.
  */
#[derive(Default)]
pub struct MidiMessageVariableLengthValue
{
    value:      i32, // default = 0
    bytes_used: i32, // default = 0
}

impl MidiMessageVariableLengthValue {
    
    pub fn new(
        value_in:      i32,
        bytes_used_in: i32) -> Self {
    
        todo!();
        /*
        : value(valueIn),
        : bytes_used(bytesUsedIn),

        
        */
    }
    
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return bytesUsed > 0;
        */
    }
}
