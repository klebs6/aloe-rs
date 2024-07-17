crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/NoiseGatePluginDemo.h]

#[no_copy]
#[leak_detector]
pub struct NoiseGate<'a> {
    base:              AudioProcessor<'a>,
    threshold:         *mut AudioParameterFloat,
    alpha:             *mut AudioParameterFloat,
    sample_count_down: i32,
    low_pass_coeff:    f32,
}

impl<'a> Default for NoiseGate<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : AudioProcessor (BusesProperties().withInput  ("Input",     AudioChannelSet::stereo())
                                               .withOutput ("Output",    AudioChannelSet::stereo())
                                               .withInput  ("Sidechain", AudioChannelSet::stereo()))

            addParameter (threshold = new AudioParameterFloat ("threshold", "Threshold", 0.0f, 1.0f, 0.5f));
            addParameter (alpha     = new AudioParameterFloat ("alpha",     "Alpha",     0.0f, 1.0f, 0.8f))
        */
    }
}

impl<'a> NoiseGate<'a> {

    pub fn is_buses_layout_supported(&self, layouts: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            // the sidechain can take any layout, the main bus needs to be the same on the input and output
            return layouts.getMainInputChannelSet() == layouts.getMainOutputChannelSet()
                     && ! layouts.getMainInputChannelSet().isDisabled();
        */
    }
    
    pub fn prepare_to_play(&mut self, _0: f64, _1: i32)  {
        
        todo!();
        /*
            lowPassCoeff = 0.0f; sampleCountDown = 0;
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn process_block(&mut self, 
        buffer: &mut AudioBuffer<f32>,
        _1:     &mut MidiBuffer)  {
        
        todo!();
        /*
            auto mainInputOutput = getBusBuffer (buffer, true, 0);
            auto sideChainInput  = getBusBuffer (buffer, true, 1);

            auto alphaCopy     = alpha->get();
            auto thresholdCopy = threshold->get();

            for (int j = 0; j < buffer.getNumSamples(); ++j)
            {
                auto mixedSamples = 0.0f;

                for (int i = 0; i < sideChainInput.getNumChannels(); ++i)
                    mixedSamples += sideChainInput.getReadPointer (i)[j];

                mixedSamples /= static_cast<float> (sideChainInput.getNumChannels());
                lowPassCoeff = (alphaCopy * lowPassCoeff) + ((1.0f - alphaCopy) * mixedSamples);

                if (lowPassCoeff >= thresholdCopy)
                    sampleCountDown = (int) getSampleRate();

                // very in-effective way of doing this
                for (int i = 0; i < mainInputOutput.getNumChannels(); ++i)
                    *mainInputOutput.getWritePointer (i, j) = sampleCountDown > 0 ? *mainInputOutput.getReadPointer (i, j) : 0.0f;

                if (sampleCountDown > 0)
                    --sampleCountDown;
            }
        */
    }
    
    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            return new GenericAudioProcessorEditor (*this);
        */
    }
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return "NoiseGate";
        */
    }
    
    pub fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn produces_midi(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_tail_length_seconds(&self) -> f64 {
        
        todo!();
        /*
            return 0.0;
        */
    }
    
    pub fn get_num_programs(&mut self) -> i32 {
        
        todo!();
        /*
            return 1;
        */
    }
    
    pub fn get_current_program(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn set_current_program(&mut self, _0: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_program_name(&mut self, _0: i32) -> String {
        
        todo!();
        /*
            return "None";
        */
    }
    
    pub fn change_program_name(&mut self, 
        _0: i32,
        _1: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn isvst2(&self) -> bool {
        
        todo!();
        /*
            return (wrapperType == wrapperType_VST);
        */
    }
    
    pub fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            MemoryOutputStream stream (destData, true);

            stream.writeFloat (*threshold);
            stream.writeFloat (*alpha);
        */
    }
    
    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            MemoryInputStream stream (data, static_cast<size_t> (sizeInBytes), false);

            threshold->setValueNotifyingHost (stream.readFloat());
            alpha->setValueNotifyingHost     (stream.readFloat());
        */
    }
}
