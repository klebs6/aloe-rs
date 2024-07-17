crate::ix!();

/**
  | Allow an IAAAudioProcessorEditor to
  | register as a listener to receive new meter
  | values directly from the audio thread.
  */
pub trait MeterListener {
    fn handle_new_meter_value(&mut self, _0: i32, _1: f32);
}

#[no_copy]
#[leak_detector]
pub struct IAAEffectEditor<'a> {
    base:                        AudioProcessorEditor<'a>,
    base3:                       Timer,
    iaa_effect_processor:        &'a mut IAAEffectProcessor<'a>,
    parameters:                  &'a mut AudioProcessorValueTreeState<'a>,
    button_size:                 f32,         // default = 30.0f
    default_button_colour:       Colour,      // default = Colours::darkgrey
    rewind_button:               ShapeButton<'a>, // default = { "Rewind", defaultButtonColour, defaultButtonColour, defaultButtonColour }
    play_button:                 ShapeButton<'a>, // default = { "Play",   defaultButtonColour, defaultButtonColour, defaultButtonColour }
    record_button:               ShapeButton<'a>, // default = { "Record", defaultButtonColour, defaultButtonColour, defaultButtonColour }
    gain_slider:                 Slider<'a>,
    gain_attachment:             AudioProcessorValueTreeStateSliderAttachment<'a>, //= { parameters, "gain", gainSlider };
    meters:                      [SimpleMeter<'a>;2],
    switch_to_host_button:       ImageButton<'a>,
    transport_text:              Label<'a>,
    switch_to_host_button_label: Label<'a>,
    last_pos_info:               AudioPlayHeadCurrentPositionInfo,
}

impl<'a> MeterListener for IAAEffectEditor<'a> {

    /**
      | Called from the audio thread.
      |
      */
    fn handle_new_meter_value(
        &mut self, 
        channel: i32,
        value:   f32
    ) {
        
        todo!();
        /*
            meters[(size_t) channel].update (value);
        */
    }
}

impl<'a> Drop for IAAEffectEditor<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                iaaEffectProcessor.removeMeterListener (*this);
             */
    }
}

impl<'a> IAAEffectEditor<'a> {

    pub fn new(
        p:   &mut IAAEffectProcessor,
        vts: &mut AudioProcessorValueTreeState) -> Self {
    
        todo!();
        /*


            : AudioProcessorEditor (p),
                  iaaEffectProcessor (p),
                  parameters (vts)

                // Register for meter value updates.
                iaaEffectProcessor.addMeterListener (*this);

                gainSlider.setSliderStyle (Slider::SliderStyle::LinearVertical);
                gainSlider.setTextBoxStyle (Slider::TextEntryBoxPosition::TextBoxAbove, false, 60, 20);
                addAndMakeVisible (gainSlider);

                for (auto& meter : meters)
                    addAndMakeVisible (meter);

                // Configure all the graphics for the transport control.

                transportText.setFont (Font (Font::getDefaultMonospacedFontName(), 18.0f, Font::plain));
                transportText.setJustificationType (Justification::topLeft);
                addChildComponent (transportText);

                Path rewindShape;
                rewindShape.addRectangle (0.0, 0.0, 5.0, buttonSize);
                rewindShape.addTriangle (0.0, buttonSize * 0.5f, buttonSize, 0.0, buttonSize, buttonSize);
                rewindButton.setShape (rewindShape, true, true, false);
                rewindButton.onClick = [this]
                {
                    if (transportControllable())
                        iaaEffectProcessor.getPlayHead()->transportRewind();
                };
                addChildComponent (rewindButton);

                Path playShape;
                playShape.addTriangle (0.0, 0.0, 0.0, buttonSize, buttonSize, buttonSize / 2);
                playButton.setShape (playShape, true, true, false);
                playButton.onClick = [this]
                {
                    if (transportControllable())
                        iaaEffectProcessor.getPlayHead()->transportPlay (! lastPosInfo.isPlaying);
                };
                addChildComponent (playButton);

                Path recordShape;
                recordShape.addEllipse (0.0, 0.0, buttonSize, buttonSize);
                recordButton.setShape (recordShape, true, true, false);
                recordButton.onClick = [this]
                {
                    if (transportControllable())
                        iaaEffectProcessor.getPlayHead()->transportRecord (! lastPosInfo.isRecording);
                };
                addChildComponent (recordButton);

                // Configure the switch to host button.

                switchToHostButtonLabel.setFont (Font (Font::getDefaultMonospacedFontName(), 18.0f, Font::plain));
                switchToHostButtonLabel.setJustificationType (Justification::centredRight);
                switchToHostButtonLabel.setText ("Switch to\nhost app:", dontSendNotification);
                addChildComponent (switchToHostButtonLabel);

                switchToHostButton.onClick = [this]
                {
                    if (transportControllable())
                    {
                        PluginHostType hostType;
                        hostType.switchToHostApplication();
                    }
                };
                addChildComponent (switchToHostButton);

                auto screenSize = Desktop::getInstance().getDisplays().getPrimaryDisplay()->userArea;
                setSize (screenSize.getWidth(), screenSize.getHeight());

                resized();

                startTimerHz (60);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getBounds().reduced (20);

                gainSlider.setBounds (area.removeFromLeft (60));

                for (auto& meter : meters)
                {
                    area.removeFromLeft (10);
                    meter.setBounds (area.removeFromLeft (20));
                }

                area.removeFromLeft (20);
                transportText.setBounds (area.removeFromTop (120));

                auto navigationArea = area.removeFromTop ((int) buttonSize);
                rewindButton.setTopLeftPosition (navigationArea.getPosition());
                navigationArea.removeFromLeft ((int) buttonSize + 10);
                playButton.setTopLeftPosition (navigationArea.getPosition());
                navigationArea.removeFromLeft ((int) buttonSize + 10);
                recordButton.setTopLeftPosition (navigationArea.getPosition());

                area.removeFromTop (30);

                auto appSwitchArea = area.removeFromTop ((int) buttonSize);
                switchToHostButtonLabel.setBounds (appSwitchArea.removeFromLeft (100));
                appSwitchArea.removeFromLeft (5);
                switchToHostButton.setBounds (appSwitchArea.removeFromLeft ((int) buttonSize));
        */
    }

    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            auto timeInfoSuccess = iaaEffectProcessor.updateCurrentTimeInfoFromHost (lastPosInfo);
                transportText.setVisible (timeInfoSuccess);

