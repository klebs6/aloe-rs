crate::ix!();

pub struct MultiBandProcessor<FloatType: num::Float> {
    lowpass:           LinkwitzRileyFilter<FloatType>,
    highpass:          LinkwitzRileyFilter<FloatType>,
    low_volume:        Gain<FloatType>,
    high_volume:       Gain<FloatType>,
    buffer_separation: AudioBuffer<f32>,
}

impl<FloatType: num::Float> Default for MultiBandProcessor<FloatType> {
    
    fn default() -> Self {
        todo!();
        /*


            forEach ([] (Gain<FloatType>& gain) { gain.setRampDurationSeconds (0.05); },
                         lowVolume,
                         highVolume);

                lowpass .setType (LinkwitzRileyFilterType::lowpass);
                highpass.setType (LinkwitzRileyFilterType::highpass)
        */
    }
}

impl<FloatType: num::Float> MultiBandProcessor<FloatType> {
    
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            prepareAll (spec, lowpass, highpass, lowVolume, highVolume);
                bufferSeparation.setSize (4, int (spec.maximumBlockSize), false, false, true);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            resetAll (lowpass, highpass, lowVolume, highVolume);
        */
    }
    
    pub fn process<Context>(&mut self, context: &mut Context)  {
    
        todo!();
        /*
            const auto& inputBlock = context.getInputBlock();

                const auto numSamples  = inputBlock.getNumSamples();
                const auto numChannels = inputBlock.getNumChannels();

                auto sepBlock = AudioBlock<FloatType> (bufferSeparation).getSubBlock (0, (size_t) numSamples);

                auto sepLowBlock  = sepBlock.getSubsetChannelBlock (0, (size_t) numChannels);
                auto sepHighBlock = sepBlock.getSubsetChannelBlock (2, (size_t) numChannels);

                sepLowBlock .copyFrom (inputBlock);
                sepHighBlock.copyFrom (inputBlock);

                auto contextLow = ProcessContextReplacing<FloatType> (sepLowBlock);
                contextLow.isBypassed = context.isBypassed;
                lowpass  .process (contextLow);
                lowVolume.process (contextLow);

                auto contextHigh = ProcessContextReplacing<FloatType> (sepHighBlock);
                contextHigh.isBypassed = context.isBypassed;
                highpass  .process (contextHigh);
                highVolume.process (contextHigh);

                if (! context.isBypassed)
                {
                    sepLowBlock.add (sepHighBlock);
                    context.getOutputBlock().copyFrom (sepLowBlock);
                }
        */
    }
}
