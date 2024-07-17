crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/CMake/AudioPlugin/PluginProcessor.h]

#[no_copy]
#[leak_detector]
pub struct AudioPluginAudioProcessor<'a> {
    base: AudioProcessor<'a>,
}

//-------------------------------------------[.cpp/Aloe/examples/CMake/AudioPlugin/PluginProcessor.cpp]
impl<'a> Default for AudioPluginAudioProcessor<'a> {

    fn default() -> Self {
    
        todo!();
        /*


            : AudioProcessor (BusesProperties()
                         #if ! AloePlugin_IsMidiEffect
                          #if ! AloePlugin_IsSynth
                           .withInput  ("Input",  AudioChannelSet::stereo(), true)
                          #endif
                           .withOutput ("Output", AudioChannelSet::stereo(), true)
                         #endif
                           )
        */
    }
}

impl<'a> AudioPluginAudioProcessor<'a> {
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return AloePlugin_Name;
        */
    }
    
    pub fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
            #if AloePlugin_WantsMidiInput
        return true;
       #else
        return false;
       #endif
        */
    }
    
    pub fn produces_midi(&self) -> bool {
        
        todo!();
        /*
            #if AloePlugin_ProducesMidiOutput
        return true;
       #else
        return false;
       #endif
        */
    }
    
    pub fn is_midi_effect(&self) -> bool {
        
        todo!();
        /*
            #if AloePlugin_IsMidiEffect
        return true;
       #else
        return false;
       #endif
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
            return 1;   // NB: some hosts don't cope very well if you tell them there are 0 programs,
                    // so this should be at least 1, even if you're not really implementing programs.
        */
    }
    
    pub fn get_current_program(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn set_current_program(&mut self, index: i32)  {
        
        todo!();
        /*
            ignoreUnused (index);
        */
    }
    
    pub fn get_program_name(&mut self, index: i32) -> String {
        
        todo!();
        /*
            ignoreUnused (index);
        return {};
        */
    }
    
    pub fn change_program_name(&mut self, 
        index:    i32,
        new_name: &String)  {
        
        todo!();
        /*
            ignoreUnused (index, newName);
        */
    }
    
    pub fn prepare_to_play(&mut self, 
        sample_rate:       f64,
        samples_per_block: i32)  {
        
        todo!();
        /*
            // Use this method as the place to do any pre-playback
        // initialisation that you need..
        ignoreUnused (sampleRate, samplesPerBlock);
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
            // When playback stops, you can use this as an opportunity to free up any
        // spare memory, etc.
        */
    }
    
    pub fn is_buses_layout_supported(&self, layouts: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            #if AloePlugin_IsMidiEffect
        ignoreUnused (layouts);
        return true;
      #else
        // This is the place where you check if the layout is supported.
        // In this template code we only support mono or stereo.
        // Some plugin hosts, such as certain GarageBand versions, will only
        // load plugins that support stereo bus layouts.
        if (layouts.getMainOutputChannelSet() != AudioChannelSet::mono()
         && layouts.getMainOutputChannelSet() != AudioChannelSet::stereo())
            return false;

        // This checks if the input layout matches the output layout
       #if ! AloePlugin_IsSynth
        if (layouts.getMainOutputChannelSet() != layouts.getMainInputChannelSet())
            return false;
       #endif

        return true;
      #endif
        */
    }
    
    pub fn process_block(&mut self, 
        buffer:        &mut AudioBuffer<f32>,
        midi_messages: &mut MidiBuffer)  {
        
        todo!();
        /*
            ignoreUnused (midiMessages);

        ScopedNoDenormals noDenormals;
        auto totalNumInputChannels  = getTotalNumInputChannels();
        auto totalNumOutputChannels = getTotalNumOutputChannels();

        // In case we have more outputs than inputs, this code clears any output
        // channels that didn't contain input data, (because these aren't
        // guaranteed to be empty - they may contain garbage).
        // This is here to avoid people getting screaming feedback
        // when they first compile a plugin, but obviously you don't need to keep
        // this code if your algorithm always overwrites all the output channels.
        for (auto i = totalNumInputChannels; i < totalNumOutputChannels; ++i)
            buffer.clear (i, 0, buffer.getNumSamples());

        // This is the place where you'd normally do the guts of your plugin's
        // audio processing...
        // Make sure to reset the state if your inner loop is processing
        // the samples and the outer loop is handling the channels.
        // Alternatively, you can process the samples with the channels
        // interleaved by keeping the same state.
        for (int channel = 0; channel < totalNumInputChannels; ++channel)
        {
            auto* channelData = buffer.getWritePointer (channel);
            ignoreUnused (channelData);
            // ..do something to the data...
        }
        */
    }
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return true; // (change this to false if you choose to not supply an editor)
        */
    }
    
    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            return new AudioPluginAudioProcessorEditor (*this);
        */
    }
    
    pub fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            // You should use this method to store your parameters in the memory block.
        // You could do that either as raw data, or use the XML or ValueTree classes
        // as intermediaries to make it easy to save and load complex data.
        ignoreUnused (destData);
        */
    }
    
    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            // You should use this method to restore your parameters from this memory block,
        // whose contents will have been created by the getStateInformation() call.
        ignoreUnused (data, sizeInBytes);
        */
    }

    /**
       This creates new instances of the plugin..
      */
    pub fn create_plugin_filter(&mut self) -> *mut AudioProcessor {
        
        todo!();
        /*
            return new AudioPluginAudioProcessor();
        */
    }
}
