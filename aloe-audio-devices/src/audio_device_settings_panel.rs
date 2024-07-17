crate::ix!();

///------------------------
#[no_copy]
#[leak_detector]
pub struct AudioDeviceSettingsPanel<'a> {
    base:                          Component<'a>,
    ty:                            &'a mut AudioIODeviceType,
    setup:                         AudioDeviceSetupDetails<'a>,
    output_device_drop_down:       Box<ComboBox<'a>>,
    input_device_drop_down:        Box<ComboBox<'a>>,
    sample_rate_drop_down:         Box<ComboBox<'a>>,
    buffer_size_drop_down:         Box<ComboBox<'a>>,
    output_device_label:           Box<Label<'a>>,
    input_device_label:            Box<Label<'a>>,
    sample_rate_label:             Box<Label<'a>>,
    buffer_size_label:             Box<Label<'a>>,
    input_chan_label:              Box<Label<'a>>,
    output_chan_label:             Box<Label<'a>>,
    test_button:                   Box<TextButton<'a>>,
    input_level_meter:             Box<Component<'a>>,
    show_ui_button:                Box<TextButton<'a>>,
    show_advanced_settings_button: Box<TextButton<'a>>,
    reset_device_button:           Box<TextButton<'a>>,
    input_chan_list:               Box<ChannelSelectorListBox<'a>>,
    output_chan_list:              Box<ChannelSelectorListBox<'a>>,
}

impl<'a> ChangeListener for AudioDeviceSettingsPanel<'a> {

    fn change_listener_callback(&mut self, _0: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            updateAllControls();
        */
    }
}

impl<'a> Drop for AudioDeviceSettingsPanel<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            setup.manager->removeChangeListener (this);
         */
    }
}

impl<'a> AudioDeviceSettingsPanel<'a> {

