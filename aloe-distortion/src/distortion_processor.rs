crate::ix!();

pub struct DistortionProcessor<FloatType: FloatSample> {
    oversamplers:               [Oversampling<FloatType>; 6],
    lowpass:                    FirstOrderTPTFilter<FloatType>,
    highpass:                   FirstOrderTPTFilter<FloatType>,
    dist_gain:                  Gain<FloatType>,
    comp_gain:                  Gain<FloatType>,
    mixer:                      DryWetMixer<FloatType>,         // default = { 10  }
    wave_shapers:               [WaveShaper<FloatType>; 2], // { { { std::tanh }, { FastMathApproximations::tanh } } };
    clipping:                   WaveShaper<FloatType>,          // default = { clip  }
    current_index_oversampling: i32,                             // default = 0
    current_index_waveshaper:   i32,                             // default = 0
}

impl<FloatType: FloatSample> Default for DistortionProcessor<FloatType> {
    
    fn default() -> Self {
        todo!();
        /*


            forEach ([] (Gain<FloatType>& gain) { gain.setRampDurationSeconds (0.05); },
                         distGain,
                         compGain);

                lowpass.setType  (FirstOrderTPTFilterType::lowpass);
                highpass.setType (FirstOrderTPTFilterType::highpass);
                mixer.setMixingRule (DryWetMixingRule::linear)
        */
    }
}

impl<FloatType: FloatSample> DistortionProcessor<FloatType> {

    pub fn default_oversamplers() -> [Oversampling<f32>; 6] {

        todo!();

        /*
           { {
           { 2, 1, Oversampling<FloatType>::filterHalfBandPolyphaseIIR, true, false },
           { 2, 2, Oversampling<FloatType>::filterHalfBandPolyphaseIIR, true, false },
           { 2, 3, Oversampling<FloatType>::filterHalfBandPolyphaseIIR, true, false },

           { 2, 1, Oversampling<FloatType>::filterHalfBandPolyphaseIIR, true, true },
           { 2, 2, Oversampling<FloatType>::filterHalfBandPolyphaseIIR, true, true },
           { 2, 3, Oversampling<FloatType>::filterHalfBandPolyphaseIIR, true, true },
           } }
           */
    }

    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            for (auto& oversampler : oversamplers)
                    oversampler.initProcessing (spec.maximumBlockSize);

                prepareAll (spec, lowpass, highpass, distGain, compGain, mixer);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            for (auto& oversampler : oversamplers)
                    oversampler.reset();

                resetAll (lowpass, highpass, distGain, compGain, mixer);
        */
    }
    
    pub fn get_latency(&self) -> f32 {
        
        todo!();
        /*
            return oversamplers[size_t (currentIndexOversampling)].getLatencyInSamples();
        */
    }
    
    pub fn process<Context>(&mut self, context: &mut Context)  {
    
        todo!();
        /*
            if (context.isBypassed)
                    return;

                const auto& inputBlock = context.getInputBlock();

                mixer.setWetLatency (getLatency());
                mixer.pushDrySamples (inputBlock);

                distGain.process (context);
                highpass.process (context);

                auto ovBlock = oversamplers[size_t (currentIndexOversampling)].processSamplesUp (inputBlock);

                ProcessContextReplacing<FloatType> waveshaperContext (ovBlock);

                if (isPositiveAndBelow (currentIndexWaveshaper, waveShapers.size()))
                {
                    waveShapers[size_t (currentIndexWaveshaper)].process (waveshaperContext);

                    if (currentIndexWaveshaper == 1)
                        clipping.process (waveshaperContext);

                    waveshaperContext.getOutputBlock() *= 0.7f;
                }

                auto& outputBlock = context.getOutputBlock();
                oversamplers[size_t (currentIndexOversampling)].processSamplesDown (outputBlock);

                lowpass.process (context);
                compGain.process (context);
                mixer.mixWetSamples (outputBlock);
        */
    }
    
    pub fn clip(in_: f32) -> f32 {
        
        todo!();
        /*
            return jlimit (-1.0f, 1.0f, in);
        */
    }
}
