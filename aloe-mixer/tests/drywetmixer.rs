crate::ix!();

#[test] fn dry_wet_mixer_tests() {

    todo!();

    /*
    pub struct DryWetMixerTests {
        base: UnitTest,
    }

    use super::*;
        crate::ix!();
        pub enum Kind { down, up }
    }

    impl Default for DryWetMixerTests {
        
        fn default() -> Self {
            todo!();
            /*
            : unit_test("DryWetMixer", UnitTestCategories::dsp),

            
            */
        }
    }

    impl DryWetMixerTests {
        
        pub fn get_ramp_buffer(
            spec: ProcessSpec,
            kind: Kind) -> Auto {
            
            todo!();
            /*
                AudioBuffer<float> buffer ((int) spec.numChannels, (int) spec.maximumBlockSize);

                    for (uint32_t sample = 0; sample < spec.maximumBlockSize; ++sample)
                    {
                        for (uint32_t channel = 0; channel < spec.numChannels; ++channel)
                        {
                            const auto ramp = kind == Kind::up ? sample : spec.maximumBlockSize - sample;

                            buffer.setSample ((int) channel,
                                              (int) sample,
                                              jmap ((float) ramp, 0.0f, (float) spec.maximumBlockSize, 0.0f, 1.0f));
                        }
                    }

                    return buffer;
            */
        }
        
        pub fn run_test(&mut self)  {
            
            todo!();
            /*
                constexpr ProcessSpec spec { 44100.0, 512, 2 };
                    constexpr auto numBlocks = 5;

                    const auto wetBuffer = getRampBuffer (spec, Kind::up);
                    const auto dryBuffer = getRampBuffer (spec, Kind::down);

                    for (auto maxLatency : { 0, 512 })
                    {
                        beginTest ("Mixer can push multiple small buffers");
                        {
                            DryWetMixer<float> mixer (maxLatency);
                            mixer.setWetMixProportion (0.5f);
                            mixer.prepare (spec);

                            for (auto block = 0; block < numBlocks; ++block)
                            {
                                // Push samples one-by-one
                                for (uint32_t sample = 0; sample < spec.maximumBlockSize; ++sample)
                                    mixer.pushDrySamples (AudioBlock<const float> (dryBuffer).getSubBlock (sample, 1));

                                // Mix wet samples in one go
                                auto outputBlock = wetBuffer;
                                mixer.mixWetSamples ({ outputBlock });

                                // The output block should contain the wet and dry samples averaged
                                for (uint32_t sample = 0; sample < spec.maximumBlockSize; ++sample)
                                {
                                    for (uint32_t channel = 0; channel < spec.numChannels; ++channel)
                                    {
                                        const auto outputValue = outputBlock.getSample ((int) channel, (int) sample);
                                        expectWithinAbsoluteError (outputValue, 0.5f, 0.0001f);
                                    }
                                }
                            }
                        }

                        beginTest ("Mixer can pop multiple small buffers");
                        {
                            DryWetMixer<float> mixer (maxLatency);
                            mixer.setWetMixProportion (0.5f);
                            mixer.prepare (spec);

                            for (auto block = 0; block < numBlocks; ++block)
                            {
                                // Push samples in one go
                                mixer.pushDrySamples ({ dryBuffer });

                                // Process wet samples one-by-one
                                for (uint32_t sample = 0; sample < spec.maximumBlockSize; ++sample)
                                {
                                    AudioBuffer<float> outputBlock ((int) spec.numChannels, 1);
                                    AudioBlock<const float> (wetBuffer).getSubBlock (sample, 1).copyTo (outputBlock);
                                    mixer.mixWetSamples ({ outputBlock });

                                    // The output block should contain the wet and dry samples averaged
                                    for (uint32_t channel = 0; channel < spec.numChannels; ++channel)
                                    {
                                        const auto outputValue = outputBlock.getSample ((int) channel, 0);
                                        expectWithinAbsoluteError (outputValue, 0.5f, 0.0001f);
                                    }
                                }
                            }
                        }

                        beginTest ("Mixer can push and pop multiple small buffers");
                        {
                            DryWetMixer<float> mixer (maxLatency);
                            mixer.setWetMixProportion (0.5f);
                            mixer.prepare (spec);

                            for (auto block = 0; block < numBlocks; ++block)
                            {
                                // Push dry samples and process wet samples one-by-one
                                for (uint32_t sample = 0; sample < spec.maximumBlockSize; ++sample)
                                {
                                    mixer.pushDrySamples (AudioBlock<const float> (dryBuffer).getSubBlock (sample, 1));

                                    AudioBuffer<float> outputBlock ((int) spec.numChannels, 1);
                                    AudioBlock<const float> (wetBuffer).getSubBlock (sample, 1).copyTo (outputBlock);
                                    mixer.mixWetSamples ({ outputBlock });

                                    // The output block should contain the wet and dry samples averaged
                                    for (uint32_t channel = 0; channel < spec.numChannels; ++channel)
                                    {
                                        const auto outputValue = outputBlock.getSample ((int) channel, 0);
                                        expectWithinAbsoluteError (outputValue, 0.5f, 0.0001f);
                                    }
                                }
                            }
                        }

                        beginTest ("Mixer can push and pop full-sized blocks after encountering a shorter block");
                        {
                            DryWetMixer<float> mixer (maxLatency);
                            mixer.setWetMixProportion (0.5f);
                            mixer.prepare (spec);

                            constexpr auto shortBlockLength = spec.maximumBlockSize / 2;
                            AudioBuffer<float> shortBlock (spec.numChannels, shortBlockLength);
                            mixer.pushDrySamples (AudioBlock<const float> (dryBuffer).getSubBlock (shortBlockLength));
                            mixer.mixWetSamples ({ shortBlock });

                            for (auto block = 0; block < numBlocks; ++block)
                            {
                                // Push a full block of dry samples
                                mixer.pushDrySamples ({ dryBuffer });

                                // Mix a full block of wet samples
                                auto outputBlock = wetBuffer;
                                mixer.mixWetSamples ({ outputBlock });

                                // The output block should contain the wet and dry samples averaged
                                for (uint32_t sample = 0; sample < spec.maximumBlockSize; ++sample)
                                {
                                    for (uint32_t channel = 0; channel < spec.numChannels; ++channel)
                                    {
                                        const auto outputValue = outputBlock.getSample ((int) channel, (int) sample);
                                        expectWithinAbsoluteError (outputValue, 0.5f, 0.0001f);
                                    }
                                }
                            }
                        }
                    }
            */
        }
    }

    lazy_static!{
        /*
        static const DryWetMixerTests dryWetMixerTests;
        */
    }
    */
}
