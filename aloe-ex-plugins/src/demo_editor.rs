crate::ix!();

/**
  | This is the editor component that our
  | filter will display.
  |
  */
pub struct AloeDemoPluginAudioProcessorEditor<'a> {
    base:                   AudioProcessorEditor<'a>,
    base2:                  Timer,
    midi_keyboard:          MidiKeyboardComponent<'a>,
    timecode_display_label: Label<'a>,
    gain_label:             Label<'a>, // default = { {}, "Throughput level:"  }
    delay_label:            Label<'a>, // default = { {}, "Delay:"  }
    gain_slider:            Slider<'a>,
    delay_slider:           Slider<'a>,
    gain_attachment:        AudioProcessorValueTreeStateSliderAttachment<'a>,
    delay_attachment:       AudioProcessorValueTreeStateSliderAttachment<'a>,
    background_colour:      Colour,

    /**
      | these are used to persist the UI's size
      | - the values are stored along with the
      | filter's other parameters, and the UI
      | component will update them when it gets
      | resized.
      */
    last_ui_width:          Value<'a>,

    /**
       | these are used to persist the UI's size
       | - the values are stored along with the
       | filter's other parameters, and the UI
       | component will update them when it gets
       | resized.
      */
    last_ui_height:         Value<'a>,
}

impl<'a> ValueListener for AloeDemoPluginAudioProcessorEditor<'a> {

    /**
      | called when the stored window size changes
      |
      */
    fn value_changed(&mut self, _0: &mut Value)  {
        
        todo!();
        /*
            setSize (lastUIWidth.getValue(), lastUIHeight.getValue());
        */
    }
}

impl<'a> AloeDemoPluginAudioProcessorEditor<'a> {

    pub fn new(owner: &mut AloeDemoPluginAudioProcessor) -> Self {
    
        todo!();
        /*


            : AudioProcessorEditor (owner),
                  midiKeyboard         (owner.keyboardState, MidiKeyboardComponent::horizontalKeyboard),
                  gainAttachment       (owner.state, "gain",  gainSlider),
                  delayAttachment      (owner.state, "delay", delaySlider)

                // add some sliders..
                addAndMakeVisible (gainSlider);
                gainSlider.setSliderStyle (Slider::Rotary);

                addAndMakeVisible (delaySlider);
                delaySlider.setSliderStyle (Slider::Rotary);

                // add some labels for the sliders..
                gainLabel.attachToComponent (&gainSlider, false);
                gainLabel.setFont (Font (11.0f));

                delayLabel.attachToComponent (&delaySlider, false);
                delayLabel.setFont (Font (11.0f));

                // add the midi keyboard component..
                addAndMakeVisible (midiKeyboard);

                // add a label that will display the current timecode and status..
                addAndMakeVisible (timecodeDisplayLabel);
                timecodeDisplayLabel.setFont (Font (Font::getDefaultMonospacedFontName(), 15.0f, Font::plain));

                // set resize limits for this plug-in
                setResizeLimits (400, 200, 1024, 700);
                setResizable (true, owner.wrapperType != wrapperType_AudioUnitv3);

                lastUIWidth .referTo (owner.state.state.getChildWithName ("uiState").getPropertyAsValue ("width",  nullptr));
                lastUIHeight.referTo (owner.state.state.getChildWithName ("uiState").getPropertyAsValue ("height", nullptr));

                // set our component's initial size to be the last one that was stored in the filter's settings
                setSize (lastUIWidth.getValue(), lastUIHeight.getValue());

                lastUIWidth. addListener (this);
                lastUIHeight.addListener (this);

                updateTrackProperties();

                // start a timer which will keep our timecode display updated
                startTimerHz (30);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (backgroundColour);
                g.fillAll();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            // This lays out our child components...

                auto r = getLocalBounds().reduced (8);

                timecodeDisplayLabel.setBounds (r.removeFromTop (26));
                midiKeyboard        .setBounds (r.removeFromBottom (70));

                r.removeFromTop (20);
                auto sliderArea = r.removeFromTop (60);
                gainSlider.setBounds  (sliderArea.removeFromLeft (jmin (180, sliderArea.getWidth() / 2)));
                delaySlider.setBounds (sliderArea.removeFromLeft (jmin (180, sliderArea.getWidth())));

                lastUIWidth  = getWidth();
                lastUIHeight = getHeight();
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            updateTimecodeDisplay (getProcessor().lastPosInfo.get());
        */
    }
    
    pub fn host_midi_controller_is_available(&mut self, controller_is_available: bool)  {
        
        todo!();
        /*
            midiKeyboard.setVisible (! controllerIsAvailable);
        */
    }
    
    pub fn get_control_parameter_index(&mut self, control: &mut Component) -> i32 {
        
        todo!();
        /*
            if (&control == &gainSlider)
                    return 0;

                if (&control == &delaySlider)
                    return 1;

                return -1;
        */
    }
    
    pub fn update_track_properties(&mut self)  {
        
        todo!();
        /*
            auto trackColour = getProcessor().getTrackProperties().colour;
                auto& lf = getLookAndFeel();

                backgroundColour = (trackColour == Colour() ? lf.findColour (ResizableWindow::backgroundColourId)
                                                            : trackColour.withAlpha (1.0f).withBrightness (0.266f));
                repaint();
        */
    }
    
    pub fn get_processor(&self) -> &mut AloeDemoPluginAudioProcessor {
        
        todo!();
        /*
            return static_cast<AloeDemoPluginAudioProcessor&> (processor);
        */
    }

    /**
      | quick-and-dirty function to format
      | a timecode string
      |
      */
    pub fn time_to_timecode_string(seconds: f64) -> String {
        
        todo!();
        /*
            auto millisecs = roundToInt (seconds * 1000.0);
                auto absMillisecs = std::abs (millisecs);

                return String::formatted ("%02d:%02d:%02d.%03d",
                                          millisecs / 3600000,
                                          (absMillisecs / 60000) % 60,
                                          (absMillisecs / 1000)  % 60,
                                          absMillisecs % 1000);
        */
    }

    /**
      | quick-and-dirty function to format
      | a bars/beats string
      |
      */
    pub fn quarter_note_position_to_bars_beats_string(
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

    /**
      | Updates the text in our position label.
      |
      */
    pub fn update_timecode_display(&mut self, pos: AudioPlayHeadCurrentPositionInfo)  {
        
        todo!();
        /*
            MemoryOutputStream displayText;

                displayText << "[" << SystemStats::getALOEVersion() << "]   "
                << String (pos.bpm, 2) << " bpm, "
                << pos.timeSigNumerator << '/' << pos.timeSigDenominator
                << "  -  " << timeToTimecodeString (pos.timeInSeconds)
                << "  -  " << quarterNotePositionToBarsBeatsString (pos.ppqPosition,
                                                                    pos.timeSigNumerator,
                                                                    pos.timeSigDenominator);

                if (pos.isRecording)
                    displayText << "  (recording)";
                else if (pos.isPlaying)
                    displayText << "  (playing)";

                timecodeDisplayLabel.setText (displayText.toString(), dontSendNotification);
        */
    }

}
