crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct AudioProcessorPlayerTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for AudioProcessorPlayerTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("AudioProcessorPlayer", UnitTestCategories::audio
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl AudioProcessorPlayerTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            struct Layout
            {
                int numIns, numOuts;
            };

            const Layout processorLayouts[] { Layout { 0, 0 },
                                              Layout { 1, 1 },
                                              Layout { 4, 4 },
                                              Layout { 4, 8 },
                                              Layout { 8, 4 } };

            beginTest ("Buffers are prepared correctly for a variety of channel layouts");
            {
                for (const auto& layout : processorLayouts)
                {
                    for (const auto numSystemInputs : { 0, 1, layout.numIns })
                    {
                        const int numSamples = 256;
                        const auto systemIns = getTestBuffer (numSystemInputs, numSamples);
                        auto systemOuts = getTestBuffer (layout.numOuts, numSamples);
                        AudioBuffer<float> tempBuffer (jmax (layout.numIns, layout.numOuts), numSamples);
                        std::vector<float*> channels ((size_t) jmax (layout.numIns, layout.numOuts), nullptr);

                        initialiseIoBuffers ({ systemIns.getArrayOfReadPointers(),   systemIns.getNumChannels() },
                                             { systemOuts.getArrayOfWritePointers(), systemOuts.getNumChannels() },
                                             numSamples,
                                             layout.numIns,
                                             layout.numOuts,
                                             tempBuffer,
                                             channels);

                        int channelIndex = 0;

                        for (const auto& channel : channels)
                        {
                            const auto value = [&]
                            {
                                // Any channels past the number of inputs should be silent.
                                if (layout.numIns <= channelIndex)
                                    return 0.0f;

                                // If there's no input, all input channels should be silent.
                                if (numSystemInputs == 0)       return 0.0f;

                                // If there's one input, all input channels should copy from that input.
                                if (numSystemInputs == 1)       return 1.0f;

                                // Otherwise, each processor input should match the corresponding system input.
                                return (float) (channelIndex + 1);
                            }();

                            expect (FloatVectorOperations::findMinAndMax (channel, numSamples) == Range<float> (value, value));

                            channelIndex += 1;
                        }
                    }
                }
            }
        */
    }
    
    pub fn get_test_buffer(
        num_channels: i32,
        num_samples:  i32) -> AudioBuffer<f32> {
        
        todo!();
        /*
            AudioBuffer<float> result (numChannels, numSamples);

            for (int i = 0; i < result.getNumChannels(); ++i)
                FloatVectorOperations::fill (result.getWritePointer (i), (float) i + 1, result.getNumSamples());

            return result;
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static AudioProcessorPlayerTests audioProcessorPlayerTests;
    */
}
