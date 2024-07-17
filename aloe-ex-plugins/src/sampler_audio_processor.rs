crate::ix!();

#[no_copy]
#[leak_detector]
pub struct SamplerAudioProcessor<'a> {
    base:                AudioProcessor<'a>,
    commands:            CommandFifo<SamplerAudioProcessor<'a>>,
    mb:                  MemoryBlock,
    reader_factory:      Box<dyn AudioFormatReaderFactory>,
    sampler_sound:       Arc<MPESamplerSound>, //= std::make_shared<MPESamplerSound>();
    synthesiser:         MPESynthesizer,

    /**
      | This mutex is used to ensure we don't
      | modify the processor state during a call to
      | createEditor, which would cause the UI to
      | become desynched with the real state of the
      | processor.
      */
    command_queue_mutex: SpinLock,

    /**
      | This is used for visualising the current
      | playback position of each voice.
      */
    playback_positions:  [Atomic<f32>; MAX_VOICES],
}

impl<'a> Default for SamplerAudioProcessor<'a> {
    
    fn default() -> Self {
        todo!();
        /*
            : AudioProcessor (BusesProperties().withOutput ("Output", AudioChannelSet::stereo(), true))

            if (auto inputStream = createAssetInputStream ("cello.wav"))
            {
                inputStream->readIntoMemoryBlock (mb);
                readerFactory.reset (new MemoryAudioFormatReaderFactory (mb.getData(), mb.getSize()));
            }

            // Set up initial sample, which we load from a binary resource
            AudioFormatManager manager;
            manager.registerBasicFormats();
            auto reader = readerFactory->make (manager);
            jassert (reader != nullptr); // Failed to load resource!

            auto sound = samplerSound;
            auto sample = std::unique_ptr<Sample> (new Sample (*reader, 10.0));
            auto lengthInSeconds = sample->getLength() / sample->getSampleRate();
            sound->setLoopPointsInSeconds ({lengthInSeconds * 0.1, lengthInSeconds * 0.9 });
            sound->setSample (move (sample));

            // Start with the max number of voices
            for (auto i = 0; i != maxVoices; ++i)
                synthesiser.addVoice (new MPESamplerVoice (sound))
        */
    }
}

impl<'a> SamplerAudioProcessor<'a> {
    
    pub fn prepare_to_play(&mut self, 
        sample_rate: f64,
        _1:          i32)  {
        
        todo!();
        /*
            synthesiser.setCurrentPlaybackSampleRate (sampleRate);
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
            return layouts.getMainOutputChannelSet() == AudioChannelSet::mono()
                || layouts.getMainOutputChannelSet() == AudioChannelSet::stereo();
        */
    }
    
    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            // This function will be called from the message thread. We lock the command
            // queue to ensure that no messages are processed for the duration of this
            // call.
            SpinLock::ScopedLockType lock (commandQueueMutex);

            ProcessorState state;
            state.synthVoices          = synthesiser.getNumVoices();
            state.legacyModeEnabled    = synthesiser.isLegacyModeEnabled();
            state.legacyChannels       = synthesiser.getLegacyModeChannelRange();
            state.legacyPitchbendRange = synthesiser.getLegacyModePitchbendRange();
            state.voiceStealingEnabled = synthesiser.isVoiceStealingEnabled();
            state.mpeZoneLayout        = synthesiser.getZoneLayout();
            state.readerFactory        = readerFactory == nullptr ? nullptr : readerFactory->clone();

            auto sound = samplerSound;
            state.loopPointsSeconds = sound->getLoopPointsInSeconds();
            state.centreFrequencyHz = sound->getCentreFrequencyInHz();
            state.loopMode          = sound->getLoopMode();

