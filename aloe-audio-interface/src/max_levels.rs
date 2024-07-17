crate::ix!();

pub trait ReadMaxLevels {

    /**
      | Finds the highest and lowest sample
      | levels from a section of the audio stream.
      | 
      | This will read a block of samples from
      | the stream, and measure the highest
      | and lowest sample levels from the channels
      | in that section, returning these as
      | normalised floating-point levels.
      | 
      | -----------
      | @param startSample
      | 
      | the offset into the audio stream to start
      | reading from. It's ok for this to be beyond
      | the start or end of the stream.
      | ----------
      | @param numSamples
      | 
      | how many samples to read
      | ----------
      | @param results
      | 
      | this array will be filled with Range
      | values for each channel.
      | 
      | The array must contain numChannels
      | elements.
      | ----------
      | @param numChannelsToRead
      | 
      | the number of channels of data to scan.
      | This must be more than zero, but not more
      | than the total number of channels that
      | the reader contains @see read
      |
      */
    fn read_max_levels(&mut self, 
        start_sample:         i64,
        num_samples:          i64,
        results:              *mut Range<f32>,
        num_channels_to_read: i32);

    /**
      | Finds the highest and lowest sample
      | levels from a section of the audio stream.
      | 
      | This will read a block of samples from
      | the stream, and measure the highest
      | and lowest sample levels from the channels
      | in that section, returning these as
      | normalised floating-point levels.
      | 
      | -----------
      | @param startSample
      | 
      | the offset into the audio stream to start
      | reading from. It's ok for this to be beyond
      | the start or end of the stream.
      | ----------
      | @param numSamples
      | 
      | how many samples to read
      | ----------
      | @param lowestLeft
      | 
      | on return, this is the lowest absolute
      | sample from the left channel
      | ----------
      | @param highestLeft
      | 
      | on return, this is the highest absolute
      | sample from the left channel
      | ----------
      | @param lowestRight
      | 
      | on return, this is the lowest absolute
      | sample from the right channel (if there
      | is one)
      | ----------
      | @param highestRight
      | 
      | on return, this is the highest absolute
      | sample from the right channel (if there
      | is one) @see read
      |
      */
    fn read_max_levels_range(&mut self, 
        start_sample:  i64,
        num_samples:   i64,
        lowest_left:   &mut f32,
        highest_left:  &mut f32,
        lowest_right:  &mut f32,
        highest_right: &mut f32);

}
