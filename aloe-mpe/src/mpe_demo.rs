crate::ix!();

#[no_copy]
#[leak_detector]
pub struct MPEDemo<'a> {

    base:  Component<'a>,

    /**
       if this PIP is running inside the demo
       runner, we'll use the shared device manager
       instead
      */
    #[cfg(not(ALOE_DEMO_RUNNER))]
    audio_device_manager: AudioDeviceManager<'a>,

    #[cfg(ALOE_DEMO_RUNNER)]
    audio_device_manager: &mut AudioDeviceManager, // { getSharedAudioDeviceManager (0, 2) };

    zone_layout:           MPEZoneLayout,
    colour_picker:         ZoneColourPicker,
    audio_setup_comp:      AudioDeviceSelectorComponent<'a>,
    mpe_setup_comp:        MPESetupComponent<'a>,
    zone_layout_comp:      ZoneLayoutComponent<'a>,
    visualiser_comp:       Visualiser<'a>,
    visualiser_viewport:   Viewport<'a>,
    visualiser_instrument: MPEInstrument,
    synth:                 MPESynthesizer,
    midi_collector:        MidiMessageCollector,
}

impl<'a> MpeSetupComponentListener for MPEDemo<'a> {

    fn zone_changed(&mut self, 
        is_lower_zone:            bool,
        num_member_channels:      i32,
        per_note_pitchbend_range: i32,
        master_pitchbend_range:   i32)  {
        
        todo!();
        /*
            auto* midiOutput = audioDeviceManager.getDefaultMidiOutput();
            if (midiOutput != nullptr)
            {
                if (isLowerZone)
                    midiOutput->sendBlockOfMessagesNow (MPEMessages::setLowerZone (numMemberChannels, perNotePitchbendRange, masterPitchbendRange));
                else
                    midiOutput->sendBlockOfMessagesNow (MPEMessages::setUpperZone (numMemberChannels, perNotePitchbendRange, masterPitchbendRange));
            }

            if (isLowerZone)
                zoneLayout.setLowerZone (numMemberChannels, perNotePitchbendRange, masterPitchbendRange);
            else
                zoneLayout.setUpperZone (numMemberChannels, perNotePitchbendRange, masterPitchbendRange);

            visualiserInstrument.setZoneLayout (zoneLayout);
            synth.setZoneLayout (zoneLayout);
            colourPicker.setZoneLayout (zoneLayout);
        */
    }
    
    fn all_zones_cleared(&mut self)  {
        
        todo!();
        /*
            auto* midiOutput = audioDeviceManager.getDefaultMidiOutput();
            if (midiOutput != nullptr)
                midiOutput->sendBlockOfMessagesNow (MPEMessages::clearAllZones());

            zoneLayout.clearAllZones();
            visualiserInstrument.setZoneLayout (zoneLayout);
            synth.setZoneLayout (zoneLayout);
            colourPicker.setZoneLayout (zoneLayout);
        */
    }
    
    fn legacy_mode_changed(&mut self, 
        legacy_mode_should_be_enabled: bool,
        pitchbend_range:               i32,
        channel_range:                 Range<i32>)  {
        
        todo!();
        /*
            colourPicker.setLegacyModeEnabled (legacyModeShouldBeEnabled);

            if (legacyModeShouldBeEnabled)
            {
                synth.enableLegacyMode (pitchbendRange, channelRange);
                visualiserInstrument.enableLegacyMode (pitchbendRange, channelRange);
            }
            else
            {
                synth.setZoneLayout (zoneLayout);
                visualiserInstrument.setZoneLayout (zoneLayout);
            }
        */
    }

    fn voice_stealing_enabled_changed(&mut self, voice_stealing_enabled: bool)  {
        
        todo!();
        /*
            synth.setVoiceStealingEnabled (voiceStealingEnabled);
        */
    }
    