    pub fn new(
        t:                                 &mut AudioIODeviceType,
        setup_details:                     &mut AudioDeviceSetupDetails,
        hide_advanced_options_with_button: bool) -> Self {
    
        todo!();
        /*
        : ty(t),
        : setup(setupDetails),

            if (hideAdvancedOptionsWithButton)
            {
                showAdvancedSettingsButton.reset (new TextButton (TRANS("Show advanced settings...")));
                addAndMakeVisible (showAdvancedSettingsButton.get());
                showAdvancedSettingsButton->setClickingTogglesState (true);
                showAdvancedSettingsButton->onClick = [this] { toggleAdvancedSettings(); };
            }

            type.scanForDevices();

            setup.manager->addChangeListener (this);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (auto* parent = findParentComponentOfClass<AudioDeviceSelectorComponent>())
            {
                Rectangle<int> r (proportionOfWidth (0.35f), 0, proportionOfWidth (0.6f), 3000);

                const int maxListBoxHeight = 100;
                const int h = parent->getItemHeight();
                const int space = h / 4;

                if (outputDeviceDropDown != nullptr)
                {
                    auto row = r.removeFromTop (h);

                    if (testButton != nullptr)
                    {
                        testButton->changeWidthToFitText (h);
                        testButton->setBounds (row.removeFromRight (testButton->getWidth()));
                        row.removeFromRight (space);
                    }

                    outputDeviceDropDown->setBounds (row);
                    r.removeFromTop (space);
                }

                if (inputDeviceDropDown != nullptr)
                {
                    auto row = r.removeFromTop (h);

                    inputLevelMeter->setBounds (row.removeFromRight (testButton != nullptr ? testButton->getWidth() : row.getWidth() / 6));
                    row.removeFromRight (space);
                    inputDeviceDropDown->setBounds (row);
                    r.removeFromTop (space);
                }

                if (outputChanList != nullptr)
                {
                    outputChanList->setRowHeight (jmin (22, h));
                    outputChanList->setBounds (r.removeFromTop (outputChanList->getBestHeight (maxListBoxHeight)));
                    outputChanLabel->setBounds (0, outputChanList->getBounds().getCentreY() - h / 2, r.getX(), h);
                    r.removeFromTop (space);
                }

                if (inputChanList != nullptr)
                {
                    inputChanList->setRowHeight (jmin (22, h));
                    inputChanList->setBounds (r.removeFromTop (inputChanList->getBestHeight (maxListBoxHeight)));
                    inputChanLabel->setBounds (0, inputChanList->getBounds().getCentreY() - h / 2, r.getX(), h);
                    r.removeFromTop (space);
                }

                r.removeFromTop (space * 2);

                if (showAdvancedSettingsButton != nullptr
                    && sampleRateDropDown != nullptr && bufferSizeDropDown != nullptr)
                {
                    showAdvancedSettingsButton->setBounds (r.removeFromTop (h));
                    r.removeFromTop (space);
                    showAdvancedSettingsButton->changeWidthToFitText();
                }

                auto advancedSettingsVisible = showAdvancedSettingsButton == nullptr
                                                  || showAdvancedSettingsButton->getToggleState();

                if (sampleRateDropDown != nullptr)
                {
                    sampleRateDropDown->setVisible (advancedSettingsVisible);

                    if (advancedSettingsVisible)
                    {
                        sampleRateDropDown->setBounds (r.removeFromTop (h));
                        r.removeFromTop (space);
                    }
                }

                if (bufferSizeDropDown != nullptr)
                {
                    bufferSizeDropDown->setVisible (advancedSettingsVisible);

                    if (advancedSettingsVisible)
                    {
                        bufferSizeDropDown->setBounds (r.removeFromTop (h));
                        r.removeFromTop (space);
                    }
                }

                r.removeFromTop (space);

                if (showUIButton != nullptr || resetDeviceButton != nullptr)
                {
                    auto buttons = r.removeFromTop (h);

                    if (showUIButton != nullptr)
                    {
                        showUIButton->setVisible (advancedSettingsVisible);
                        showUIButton->changeWidthToFitText (h);
                        showUIButton->setBounds (buttons.removeFromLeft (showUIButton->getWidth()));
                        buttons.removeFromLeft (space);
                    }

                    if (resetDeviceButton != nullptr)
                    {
                        resetDeviceButton->setVisible (advancedSettingsVisible);
                        resetDeviceButton->changeWidthToFitText (h);
                        resetDeviceButton->setBounds (buttons.removeFromLeft (resetDeviceButton->getWidth()));
                    }

                    r.removeFromTop (space);
                }

                setSize (getWidth(), r.getY());
            }
            else
            {
                jassertfalse;
            }
        */
    }
    
    pub fn update_config(&mut self, 
        update_output_device: bool,
        update_input_device:  bool,
        update_sample_rate:   bool,
        update_buffer_size:   bool)  {
        
        todo!();
        /*
            auto config = setup.manager->getAudioDeviceSetup();
            String error;

            if (updateOutputDevice || updateInputDevice)
            {
                if (outputDeviceDropDown != nullptr)
                    config.outputDeviceName = outputDeviceDropDown->getSelectedId() < 0 ? String()
                                                                                        : outputDeviceDropDown->getText();

                if (inputDeviceDropDown != nullptr)
                    config.inputDeviceName = inputDeviceDropDown->getSelectedId() < 0 ? String()
                                                                                      : inputDeviceDropDown->getText();

                if (! type.hasSeparateInputsAndOutputs())
                    config.inputDeviceName = config.outputDeviceName;

                if (updateInputDevice)
                    config.useDefaultInputChannels = true;
                else
                    config.useDefaultOutputChannels = true;

                error = setup.manager->setAudioDeviceSetup (config, true);

                showCorrectDeviceName (inputDeviceDropDown.get(), true);
                showCorrectDeviceName (outputDeviceDropDown.get(), false);

                updateControlPanelButton();
                resized();
            }
            else if (updateSampleRate)
            {
                if (sampleRateDropDown->getSelectedId() > 0)
                {
                    config.sampleRate = sampleRateDropDown->getSelectedId();
                    error = setup.manager->setAudioDeviceSetup (config, true);
                }
            }
            else if (updateBufferSize)
            {
                if (bufferSizeDropDown->getSelectedId() > 0)
                {
                    config.bufferSize = bufferSizeDropDown->getSelectedId();
                    error = setup.manager->setAudioDeviceSetup (config, true);
                }
            }

            if (error.isNotEmpty())
                AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                                  TRANS("Error when trying to open audio device!"),
                                                  error);
        */
    }
    
