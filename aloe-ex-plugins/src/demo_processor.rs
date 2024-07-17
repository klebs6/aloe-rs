crate::ix!();

/**
  | As the name suggest, this class does
  | the actual audio processing.
  |
  */
#[no_copy]
#[leak_detector]
pub struct AloeDemoPluginAudioProcessor<'a> {

    base:                  AudioProcessor<'a>,

    /*
      | These properties are public so that our
      | editor component can access them
      |
      | A bit of a hacky way to do it, but it's
      | only a demo! Obviously in your own code
      | you'll do this much more neatly..
      */

    /**
      | this is kept up to date with the midi
      | messages that arrive, and the UI component
      | registers with it so it can represent the
      | incoming messages
      */
    keyboard_state:        MidiKeyboardState,

    /**
      | this keeps a copy of the last set of time
      | info that was acquired during an audio
      | callback - the UI component will read this
      | and display it.
      */
    last_pos_info:         SpinLockedPosInfo,

    /**
       Our plug-in's current state
      */
    state:                 AudioProcessorValueTreeState<'a>,

    delay_buffer_float:    AudioBuffer<f32>,
    delay_buffer_double:   AudioBuffer<f64>,
    delay_position:        i32, // default = 0
    synth:                 Synthesizer,
    track_properties_lock: CriticalSection,
    track_properties:      AudioProcessorTrackProperties,
}

impl<'a> Default for AloeDemoPluginAudioProcessor<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : AudioProcessor (getBusesProperties()),
              state (*this, nullptr, "state",
                     { std::make_unique<AudioParameterFloat> ("gain",  "Gain",           NormalisableRange<float> (0.0f, 1.0f), 0.9f),
                       std::make_unique<AudioParameterFloat> ("delay", "Delay Feedback", NormalisableRange<float> (0.0f, 1.0f), 0.5f) })

            // Add a sub-tree to store the state of our UI
            state.state.addChild ({ "uiState", { { "width",  400 }, { "height", 200 } }, {} }, -1, nullptr);

            initialiseSynth()
        */
    }
}

impl<'a> AloeDemoPluginAudioProcessor<'a> {
    
    pub fn is_buses_layout_supported(&self, layouts: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            // Only mono/stereo and input/output must have same layout
            const auto& mainOutput = layouts.getMainOutputChannelSet();
            const auto& mainInput  = layouts.getMainInputChannelSet();

            // input and output layout must either be the same or the input must be disabled altogether
            if (! mainInput.isDisabled() && mainInput != mainOutput)
                return false;

            // only allow stereo and mono
            if (mainOutput.size() > 2)
                return false;

            return true;
        */
    }
    
    pub fn prepare_to_play(&mut self, 
        new_sample_rate:   f64,
        samples_per_block: i32)  {
        
        todo!();
        /*
            // Use this method as the place to do any pre-playback
            // initialisation that you need..
            synth.setCurrentPlaybackSampleRate (newSampleRate);
            keyboardState.reset();

            if (isUsingDoublePrecision())
            {
                delayBufferDouble.setSize (2, 12000);
                delayBufferFloat .setSize (1, 1);
            }
            else
            {
                delayBufferFloat .setSize (2, 12000);
                delayBufferDouble.setSize (1, 1);
            }

            reset();
        */
    }
    
    pub fn process_block(&mut self, 
        buffer:        &mut AudioBuffer<f32>,
        midi_messages: &mut MidiBuffer)  {
        
        todo!();
        /*
            jassert (! isUsingDoublePrecision());
            process (buffer, midiMessages, delayBufferFloat);
        */
    }
    
