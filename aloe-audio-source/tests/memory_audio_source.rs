crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
impl PartialEq<AudioBuffer<f32>> for AudioBuffer<f32> {
    
    #[inline] fn eq(&self, other: &AudioBuffer<f32>) -> bool {
        todo!();
        /*
            if (a.getNumChannels() != b.getNumChannels())
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
impl Eq for AudioBuffer<f32> {}

///-----------------------
#[cfg(ALOE_UNIT_TESTS)]
pub struct MemoryAudioSourceTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for MemoryAudioSourceTests {
    
    fn default() -> Self {
        todo!();
        /*
        : unit_test("MemoryAudioSource", UnitTestCategories::audio),

        
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl MemoryAudioSourceTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            constexpr int blockSize = 512;
            AudioBuffer<float> bufferToFill { 2, blockSize };
            AudioSourceChannelInfo channelInfo { bufferToFill };

            beginTest ("A zero-length buffer produces silence, whether or not looping is enabled");
            {
                for (const bool enableLooping : { false, true })
                {
                    AudioBuffer<float> buffer;
                    MemoryAudioSource source { buffer, true, false };
                    source.setLooping (enableLooping);
                    source.prepareToPlay (blockSize, 44100.0);

                    for (int i = 0; i < 2; ++i)
                    {
                        play (source, channelInfo);
                        expect (isSilent (bufferToFill));
                    }
                }
            }

            beginTest ("A short buffer without looping is played once and followed by silence");
            {
                auto buffer = getShortBuffer();
                MemoryAudioSource source { buffer, true, false };
                source.setLooping (false);
                source.prepareToPlay (blockSize, 44100.0);

                play (source, channelInfo);

                auto copy = buffer;
                copy.setSize (buffer.getNumChannels(), blockSize, true, true, false);

                expect (bufferToFill == copy);

                play (source, channelInfo);

                expect (isSilent (bufferToFill));
            }

            beginTest ("A short buffer with looping is played multiple times");
            {
                auto buffer = getShortBuffer();
                MemoryAudioSource source { buffer, true, false };
                source.setLooping (true);
                source.prepareToPlay (blockSize, 44100.0);

                play (source, channelInfo);

                for (int sample = 0; sample < buffer.getNumSamples(); ++sample)
                    expect (bufferToFill.getSample (0, sample + buffer.getNumSamples()) == buffer.getSample (0, sample));

                expect (! isSilent (bufferToFill));
            }

            beginTest ("A long buffer without looping is played once");
            {
                auto buffer = getLongBuffer();
                MemoryAudioSource source { buffer, true, false };
                source.setLooping (false);
                source.prepareToPlay (blockSize, 44100.0);

                play (source, channelInfo);

                auto copy = buffer;
                copy.setSize (buffer.getNumChannels(), blockSize, true, true, false);

                expect (bufferToFill == copy);

                for (int i = 0; i < 10; ++i)
                    play (source, channelInfo);

                expect (isSilent (bufferToFill));
            }

            beginTest ("A long buffer with looping is played multiple times");
            {
                auto buffer = getLongBuffer();
                MemoryAudioSource source { buffer, true, false };
                source.setLooping (true);
                source.prepareToPlay (blockSize, 44100.0);

                for (int i = 0; i < 100; ++i)
                {
                    play (source, channelInfo);
                    expect (bufferToFill.getSample (0, 0) == buffer.getSample (0, (i * blockSize) % buffer.getNumSamples()));
                }
            }
        */
    }
    
    pub fn get_test_buffer(length: i32) -> AudioBuffer<f32> {
        
        todo!();
        /*
            AudioBuffer<float> buffer { 2, length };

            for (int channel = 0; channel < buffer.getNumChannels(); ++channel)
                for (int sample = 0; sample < buffer.getNumSamples(); ++sample)
                    buffer.setSample (channel, sample, jmap ((float) sample, 0.0f, (float) length, -1.0f, 1.0f));

            return buffer;
        */
    }
    
    pub fn get_short_buffer() -> AudioBuffer<f32> {
        
        todo!();
        /*
            return getTestBuffer (5);
        */
    }
    
    pub fn get_long_buffer() -> AudioBuffer<f32> {
        
        todo!();
        /*
            return getTestBuffer (1000);
        */
    }
    
    pub fn play(
        source: &mut MemoryAudioSource,
        info:   &mut AudioSourceChannelInfo)  {
        
        todo!();
        /*
            info.clearActiveBufferRegion();
            source.getNextAudioBlock (info);
        */
    }
    
    pub fn is_silent(b: &AudioBuffer<f32>) -> bool {
        
        todo!();
        /*
            for (int channel = 0; channel < b.getNumChannels(); ++channel)
                if (b.findMinMax (channel, 0, b.getNumSamples()) != Range<float>{})
                    return false;

            return true;
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static MemoryAudioSourceTests memoryAudioSourceTests;
    */
}