    pub fn show_device_control_panel(&mut self) -> bool {
        
        todo!();
        /*
            if (auto* device = setup.manager->getCurrentAudioDevice())
            {
                Component modalWindow;
                modalWindow.setOpaque (true);
                modalWindow.addToDesktop (0);
                modalWindow.enterModalState();

                return device->showControlPanel();
            }

            return false;
        */
    }
    
    pub fn toggle_advanced_settings(&mut self)  {
        
        todo!();
        /*
            showAdvancedSettingsButton->setButtonText ((showAdvancedSettingsButton->getToggleState() ? "Hide " : "Show ")
                                                       + String ("advanced settings..."));
            resized();
        */
    }
    
    pub fn show_device_ui_panel(&mut self)  {
        
        todo!();
        /*
            if (showDeviceControlPanel())
            {
                setup.manager->closeAudioDevice();
                setup.manager->restartLastAudioDevice();
                getTopLevelComponent()->toFront (true);
            }
        */
    }
    
    pub fn play_test_sound(&mut self)  {
        
        todo!();
        /*
            setup.manager->playTestSound();
        */
    }
    
    pub fn update_all_controls(&mut self)  {
        
        todo!();
        /*
            updateOutputsComboBox();
            updateInputsComboBox();

            updateControlPanelButton();
            updateResetButton();

            if (auto* currentDevice = setup.manager->getCurrentAudioDevice())
            {
                if (setup.maxNumOutputChannels > 0
                     && setup.minNumOutputChannels < setup.manager->getCurrentAudioDevice()->getOutputChannelNames().size())
                {
                    if (outputChanList == nullptr)
                    {
                        outputChanList.reset (new ChannelSelectorListBox (setup, ChannelSelectorListBox::audioOutputType,
                                                                          TRANS ("(no audio output channels found)")));
                        addAndMakeVisible (outputChanList.get());
                        outputChanLabel.reset (new Label ({}, TRANS("Active output channels:")));
                        outputChanLabel->setJustificationType (Justification::centredRight);
                        outputChanLabel->attachToComponent (outputChanList.get(), true);
                    }

                    outputChanList->refresh();
                }
                else
                {
                    outputChanLabel.reset();
                    outputChanList.reset();
                }

                if (setup.maxNumInputChannels > 0
                     && setup.minNumInputChannels < setup.manager->getCurrentAudioDevice()->getInputChannelNames().size())
                {
                    if (inputChanList == nullptr)
                    {
                        inputChanList.reset (new ChannelSelectorListBox (setup, ChannelSelectorListBox::audioInputType,
                                                                         TRANS("(no audio input channels found)")));
                        addAndMakeVisible (inputChanList.get());
                        inputChanLabel.reset (new Label ({}, TRANS("Active input channels:")));
                        inputChanLabel->setJustificationType (Justification::centredRight);
                        inputChanLabel->attachToComponent (inputChanList.get(), true);
                    }

                    inputChanList->refresh();
                }
                else
                {
                    inputChanLabel.reset();
                    inputChanList.reset();
                }

                updateSampleRateComboBox (currentDevice);
                updateBufferSizeComboBox (currentDevice);
            }
            else
            {
                jassert (setup.manager->getCurrentAudioDevice() == nullptr); // not the correct device type!

                inputChanLabel.reset();
                outputChanLabel.reset();
                sampleRateLabel.reset();
                bufferSizeLabel.reset();

                inputChanList.reset();
                outputChanList.reset();
                sampleRateDropDown.reset();
                bufferSizeDropDown.reset();

                if (outputDeviceDropDown != nullptr)
                    outputDeviceDropDown->setSelectedId (-1, dontSendNotification);

                if (inputDeviceDropDown != nullptr)
                    inputDeviceDropDown->setSelectedId (-1, dontSendNotification);
            }

            sendLookAndFeelChange();
            resized();
            setSize (getWidth(), getLowestY() + 4);
        */
    }
    
