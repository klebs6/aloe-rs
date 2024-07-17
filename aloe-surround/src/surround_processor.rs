crate::ix!();

#[no_copy]
#[leak_detector]
pub struct SurroundProcessor<'a> {
    base:            AudioProcessor<'a>,
    base3:           AsyncUpdater<'a>,
    channel_active:  Vec<i32>,
    alpha_coeffs:    Vec<f32>,
    channel_clicked: i32,
    sample_offset:   i32,
}

impl<'a> ChannelClickListener for SurroundProcessor<'a> {

    fn channel_button_clicked(&mut self, channel_index: i32)  {
        
        todo!();
        /*
            channelClicked = channelIndex;
            sampleOffset = 0;
        */
    }
    
    fn is_channel_active(&mut self, channel_index: i32) -> bool {
        
        todo!();
        /*
            return channelActive[channelIndex] > 0;
        */
    }
}

impl<'a> Default for SurroundProcessor<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : AudioProcessor(BusesProperties().withInput  ("Input",  AudioChannelSet::stereo())
                                              .withOutput ("Output", AudioChannelSet::stereo())
        */
    }
}

impl<'a> SurroundProcessor<'a> {

    pub fn prepare_to_play(&mut self, 
        sample_rate:       f64,
        samples_per_block: i32)  {
        
        todo!();
        /*
            channelClicked = 0;
            sampleOffset = static_cast<int> (std::ceil (sampleRate));

            auto numChannels = getChannelCountOfBus (true, 0);
            channelActive.resize (numChannels);
            alphaCoeffs  .resize (numChannels);
            reset();

            triggerAsyncUpdate();

            ignoreUnused (samplesPerBlock);
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
            reset();
        */
    }
    
    pub fn process_block(&mut self, 
        buffer: &mut AudioBuffer<f32>,
        _1:     &mut MidiBuffer)  {
        
        todo!();
        /*
            for (auto ch = 0; ch < buffer.getNumChannels(); ++ch)
            {
                auto& channelTime = channelActive.getReference (ch);
                auto& alpha       = alphaCoeffs  .getReference (ch);

                for (auto j = 0; j < buffer.getNumSamples(); ++j)
                {
                    auto sample = buffer.getReadPointer (ch)[j];
                    alpha = (0.8f * alpha) + (0.2f * sample);

                    if (std::abs (alpha) >= 0.1f)
                        channelTime = static_cast<int> (getSampleRate() / 2.0);
                }

                channelTime = jmax (0, channelTime - buffer.getNumSamples());
            }

            auto fillSamples = jmin (static_cast<int> (std::ceil (getSampleRate())) - sampleOffset,
                                     buffer.getNumSamples());

            if (isPositiveAndBelow (channelClicked, buffer.getNumChannels()))
            {
                auto* channelBuffer = buffer.getWritePointer (channelClicked);
                auto freq = (float) (440.0 / getSampleRate());

                for (auto i = 0; i < fillSamples; ++i)
                    channelBuffer[i] += std::sin (MathConstants<float>::twoPi * freq * static_cast<float> (sampleOffset++));
            }
        */
    }
    
    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            return new SurroundEditor (*this);
        */
    }
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn is_buses_layout_supported(&self, layouts: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            return ((! layouts.getMainInputChannelSet() .isDiscreteLayout())
                 && (! layouts.getMainOutputChannelSet().isDiscreteLayout())
                 && (layouts.getMainInputChannelSet() == layouts.getMainOutputChannelSet())
                 && (! layouts.getMainInputChannelSet().isDisabled()));
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            for (auto& channel : channelActive)
                channel = 0;
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return "Surround PlugIn";
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
            return 0;
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
    
    pub fn get_state_information(&mut self, _0: &mut MemoryBlock)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn set_state_information(&mut self, 
        _0: *const c_void,
        _1: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            if (auto* editor = getActiveEditor())
                if (auto* surroundEditor = dynamic_cast<SurroundEditor*> (editor))
                    surroundEditor->updateGUI();
        */
    }
}