    fn number_of_voices_changed(&mut self, number_of_voices: i32)  {
        
        todo!();
        /*
            if (numberOfVoices < synth.getNumVoices())
                synth.reduceNumVoices (numberOfVoices);
            else
                while (synth.getNumVoices() < numberOfVoices)
                    synth.addVoice (new MPEDemoSynthVoice());
        */
    }
}

impl<'a> MidiInputCallback for MPEDemo<'a> {

    fn handle_incoming_midi_message(&mut self, 
        source:  *mut MidiInput,
        message: &MidiMessage)  {
        
        todo!();
        /*
            visualiserInstrument.processNextMidiEvent (message);
            midiCollector.addMessageToQueue (message);
        */
    }
}

impl<'a> AudioIODeviceCallback for MPEDemo<'a> {

    fn audio_device_io_callback(&mut self, 
        input_channel_data:  *const *const f32,
        num_input_channels:  i32,
        output_channel_data: *mut *mut f32,
        num_output_channels: i32,
        num_samples:         i32)  {
        
        todo!();
        /*
            AudioBuffer<float> buffer (outputChannelData, numOutputChannels, numSamples);
            buffer.clear();

            MidiBuffer incomingMidi;
            midiCollector.removeNextBlockOfMessages (incomingMidi, numSamples);
            synth.renderNextBlock (buffer, incomingMidi, 0, numSamples);
        */
    }
    
    fn audio_device_about_to_start(&mut self, device: *mut dyn AudioIODeviceInterface)  {
        
        todo!();
        /*
            auto sampleRate = device->getCurrentSampleRate();
            midiCollector.reset (sampleRate);
            synth.setCurrentPlaybackSampleRate (sampleRate);
        */
    }
    
    fn audio_device_stopped(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> Default for MPEDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : audioSetupComp (audioDeviceManager, 0, 0, 0, 256, true, true, true, false),
              zoneLayoutComp (colourPicker),
              visualiserComp (colourPicker)

           #ifndef ALOE_DEMO_RUNNER
            audioDeviceManager.initialise (0, 2, nullptr, true, {}, nullptr);
           #endif

            audioDeviceManager.addMidiInputDeviceCallback ({}, this);
            audioDeviceManager.addAudioCallback (this);

            addAndMakeVisible (audioSetupComp);
            addAndMakeVisible (MPESetupComp);
            addAndMakeVisible (zoneLayoutComp);
            addAndMakeVisible (visualiserViewport);

            visualiserViewport.setScrollBarsShown (false, true);
            visualiserViewport.setViewedComponent (&visualiserComp, false);
            visualiserViewport.setViewPositionProportionately (0.5, 0.0);

            MPESetupComp.addListener (&zoneLayoutComp);
            MPESetupComp.addListener (this);
            visualiserInstrument.addListener (&visualiserComp);

            synth.setVoiceStealingEnabled (false);
            for (auto i = 0; i < 15; ++i)
                synth.addVoice (new MPEDemoSynthVoice());

            setSize (880, 720)
        */
    }
}

impl<'a> Drop for MPEDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            audioDeviceManager.removeMidiInputDeviceCallback ({}, this);
            audioDeviceManager.removeAudioCallback (this);
         */
    }
}

impl<'a> MPEDemo<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto visualiserCompWidth  = 2800;
            auto visualiserCompHeight = 300;
            auto zoneLayoutCompHeight = 60;
            auto audioSetupCompRelativeWidth = 0.55f;

            auto r = getLocalBounds();

            visualiserViewport.setBounds (r.removeFromBottom (visualiserCompHeight));
            visualiserComp    .setBounds ({ visualiserCompWidth,
                                            visualiserViewport.getHeight() - visualiserViewport.getScrollBarThickness() });

            zoneLayoutComp.setBounds (r.removeFromBottom (zoneLayoutCompHeight));
            audioSetupComp.setBounds (r.removeFromLeft (proportionOfWidth (audioSetupCompRelativeWidth)));
            MPESetupComp  .setBounds (r);
        */
    }
    
    
    
}
