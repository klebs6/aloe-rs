crate::ix!();

pub trait GetRawData {

    /**
      | Returns a pointer to the raw midi data.
      | @see getRawDataSize
      |
      */
    fn get_raw_data(&self) -> *const u8;
}

pub trait GetRawDataSize {

    /**
      | Returns the number of bytes of data in
      | the message. @see getRawData
      |
      */
    fn get_raw_data_size(&self) -> i32;
}

pub trait GetData {

    fn get_data(&self) -> *mut u8;
}