    pub fn reset_device(&mut self)  {
        
        todo!();
        /*
            setup.manager->closeAudioDevice();
            setup.manager->restartLastAudioDevice();
        */
    }
    
    pub fn show_correct_device_name(&mut self, 
        box_:     *mut ComboBox,
        is_input: bool)  {
        
        todo!();
        /*
            if (box != nullptr)
            {
                auto* currentDevice = setup.manager->getCurrentAudioDevice();
                auto index = type.getIndexOfDevice (currentDevice, isInput);

                box->setSelectedId (index < 0 ? index : index + 1, dontSendNotification);

                if (testButton != nullptr && ! isInput)
                    testButton->setEnabled (index >= 0);
            }
        */
    }
    
    pub fn add_names_to_device_box(&mut self, 
        combo:     &mut ComboBox,
        is_inputs: bool)  {
        
        todo!();
        /*
            const Vec<String> devs (type.getDeviceNames (isInputs));

            combo.clear (dontSendNotification);

            for (int i = 0; i < devs.size(); ++i)
                combo.addItem (devs[i], i + 1);

            combo.addItem (getNoDeviceString(), -1);
            combo.setSelectedId (-1, dontSendNotification);
        */
    }
    
    pub fn get_lowesty(&self) -> i32 {
        
        todo!();
        /*
            int y = 0;

            for (auto* c : getChildren())
                y = jmax (y, c->getBottom());

            return y;
        */
    }
    
    pub fn update_control_panel_button(&mut self)  {
        
        todo!();
        /*
            auto* currentDevice = setup.manager->getCurrentAudioDevice();
            showUIButton.reset();

            if (currentDevice != nullptr && currentDevice->hasControlPanel())
            {
                showUIButton.reset (new TextButton (TRANS ("Control Panel"),
                                                    TRANS ("Opens the device's own control panel")));
                addAndMakeVisible (showUIButton.get());
                showUIButton->onClick = [this] { showDeviceUIPanel(); };
            }

            resized();
        */
    }
    
    pub fn update_reset_button(&mut self)  {
        
        todo!();
        /*
            if (auto* currentDevice = setup.manager->getCurrentAudioDevice())
            {
                if (currentDevice->hasControlPanel())
                {
                    if (resetDeviceButton == nullptr)
                    {
                        resetDeviceButton.reset (new TextButton (TRANS ("Reset Device"),
                                                                 TRANS ("Resets the audio interface - sometimes needed after changing a device's properties in its custom control panel")));
                        addAndMakeVisible (resetDeviceButton.get());
                        resetDeviceButton->onClick = [this] { resetDevice(); };
                        resized();
                    }

                    return;
                }
            }

            resetDeviceButton.reset();
        */
    }
    
