crate::ix!();

pub struct ConvolutionMixer {
    volume_dry:          [SmoothedValue<f32>; 2],
    volume_wet:          [SmoothedValue<f32>; 2],
    dry_block:           AudioBlock<f32>,
    dry_block_storage:   HeapBlock<u8>,
    sample_rate:         f64, // default = 0
    current_is_bypassed: bool, // default = false
}

impl ConvolutionMixer {

    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            for (auto& dry : volumeDry)
            dry.reset (spec.sampleRate, 0.05);

        for (auto& wet : volumeWet)
            wet.reset (spec.sampleRate, 0.05);

        sampleRate = spec.sampleRate;

        dryBlock = AudioBlock<float> (dryBlockStorage,
                                      jmin (spec.numChannels, 2u),
                                      spec.maximumBlockSize);
        */
    }
    
    pub fn process_samples<ProcessWet>(&mut self, 
        input:       &AudioBlock<f32>,
        output:      &mut AudioBlock<f32>,
        is_bypassed: bool,
        process_wet: ProcessWet)  {
    
        todo!();
        /*
            const auto numChannels = jmin (input.getNumChannels(), volumeDry.size());
        const auto numSamples  = jmin (input.getNumSamples(), output.getNumSamples());

        auto dry = dryBlock.getSubsetChannelBlock (0, numChannels);

        if (volumeDry[0].isSmoothing())
        {
            dry.copyFrom (input);

            for (size_t channel = 0; channel < numChannels; ++channel)
                volumeDry[channel].applyGain (dry.getChannelPointer (channel), (int) numSamples);

            processWet (input, output);

            for (size_t channel = 0; channel < numChannels; ++channel)
                volumeWet[channel].applyGain (output.getChannelPointer (channel), (int) numSamples);

            output += dry;
        }
        else
        {
            if (! currentIsBypassed)
                processWet (input, output);

            if (isBypassed != currentIsBypassed)
            {
                currentIsBypassed = isBypassed;

                for (size_t channel = 0; channel < numChannels; ++channel)
                {
                    volumeDry[channel].setTargetValue (isBypassed ? 0.0f : 1.0f);
                    volumeDry[channel].reset (sampleRate, 0.05);
                    volumeDry[channel].setTargetValue (isBypassed ? 1.0f : 0.0f);

                    volumeWet[channel].setTargetValue (isBypassed ? 1.0f : 0.0f);
                    volumeWet[channel].reset (sampleRate, 0.05);
                    volumeWet[channel].setTargetValue (isBypassed ? 0.0f : 1.0f);
                }
            }
        }
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            dryBlock.clear();
        */
    }
}
