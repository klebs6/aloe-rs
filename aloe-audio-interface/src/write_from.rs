crate::ix!();

pub trait WriteFromAudioReader {

    /**
      | Reads a section of samples from an AudioFormatReader,
      | and writes these to the output.
      | 
      | This will take care of any floating-point
      | conversion that's required to convert
      | between the two formats. It won't deal
      | with sample-rate conversion, though.
      | 
      | If numSamplesToRead < 0, it will write
      | the entire length of the reader.
      | 
      | 
      | -----------
      | @return
      | 
      | false if it can't read or write properly
      | during the operation
      |
      */
    fn write_from_audio_reader(
        &mut self, 
        reader:              &mut dyn AudioFormatReaderInterface,
        start_sample:        i64,
        num_samples_to_read: i64

    ) -> bool;
}

pub trait WriteFromAudioSource {

    /**
      | Reads some samples from an AudioSource,
      | and writes these to the output.
      | 
      | The source must already have been initialised
      | with the AudioSource::prepareToPlay()
      | method
      | 
      | -----------
      | @param source
      | 
      | the source to read from
      | ----------
      | @param numSamplesToRead
      | 
      | total number of samples to read and write
      | ----------
      | @param samplesPerBlock
      | 
      | the maximum number of samples to fetch
      | from the source
      | 
      | -----------
      | @return
      | 
      | false if it can't read or write properly
      | during the operation
      |
      */
    fn write_from_audio_source(
        &mut self, 
        source:              &mut dyn AudioSource,
        num_samples_to_read: i32,
        samples_per_block:   i32

    ) -> bool;
}

pub trait WriteFromFloatArrays {

    /**
      | Writes some samples from a set of float
      | data channels.
      |
      */
    fn write_from_float_arrays(
        &mut self, 
        channels:            *const *const f32,
        num_source_channels: i32,
        num_samples:         i32

    ) -> bool;
}

pub trait WriteFromAudioSampleBuffer {

    /**
      | Writes some samples from an AudioBuffer.
      |
      */
    fn write_from_audio_sample_buffer(
        &mut self, 
        source:       &AudioBuffer<f32>,
        start_sample: i32,
        num_samples:  i32

    ) -> bool;
}