                if (timeInfoSuccess)
                    updateTransportTextDisplay();

                updateTransportButtonsDisplay();

                updateSwitchToHostDisplay();
        */
    }
    
    pub fn transport_controllable(&mut self) -> bool {
        
        todo!();
        /*
            auto processorPlayHead = iaaEffectProcessor.getPlayHead();
                return processorPlayHead != nullptr && processorPlayHead->canControlTransport();
        */
    }

    /**
       quick-and-dirty function to format
       a timecode string
      */
    pub fn time_to_timecode_string(&mut self, seconds: f64) -> String {
        
        todo!();
        /*
            auto millisecs = roundToInt (seconds * 1000.0);
                auto absMillisecs = std::abs (millisecs);

                return String::formatted ("%02d:%02d:%02d.%03d",
                                          millisecs / 360000,
                                          (absMillisecs / 60000) % 60,
                                          (absMillisecs / 1000) % 60,
                                          absMillisecs % 1000);
        */
    }

    /**
       A quick-and-dirty function to format
       a bars/beats string.
      */
    pub fn quarter_note_position_to_bars_beats_string(&mut self, 
        quarter_notes: f64,
        numerator:     i32,
        denominator:   i32) -> String {
        
        todo!();
        /*
            if (numerator == 0 || denominator == 0)
                    return "1|1|000";

                auto quarterNotesPerBar = (numerator * 4 / denominator);
                auto beats  = (fmod (quarterNotes, quarterNotesPerBar) / quarterNotesPerBar) * numerator;

                auto bar    = ((int) quarterNotes) / quarterNotesPerBar + 1;
                auto beat   = ((int) beats) + 1;
                auto ticks  = ((int) (fmod (beats, 1.0) * 960.0 + 0.5));

                return String::formatted ("%d|%d|%03d", bar, beat, ticks);
        */
    }
    
    pub fn update_transport_text_display(&mut self)  {
        
        todo!();
        /*
            MemoryOutputStream displayText;

                displayText << "[" << SystemStats::getALOEVersion() << "]\n"
                            << String (lastPosInfo.bpm, 2) << " bpm\n"
                            << lastPosInfo.timeSigNumerator << '/' << lastPosInfo.timeSigDenominator << "\n"
                            << timeToTimecodeString (lastPosInfo.timeInSeconds) << "\n"
                            << quarterNotePositionToBarsBeatsString (lastPosInfo.ppqPosition,
                                                         lastPosInfo.timeSigNumerator,
                                                         lastPosInfo.timeSigDenominator) << "\n";

                if (lastPosInfo.isRecording)
                    displayText << "(recording)";
                else if (lastPosInfo.isPlaying)
                    displayText << "(playing)";

                transportText.setText (displayText.toString(), dontSendNotification);
        */
    }
    
    pub fn update_transport_buttons_display(&mut self)  {
        
        todo!();
        /*
            auto visible = iaaEffectProcessor.getPlayHead() != nullptr
                            && iaaEffectProcessor.getPlayHead()->canControlTransport();

                if (rewindButton.isVisible() != visible)
                {
                    rewindButton.setVisible (visible);
                    playButton.setVisible   (visible);
                    recordButton.setVisible (visible);
                }

                if (visible)
                {
                    auto playColour = lastPosInfo.isPlaying ? Colours::green : defaultButtonColour;
                    playButton.setColours (playColour, playColour, playColour);
                    playButton.repaint();

                    auto recordColour = lastPosInfo.isRecording ? Colours::red : defaultButtonColour;
                    recordButton.setColours (recordColour, recordColour, recordColour);
                    recordButton.repaint();
                }
        */
    }
    
    pub fn update_switch_to_host_display(&mut self)  {
        
        todo!();
        /*
            PluginHostType hostType;
                auto visible = hostType.isInterAppAudioConnected();

                if (switchToHostButtonLabel.isVisible() != visible)
                {
                    switchToHostButtonLabel.setVisible (visible);
                    switchToHostButton.setVisible (visible);

                    if (visible)
                    {
                        auto icon = hostType.getHostIcon ((int) buttonSize);
                        switchToHostButton.setImages(false, true, true,
                                                     icon, 1.0, Colours::transparentBlack,
                                                     icon, 1.0, Colours::transparentBlack,
                                                     icon, 1.0, Colours::transparentBlack);
                    }
                }
        */
    }
}
