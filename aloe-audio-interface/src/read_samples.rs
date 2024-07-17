crate::ix!();

pub trait ReadSamples {

    /**
      | Subclasses must implement this method
      | to perform the low-level read operation.
      | 
      | Callers should use read() instead of
      | calling this directly.
      | 
      | -----------
      | @param destChannels
      | 
      | the array of destination buffers to
      | fill. Some of these pointers may be null
      | ----------
      | @param numDestChannels
      | 
      | the number of items in the destChannels
      | array. This value is guaranteed not
      | to be greater than the number of channels
      | that this reader object contains
      | ----------
      | @param startOffsetInDestBuffer
      | 
      | the number of samples from the start
      | of the dest data at which to begin writing
      | ----------
      | @param startSampleInFile
      | 
      | the number of samples into the source
      | data at which to begin reading. This
      | value is guaranteed to be >= 0.
      | ----------
      | @param numSamples
      | 
      | the number of samples to read
      |
      */
    fn read_samples(&mut self, 
        dest_channels:               *mut *mut i32,
        num_dest_channels:           i32,
        start_offset_in_dest_buffer: i32,
        start_sample_in_file:        i64,
        num_samples:                 i32) -> bool;
}
