crate::ix!();

///--------------------------
pub struct CrossoverMixer {
    smoother:        LinearSmoothedValue<f32>,
    smoother_buffer: AudioBuffer<f32>,
    mix_buffer:      AudioBuffer<f32>,
}

impl CrossoverMixer {

    pub fn reset(&mut self)  {
        
        todo!();
        /*
            smoother.setCurrentAndTargetValue (1.0f);
        */
    }
    
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            smoother.reset (spec.sampleRate, 0.05);
            smootherBuffer.setSize (1, static_cast<int> (spec.maximumBlockSize));
            mixBuffer.setSize (static_cast<int> (spec.numChannels), static_cast<int> (spec.maximumBlockSize));
            reset();
        */
    }
    
    pub fn process_samples<ProcessCurrent, ProcessPrevious, NotifyDone>(&mut self, 
        input:       &AudioBlock<f32>,
        output:      &mut AudioBlock<f32>,
        current:     ProcessCurrent,
        previous:    ProcessPrevious,
        notify_done: NotifyDone)  {
    
        todo!();
        /*
            if (smoother.isSmoothing())
            {
                const auto numSamples = static_cast<int> (input.getNumSamples());

                for (auto sample = 0; sample != numSamples; ++sample)
                    smootherBuffer.setSample (0, sample, smoother.getNextValue());

                AudioBlock<float> mixBlock (mixBuffer);
                mixBlock.clear();
                previous (input, mixBlock);

                for (size_t channel = 0; channel != output.getNumChannels(); ++channel)
                {
                    FloatVectorOperations::multiply (mixBlock.getChannelPointer (channel),
                                                     smootherBuffer.getReadPointer (0),
                                                     numSamples);
                }

                FloatVectorOperations::multiply (smootherBuffer.getWritePointer (0), -1.0f, numSamples);
                FloatVectorOperations::add (smootherBuffer.getWritePointer (0), 1.0f, numSamples);

                current (input, output);

                for (size_t channel = 0; channel != output.getNumChannels(); ++channel)
                {
                    FloatVectorOperations::multiply (output.getChannelPointer (channel),
                                                     smootherBuffer.getReadPointer (0),
                                                     numSamples);
                    FloatVectorOperations::add (output.getChannelPointer (channel),
                                                mixBlock.getChannelPointer (channel),
                                                numSamples);
                }

                if (! smoother.isSmoothing())
                    notifyDone();
            }
            else
            {
                current (input, output);
            }
        */
    }
    
    pub fn begin_transition(&mut self)  {
        
        todo!();
        /*
            smoother.setCurrentAndTargetValue (1.0f);
            smoother.setTargetValue (0.0f);
        */
    }
}