    pub fn update_outputs_combo_box(&mut self)  {
        
        todo!();
        /*
            if (setup.maxNumOutputChannels > 0 || ! type.hasSeparateInputsAndOutputs())
            {
                if (outputDeviceDropDown == nullptr)
                {
                    outputDeviceDropDown.reset (new ComboBox());
                    outputDeviceDropDown->onChange = [this] { updateConfig (true, false, false, false); };

                    addAndMakeVisible (outputDeviceDropDown.get());

                    outputDeviceLabel.reset (new Label ({}, type.hasSeparateInputsAndOutputs() ? TRANS("Output:")
                                                                                               : TRANS("Device:")));
                    outputDeviceLabel->attachToComponent (outputDeviceDropDown.get(), true);

                    if (setup.maxNumOutputChannels > 0)
                    {
                        testButton.reset (new TextButton (TRANS("Test"), TRANS("Plays a test tone")));
                        addAndMakeVisible (testButton.get());
                        testButton->onClick = [this] { playTestSound(); };
                    }
                }

                addNamesToDeviceBox (*outputDeviceDropDown, false);
            }

            showCorrectDeviceName (outputDeviceDropDown.get(), false);
        */
    }
    
    pub fn update_inputs_combo_box(&mut self)  {
        
        todo!();
        /*
            if (setup.maxNumInputChannels > 0 && type.hasSeparateInputsAndOutputs())
            {
                if (inputDeviceDropDown == nullptr)
                {
                    inputDeviceDropDown.reset (new ComboBox());
                    inputDeviceDropDown->onChange = [this] { updateConfig (false, true, false, false); };
                    addAndMakeVisible (inputDeviceDropDown.get());

                    inputDeviceLabel.reset (new Label ({}, TRANS("Input:")));
                    inputDeviceLabel->attachToComponent (inputDeviceDropDown.get(), true);

                    inputLevelMeter.reset (new SimpleDeviceManagerInputLevelMeter (*setup.manager));
                    addAndMakeVisible (inputLevelMeter.get());
                }

                addNamesToDeviceBox (*inputDeviceDropDown, true);
            }

            showCorrectDeviceName (inputDeviceDropDown.get(), true);
        */
    }
    
    pub fn update_sample_rate_combo_box(&mut self, current_device: *mut AudioIODevice)  {
        
        todo!();
        /*
            if (sampleRateDropDown == nullptr)
            {
                sampleRateDropDown.reset (new ComboBox());
                addAndMakeVisible (sampleRateDropDown.get());

                sampleRateLabel.reset (new Label ({}, TRANS("Sample rate:")));
                sampleRateLabel->attachToComponent (sampleRateDropDown.get(), true);
            }
            else
            {
                sampleRateDropDown->clear();
                sampleRateDropDown->onChange = nullptr;
            }

            for (auto rate : currentDevice->getAvailableSampleRates())
            {
                auto intRate = roundToInt (rate);
                sampleRateDropDown->addItem (String (intRate) + " Hz", intRate);
            }

            sampleRateDropDown->setSelectedId (roundToInt (currentDevice->getCurrentSampleRate()), dontSendNotification);
            sampleRateDropDown->onChange = [this] { updateConfig (false, false, true, false); };
        */
    }
    
    pub fn update_buffer_size_combo_box(&mut self, current_device: *mut AudioIODevice)  {
        
        todo!();
        /*
            if (bufferSizeDropDown == nullptr)
            {
                bufferSizeDropDown.reset (new ComboBox());
                addAndMakeVisible (bufferSizeDropDown.get());

                bufferSizeLabel.reset (new Label ({}, TRANS("Audio buffer size:")));
                bufferSizeLabel->attachToComponent (bufferSizeDropDown.get(), true);
            }
            else
            {
                bufferSizeDropDown->clear();
                bufferSizeDropDown->onChange = nullptr;
            }

            auto currentRate = currentDevice->getCurrentSampleRate();

            if (currentRate == 0)
                currentRate = 48000.0;

            for (auto bs : currentDevice->getAvailableBufferSizes())
                bufferSizeDropDown->addItem (String (bs) + " samples (" + String (bs * 1000.0 / currentRate, 1) + " ms)", bs);

            bufferSizeDropDown->setSelectedId (currentDevice->getCurrentBufferSizeSamples(), dontSendNotification);
            bufferSizeDropDown->onChange = [this] { updateConfig (false, false, false, true); };
        */
    }
}
