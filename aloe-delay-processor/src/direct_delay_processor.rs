crate::ix!();

pub const DIRECT_DELAY_PROCESSOR_DIRECT_DELAY_BUFFER_SIZE: usize = 44100;

pub struct DirectDelayProcessor<FloatType: FloatSample> {

    no_interpolation:       DelayLine<f32, delay_line_interpolation_types::NoInterpolation>, // default = { directDelayBufferSize  }
    linear:                 DelayLine<f32, delay_line_interpolation_types::Linear>, // default = { directDelayBufferSize  }
    lagrange:               DelayLine<f32, delay_line_interpolation_types::Lagrange3rd>, // default = { directDelayBufferSize  }
    thiran:                 DelayLine<f32, delay_line_interpolation_types::Thiran>, // default = { directDelayBufferSize  }

    /**
      | Double precision to avoid some approximation
      | issues
      |
      */
    smooth_filter:          FirstOrderTPTFilter<f64>,

    mixer:                  DryWetMixer<FloatType>,
    delay_direct_value:     [f64;2], // default = { {}  }
    delay_line_direct_type: i32,          // default = 1
}

impl<FloatType: FloatSample> Default for DirectDelayProcessor<FloatType> {
    
    fn default() -> Self {
        todo!();
        /*


            smoothFilter.setType (FirstOrderTPTFilterType::lowpass);
                mixer.setMixingRule (DryWetMixingRule::linear)
        */
    }
}

impl<FloatType: FloatSample> DirectDelayProcessor<FloatType> {
    
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            prepareAll (spec, noInterpolation, linear, lagrange, thiran, smoothFilter, mixer);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            resetAll (noInterpolation, linear, lagrange, thiran, smoothFilter, mixer);
        */
    }
    
    pub fn process<Context>(&mut self, context: &mut Context)  {
    
        todo!();
        /*
            if (context.isBypassed)
                    return;

                const auto& inputBlock  = context.getInputBlock();
                const auto& outputBlock = context.getOutputBlock();

                mixer.pushDrySamples (inputBlock);

                const auto numChannels = inputBlock.getNumChannels();
                const auto numSamples  = inputBlock.getNumSamples();

                for (size_t channel = 0; channel < numChannels; ++channel)
                {
                    auto* samplesIn  = inputBlock .getChannelPointer (channel);
                    auto* samplesOut = outputBlock.getChannelPointer (channel);

                    for (size_t i = 0; i < numSamples; ++i)
                    {
                        const auto delay = smoothFilter.processSample (int (channel), delayDirectValue[channel]);

                        samplesOut[i] = [&]
                        {
                            switch (delayLineDirectType)
                            {
                                case 0:
                                    noInterpolation.pushSample (int (channel), samplesIn[i]);
                                    noInterpolation.setDelay ((float) delay);
                                    return noInterpolation.popSample (int (channel));

                                case 1:
                                    linear.pushSample (int (channel), samplesIn[i]);
                                    linear.setDelay ((float) delay);
                                    return linear.popSample (int (channel));

                                case 2:
                                    lagrange.pushSample (int (channel), samplesIn[i]);
                                    lagrange.setDelay ((float) delay);
                                    return lagrange.popSample (int (channel));

                                case 3:
                                    thiran.pushSample (int (channel), samplesIn[i]);
                                    thiran.setDelay ((float) delay);
                                    return thiran.popSample (int (channel));

                                default:
                                    break;
                            }

                            jassertfalse;
                            return 0.0f;
                        }();
                    }
                }

                mixer.mixWetSamples (outputBlock);
        */
    }
}
