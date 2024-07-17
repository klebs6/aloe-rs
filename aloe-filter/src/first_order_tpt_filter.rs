crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_FirstOrderTPTFilter.h]

pub enum FirstOrderTPTFilterType
{
    lowpass,
    highpass,
    allpass
}

/**
  | A first order filter class using the
  | TPT (Topology-Preserving Transform)
  | structure.
  | 
  | This filter can be modulated at high
  | rates without producing audio artefacts.
  | See Vadim Zavalishin's documentation
  | about TPT structures for more information.
  | 
  | Note: Using this class prevents some
  | loud audio artefacts commonly encountered
  | when changing the cutoff frequency
  | using of other filter simulation structures
  | and IIR filter classes. However, this
  | class may still require additional
  | smoothing for cutoff frequency changes.
  | 
  | see StateVariableFilter, IIRFilter,
  | SmoothedValue
  | 
  | @tags{DSP}
  |
  */
pub struct FirstOrderTPTFilter<SampleType> {
    g:                SampleType,              // default = 0
    s1:               Vec<SampleType>,         // default = 2 
    sample_rate:      f64,                     // default = 44100.0
    filter_type:      FirstOrderTptFilterType, // default = Type::lowpass
    cutoff_frequency: SampleType,              // default = 1000.0
}

pub type FirstOrderTptFilterType = FirstOrderTPTFilterType;

impl<SampleType: SampleTypeInterface> Default for FirstOrderTPTFilter<SampleType> {
    
    fn default() -> Self {
        todo!();
        /*


            update()
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_FirstOrderTPTFilter.cpp]
impl<SampleType: SampleTypeInterface> FirstOrderTPTFilter<SampleType> {

    /**
      | Returns the type of the filter.
      |
      */
    pub fn get_type(&self) -> FirstOrderTptFilterType {
        
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
    pub fn set_type(&mut self, new_value: FirstOrderTptFilterType)  {
        
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
      | cutoff frequency in Hz.
      |
      */
    pub fn set_cutoff_frequency(&mut self, new_value: SampleType)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (newValue, static_cast<SampleType> (sampleRate * 0.5)));

        cutoffFrequency = newValue;
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

        update();
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
            std::fill (s1.begin(), s1.end(), newValue);
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
            auto& s = s1[(size_t) channel];

        auto v = G * (inputValue - s);
        auto y = v + s;
        s = y + v;

        switch (filterType)
        {
            case FirstOrderTptFilterType::lowpass:   return y;
            case FirstOrderTptFilterType::highpass:  return inputValue - y;
            case FirstOrderTptFilterType::allpass:   return 2 * y - inputValue;
            default:              break;
        }

        jassertfalse;
        return y;
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
            for (auto& s : s1)
            util::snapToZero (s);
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
            auto g = SampleType (std::tan (MathConstants<double>::pi * cutoffFrequency / sampleRate));
        G = g / (1 + g);
        */
    }
}
