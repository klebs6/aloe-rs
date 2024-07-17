crate::ix!();

pub trait GetCurrentBufferSizeSamples {

    /**
      | Returns the buffer size that the device
      | is currently using.
      | 
      | If the device isn't actually open, this
      | value doesn't really mean much.
      |
      */
    fn get_current_buffer_size_samples(&mut self) -> i32;
}

pub trait GetAvailableBufferSizes {

    /**
      | Returns the set of buffer sizes that
      | are available. @see getCurrentBufferSizeSamples,
      | getDefaultBufferSize
      |
      */
    fn get_available_buffer_sizes(&mut self) -> Vec<i32>;
}

pub trait GetDefaultBufferSize {

    /**
      | Returns the default buffer-size to
      | use.
      | 
      | -----------
      | @return
      | 
      | a number of samples @see getAvailableBufferSizes
      |
      */
    fn get_default_buffer_size(&mut self) -> i32;
}