    pub fn process_block_f64(
        &mut self, 
        buffer:        &mut AudioBuffer<f64>,
        midi_messages: &mut MidiBuffer
    ) {
        
        todo!();
        /*
            jassert (isUsingDoublePrecision());
            process (buffer, midiMessages, delayBufferDouble);
        */
    }
    
    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            return new AloeDemoPluginAudioProcessorEditor (*this);
        */
    }
    
    pub fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            // Store an xml representation of our state.
            if (auto xmlState = state.copyState().createXml())
                copyXmlToBinary (*xmlState, destData);
        */
    }
    
    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            // Restore our plug-in's state from the xml representation stored in the above
            // method.
            if (auto xmlState = getXmlFromBinary (data, sizeInBytes))
                state.replaceState (ValueTree::fromXml (*xmlState));
        */
    }
    
    pub fn update_track_properties(&mut self, properties: &AudioProcessorTrackProperties)  {
        
        todo!();
        /*
            {
                const ScopedLock sl (trackPropertiesLock);
                trackProperties = properties;
            }

            MessageManager::callAsync ([this]
            {
                if (auto* editor = dynamic_cast<AloeDemoPluginAudioProcessorEditor*> (getActiveEditor()))
                     editor->updateTrackProperties();
            });
        */
    }
    
    pub fn process<FloatType>(&mut self, 
        buffer:        &mut AudioBuffer<FloatType>,
        midi_messages: &mut MidiBuffer,
        delay_buffer:  &mut AudioBuffer<FloatType>)  {
    
        todo!();
        /*
            auto gainParamValue  = state.getParameter ("gain") ->getValue();
            auto delayParamValue = state.getParameter ("delay")->getValue();
            auto numSamples = buffer.getNumSamples();

            // In case we have more outputs than inputs, we'll clear any output
            // channels that didn't contain input data, (because these aren't
            // guaranteed to be empty - they may contain garbage).
            for (auto i = getTotalNumInputChannels(); i < getTotalNumOutputChannels(); ++i)
                buffer.clear (i, 0, numSamples);

            // Now pass any incoming midi messages to our keyboard state object, and let it
            // add messages to the buffer if the user is clicking on the on-screen keys
            keyboardState.processNextMidiBuffer (midiMessages, 0, numSamples, true);

            // and now get our synth to process these midi events and generate its output.
            synth.renderNextBlock (buffer, midiMessages, 0, numSamples);

            // Apply our delay effect to the new output..
            applyDelay (buffer, delayBuffer, delayParamValue);

            // Apply our gain change to the outgoing data..
            applyGain (buffer, delayBuffer, gainParamValue);

            // Now ask the host for the current time so we can store it to be displayed later...
            updateCurrentTimeInfoFromHost();
        */
    }
    
    pub fn apply_gain<FloatType>(&mut self, 
        buffer:       &mut AudioBuffer<FloatType>,
        delay_buffer: &mut AudioBuffer<FloatType>,
        gain_level:   f32)  {
    
        todo!();
        /*
            ignoreUnused (delayBuffer);

            for (auto channel = 0; channel < getTotalNumOutputChannels(); ++channel)
                buffer.applyGain (channel, 0, buffer.getNumSamples(), gainLevel);
        */
    }
    
    pub fn apply_delay<FloatType>(&mut self, 
        buffer:       &mut AudioBuffer<FloatType>,
        delay_buffer: &mut AudioBuffer<FloatType>,
        delay_level:  f32)  {
    
        todo!();
        /*
            auto numSamples = buffer.getNumSamples();

            auto delayPos = 0;

            for (auto channel = 0; channel < getTotalNumOutputChannels(); ++channel)
            {
                auto channelData = buffer.getWritePointer (channel);
                auto delayData = delayBuffer.getWritePointer (jmin (channel, delayBuffer.getNumChannels() - 1));
                delayPos = delayPosition;

                for (auto i = 0; i < numSamples; ++i)
                {
                    auto in = channelData[i];
                    channelData[i] += delayData[delayPos];
                    delayData[delayPos] = (delayData[delayPos] + in) * delayLevel;

                    if (++delayPos >= delayBuffer.getNumSamples())
                        delayPos = 0;
                }
            }

            delayPosition = delayPos;
        */
    }
}

pub trait GetTrackProperties {

    fn get_track_properties(&self) -> AudioProcessorTrackProperties;
}

impl<'a> GetTrackProperties for AloeDemoPluginAudioProcessor<'a> {

    fn get_track_properties(&self) -> AudioProcessorTrackProperties {
        
        todo!();
        /*
            const ScopedLock sl (trackPropertiesLock);
            return trackProperties;
        */
    }
}

