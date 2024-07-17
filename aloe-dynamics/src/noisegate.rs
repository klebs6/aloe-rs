crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_NoiseGate.h]

/**
  | A simple noise gate with standard threshold,
  | ratio, attack time and release time
  | controls. Can be used as an expander
  | if the ratio is low.
  | 
  | @tags{DSP}
  |
  */
pub struct NoiseGate<SampleType: SampleTypeInterface> {
    threshold:         SampleType,
    threshold_inverse: SampleType,
    current_ratio:     SampleType,
    envelope_filter:   BallisticsFilter<SampleType>,
    rms_filter:        BallisticsFilter<SampleType>,
    sample_rate:       f64, // default = 44100.0
    thresholddb:       SampleType, // default = -100
    ratio:             SampleType, // default = 10.0
    attack_time:       SampleType, // default = 1.0
    release_time:      SampleType, // default = 100.0
}

impl<SampleType: SampleTypeInterface> Default for NoiseGate<SampleType> {
    
    fn default() -> Self {
        todo!();
        /*
        update();

        RMSFilter.setLevelCalculationType (BallisticsFilterLevelCalculationType::RMS);
        RMSFilter.setAttackTime  (static_cast<SampleType> (0.0));
        RMSFilter.setReleaseTime (static_cast<SampleType> (50.0))
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_NoiseGate.cpp]

impl<SampleType: SampleTypeInterface> NoiseGate<SampleType> {
    
    /**
      | Processes the input and output samples
      | supplied in the processing context.
      |
      */
    pub fn process<ProcessContext>(&mut self, context: &ProcessContext)  {
    
        todo!();
        /*
            const auto& inputBlock = context.getInputBlock();
            auto& outputBlock      = context.getOutputBlock();
            const auto numChannels = outputBlock.getNumChannels();
            const auto numSamples  = outputBlock.getNumSamples();

            jassert (inputBlock.getNumChannels() == numChannels);
            jassert (inputBlock.getNumSamples() == numSamples);

            if (context.isBypassed)
            {
                outputBlock.copyFrom (inputBlock);
                return;
            }

            for (size_t channel = 0; channel < numChannels; ++channel)
            {
                auto* inputSamples  = inputBlock .getChannelPointer (channel);
                auto* outputSamples = outputBlock.getChannelPointer (channel);

                for (size_t i = 0; i < numSamples; ++i)
                    outputSamples[i] = processSample ((int) channel, inputSamples[i]);
            }
        */
    }

    /**
      | Sets the threshold in dB of the noise-gate.
      |
      */
    pub fn set_threshold(&mut self, new_value: SampleType)  {
        
        todo!();
        /*
            thresholddB = newValue;
        update();
        */
    }
    
    /**
      | Sets the ratio of the noise-gate (must
      | be higher or equal to 1).
      |
      */
    pub fn set_ratio(&mut self, new_ratio: SampleType)  {
        
        todo!();
        /*
            jassert (newRatio >= static_cast<SampleType> (1.0));

        ratio = newRatio;
        update();
        */
    }
    
    /**
      | Sets the attack time in milliseconds
      | of the noise-gate.
      |
      */
    pub fn set_attack(&mut self, new_attack: SampleType)  {
        
        todo!();
        /*
            attackTime = newAttack;
        update();
        */
    }
    
    /**
      | Sets the release time in milliseconds
      | of the noise-gate.
      |
      */
    pub fn set_release(&mut self, new_release: SampleType)  {
        
        todo!();
        /*
            releaseTime = newRelease;
        update();
        */
    }
    
    /**
      | Initialises the processor.
      |
      */
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            jassert (spec.sampleRate > 0);
        jassert (spec.numChannels > 0);

        sampleRate = spec.sampleRate;

        RMSFilter.prepare (spec);
        envelopeFilter.prepare (spec);

        update();
        reset();
        */
    }
    
    /**
      | Resets the internal state variables
      | of the processor.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            RMSFilter.reset();
        envelopeFilter.reset();
        */
    }
    
    /**
      | Performs the processing operation
      | on a single sample at a time.
      |
      */
    pub fn process_sample(&mut self, 
        channel: i32,
        sample:  SampleType) -> SampleType {
        
        todo!();
        /*
            // RMS ballistics filter
        auto env = RMSFilter.processSample (channel, sample);

        // Ballistics filter
        env = envelopeFilter.processSample (channel, env);

        // VCA
        auto gain = (env > threshold) ? static_cast<SampleType> (1.0)
                                      : std::pow (env * thresholdInverse, currentRatio - static_cast<SampleType> (1.0));

        // Output
        return gain * sample;
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
            threshold = Decibels::decibelsToGain (thresholddB, static_cast<SampleType> (-200.0));
        thresholdInverse = static_cast<SampleType> (1.0) / threshold;
        currentRatio = ratio;

        envelopeFilter.setAttackTime  (attackTime);
        envelopeFilter.setReleaseTime (releaseTime);
        */
    }
}
