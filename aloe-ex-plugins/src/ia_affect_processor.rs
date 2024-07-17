crate::ix!();

/**
  | A simple Inter-App Audio plug-in with
  | a gain control and some meters.
  |
  */
#[no_copy]
#[leak_detector]
pub struct IAAEffectProcessor<'a> {
    base:            AudioProcessor<'a>,
    parameters:      AudioProcessorValueTreeState<'a>,
    previous_gain:   f32, // default = 0.0f

    /**
      | This keeps a copy of the last set of timing
      | info that was acquired during an audio
      | callback - the UI component will display
      | this.
      */
    last_pos_info:   AudioPlayHeadCurrentPositionInfo,

    meter_listeners: ListenerList<Box<dyn MeterListener>>,
}

impl<'a> Default for IAAEffectProcessor<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : AudioProcessor (BusesProperties()
                               .withInput  ("Input",  AudioChannelSet::stereo(), true)
                               .withOutput ("Output", AudioChannelSet::stereo(), true)),
               parameters (*this, nullptr, "InterAppAudioEffect",
                           { std::make_unique<AudioParameterFloat> ("gain", "Gain", NormalisableRange<float> (0.0f, 1.0f), 1.0f / 3.14f) }
        */
    }
}

impl<'a> IAAEffectProcessor<'a> {

    pub fn prepare_to_play(&mut self, _0: f64, _1: i32)  {
        
        todo!();
        /*
            previousGain = *parameters.getRawParameterValue ("gain");
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn is_buses_layout_supported(&self, layouts: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            if (layouts.getMainInputChannels() > 2)
                return false;

            if (layouts.getMainOutputChannelSet() != layouts.getMainInputChannelSet())
                return false;

            return true;
        */
    }
    
    pub fn process_block(&mut self, 
        buffer: &mut AudioBuffer<f32>,
        _1:     &mut MidiBuffer)  {
        
        todo!();
        /*
            float gain = *parameters.getRawParameterValue ("gain");

            auto totalNumInputChannels  = getTotalNumInputChannels();
            auto totalNumOutputChannels = getTotalNumOutputChannels();

            auto numSamples = buffer.getNumSamples();

            for (auto i = totalNumInputChannels; i < totalNumOutputChannels; ++i)
                buffer.clear (i, 0, buffer.getNumSamples());

            // Apply the gain to the samples using a ramp to avoid discontinuities in
            // the audio between processed buffers.
            for (auto channel = 0; channel < totalNumInputChannels; ++channel)
            {
                buffer.applyGainRamp (channel, 0, numSamples, previousGain, gain);
                auto newLevel = buffer.getMagnitude (channel, 0, numSamples);

                meterListeners.call ([=] (MeterListener& l) { l.handleNewMeterValue (channel, newLevel); });
            }

            previousGain = gain;

            // Now ask the host for the current time so we can store it to be displayed later.
            updateCurrentTimeInfoFromHost (lastPosInfo);
        */
    }
    
    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            return new IAAEffectEditor (*this, parameters);
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
            return "InterAppAudioEffectPlugin";
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
    
    pub fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            if (auto xml = parameters.state.createXml())
                copyXmlToBinary (*xml, destData);
        */
    }
    
    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            if (auto xmlState = getXmlFromBinary (data, sizeInBytes))
                if (xmlState->hasTagName (parameters.state.getType()))
                    parameters.state = ValueTree::fromXml (*xmlState);
        */
    }
    
    pub fn update_current_time_info_from_host(&mut self, pos_info: &mut AudioPlayHeadCurrentPositionInfo) -> bool {
        
        todo!();
        /*
            if (auto* ph = getPlayHead())
            {
                AudioPlayHeadCurrentPositionInfo newTime;

                if (ph->getCurrentPosition (newTime))
                {
                    posInfo = newTime;  // Successfully got the current time from the host.
                    return true;
                }
            }

            // If the host fails to provide the current time, we'll just reset our copy to a default.
            lastPosInfo.resetToDefault();

            return false;
        */
    }
    
    pub fn add_meter_listener(&mut self, listener: &mut dyn MeterListener)  {
        
        todo!();
        /*
            meterListeners.add    (&listener);
        */
    }
    
    pub fn remove_meter_listener(&mut self, listener: &mut dyn MeterListener)  {
        
        todo!();
        /*
            meterListeners.remove (&listener);
        */
    }
}
