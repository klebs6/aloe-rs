crate::ix!();

pub const BUFFERING_AUDIO_READER_SAMPLES_PER_BLOCK: i32 = 32768;

pub struct BufferedBlock {
    range:  Range<i64>,
    buffer: AudioBuffer<f32>,
}

impl BufferedBlock {

    pub fn new<'a>(
        reader:      &mut AudioFormatReader<'a>,
        pos:         i64,
        num_samples: i32) -> Self {
    
        todo!();
        /*

        : range (pos, pos + numSamples),
        buffer ((int) reader.numChannels, numSamples)

        reader.read (&buffer, 0, numSamples, pos, true, true);
        */
    }
}
