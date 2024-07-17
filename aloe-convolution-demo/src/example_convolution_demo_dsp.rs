crate::ix!();

pub struct ConvolutionDemoDSP<'a> {
    sample_rate:          f64,  // default = 0.0
    bypass:               bool, // default = false
    current_cabinet_data: MemoryBlock,
    convolution:          Convolution<'a>,
    buffer_transfer:      BufferTransfer,
    cabinet_param:        ChoiceParameter<'a>,                //{ { "Bypass", "Guitar amplifier 8''", "Cassette recorder" }, 1, "Cabinet Type" };
    parameters:           Vec<*mut DSPDemoParameterBase<'a>>, // { &cabinetParam };
}

impl<'a> ConvolutionDemoDSP<'a> {

    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            sampleRate = spec.sampleRate;
            convolution.prepare (spec);
            updateParameters();
        */
    }
    
    pub fn process(&mut self, context: ProcessContextReplacing<f32>)  {
        
        todo!();
        /*
            context.isBypassed = bypass;

            // Load a new IR if there's one available. Note that this doesn't lock or allocate!
            bufferTransfer.get ([this] (BufferWithSampleRate& buf)
            {
                convolution.loadImpulseResponse (std::move (buf.buffer),
                                                 buf.sampleRate,
                                                 Convolution::Stereo::yes,
                                                 Convolution::Trim::yes,
                                                 Convolution::Normalise::yes);
            });

            convolution.process (context);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            convolution.reset();
        */
    }
    
    pub fn update_parameters(&mut self)  {
        
        todo!();
        /*
            auto* cabinetTypeParameter = dynamic_cast<ChoiceParameter*> (parameters[0]);

            if (cabinetTypeParameter == nullptr)
            {
                jassertfalse;
                return;
            }

            if (cabinetTypeParameter->getCurrentSelectedID() == 1)
            {
                bypass = true;
            }
            else
            {
                bypass = false;

                auto selectedType = cabinetTypeParameter->getCurrentSelectedID();
                auto assetName = (selectedType == 2 ? "guitar_amp.wav" : "cassette_recorder.wav");

                auto assetInputStream = createAssetInputStream (assetName);

                if (assetInputStream == nullptr)
                {
                    jassertfalse;
                    return;
                }

                AudioFormatManager manager;
                manager.registerBasicFormats();
                std::unique_ptr<AudioFormatReader> reader { manager.createReaderFor (std::move (assetInputStream)) };

                if (reader == nullptr)
                {
                    jassertfalse;
                    return;
                }

                AudioBuffer<float> buffer (static_cast<int> (reader->numChannels),
                                           static_cast<int> (reader->lengthInSamples));
                reader->read (buffer.getArrayOfWritePointers(), buffer.getNumChannels(), 0, buffer.getNumSamples());

                bufferTransfer.set (BufferWithSampleRate { std::move (buffer), reader->sampleRate });
            }
        */
    }
}
