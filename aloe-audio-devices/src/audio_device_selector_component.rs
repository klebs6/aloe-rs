crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_AudioDeviceSelectorComponent.h]

/**
  | A component containing controls to
  | let the user change the audio settings
  | of an AudioDeviceManager object.
  | 
  | Very easy to use - just create one of these
  | and show it to the user.
  | 
  | @see AudioDeviceManager
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioDeviceSelectorComponent<'a> {

    base:                              Component<'a>,
    base3:                             Timer,

    /**
      | The device manager that this component
      | is controlling
      |
      */
    device_manager:                    &'a mut AudioDeviceManager<'a>,
    device_type_drop_down:             Box<ComboBox<'a>>,
    device_type_drop_down_label:       Box<Label<'a>>,
    audio_device_settings_comp:        Box<Component<'a>>,
    audio_device_settings_comp_type:   String,
    item_height:                       i32, // default = 0
    min_output_channels:               i32,
    max_output_channels:               i32,
    min_input_channels:                i32,
    max_input_channels:                i32,
    show_channels_as_stereo_pairs:     bool,
    hide_advanced_options_with_button: bool,
    current_midi_outputs:              Vec<MidiDeviceInfo>,
    midi_inputs_list:                  Box<MidiInputSelectorComponentListBox<'a>>,
    midi_output_selector:              Box<ComboBox<'a>>,
    midi_inputs_label:                 Box<Label<'a>>,
    midi_output_label:                 Box<Label<'a>>,
    bluetooth_button:                  Box<TextButton<'a>>,
}

impl<'a> ChangeListener for AudioDeviceSelectorComponent<'a> {

    fn change_listener_callback(&mut self, _0: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            updateAllControls();
        */
    }
}

impl<'a> Drop for AudioDeviceSelectorComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        deviceManager.removeChangeListener (this);
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_AudioDeviceSelectorComponent.cpp]
impl<'a> AudioDeviceSelectorComponent<'a> {

    /**
      | Returns the standard height used for
      | items in the panel.
      |
      */
    pub fn get_item_height(&self) -> i32 {
        
        todo!();
        /*
            return itemHeight;
        */
    }

