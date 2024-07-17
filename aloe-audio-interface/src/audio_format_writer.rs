crate::ix!();

pub trait AudioFormatWriterInterface 

/*
   | Returns a description of what type of
   | format this is.
   | 
   | E.g. "AIFF file"
   |
   */
: GetFormatName
+ GetSampleRate
+ GetNumChannels
+ GetBitsPerSample
+ IsFloatingPoint
+ WriteFromAudioReader
+ WriteFromAudioSource
+ WriteFromFloatArrays
+ WriteFromAudioSampleBuffer
+ Flush
{ }

pub trait CreateWriterFor {

    fn create_writer_for(
        &mut self, 
        stream_to_write_to:   &mut dyn Write,
        sample_rate_to_use:   f64,
        channel_layout:       &AudioChannelSet,
        bits_per_sample:      i32,
        metadata_values:      &Vec<(String,String)>,
        quality_option_index: i32

    ) -> Box<dyn AudioFormatWriterInterface>;
}
