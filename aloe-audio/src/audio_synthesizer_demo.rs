crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Audio/AudioSynthesizerDemo.h]

#[no_copy]
#[leak_detector]
pub struct AudioSynthesizerDemo<'a> {

    base: Component<'a>,

    /**
      | if this PIP is running inside the demo
      | runner, we'll use the shared device
      | manager instead
      |
      */
    #[cfg(not(ALOE_DEMO_RUNNER))]
    audio_device_manager:    AudioDeviceManager<'a>,

    #[cfg(ALOE_DEMO_RUNNER)]
    audio_device_manager:    &mut AudioDeviceManager, // { getSharedAudioDeviceManager (0, 2) };

    keyboard_state:          MidiKeyboardState,
    audio_source_player:     AudioSourcePlayer,
    synth_audio_source:      SynthAudioSource<'a>,        // { keyboardState };
    keyboard_component:      MidiKeyboardComponent<'a>,   // { keyboardState, MidiKeyboardComponent::horizontalKeyboard};
    sine_button:             ToggleButton<'a>,            // { "Use sine wave" };
    sampled_button:          ToggleButton<'a>,            // { "Use sampled sound" };
    live_audio_display_comp: LiveScrollingAudioDisplay<'a>,
}

impl<'a> Default for AudioSynthesizerDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (keyboardComponent);

            addAndMakeVisible (sineButton);
            sineButton.setRadioGroupId (321);
            sineButton.setToggleState (true, dontSendNotification);
            sineButton.onClick = [this] { synthAudioSource.setUsingSineWaveSound(); };

            addAndMakeVisible (sampledButton);
            sampledButton.setRadioGroupId (321);
            sampledButton.onClick = [this] { synthAudioSource.setUsingSampledSound(); };

            addAndMakeVisible (liveAudioDisplayComp);
            audioDeviceManager.addAudioCallback (&liveAudioDisplayComp);
            audioSourcePlayer.setSource (&synthAudioSource);

           #ifndef ALOE_DEMO_RUNNER
            RuntimePermissions::request (RuntimePermissions::recordAudio,
                                         [this] (bool granted)
                                         {
                                             int numInputChannels = granted ? 2 : 0;
                                             audioDeviceManager.initialise (numInputChannels, 2, nullptr, true, {}, nullptr);
                                         });
           #endif

            audioDeviceManager.addAudioCallback (&audioSourcePlayer);
            audioDeviceManager.addMidiInputDeviceCallback ({}, &(synthAudioSource.midiCollector));

            setOpaque (true);
            setSize (640, 480)
        */
    }
}

impl<'a> Drop for AudioSynthesizerDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            audioSourcePlayer.setSource (nullptr);
            audioDeviceManager.removeMidiInputDeviceCallback ({}, &(synthAudioSource.midiCollector));
            audioDeviceManager.removeAudioCallback (&audioSourcePlayer);
            audioDeviceManager.removeAudioCallback (&liveAudioDisplayComp);
         */
    }
}

impl<'a> AudioSynthesizerDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            keyboardComponent   .setBounds (8, 96, getWidth() - 16, 64);
            sineButton          .setBounds (16, 176, 150, 24);
            sampledButton       .setBounds (16, 200, 150, 24);
            liveAudioDisplayComp.setBounds (8, 8, getWidth() - 16, 64);
        */
    }
}