    /**
      | Creates the component.
      | 
      | If your app needs only output channels,
      | you might ask for a maximum of 0 input
      | channels, and the component won't display
      | any options for choosing the input channels.
      | And likewise if you're doing an input-only
      | app.
      | 
      | -----------
      | @param deviceManager
      | 
      | the device manager that this component
      | should control
      | ----------
      | @param minAudioInputChannels
      | 
      | the minimum number of audio input channels
      | that the application needs
      | ----------
      | @param maxAudioInputChannels
      | 
      | the maximum number of audio input channels
      | that the application needs
      | ----------
      | @param minAudioOutputChannels
      | 
      | the minimum number of audio output channels
      | that the application needs
      | ----------
      | @param maxAudioOutputChannels
      | 
      | the maximum number of audio output channels
      | that the application needs
      | ----------
      | @param showMidiInputOptions
      | 
      | if true, the component will allow the
      | user to select which midi inputs are
      | enabled
      | ----------
      | @param showMidiOutputSelector
      | 
      | if true, the component will let the user
      | choose a default midi output device
      | ----------
      | @param showChannelsAsStereoPairs
      | 
      | if true, channels will be treated as
      | pairs; if false, channels will be treated
      | as a set of separate mono channels.
      | ----------
      | @param hideAdvancedOptionsWithButton
      | 
      | if true, only the minimum amount of UI
      | components are shown, with an "advanced"
      | button that shows the rest of them
      |
      */
    pub fn new(
        dm:                                       &mut AudioDeviceManager,
        min_input_channels_to_use:                i32,
        max_input_channels_to_use:                i32,
        min_output_channels_to_use:               i32,
        max_output_channels_to_use:               i32,
        show_midi_input_options:                  bool,
        show_midi_output_selector:                bool,
        show_channels_as_stereo_pairs_to_use:     bool,
        hide_advanced_options_with_button_to_use: bool) -> Self {
    
        todo!();
        /*
        : device_manager(dm),
        : item_height(24),
        : min_output_channels(minOutputChannelsToUse),
        : max_output_channels(maxOutputChannelsToUse),
        : min_input_channels(minInputChannelsToUse),
        : max_input_channels(maxInputChannelsToUse),
        : show_channels_as_stereo_pairs(showChannelsAsStereoPairsToUse),
        : hide_advanced_options_with_button(hideAdvancedOptionsWithButtonToUse),

            jassert (minOutputChannels >= 0 && minOutputChannels <= maxOutputChannels);
        jassert (minInputChannels >= 0 && minInputChannels <= maxInputChannels);

        const Vec<Box<AudioIODeviceType>>& types = deviceManager.getAvailableDeviceTypes();

        if (types.size() > 1)
        {
            deviceTypeDropDown.reset (new ComboBox());

            for (int i = 0; i < types.size(); ++i)
                deviceTypeDropDown->addItem (types.getUnchecked(i)->getTypeName(), i + 1);

            addAndMakeVisible (deviceTypeDropDown.get());
            deviceTypeDropDown->onChange = [this] { updateDeviceType(); };

            deviceTypeDropDownLabel.reset (new Label ({}, TRANS("Audio device type:")));
            deviceTypeDropDownLabel->setJustificationType (Justification::centredRight);
            deviceTypeDropDownLabel->attachToComponent (deviceTypeDropDown.get(), true);
        }

        if (showMidiInputOptions)
        {
            midiInputsList.reset (new MidiInputSelectorComponentListBox (deviceManager,
                                                                         "(" + TRANS("No MIDI inputs available") + ")"));
            addAndMakeVisible (midiInputsList.get());

            midiInputsLabel.reset (new Label ({}, TRANS ("Active MIDI inputs:")));
            midiInputsLabel->setJustificationType (Justification::topRight);
            midiInputsLabel->attachToComponent (midiInputsList.get(), true);

            if (BluetoothMidiDevicePairingDialogue::isAvailable())
            {
                bluetoothButton.reset (new TextButton (TRANS("Bluetooth MIDI"), TRANS("Scan for bluetooth MIDI devices")));
                addAndMakeVisible (bluetoothButton.get());
                bluetoothButton->onClick = [this] { handleBluetoothButton(); };
            }
        }
        else
        {
            midiInputsList.reset();
            midiInputsLabel.reset();
            bluetoothButton.reset();
        }

        if (showMidiOutputSelector)
        {
            midiOutputSelector.reset (new ComboBox());
            addAndMakeVisible (midiOutputSelector.get());
            midiOutputSelector->onChange = [this] { updateMidiOutput(); };

            midiOutputLabel.reset (new Label ("lm", TRANS("MIDI Output:")));
            midiOutputLabel->attachToComponent (midiOutputSelector.get(), true);
        }
        else
        {
            midiOutputSelector.reset();
            midiOutputLabel.reset();
        }

        deviceManager.addChangeListener (this);
        updateAllControls();
        startTimer (1000);
        */
    }
    
    /**
      | Sets the standard height used for items
      | in the panel.
      |
      */
    pub fn set_item_height(&mut self, new_item_height: i32)  {
        
        todo!();
        /*
            itemHeight = newItemHeight;
        resized();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            Rectangle<int> r (proportionOfWidth (0.35f), 15, proportionOfWidth (0.6f), 3000);
        auto space = itemHeight / 4;

        if (deviceTypeDropDown != nullptr)
        {
            deviceTypeDropDown->setBounds (r.removeFromTop (itemHeight));
            r.removeFromTop (space * 3);
        }

        if (audioDeviceSettingsComp != nullptr)
        {
            audioDeviceSettingsComp->resized();
            audioDeviceSettingsComp->setBounds (r.removeFromTop (audioDeviceSettingsComp->getHeight())
                                                    .withX (0).withWidth (getWidth()));
            r.removeFromTop (space);
        }

        if (midiInputsList != nullptr)
        {
            midiInputsList->setRowHeight (jmin (22, itemHeight));
            midiInputsList->setBounds (r.removeFromTop (midiInputsList->getBestHeight (jmin (itemHeight * 8,
                                                                                             getHeight() - r.getY() - space - itemHeight))));
            r.removeFromTop (space);
        }

        if (bluetoothButton != nullptr)
        {
            bluetoothButton->setBounds (r.removeFromTop (24));
            r.removeFromTop (space);
        }

        if (midiOutputSelector != nullptr)
            midiOutputSelector->setBounds (r.removeFromTop (itemHeight));

        r.removeFromTop (itemHeight);
        setSize (getWidth(), r.getY());
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            // TODO
        // unfortunately, the AudioDeviceManager only gives us changeListenerCallbacks
        // if an audio device has changed, but not if a MIDI device has changed.
        // This needs to be implemented properly. Until then, we use a workaround
        // where we update the whole component once per second on a timer callback.
        updateAllControls();
        */
    }
    
