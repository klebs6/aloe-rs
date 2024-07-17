crate::ix!();

#[derive(Default)]
pub struct BufferWithSampleRate {
    buffer:      AudioBuffer<f32>,
    sample_rate: f64, // default = 0.0
}

impl BufferWithSampleRate {

    pub fn new(
        buffer_in:      AudioBuffer<f32>,
        sample_rate_in: f64) -> Self {
    
        todo!();
        /*
        : buffer(std::move (bufferIn)),
        : sample_rate(sampleRateIn),

        
        */
    }
}

pub fn load_stream_to_buffer<R: Read>(stream: &R, max_length: usize) -> BufferWithSampleRate {
    
    todo!();
    /*
        AudioFormatManager manager;
        manager.registerBasicFormats();
        std::unique_ptr<AudioFormatReader> formatReader (manager.createReaderFor (std::move (stream)));

        if (formatReader == nullptr)
            return {};

        const auto fileLength = static_cast<size_t> (formatReader->lengthInSamples);
        const auto lengthToLoad = maxLength == 0 ? fileLength : jmin (maxLength, fileLength);

        BufferWithSampleRate result { { jlimit (1, 2, static_cast<int> (formatReader->numChannels)),
                                        static_cast<int> (lengthToLoad) },
                                      formatReader->sampleRate };

        formatReader->read (result.buffer.getArrayOfWritePointers(),
                            result.buffer.getNumChannels(),
                            0,
                            result.buffer.getNumSamples());

        return result;
    */
}
