crate::ix!();

/**
  | Represents the constant parts of an audio
  | sample: its name, sample rate, length, and the
  | audio sample data itself.
  |
  | Samples might be pretty big, so we'll keep
  | shared_ptrs to them most of the time, to reduce
  | duplication and copying.
  */
pub struct Sample {
    source_sample_rate: f64,
    length:             i32,
    data:               AudioBuffer<f32>,
}

impl Sample {

    pub fn new(
        source:                 &mut AudioFormatReader,
        max_sample_length_secs: f64) -> Self {
    
        todo!();
        /*


            : sourceSampleRate (source.sampleRate),
              length (jmin (int (source.lengthInSamples),
                            int (maxSampleLengthSecs * sourceSampleRate))),
              data (jmin (2, int (source.numChannels)), length + 4)

            if (length == 0)
                throw std::runtime_error ("Unable to load sample");

            source.read (&data, 0, length + 4, 0, true, true);
        */
    }
    
    pub fn get_sample_rate(&self) -> f64 {
        
        todo!();
        /*
            return sourceSampleRate;
        */
    }
    
    pub fn get_length(&self) -> i32 {
        
        todo!();
        /*
            return length;
        */
    }
    
    pub fn get_buffer(&self) -> &AudioBuffer<f32> {
        
        todo!();
        /*
            return data;
        */
    }
}