    pub fn update_device_type(&mut self)  {
        
        todo!();
        /*
            if (auto* type = deviceManager.getAvailableDeviceTypes() [deviceTypeDropDown->getSelectedId() - 1])
        {
            audioDeviceSettingsComp.reset();
            deviceManager.setCurrentAudioDeviceType (type->getTypeName(), true);
            updateAllControls(); // needed in case the type hasn't actually changed
        }
        */
    }
    
    pub fn update_midi_output(&mut self)  {
        
        todo!();
        /*
            auto selectedId = midiOutputSelector->getSelectedId();

        if (selectedId == -1)
            deviceManager.setDefaultMidiOutputDevice ({});
        else
            deviceManager.setDefaultMidiOutputDevice (currentMidiOutputs[selectedId - 1].identifier);
        */
    }
    
    pub fn update_all_controls(&mut self)  {
        
        todo!();
        /*
            if (deviceTypeDropDown != nullptr)
            deviceTypeDropDown->setText (deviceManager.getCurrentAudioDeviceType(), dontSendNotification);

        if (audioDeviceSettingsComp == nullptr
             || audioDeviceSettingsCompType != deviceManager.getCurrentAudioDeviceType())
        {
            audioDeviceSettingsCompType = deviceManager.getCurrentAudioDeviceType();
            audioDeviceSettingsComp.reset();

            if (auto* type = deviceManager.getAvailableDeviceTypes() [deviceTypeDropDown == nullptr
                                                                       ? 0 : deviceTypeDropDown->getSelectedId() - 1])
            {
                AudioDeviceSetupDetails details;
                details.manager = &deviceManager;
                details.minNumInputChannels = minInputChannels;
                details.maxNumInputChannels = maxInputChannels;
                details.minNumOutputChannels = minOutputChannels;
                details.maxNumOutputChannels = maxOutputChannels;
                details.useStereoPairs = showChannelsAsStereoPairs;

                auto* sp = new AudioDeviceSettingsPanel (*type, details, hideAdvancedOptionsWithButton);
                audioDeviceSettingsComp.reset (sp);
                addAndMakeVisible (sp);
                sp->updateAllControls();
            }
        }

        if (midiInputsList != nullptr)
        {
            midiInputsList->updateDevices();
            midiInputsList->updateContent();
            midiInputsList->repaint();
        }

        if (midiOutputSelector != nullptr)
        {
            midiOutputSelector->clear();

            currentMidiOutputs = MidiOutput::getAvailableDevices();

            midiOutputSelector->addItem (getNoDeviceString(), -1);
            midiOutputSelector->addSeparator();

            auto defaultOutputIdentifier = deviceManager.getDefaultMidiOutputIdentifier();
            int i = 0;

            for (auto& out : currentMidiOutputs)
            {
                midiOutputSelector->addItem (out.name, i + 1);

                if (defaultOutputIdentifier.isNotEmpty() && out.identifier == defaultOutputIdentifier)
                    midiOutputSelector->setSelectedId (i + 1);

                ++i;
            }
        }

        resized();
        */
    }
    
    pub fn handle_bluetooth_button(&mut self)  {
        
        todo!();
        /*
            if (! RuntimePermissions::isGranted (RuntimePermissions::bluetoothMidi))
            RuntimePermissions::request (RuntimePermissions::bluetoothMidi, nullptr);

        if (RuntimePermissions::isGranted (RuntimePermissions::bluetoothMidi))
            BluetoothMidiDevicePairingDialogue::open();
        */
    }
    
    /**
      | Returns the ListBox that's being used
      | to show the midi inputs, or nullptr if
      | there isn't one.
      |
      */
    pub fn get_midi_input_selector_list_box(&self) -> *mut ListBox {
        
        todo!();
        /*
            return midiInputsList.get();
        */
    }
}
