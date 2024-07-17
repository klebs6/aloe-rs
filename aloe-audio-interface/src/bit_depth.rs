crate::ix!();

pub trait GetBitsPerSample {

    /**
      | Returns the bit-depth of the data being
      | written.
      |
      */
    fn get_bits_per_sample(&self) -> i32;
}

pub trait GetCurrentBitDepth {

    /**
      | Returns the device's current physical
      | bit-depth.
      | 
      | If the device isn't actually open, this
      | value doesn't really mean much.
      |
      */
    fn get_current_bit_depth(&mut self) -> i32;
}
