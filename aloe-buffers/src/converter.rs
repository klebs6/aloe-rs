crate::ix!();

/** 
 | A base class for objects that are used to
 | convert between two different sample formats.
 |
 | The AudioData::ConverterInstance
 | implements this base class and can be
 | templated, so you can create an instance
 | that converts between two particular
 | formats, and then store this in the
 | abstract base class.
 |
 | @see AudioData::ConverterInstance
*/
pub trait Converter {

    /**
      | Converts a sequence of samples from
      | the converter's source format into
      | the dest format.
      |
      */
    fn convert_samples(&self, 
            dest_samples:   *mut c_void,
            source_samples: *const c_void,
            num_samples:    i32);

    /**
      | Converts a sequence of samples from
      | the converter's source format into
      | the dest format.
      | 
      | This method takes sub-channel indexes,
      | which can be used with interleaved formats
      | in order to choose a particular sub-channel
      | of the data to be used.
      |
      */
    fn convert_samples_with_sub_channel(&self, 
            dest_samples:       *mut c_void,
            dest_sub_channel:   i32,
            source_samples:     *const c_void,
            source_sub_channel: i32,
            num_samples:        i32);
}
