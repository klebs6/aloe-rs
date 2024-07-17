crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_StateVariableTPTFilter.h]

pub enum StateVariableTPTFilterType
{
    lowpass,
    bandpass,
    highpass
}

/**
  | An IIR filter that can perform low, band
  | and high-pass filtering on an audio
  | signal, with 12 dB of attenuation per
  | octave, using a TPT structure, designed
  | for fast modulation (see Vadim Zavalishin's
  | documentation about TPT structures
  | for more information). Its behaviour
  | is based on the analog state variable
  | filter circuit.
  | 
  | Note: The bandpass here is not the one
  | in the RBJ CookBook as its gain can be
  | higher than 0 dB. For the classic 0 dB
  | bandpass, we need to multiply the result
  | by R2.
  | 
  | Note 2: Using this class prevents some
  | loud audio artefacts commonly encountered
  | when changing the cutoff frequency
  | using other filter simulation structures
  | and IIR filter classes. However, this
  | class may still require additional
  | smoothing for cutoff frequency changes.
  | 
  | see IIRFilter, SmoothedValue
  | 
  | @tags{DSP}
  |
  */
pub struct StateVariableTPTFilter<SampleType: SampleTypeInterface> {
    g:                SampleType,
    h:                SampleType,
    r2:               SampleType,
    s1:               Vec<SampleType>, // default = 2 
    s2:               Vec<SampleType>, // default = 2 
    sample_rate:      f64,             // default = 44100.0
    filter_type:      StateVariableTPTFilterType,            // default = StateVariableTPTFilterType::lowpass
    cutoff_frequency: SampleType,      // = static_cast<SampleType> (1000.0);
    resonance:        SampleType,      // = static_cast<SampleType> (1.0 / std::sqrt (2.0));
}

impl<SampleType: SampleTypeInterface> Default for StateVariableTPTFilter<SampleType> {
    
    fn default() -> Self {
        todo!();
        /*
            update()
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_StateVariableTPTFilter.cpp]
impl<SampleType: SampleTypeInterface> StateVariableTPTFilter<SampleType> {

    /**
      | Returns the type of the filter.
      |
      */
    pub fn get_type(&self) -> StateVariableTPTFilterType {
        
        todo!();
        /*
            return filterType;
        */
    }

    /**
      | Returns the cutoff frequency of the
      | filter.
      |
      */
    pub fn get_cutoff_frequency(&self) -> SampleType {
        
        todo!();
        /*
            return cutoffFrequency;
        */
    }

    /**
      | Returns the resonance of the filter.
      |
      */
    pub fn get_resonance(&self) -> SampleType {
        
        todo!();
        /*
            return resonance;
        */
    }
    
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

            jassert (inputBlock.getNumChannels() <= s1.size());
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
      | Sets the filter type.
      |
      */
    pub fn set_type(&mut self, new_value: StateVariableTPTFilterType)  {
        
        todo!();
        /*
            filterType = newValue;
        */
    }
    
    /**
      | Sets the cutoff frequency of the filter.
      | 
      | -----------
      | @param newFrequencyHz
      | 
      | the new cutoff frequency in Hz.
      |
      */
    pub fn set_cutoff_frequency(&mut self, new_cutoff_frequency_hz: SampleType)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (newCutoffFrequencyHz, static_cast<SampleType> (sampleRate * 0.5)));

        cutoffFrequency = newCutoffFrequencyHz;
        update();
        */
    }
    
    /**
      | Sets the resonance of the filter.
      | 
      | -----------
      | @note
      | 
      | The bandwidth of the resonance increases
      | with the value of the parameter. To have
      | a standard 12 dB / octave filter, the
      | value must be set at 1 / sqrt(2).
      |
      */
    pub fn set_resonance(&mut self, new_resonance: SampleType)  {
        
        todo!();
        /*
            jassert (newResonance > static_cast<SampleType> (0));

        resonance = newResonance;
        update();
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

        s1.resize (spec.numChannels);
        s2.resize (spec.numChannels);

        reset();
        update();
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
            reset (static_cast<SampleType> (0));
        */
    }
    
    /**
      | Resets the internal state variables
      | of the filter to a given value.
      |
      */
    pub fn reset_with_new_value(&mut self, new_value: SampleType)  {
        
        todo!();
        /*
            for (auto v : { &s1, &s2 })
            std::fill (v->begin(), v->end(), newValue);
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
            for (auto v : { &s1, &s2 })
            for (auto& element : *v)
                util::snapToZero (element);
        */
    }
    
    /**
      | Processes one sample at a time on a given
      | channel.
      |
      */
    pub fn process_sample(&mut self, 
        channel:     i32,
        input_value: SampleType) -> SampleType {
        
        todo!();
        /*
            auto& ls1 = s1[(size_t) channel];
        auto& ls2 = s2[(size_t) channel];

        auto yHP = h * (inputValue - ls1 * (g + R2) - ls2);

        auto yBP = yHP * g + ls1;
        ls1      = yHP * g + yBP;

        auto yLP = yBP * g + ls2;
        ls2      = yBP * g + yLP;

        switch (filterType)
        {
            case StateVariableTPTFilterType::lowpass:   return yLP;
            case StateVariableTPTFilterType::bandpass:  return yBP;
            case StateVariableTPTFilterType::highpass:  return yHP;
            default:              return yLP;
        }
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
            g  = static_cast<SampleType> (std::tan (MathConstants<double>::pi * cutoffFrequency / sampleRate));
        R2 = static_cast<SampleType> (1.0 / resonance);
        h  = static_cast<SampleType> (1.0 / (1.0 + R2 * g + g * g));
        */
    }
}
