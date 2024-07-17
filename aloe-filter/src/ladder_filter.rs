crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_LadderFilter.h]

/**
  | Multi-mode filter based on the Moog
  | ladder filter.
  | 
  | @tags{DSP}
  |
  */
pub struct LadderFilter<SampleType: FloatSample> {
    drive:                     SampleType,
    drive2:                    SampleType,
    gain:                      SampleType,
    gain2:                     SampleType,
    comp:                      SampleType,
    state:                     Vec<[SampleType;LADDER_FILTER_NUM_STATES]>,
    a:                         [SampleType; LADDER_FILTER_NUM_STATES],
    cutoff_transform_smoother: SmoothedValue<SampleType>,
    scaled_resonance_smoother: SmoothedValue<SampleType>,
    cutoff_transform_value:    SampleType,
    scaled_resonance_value:    SampleType,
    saturationlut:             LookupTableTransform<SampleType>, // { [] (SampleType x) { return std::tanh (x); }, SampleType (-5), SampleType (5), 128 };
    cutoff_freq_hz:            SampleType,                       // { SampleType (200) };
    resonance:                 SampleType,
    cutoff_freq_scaler:        SampleType,
    mode:                      LadderFilterMode,
    enabled:                   bool, // default = true
}

pub const LADDER_FILTER_NUM_STATES: usize = 5;

impl<SampleType: FloatSample> Default for LadderFilter<SampleType> {
    
    /**
      | Creates an uninitialised filter. Call
      | prepare() before first use.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        : state(2),

            setSampleRate (SampleType (1000));  // intentionally setting unrealistic default
                                            // sample rate to catch missing initialisation bugs
        setResonance (SampleType (0));
        setDrive     (SampleType (1.2));

        mode = LadderFilterMode::LPF24;
        setMode (LadderFilterMode::LPF12)
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_LadderFilter.cpp]
impl<SampleType: FloatSample> LadderFilter<SampleType> {

    /**
      | Enables or disables the filter. If disabled
      | it will simply pass through the input
      | signal.
      |
      */
    pub fn set_enabled(&mut self, is_enabled: bool)  {
        
        todo!();
        /*
            enabled = isEnabled;
        */
    }

    /**
      | Returns the current number of channels.
      |
      */
    pub fn get_num_channels(&self) -> usize {
        
        todo!();
        /*
            return state.size();
        */
    }

    pub fn process<ProcessContext>(&mut self, context: &ProcessContext)  {
    
        todo!();
        /*
            const auto& inputBlock = context.getInputBlock();
            auto& outputBlock      = context.getOutputBlock();
            const auto numChannels = outputBlock.getNumChannels();
            const auto numSamples  = outputBlock.getNumSamples();

            jassert (inputBlock.getNumChannels() <= getNumChannels());
            jassert (inputBlock.getNumChannels() == numChannels);
            jassert (inputBlock.getNumSamples()  == numSamples);

            if (! enabled || context.isBypassed)
            {
                outputBlock.copyFrom (inputBlock);
                return;
            }

            for (size_t n = 0; n < numSamples; ++n)
            {
                updateSmoothers();

                for (size_t ch = 0; ch < numChannels; ++ch)
                    outputBlock.getChannelPointer (ch)[n] = processSample (inputBlock.getChannelPointer (ch)[n], ch);
            }
        */
    }
    
    pub fn set_num_channels(&mut self, new_value: usize)
    {
        todo!();

        /*
            state.resize (newValue);
        */
    }

    pub fn update_cutoff_freq(&mut self)  {
        
        todo!();
        /*
            cutoffTransformSmoother.setTargetValue (std::exp (cutoffFreqHz * cutoffFreqScaler));
        */
    }

    pub fn update_resonance(&mut self)  {
        
        todo!();
        /*
            scaledResonanceSmoother.setTargetValue (jmap (resonance, SampleType (0.1), SampleType (1.0)));
        */
    }
    
