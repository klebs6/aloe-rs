crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Audio/AudioSettingsDemo.h]

#[no_copy]
#[leak_detector]
pub struct AudioSettingsDemo<'a> {

    base:                 Component<'a>,

    /**
      | if this PIP is running inside the demo
      | runner, we'll use the shared device
      | manager instead
      |
      */
    #[cfg(not(ALOE_DEMO_RUNNER))]
    audio_device_manager: AudioDeviceManager<'a>,

    #[cfg(ALOE_DEMO_RUNNER)]
    audio_device_manager: &mut AudioDeviceManager, // { getSharedAudioDeviceManager() };

    audio_setup_comp:     Box<AudioDeviceSelectorComponent<'a>>,
    diagnostics_box:      TextEditor<'a>,
}

impl<'a> ChangeListener for AudioSettingsDemo<'a> {
    fn change_listener_callback(&mut self, _: *mut aloe_events::ChangeBroadcaster<'_>) { todo!() }
}

impl<'a> Default for AudioSettingsDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

           #ifndef ALOE_DEMO_RUNNER
            RuntimePermissions::request (RuntimePermissions::recordAudio,
                                         [this] (bool granted)
                                         {
                                             int numInputChannels = granted ? 2 : 0;
                                             audioDeviceManager.initialise (numInputChannels, 2, nullptr, true, {}, nullptr);
                                         });
           #endif

            audioSetupComp.reset (new AudioDeviceSelectorComponent (audioDeviceManager,
                                                                    0, 256, 0, 256, true, true, true, false));
            addAndMakeVisible (audioSetupComp.get());

            addAndMakeVisible (diagnosticsBox);
            diagnosticsBox.setMultiLine (true);
            diagnosticsBox.setReturnKeyStartsNewLine (true);
            diagnosticsBox.setReadOnly (true);
            diagnosticsBox.setScrollbarsShown (true);
            diagnosticsBox.setCaretVisible (false);
            diagnosticsBox.setPopupMenuEnabled (true);

            audioDeviceManager.addChangeListener (this);

            logMessage ("Audio device diagnostics:\n");
            dumpDeviceInfo();

            setSize (500, 600)
        */
    }
}

impl<'a> Drop for AudioSettingsDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            audioDeviceManager.removeChangeListener (this);
         */
    }
}

impl<'a> AudioSettingsDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r =  getLocalBounds().reduced (4);
            audioSetupComp->setBounds (r.removeFromTop (proportionOfHeight (0.65f)));
            diagnosticsBox.setBounds (r);
        */
    }
    
    pub fn dump_device_info(&mut self)  {
        
        todo!();
        /*
            logMessage ("--------------------------------------");
            logMessage ("Current audio device type: " + (audioDeviceManager.getCurrentDeviceTypeObject() != nullptr
                                                         ? audioDeviceManager.getCurrentDeviceTypeObject()->getTypeName()
                                                         : "<none>"));

            if (AudioIODevice* device = audioDeviceManager.getCurrentAudioDevice())
            {
                logMessage ("Current audio device: "   + device->getName().quoted());
                logMessage ("Sample rate: "    + String (device->getCurrentSampleRate()) + " Hz");
                logMessage ("Block size: "     + String (device->getCurrentBufferSizeSamples()) + " samples");
                logMessage ("Output Latency: " + String (device->getOutputLatencyInSamples())   + " samples");
                logMessage ("Input Latency: "  + String (device->getInputLatencyInSamples())    + " samples");
                logMessage ("Bit depth: "      + String (device->getCurrentBitDepth()));
                logMessage ("Input channel names: "    + device->getInputChannelNames().joinIntoString (", "));
                logMessage ("Active input channels: "  + getListOfActiveBits (device->getActiveInputChannels()));
                logMessage ("Output channel names: "   + device->getOutputChannelNames().joinIntoString (", "));
                logMessage ("Active output channels: " + getListOfActiveBits (device->getActiveOutputChannels()));
            }
            else
            {
                logMessage ("No audio device open");
            }
        */
    }
    
    pub fn log_message(&mut self, m: &String)  {
        
        todo!();
        /*
            diagnosticsBox.moveCaretToEnd();
            diagnosticsBox.insertTextAtCaret (m + newLine);
        */
    }
    
    pub fn change_listener_callback(&mut self, _0: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            dumpDeviceInfo();
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            diagnosticsBox.applyFontToAllText (diagnosticsBox.getFont());
        */
    }
    
    pub fn get_list_of_active_bits(b: &BigInteger) -> String {
        
        todo!();
        /*
            StringArray bits;

            for (int i = 0; i <= b.getHighestBit(); ++i)
                if (b[i])
                    bits.add (String (i));

            return bits.joinIntoString (", ");
        */
    }
}
