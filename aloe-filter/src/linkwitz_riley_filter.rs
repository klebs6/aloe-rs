crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_LinkwitzRileyFilter.h]

pub enum LinkwitzRileyFilterType
{
    lowpass,
    highpass,
    allpass
}

/**
  | A filter class designed to perform multi-band
  | separation using the TPT (Topology-Preserving
  | Transform) structure.
  | 
  | Linkwitz-Riley filters are widely
  | used in audio crossovers that have two
  | outputs, a low-pass and a high-pass,
  | such that their sum is equivalent to
  | an all-pass filter with a flat magnitude
  | frequency response. The Linkwitz-Riley
  | filters available in this class are
  | designed to have a -24 dB/octave slope
  | (LR 4th order).
  | 
  | @tags{DSP}
  |
  */
pub struct LinkwitzRileyFilter<SampleType> {
    g:                SampleType,
    r2:               SampleType,
    h:                SampleType,
    s1:               Vec<SampleType>,
    s2:               Vec<SampleType>,
    s3:               Vec<SampleType>,
    s4:               Vec<SampleType>,
    sample_rate:      f64, // default = 44100.0
    cutoff_frequency: SampleType, // default = 2000.0
    filter_type:      LinkwitzRileyFilterType, // default = LinkwitzRileyFilterType::lowpass
}

impl<SampleType: SampleTypeInterface> Default for LinkwitzRileyFilter<SampleType> {
    
    fn default() -> Self {
        todo!();
        /*


            update()
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_LinkwitzRileyFilter.cpp]
impl<SampleType: SampleTypeInterface> LinkwitzRileyFilter<SampleType> {

    /**
      | Returns the type of the filter.
      |
      */
    pub fn get_type(&self) -> LinkwitzRileyFilterType {
        
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
                auto* inputSamples = inputBlock.getChannelPointer (channel);
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
    pub fn set_type(&mut self, new_type: LinkwitzRileyFilterType)  {
        
        todo!();
        /*
            filterType = newType;
        */
    }
    
    /**
      | Sets the cutoff frequency of the filter
      | in Hz.
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
      | Initialises the filter.
      |
      */
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            jassert (spec.sampleRate > 0);
        jassert (spec.numChannels > 0);

        sampleRate = spec.sampleRate;
        update();

        s1.resize (spec.numChannels);
        s2.resize (spec.numChannels);
        s3.resize (spec.numChannels);
        s4.resize (spec.numChannels);

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
            for (auto s : { &s1, &s2, &s3, &s4 })
            std::fill (s->begin(), s->end(), static_cast<SampleType> (0));
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
            for (auto s : { &s1, &s2, &s3, &s4 })
            for (auto& element : *s)
                util::snapToZero (element);
        */
    }
    
    /**
      | Performs the filter operation on a single
      | sample at a time.
      |
      */
    pub fn process_sample(&mut self, 
        channel:     i32,
        input_value: SampleType) -> SampleType {
        
        todo!();
        /*
            auto yH = (inputValue - (R2 + g) * s1[(size_t) channel] - s2[(size_t) channel]) * h;

        auto yB = g * yH + s1[(size_t) channel];
        s1[(size_t) channel] = g * yH + yB;

        auto yL = g * yB + s2[(size_t) channel];
        s2[(size_t) channel] = g * yB + yL;

        if (filterType == LinkwitzRileyFilterType::allpass)
            return yL - R2 * yB + yH;

        auto yH2 = ((filterType == LinkwitzRileyFilterType::lowpass ? yL : yH) - (R2 + g) * s3[(size_t) channel] - s4[(size_t) channel]) * h;

        auto yB2 = g * yH2 + s3[(size_t) channel];
        s3[(size_t) channel] = g * yH2 + yB2;

        auto yL2 = g * yB2 + s4[(size_t) channel];
        s4[(size_t) channel] = g * yB2 + yL2;

        return filterType == LinkwitzRileyFilterType::lowpass ? yL2 : yH2;
        */
    }
    
    /**
      | Performs the filter operation on a single
      | sample at a time, and returns both the
      | low-pass and the high-pass outputs
      | of the TPT structure.
      |
      */
    pub fn process_sample_with_output_range(
        &mut self, 
        channel:     i32,
        input_value: SampleType,
        output_low:  &mut SampleType,
        output_high: &mut SampleType

    ) {
        
        todo!();
        /*
            auto yH = (inputValue - (R2 + g) * s1[(size_t) channel] - s2[(size_t) channel]) * h;

        auto yB = g * yH + s1[(size_t) channel];
        s1[(size_t) channel] = g * yH + yB;

        auto yL = g * yB + s2[(size_t) channel];
        s2[(size_t) channel] = g * yB + yL;

        auto yH2 = (yL - (R2 + g) * s3[(size_t) channel] - s4[(size_t) channel]) * h;

        auto yB2 = g * yH2 + s3[(size_t) channel];
        s3[(size_t) channel] = g * yH2 + yB2;

        auto yL2 = g * yB2 + s4[(size_t) channel];
        s4[(size_t) channel] = g * yB2 + yL2;

        outputLow = yL2;
        outputHigh = yL - R2 * yB + yH - yL2;
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
            g  = (SampleType) std::tan (MathConstants<double>::pi * cutoffFrequency / sampleRate);
        R2 = (SampleType) std::sqrt (2.0);
        h  = (SampleType) (1.0 / (1.0 + R2 * g + g * g));
        */
    }
}