    /**
      | Sets filter mode.
      |
      */
    pub fn set_mode(&mut self, new_mode: LadderFilterMode)  {
        
        todo!();
        /*
            if (newMode == mode)
            return;

        switch (newMode)
        {
            case LadderFilterMode::LPF12:   A = {{ SampleType (0), SampleType (0),  SampleType (1), SampleType (0),  SampleType (0) }}; comp = SampleType (0.5);  break;
            case LadderFilterMode::HPF12:   A = {{ SampleType (1), SampleType (-2), SampleType (1), SampleType (0),  SampleType (0) }}; comp = SampleType (0);    break;
            case LadderFilterMode::BPF12:   A = {{ SampleType (0), SampleType (0), SampleType (-1), SampleType (1),  SampleType (0) }}; comp = SampleType (0.5);  break;
            case LadderFilterMode::LPF24:   A = {{ SampleType (0), SampleType (0),  SampleType (0), SampleType (0),  SampleType (1) }}; comp = SampleType (0.5);  break;
            case LadderFilterMode::HPF24:   A = {{ SampleType (1), SampleType (-4), SampleType (6), SampleType (-4), SampleType (1) }}; comp = SampleType (0);    break;
            case LadderFilterMode::BPF24:   A = {{ SampleType (0), SampleType (0),  SampleType (1), SampleType (-2), SampleType (1) }}; comp = SampleType (0.5);  break;
            default:            jassertfalse;                                                                                                         break;
        }

        static constexpr auto outputGain = SampleType (1.2);

        for (auto& a : A)
            a *= outputGain;

        mode = newMode;
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
            setSampleRate (SampleType (spec.sampleRate));
        setNumChannels (spec.numChannels);
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
            for (auto& s : state)
            s.fill (SampleType (0));

        cutoffTransformSmoother.setCurrentAndTargetValue (cutoffTransformSmoother.getTargetValue());
        scaledResonanceSmoother.setCurrentAndTargetValue (scaledResonanceSmoother.getTargetValue());
        */
    }
    
    /**
      | Sets the cutoff frequency of the filter.
      | 
      | -----------
      | @param newCutoff
      | 
      | cutoff frequency in Hz
      |
      */
    pub fn set_cutoff_frequency_hz(&mut self, new_cutoff: SampleType)  {
        
        todo!();
        /*
            jassert (newCutoff > SampleType (0));
        cutoffFreqHz = newCutoff;
        updateCutoffFreq();
        */
    }
    
    /**
      | Sets the resonance of the filter.
      | 
      | -----------
      | @param newResonance
      | 
      | a value between 0 and 1; higher values
      | increase the resonance and can result
      | in self oscillation!
      |
      */
    pub fn set_resonance(&mut self, new_resonance: SampleType)  {
        
        todo!();
        /*
            jassert (newResonance >= SampleType (0) && newResonance <= SampleType (1));
        resonance = newResonance;
        updateResonance();
        */
    }
    
    /**
      | Sets the amount of saturation in the
      | filter.
      | 
      | -----------
      | @param newDrive
      | 
      | saturation amount; it can be any number
      | greater than or equal to one. Higher
      | values result in more distortion.
      |
      */
    pub fn set_drive(&mut self, new_drive: SampleType)  {
        
        todo!();
        /*
            jassert (newDrive >= SampleType (1));

        drive = newDrive;
        gain = std::pow (drive, SampleType (-2.642))   * SampleType (0.6103) + SampleType (0.3903);
        drive2 = drive                                 * SampleType (0.04)   + SampleType (0.96);
        gain2 = std::pow (drive2, SampleType (-2.642)) * SampleType (0.6103) + SampleType (0.3903);
        */
    }
    
    pub fn process_sample(&mut self, 
        input_value:    SampleType,
        channel_to_use: usize) -> SampleType {
        
        todo!();
        /*
            auto& s = state[channelToUse];

        const auto a1 = cutoffTransformValue;
        const auto g = a1 * SampleType (-1) + SampleType (1);
        const auto b0 = g * SampleType (0.76923076923);
        const auto b1 = g * SampleType (0.23076923076);

        const auto dx = gain * saturationLUT (drive * inputValue);
        const auto a  = dx + scaledResonanceValue * SampleType (-4) * (gain2 * saturationLUT (drive2 * s[4]) - dx * comp);

        const auto b = b1 * s[0] + a1 * s[1] + b0 * a;
        const auto c = b1 * s[1] + a1 * s[2] + b0 * b;
        const auto d = b1 * s[2] + a1 * s[3] + b0 * c;
        const auto e = b1 * s[3] + a1 * s[4] + b0 * d;

        s[0] = a;
        s[1] = b;
        s[2] = c;
        s[3] = d;
        s[4] = e;

        return a * A[0] + b * A[1] + c * A[2] + d * A[3] + e * A[4];
        */
    }
    
    pub fn update_smoothers(&mut self)  {
        
        todo!();
        /*
            cutoffTransformValue = cutoffTransformSmoother.getNextValue();
        scaledResonanceValue = scaledResonanceSmoother.getNextValue();
        */
    }
    
    pub fn set_sample_rate(&mut self, new_value: SampleType)  {
        
        todo!();
        /*
            jassert (newValue > SampleType (0));
        cutoffFreqScaler = SampleType (-2.0 * MathConstants<double>::pi) / newValue;

        static constexpr SampleType smootherRampTimeSec = SampleType (0.05);
        cutoffTransformSmoother.reset (newValue, smootherRampTimeSec);
        scaledResonanceSmoother.reset (newValue, smootherRampTimeSec);

        updateCutoffFreq();
        */
    }
}
