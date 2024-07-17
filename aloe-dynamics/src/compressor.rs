crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_Compressor.h]

/**
  | A simple compressor with standard threshold,
  | ratio, attack time and release time
  | controls.
  | 
  | @tags{DSP}
  |
  */
pub struct Compressor<SampleType: SampleTypeInterface> {
    threshold:         SampleType,
    threshold_inverse: SampleType,
    ratio_inverse:     SampleType,
    envelope_filter:   BallisticsFilter<SampleType>,
    sample_rate:       f64, // default = 44100.0
    thresholddb:       SampleType, // default = 0.0
    ratio:             SampleType, // default = 1.0
    attack_time:       SampleType, // default = 1.0
    release_time:      SampleType, // default = 100.0
}

impl<SampleType: SampleTypeInterface> Default for Compressor<SampleType> {
    
    fn default() -> Self {

        todo!();

        /*
            update()
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_Compressor.cpp]
impl<SampleType: SampleTypeInterface> Compressor<SampleType> {

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
            jassert (inputBlock.getNumSamples()  == numSamples);

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
      | Sets the threshold in dB of the compressor.
      |
      */
    pub fn set_threshold(&mut self, new_threshold: SampleType)  {
        
        todo!();
        /*
            thresholddB = newThreshold;
        update();
        */
    }
    
    /**
      | Sets the ratio of the compressor (must
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
      | of the compressor.
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
      | of the compressor.
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
            envelopeFilter.reset();
        */
    }
    
    /**
      | Performs the processing operation
      | on a single sample at a time.
      |
      */
    pub fn process_sample(&mut self, 
        channel:     i32,
        input_value: SampleType) -> SampleType {
        
        todo!();
        /*
        // Ballistics filter with peak rectifier
        auto env = envelopeFilter.processSample (channel, inputValue);

        // VCA
        auto gain = (env < threshold) ? static_cast<SampleType> (1.0)
                                      : std::pow (env * thresholdInverse, ratioInverse - static_cast<SampleType> (1.0));

        // Output
        return gain * inputValue;
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
        threshold        = Decibels::decibelsToGain (thresholddB, static_cast<SampleType> (-200.0));
        thresholdInverse = static_cast<SampleType> (1.0) / threshold;
        ratioInverse     = static_cast<SampleType> (1.0) / ratio;

        envelopeFilter.setAttackTime (attackTime);
        envelopeFilter.setReleaseTime (releaseTime);
        */
    }
}
