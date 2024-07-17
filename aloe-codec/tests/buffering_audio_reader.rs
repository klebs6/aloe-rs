crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
impl PartialEq<AudioBuffer<f32>> for AudioBuffer {
    
    #[inline] fn eq(&self, other: &AudioBuffer<f32>) -> bool {
        todo!();
        /*
            if (a.getNumChannels() != b.getNumChannels() || a.getNumSamples() != b.getNumSamples())
            return false;

        for (int channel = 0; channel < a.getNumChannels(); ++channel)
        {
            auto* aPtr = a.getReadPointer (channel);
            auto* bPtr = b.getReadPointer (channel);

            if (std::vector<float> (aPtr, aPtr + a.getNumSamples())
                != std::vector<float> (bPtr, bPtr + b.getNumSamples()))
            {
                return false;
            }
        }

        return true;
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
pub fn is_silent(b: &AudioBuffer<f32>) -> bool {
    
    todo!();
        /*
            for (int channel = 0; channel < b.getNumChannels(); ++channel)
            if (b.findMinMax (channel, 0, b.getNumSamples()) != Range<float>{})
                return false;

        return true;
        */
}

///--------------------
#[cfg(ALOE_UNIT_TESTS)]
pub struct TestAudioFormatReader {
    base:   AudioFormatReader,
    buffer: &AudioBuffer<f32>,
}

#[cfg(ALOE_UNIT_TESTS)]
impl TestAudioFormatReader {

    pub fn new(b: &mut AudioBuffer<f32>) -> Self {
    
        todo!();
        /*

            : AudioFormatReader (nullptr, {}),
              buffer (b)

            sampleRate            = 44100.0f;
            bitsPerSample         = 32;
            usesFloatingPointData = true;
            lengthInSamples       = buffer.getNumSamples();
            numChannels           = (unsigned int) buffer.getNumChannels();
        */
    }
    
    pub fn read_samples(&mut self, 
        dest_channels:               *mut *mut i32,
        num_dest_channels:           i32,
        start_offset_in_dest_buffer: i32,
        start_sample_in_file:        i64,
        num_samples:                 i32) -> bool {
        
        todo!();
        /*
            clearSamplesBeyondAvailableLength (destChannels, numDestChannels, startOffsetInDestBuffer,
                                               startSampleInFile, numSamples, lengthInSamples);

            for (int j = 0; j < numDestChannels; ++j)
            {
                static_assert (sizeof (int) == sizeof (float),
                               "Int and float size must match in order for pointer arithmetic to work correctly");

                if (auto* dest = reinterpret_cast<float*> (destChannels[j]))
                {
                    dest += startOffsetInDestBuffer;

                    if (j < (int) numChannels)
                        FloatVectorOperations::copy (dest, buffer.getReadPointer (j, (int) startSampleInFile), numSamples);
                    else
                        FloatVectorOperations::clear (dest, numSamples);
                }
            }

            return true;
        */
    }
}

///----------------------
#[cfg(ALOE_UNIT_TESTS)]
pub struct BufferingAudioReaderTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for BufferingAudioReaderTests {
    
    fn default() -> Self {
        todo!();
        /*

            : UnitTest ("BufferingAudioReader", UnitTestCategories::audio)
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl BufferingAudioReaderTests {
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            TimeSliceThread timeSlice ("TestBackgroundThread");
            timeSlice.startThread (5);

            beginTest ("Timeout");
            {
                struct BlockingReader  : public AudioFormatReader
                {
                    BlockingReader()
                        : AudioFormatReader (nullptr, {})
                    {
                        sampleRate            = 44100.0f;
                        bitsPerSample         = 32;
                        usesFloatingPointData = true;
                        lengthInSamples       = 1024;
                        numChannels           = 2;
                    }

                    bool readSamples (int**, int, int, int64, int) override
                    {
                        Thread::sleep (100);
                        return true;
                    }
                };

                BufferingAudioReader bufferingReader (new BlockingReader(), timeSlice, 64);
                bufferingReader.setReadTimeout (10);

                AudioBuffer<float> readBuffer { 2, 1024 };
                read (bufferingReader, readBuffer);

                expect (isSilent (readBuffer));
            }

            beginTest ("Read samples");
            {
                for (auto i = 4; i < 18; ++i)
                {
                    const auto backgroundBufferSize = 1 << i;
                    auto buffer = generateTestBuffer (backgroundBufferSize);

                    BufferingAudioReader bufferingReader (new TestAudioFormatReader (buffer), timeSlice, backgroundBufferSize);
                    bufferingReader.setReadTimeout (-1);

                    AudioBuffer<float> readBuffer { buffer.getNumChannels(), buffer.getNumSamples() };
                    read (bufferingReader, readBuffer);

                    expect (buffer == readBuffer);
                }
            }
        */
    }
    
    pub fn generate_test_buffer(&self, buffer_size: i32) -> AudioBuffer<f32> {
        
        todo!();
        /*
            auto random = getRandom();

            AudioBuffer<float> buffer { 2, random.nextInt ({ bufferSize, bufferSize * 10 }) };

            for (int channel = 0; channel < buffer.getNumChannels(); ++channel)
                for (int sample = 0; sample < buffer.getNumSamples(); ++sample)
                    buffer.setSample (channel, sample, random.nextFloat());

            return buffer;
        */
    }
    
    pub fn read(&mut self, 
        reader:      &mut BufferingAudioReader,
        read_buffer: &mut AudioBuffer<f32>)  {
        
        todo!();
        /*
            constexpr int blockSize = 1024;

            const auto numSamples = readBuffer.getNumSamples();
            int readPos = 0;

            for (;;)
            {
                reader.read (&readBuffer, readPos, jmin (blockSize, numSamples - readPos), readPos, true, true);

                readPos += blockSize;

                if (readPos >= numSamples)
                    break;
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static BufferingAudioReaderTests bufferingAudioReaderTests;
    */
}
