crate::ix!();

pub struct ConvolutionProcessor<'a, FloatType: FloatSample> {
    queue:          ConvolutionMessageQueue,
    cabinet:        Convolution<'a>,        //{ Convolution::NonUniform { 512 }, queue };
    reverb:         Convolution<'a>,        //{ Convolution::NonUniform { 512 }, queue };
    mixer:          DryWetMixer<FloatType>,
    cab_enabled:    bool,                    // default = false
    reverb_enabled: bool,                    // default = false
}

impl<'a, FloatType: FloatSample> Default for ConvolutionProcessor<'a, FloatType> {
    
    fn default() -> Self {
        todo!();
        /*


            loadImpulseResponse (cabinet, "guitar_amp.wav");
                loadImpulseResponse (reverb,  "reverb_ir.wav");
                mixer.setMixingRule (DryWetMixingRule::balanced)
        */
    }
}

impl<'a, FloatType: FloatSample> ConvolutionProcessor<'a, FloatType> {
    
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            prepareAll (spec, cabinet, reverb, mixer);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            resetAll (cabinet, reverb, mixer);
        */
    }
    
    pub fn process<Context>(&mut self, context: &mut Context)  {
    
        todo!();
        /*
            auto contextConv = context;
                contextConv.isBypassed = (! cabEnabled) || context.isBypassed;
                cabinet.process (contextConv);

                if (cabEnabled)
                    context.getOutputBlock().multiplyBy (4.0f);

                if (reverbEnabled)
                    mixer.pushDrySamples (context.getInputBlock());

                contextConv.isBypassed = (! reverbEnabled) || context.isBypassed;
                reverb.process (contextConv);

                if (reverbEnabled)
                {
                    const auto& outputBlock = context.getOutputBlock();
                    outputBlock.multiplyBy (4.0f);
                    mixer.mixWetSamples (outputBlock);
                }
        */
    }
    
    pub fn get_latency(&self) -> i32 {
        
        todo!();
        /*
            auto latency = 0;

                if (cabEnabled)
                    latency += cabinet.getLatency();

                if (reverbEnabled)
                    latency += reverb.getLatency();

                return latency;
        */
    }
    
    pub fn load_impulse_response(
        convolution: &mut Convolution,
        filename:    *const u8)  {
        
        todo!();
        /*
            auto stream = createAssetInputStream (filename);

                if (stream == nullptr)
                {
                    jassertfalse;
                    return;
                }

                AudioFormatManager manager;
                manager.registerBasicFormats();
                std::unique_ptr<AudioFormatReader> reader { manager.createReaderFor (std::move (stream)) };

                if (reader == nullptr)
                {
                    jassertfalse;
                    return;
                }

                AudioBuffer<float> buffer (static_cast<int> (reader->numChannels),
                                           static_cast<int> (reader->lengthInSamples));
                reader->read (buffer.getArrayOfWritePointers(), buffer.getNumChannels(), 0, buffer.getNumSamples());

                convolution.loadImpulseResponse (std::move (buffer),
                                                 reader->sampleRate,
                                                 Convolution::Stereo::yes,
                                                 Convolution::Trim::yes,
                                                 Convolution::Normalise::yes);
        */
    }
}