            return new SamplerAudioProcessorEditor (*this, std::move (state));
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
            return "SamplerPlugin";
        */
    }
    
    pub fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn produces_midi(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn is_midi_effect(&self) -> bool {
        
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
    
    pub fn process_block(&mut self, 
        buffer: &mut AudioBuffer<f32>,
        midi:   &mut MidiBuffer)  {
        
        todo!();
        /*
            process (buffer, midi);
        */
    }
    
    pub fn process_block_f64(
        &mut self, 
        buffer: &mut AudioBuffer<f64>,
        midi:   &mut MidiBuffer
    ) {
        
        todo!();
        /*
            process (buffer, midi);
        */
    }

    /**
      | These should be called from the GUI thread,
      | and will block until the command buffer has
      | enough room to accept a command.
      */
    pub fn set_sample(&mut self, 
        fact:           Box<dyn AudioFormatReaderFactory>,
        format_manager: &mut AudioFormatManager)  {
        
        todo!();
        /*
            class SetSampleCommand
            {
            
                SetSampleCommand (std::unique_ptr<AudioFormatReaderFactory> r,
                                  std::unique_ptr<Sample> sampleIn,
                                  std::vector<std::unique_ptr<MPESamplerVoice>> newVoicesIn)
                    : readerFactory (std::move (r)),
                      sample (std::move (sampleIn)),
                      newVoices (std::move (newVoicesIn))
                {}

                void operator() (SamplerAudioProcessor& proc)
                {
                    proc.readerFactory = move (readerFactory);
                    auto sound = proc.samplerSound;
                    sound->setSample (std::move (sample));
                    auto numberOfVoices = proc.synthesiser.getNumVoices();
                    proc.synthesiser.clearVoices();

                    for (auto it = begin (newVoices); proc.synthesiser.getNumVoices() < numberOfVoices; ++it)
                    {
                        proc.synthesiser.addVoice (it->release());
                    }
                }

            
                std::unique_ptr<AudioFormatReaderFactory> readerFactory;
                std::unique_ptr<Sample> sample;
                std::vector<std::unique_ptr<MPESamplerVoice>> newVoices;
            };

            // Note that all allocation happens here, on the main message thread. Then,
            // we transfer ownership across to the audio thread.
            auto loadedSamplerSound = samplerSound;
            std::vector<std::unique_ptr<MPESamplerVoice>> newSamplerVoices;
            newSamplerVoices.reserve (maxVoices);

            for (auto i = 0; i != maxVoices; ++i)
                newSamplerVoices.emplace_back (new MPESamplerVoice (loadedSamplerSound));

            if (fact == nullptr)
            {
                commands.push (SetSampleCommand (move (fact),
                                                 nullptr,
                                                 move (newSamplerVoices)));
            }
            else if (auto reader = fact->make (formatManager))
            {
                commands.push (SetSampleCommand (move (fact),
                                                 std::unique_ptr<Sample> (new Sample (*reader, 10.0)),
                                                 move (newSamplerVoices)));
            }
        */
    }
    
    pub fn set_centre_frequency(&mut self, centre_frequency: f64)  {
        
        todo!();
        /*
            commands.push ([centreFrequency] (SamplerAudioProcessor& proc)
                           {
                               auto loaded = proc.samplerSound;
                               if (loaded != nullptr)
                                   loaded->setCentreFrequencyInHz (centreFrequency);
                           });
        */
    }
    
    pub fn set_loop_mode(&mut self, loop_mode: LoopMode)  {
        
        todo!();
        /*
            commands.push ([loopMode] (SamplerAudioProcessor& proc)
                           {
                               auto loaded = proc.samplerSound;
                               if (loaded != nullptr)
                                   loaded->setLoopMode (loopMode);
                           });
        */
    }
    
    pub fn set_loop_points(&mut self, loop_points: Range<f64>)  {
        
        todo!();
        /*
            commands.push ([loopPoints] (SamplerAudioProcessor& proc)
                           {
                               auto loaded = proc.samplerSound;
                               if (loaded != nullptr)
                                   loaded->setLoopPointsInSeconds (loopPoints);
                           });
        */
    }
    
    pub fn set_mpe_zone_layout(&mut self, layout: MPEZoneLayout)  {
        
        todo!();
        /*
            commands.push ([layout] (SamplerAudioProcessor& proc)
                           {
                               // setZoneLayout will lock internally, so we don't care too much about
                               // ensuring that the layout doesn't get copied or destroyed on the
                               // audio thread. If the audio glitches while updating midi settings
                               // it doesn't matter too much.
                               proc.synthesiser.setZoneLayout (layout);
                           });
        */
    }
    
    pub fn set_legacy_mode_enabled(&mut self, 
        pitchbend_range: i32,
        channel_range:   Range<i32>)  {
        
        todo!();
        /*
            commands.push ([pitchbendRange, channelRange] (SamplerAudioProcessor& proc)
                           {
                               proc.synthesiser.enableLegacyMode (pitchbendRange, channelRange);
                           });
        */
    }
    
    pub fn set_voice_stealing_enabled(&mut self, voice_stealing_enabled: bool)  {
        
        todo!();
        /*
            commands.push ([voiceStealingEnabled] (SamplerAudioProcessor& proc)
                           {
                               proc.synthesiser.setVoiceStealingEnabled (voiceStealingEnabled);
                           });
        */
    }
    
    pub fn set_number_of_voices(&mut self, number_of_voices: i32)  {
        
        todo!();
        /*
            // We don't want to call 'new' on the audio thread. Normally, we'd
            // construct things here, on the GUI thread, and then move them into the
            // command lambda. Unfortunately, C++11 doesn't have extended lambda
            // capture, so we use a custom struct instead.

            class SetNumVoicesCommand
            {
            
                SetNumVoicesCommand (std::vector<std::unique_ptr<MPESamplerVoice>> newVoicesIn)
                    : newVoices (std::move (newVoicesIn))
                {}

                void operator() (SamplerAudioProcessor& proc)
                {
                    if ((int) newVoices.size() < proc.synthesiser.getNumVoices())
                        proc.synthesiser.reduceNumVoices (int (newVoices.size()));
                    else
                        for (auto it = begin (newVoices); (size_t) proc.synthesiser.getNumVoices() < newVoices.size(); ++it)
                            proc.synthesiser.addVoice (it->release());
                }

            
                std::vector<std::unique_ptr<MPESamplerVoice>> newVoices;
            };

            numberOfVoices = std::min ((int) maxVoices, numberOfVoices);
            auto loadedSamplerSound = samplerSound;
            std::vector<std::unique_ptr<MPESamplerVoice>> newSamplerVoices;
            newSamplerVoices.reserve ((size_t) numberOfVoices);

            for (auto i = 0; i != numberOfVoices; ++i)
                newSamplerVoices.emplace_back (new MPESamplerVoice (loadedSamplerSound));

            commands.push (SetNumVoicesCommand (move (newSamplerVoices)));
        */
    }

    /**
      | These accessors are just for an 'overview'
      | and won't give the exact state of the audio
      | engine at a particular point in time.
      |
      | If you call getNumVoices(), get the result
      | '10', and then call getPlaybackPosiiton(9),
      | there's a chance the audio engine will have
      | been updated to remove some voices in the
      | meantime, so the returned value won't
      | correspond to an existing voice.
      */
    pub fn get_num_voices(&self) -> i32 {
        
        todo!();
        /*
            return synthesiser.getNumVoices();
        */
    }
    
    pub fn get_playback_position(&self, voice: i32) -> f32 {
        
        todo!();
        /*
            return playbackPositions.at ((size_t) voice);
        */
    }
    
    pub fn process<Element>(&mut self, 
        buffer:        &mut AudioBuffer<Element>,
        midi_messages: &mut MidiBuffer)  {
    
        todo!();
        /*
            // Try to acquire a lock on the command queue.
            // If we were successful, we pop all pending commands off the queue and
            // apply them to the processor.
            // If we weren't able to acquire the lock, it's because someone called
            // createEditor, which requires that the processor data model stays in
            // a valid state for the duration of the call.
            const GenericScopedTryLock<SpinLock> lock (commandQueueMutex);

            if (lock.isLocked())
                commands.call (*this);

            synthesiser.renderNextBlock (buffer, midiMessages, 0, buffer.getNumSamples());

            auto loadedSamplerSound = samplerSound;

            if (loadedSamplerSound->getSample() == nullptr)
                return;

            auto numVoices = synthesiser.getNumVoices();

            // Update the current playback positions
            for (auto i = 0; i < maxVoices; ++i)
            {
                auto* voicePtr = dynamic_cast<MPESamplerVoice*> (synthesiser.getVoice (i));

                if (i < numVoices && voicePtr != nullptr)
                    playbackPositions[(size_t) i] = static_cast<float> (voicePtr->getCurrentSamplePosition() / loadedSamplerSound->getSample()->getSampleRate());
                else
                    playbackPositions[(size_t) i] = 0.0f;
            }
        */
    }
}
