crate::ix!();

pub const DELAY_EFFECT_PROCESSOR_EFFECT_DELAY_SAMPLES: usize = 192000;

///-----------------------
pub struct DelayEffectProcessor<FloatType: FloatSample> {

    no_interpolation:         DelayLine<f32, delay_line_interpolation_types::NoInterpolation>, // default = { effectDelaySamples  }
    linear:                   DelayLine<f32, delay_line_interpolation_types::Linear>, // default = { effectDelaySamples  }
    lagrange:                 DelayLine<f32, delay_line_interpolation_types::Lagrange3rd>, // default = { effectDelaySamples  }
    thiran:                   DelayLine<f32, delay_line_interpolation_types::Thiran>, // default = { effectDelaySamples  }

    /**
      | Double precision to avoid some approximation
      | issues
      |
      */
    smooth_filter:            FirstOrderTPTFilter<f64>,

    delay_effect_value:       [f64; 2],
    delay_feedback_volume:    [LinearSmoothedValue<f32>; 2],
    lowpass:                  FirstOrderTPTFilter<FloatType>,
    mixer:                    DryWetMixer<FloatType>,
    last_delay_effect_output: [f32; 2],
    delay_effect_type:        i32, // default = 1
}

impl<FloatType: FloatSample> Default for DelayEffectProcessor<FloatType> {
    
    fn default() -> Self {
        todo!();
        /*


            smoothFilter.setType (FirstOrderTPTFilterType::lowpass);
                lowpass.setType      (FirstOrderTPTFilterType::lowpass);
                mixer.setMixingRule (DryWetMixingRule::linear)
        */
    }
}

impl<FloatType: FloatSample> DelayEffectProcessor<FloatType> {

    
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            prepareAll (spec, noInterpolation, linear, lagrange, thiran, smoothFilter, lowpass, mixer);

                for (auto& volume : delayFeedbackVolume)
                    volume.reset (spec.sampleRate, 0.05);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            resetAll (noInterpolation, linear, lagrange, thiran, smoothFilter, lowpass, mixer);
                std::fill (lastDelayEffectOutput.begin(), lastDelayEffectOutput.end(), 0.0f);
        */
    }
    
    pub fn process<Context>(&mut self, context: &mut Context)  {
    
        todo!();
        /*
            if (context.isBypassed)
                    return;

                const auto& inputBlock  = context.getInputBlock();
                const auto& outputBlock = context.getOutputBlock();
                const auto numSamples  = inputBlock.getNumSamples();
                const auto numChannels = inputBlock.getNumChannels();

                mixer.pushDrySamples (inputBlock);

                for (size_t channel = 0; channel < numChannels; ++channel)
                {
                    auto* samplesIn  = inputBlock .getChannelPointer (channel);
                    auto* samplesOut = outputBlock.getChannelPointer (channel);

                    for (size_t i = 0; i < numSamples; ++i)
                    {
                        auto input = samplesIn[i] - lastDelayEffectOutput[channel];

                        auto delay = smoothFilter.processSample (int (channel), delayEffectValue[channel]);

                        const auto output = [&]
                        {
                            switch (delayEffectType)
                            {
                                case 0:
                                    noInterpolation.pushSample (int (channel), input);
                                    noInterpolation.setDelay ((float) delay);
                                    return noInterpolation.popSample (int (channel));

                                case 1:
                                    linear.pushSample (int (channel), input);
                                    linear.setDelay ((float) delay);
                                    return linear.popSample (int (channel));

                                case 2:
                                    lagrange.pushSample (int (channel), input);
                                    lagrange.setDelay ((float) delay);
                                    return lagrange.popSample (int (channel));

                                case 3:
                                    thiran.pushSample (int (channel), input);
                                    thiran.setDelay ((float) delay);
                                    return thiran.popSample (int (channel));

                                default:
                                    break;
                            }

                            jassertfalse;
                            return 0.0f;
                        }();

                        const auto processed = lowpass.processSample (int (channel), output);

                        samplesOut[i] = processed;
                        lastDelayEffectOutput[channel] = processed * delayFeedbackVolume[channel].getNextValue();
                    }
                }

                mixer.mixWetSamples (outputBlock);
        */
    }
}

