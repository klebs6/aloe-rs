crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_BallisticsFilter.h]

pub enum BallisticsFilterLevelCalculationType
{
    peak,
    RMS
}

/**
  | A processor to apply standard attack
  | / release ballistics to an input signal.
  | 
  | This is useful in dynamics processors,
  | envelope followers, modulated audio
  | effects and for smoothing animation
  | in data visualisation.
  | 
  | @tags{DSP}
  |
  */
pub struct BallisticsFilter<SampleType: SampleTypeInterface> {
    yold:         Vec<SampleType>,
    sample_rate:  f64, // default = 44100.0
    exp_factor:   f64, // default = -0.142
    attack_time:  SampleType, // default = 1.0
    release_time: SampleType, // default = 100.0
    cteat:        SampleType, // default = 0.0
    cterl:        SampleType, // default = 0.0
    level_type:   BallisticsFilterLevelCalculationType, // default = BallisticsFilterLevelCalculationType::peak
}

impl<SampleType: SampleTypeInterface> Default for BallisticsFilter<SampleType> {
    
    fn default() -> Self {

        todo!();

        /*
        setAttackTime (attackTime);
        setReleaseTime (releaseTime)
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_BallisticsFilter.cpp]
impl<SampleType: SampleTypeInterface> BallisticsFilter<SampleType> {

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

            jassert (inputBlock.getNumChannels() <= yold.size());
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

           #if ALOE_DSP_ENABLE_SNAP_TO_ZERO
            snapToZero();
           #endif
        */
    }

    /**
      | Sets the attack time in ms.
      | 
      | Attack times less than 0.001 ms will
      | be snapped to zero and very long attack
      | times will eventually saturate depending
      | on the numerical precision used.
      |
      */
    pub fn set_attack_time(&mut self, attack_time_ms: SampleType)  {
        
        todo!();
        /*
            attackTime = attackTimeMs;
        cteAT = calculateLimitedCte (static_cast<SampleType> (attackTime));
        */
    }
    
    /**
      | Sets the release time in ms.
      | 
      | Release times less than 0.001 ms will
      | be snapped to zero and very long release
      | times will eventually saturate depending
      | on the numerical precision used.
      |
      */
    pub fn set_release_time(&mut self, release_time_ms: SampleType)  {
        
        todo!();
        /*
            releaseTime = releaseTimeMs;
        cteRL = calculateLimitedCte (static_cast<SampleType> (releaseTime));
        */
    }
    
    /**
      | Sets how the filter levels are calculated.
      | 
      | Level calculation in digital envelope
      | followers is usually performed using
      | peak detection with a rectifier function
      | (like std::abs) and filtering, which
      | returns an envelope dependant on the
      | peak or maximum values of the signal
      | amplitude.
      | 
      | To perform an estimation of the average
      | value of the signal you can use an RMS
      | (root mean squared) implementation
      | of the ballistics filter instead. This
      | is useful in some compressor and noise-gate
      | designs, or in specific types of volume
      | meters.
      |
      */
    pub fn set_level_calculation_type(&mut self, new_level_type: BallisticsFilterLevelCalculationType)  {
        
        todo!();
        /*
            levelType = newLevelType;
        reset();
        */
    }
    
    /**
      | Initialises the filter.
      |
      */
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            jassert (spec.sampleRate > 0);
        jassert (spec.numChannels > 0);

        sampleRate = spec.sampleRate;
        expFactor  = -2.0 * MathConstants<double>::pi * 1000.0 / sampleRate;

        setAttackTime  (attackTime);
        setReleaseTime (releaseTime);

        yold.resize (spec.numChannels);

        reset();
        */
    }
    
    /**
      | Resets the internal state variables
      | of the filter.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            reset (0);
        */
    }
    
    /**
      | Resets the internal state variables
      | of the filter to the given initial value.
      |
      */
    pub fn reset_with_initial_value(&mut self, initial_value: SampleType)  {
        
        todo!();
        /*
            for (auto& old : yold)
            old = initialValue;
        */
    }
    
    /**
      | Processes one sample at a time on a given
      | channel.
      |
      */
    pub fn process_sample(
        &mut self, 
        channel:     i32,
        input_value: SampleType

    ) -> SampleType {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channel, yold.size()));

        if (levelType == BallisticsFilterLevelCalculationType::RMS)
            inputValue *= inputValue;
        else
            inputValue = std::abs (inputValue);

        SampleType cte = (inputValue > yold[(size_t) channel] ? cteAT : cteRL);

        SampleType result = inputValue + cte * (yold[(size_t) channel] - inputValue);
        yold[(size_t) channel] = result;

        if (levelType == BallisticsFilterLevelCalculationType::RMS)
            return std::sqrt (result);

        return result;
        */
    }
    
    /**
      | Ensure that the state variables are
      | rounded to zero if the state variables
      | are denormals. This is only needed if
      | you are doing sample by sample processing.
      |
      */
    pub fn snap_to_zero(&mut self)  {
        
        todo!();
        /*
            for (auto& old : yold)
            util::snapToZero (old);
        */
    }
    
    pub fn calculate_limited_cte(&self, time_ms: SampleType) -> SampleType {
        
        todo!();
        /*
            return timeMs < static_cast<SampleType> (1.0e-3) ? 0
                                                         : static_cast<SampleType> (std::exp (expFactor / timeMs));
        */
    }
}
