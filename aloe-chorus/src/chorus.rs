crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_Chorus.h]

/**
  | A simple chorus DSP widget that modulates
  | the delay of a delay line in order to create
  | sweeping notches in the magnitude frequency
  | response.
  | 
  | This audio effect can be controlled
  | via the speed and depth of the LFO controlling
  | the frequency response, a mix control,
  | a feedback control, and the centre delay
  | of the modulation.
  | 
  | -----------
  | @note
  | 
  | To get classic chorus sounds try to use
  | a centre delay time around 7-8 ms with
  | a low feeback volume and a low depth.
  | This effect can also be used as a flanger
  | with a lower centre delay time and a lot
  | of feedback, and as a vibrato effect
  | if the mix value is 1.
  | 
  | @tags{DSP}
  |
  */
pub struct Chorus<SampleType: FloatSample> {
    osc:                Oscillator<SampleType>,
    delay:              DelayLine<SampleType,delay_line_interpolation_types::Linear>,
    osc_volume:         SmoothedValue<SampleType, value_smoothing_types::Linear>,
    feedback_volume:    Vec<SmoothedValue<SampleType,value_smoothing_types::Linear>>, // default = 2 
    dry_wet:            DryWetMixer<SampleType>,
    last_output:        Vec<SampleType>,         // default = 2 
    buffer_delay_times: AudioBuffer<SampleType>,
    sample_rate:        f64,                     // default = 44100.0
    rate:               SampleType,              // default = 1.0
    depth:              SampleType,              // default = 0.25
    feedback:           SampleType,              // default = 0.0
    mix:                SampleType,              // default = 0.5
    centre_delay:       SampleType,              // default = 7.0
}

impl<SampleType: FloatSample> Default for Chorus<SampleType> {
    
    fn default() -> Self {
        todo!();
        /*


            auto oscFunction = [] (SampleType x) { return std::sin (x); };
        osc.initialise (oscFunction);

        dryWet.setMixingRule (DryWetMixingRule::linear)
        */
    }
}

generic_float_const!(ChorusMaxDepth,               chorus_max_depth,                 1.0);
generic_float_const!(ChorusMaxCenterDelayMs,       chorus_max_center_delay_ms,     100.0);
generic_float_const!(ChorusOscVolumeMultiplier,    chorus_osc_volume_multiplier,     0.5);
generic_float_const!(ChorusMaximumDelayModulation, chorus_maximum_delay_modulation, 20.0);

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_Chorus.cpp]
impl<SampleType: FloatSample> Chorus<SampleType> {

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
            jassert (inputBlock.getNumChannels() == lastOutput.size());
            jassert (inputBlock.getNumSamples()  == numSamples);

            if (context.isBypassed)
            {
                outputBlock.copyFrom (inputBlock);
                return;
            }

            auto delayValuesBlock = AudioBlock<SampleType>(bufferDelayTimes).getSubBlock (0, numSamples);
            auto contextDelay = ProcessContextReplacing<SampleType> (delayValuesBlock);
            delayValuesBlock.clear();

            osc.process (contextDelay);
            delayValuesBlock.multiplyBy (oscVolume);

            auto* delaySamples = bufferDelayTimes.getWritePointer (0);

            for (size_t i = 0; i < numSamples; ++i)
            {
                auto lfo = jmax (static_cast<SampleType> (1.0), chorus_maximum_delay_modulation() * delaySamples[i] + centreDelay);
                delaySamples[i] = static_cast<SampleType> (lfo * sampleRate / 1000.0);
            }

            dryWet.pushDrySamples (inputBlock);

            for (size_t channel = 0; channel < numChannels; ++channel)
            {
                auto* inputSamples  = inputBlock .getChannelPointer (channel);
                auto* outputSamples = outputBlock.getChannelPointer (channel);

                for (size_t i = 0; i < numSamples; ++i)
                {
                    auto input = inputSamples[i];
                    auto output = input - lastOutput[channel];

                    delay.pushSample ((int) channel, output);
                    delay.setDelay (delaySamples[i]);
                    output = delay.popSample ((int) channel);

                    outputSamples[i] = output;
                    lastOutput[channel] = output * feedbackVolume[channel].getNextValue();
                }
            }

            dryWet.mixWetSamples (outputBlock);
        */
    }
    
    /**
      | Sets the rate (in Hz) of the LFO modulating
      | the chorus delay line. This rate must
      | be lower than 100 Hz.
      |
      */
    pub fn set_rate(&mut self, new_rate_hz: SampleType)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (newRateHz, static_cast<SampleType> (100.0)));

        rate = newRateHz;
        update();
        */
    }
    
    /**
      | Sets the volume of the LFO modulating
      | the chorus delay line (between 0 and
      | 1).
      |
      */
    pub fn set_depth(&mut self, new_depth: SampleType)  {
        
        todo!();
        /*
            jassert (isPositiveAndNotGreaterThan (newDepth, chorus_max_depth()));

        depth = newDepth;
        update();
        */
    }
    
    /**
      | Sets the centre delay in milliseconds
      | of the chorus delay line modulation.
      | 
      | This delay must be between 1 and 100 ms.
      |
      */
    pub fn set_centre_delay(&mut self, new_delay_ms: SampleType)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (newDelayMs, max_center_delay_ms()));

        centreDelay = jlimit (static_cast<SampleType> (1.0), max_center_delay_ms(), newDelayMs);
        */
    }
    
    /**
      | Sets the feedback volume (between -1
      | and 1) of the chorus delay line.
      | 
      | Negative values can be used to get specific
      | chorus sounds.
      |
      */
    pub fn set_feedback(&mut self, new_feedback: SampleType)  {
        
        todo!();
        /*
            jassert (newFeedback >= static_cast<SampleType> (-1.0) && newFeedback <= static_cast<SampleType> (1.0));

        feedback = newFeedback;
        update();
        */
    }
    
    /**
      | Sets the amount of dry and wet signal
      | in the output of the chorus (between
      | 0 for full dry and 1 for full wet).
      |
      */
    pub fn set_mix(&mut self, new_mix: SampleType)  {
        
        todo!();
        /*
            jassert (isPositiveAndNotGreaterThan (newMix, static_cast<SampleType> (1.0)));

        mix = newMix;
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

        const auto maxPossibleDelay = std::ceil ((chorus_maximum_delay_modulation() * chorus_max_depth() * chorus_osc_volume_multiplier() + max_center_delay_ms())
                                                 * sampleRate / 1000.0);
        delay = DelayLine<SampleType, DelayLineInterpolationTypes::Linear>{ static_cast<int> (maxPossibleDelay) };
        delay.prepare (spec);

        dryWet.prepare (spec);
        feedbackVolume.resize (spec.numChannels);
        lastOutput.resize (spec.numChannels);

        osc.prepare (spec);
        bufferDelayTimes.setSize (1, (int) spec.maximumBlockSize, false, false, true);

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
            std::fill (lastOutput.begin(), lastOutput.end(), static_cast<SampleType> (0));

        delay.reset();
        osc.reset();
        dryWet.reset();

        oscVolume.reset (sampleRate, 0.05);

        for (auto& vol : feedbackVolume)
            vol.reset (sampleRate, 0.05);
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
            osc.setFrequency (rate);
        oscVolume.setTargetValue (depth * chorus_osc_volume_multiplier());
        dryWet.setWetMixProportion (mix);

        for (auto& vol : feedbackVolume)
            vol.setTargetValue (feedback);
        */
    }
}