pub trait InitializeSynth {

    fn initialise_synth(&mut self);
}

impl<'a> InitializeSynth for AloeDemoPluginAudioProcessor<'a> {

    fn initialise_synth(&mut self)  {
        
        todo!();
        /*
            auto numVoices = 8;

            // Add some voices...
            for (auto i = 0; i < numVoices; ++i)
                synth.addVoice (new SineWaveVoice());

            // ..and give the synth a sound to play
            synth.addSound (new SineWaveSound());
        */
    }
}

impl<'a> GetCurrentProgram for AloeDemoPluginAudioProcessor<'a> {
    fn get_current_program(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
}

impl<'a> GetNumPrograms for AloeDemoPluginAudioProcessor<'a> {
    fn get_num_programs(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
}

impl<'a> SetCurrentProgram for AloeDemoPluginAudioProcessor<'a> {
    fn set_current_program(&mut self, _0: i32)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> GetProgramName for AloeDemoPluginAudioProcessor<'a> {
    fn get_program_name(&mut self, _0: i32) -> String {
        
        todo!();
        /*
            return "None";
        */
    }
}

impl<'a> ChangeProgramName for AloeDemoPluginAudioProcessor<'a> {

    fn change_program_name(
        &mut self, 
        _0: i32,
        _1: &str)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> GetTailLengthSeconds for AloeDemoPluginAudioProcessor<'a> {
    fn get_tail_length_seconds(&self) -> f64 {
        
        todo!();
        /*
            return 0.0;
        */
    }
}

pub trait Reset {

    fn reset(&mut self);
}

impl<'a> Reset for AloeDemoPluginAudioProcessor<'a> {
    fn reset(&mut self)  {
        
        todo!();
        /*
            // Use this method as the place to clear any delay lines, buffers, etc, as it
            // means there's been a break in the audio's continuity.
            delayBufferFloat .clear();
            delayBufferDouble.clear();
        */
    }
}

impl<'a> ReleaseResources for AloeDemoPluginAudioProcessor<'a> {
    fn release_resources(&mut self)  {
        
        todo!();
        /*
            // When playback stops, you can use this as an opportunity to free up any
            // spare memory, etc.
            keyboardState.reset();
        */
    }
}

impl<'a> HasEditor for AloeDemoPluginAudioProcessor<'a> {
    fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

impl<'a> GetName for AloeDemoPluginAudioProcessor<'a> {
    fn get_name(&self) -> String {
        
        todo!();
        /*
            return "AudioPluginDemo";
        */
    }
}

impl<'a> AcceptsMidi for AloeDemoPluginAudioProcessor<'a> {
    fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

impl<'a> ProducesMidi for AloeDemoPluginAudioProcessor<'a> {
    fn produces_midi(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

pub trait GetBusesProperties {

    fn get_buses_properties() -> AudioProcessorBusesProperties;
}

impl<'a> GetBusesProperties for AloeDemoPluginAudioProcessor<'a> {

    fn get_buses_properties() -> AudioProcessorBusesProperties {
        
        todo!();
        /*
            return BusesProperties().withInput  ("Input",  AudioChannelSet::stereo(), true)
                                    .withOutput ("Output", AudioChannelSet::stereo(), true);
        */
    }
}

pub trait UpdateCurrentTimeInfoFromHose {

    fn update_current_time_info_from_host(&mut self);
}

impl<'a> UpdateCurrentTimeInfoFromHose for AloeDemoPluginAudioProcessor<'a> {

    fn update_current_time_info_from_host(&mut self)  {
        
        todo!();
        /*
            const auto newInfo = [&]
            {
                if (auto* ph = getPlayHead())
                {
                    AudioPlayHeadCurrentPositionInfo result;

                    if (ph->getCurrentPosition (result))
                        return result;
                }

                // If the host fails to provide the current time, we'll just use default values
                AudioPlayHeadCurrentPositionInfo result;
                result.resetToDefault();
                return result;
            }();

            lastPosInfo.set (newInfo);
        */
    }
}
