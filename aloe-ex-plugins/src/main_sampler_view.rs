crate::ix!();

pub struct MainSamplerView<'a> {
    base:                   Component<'a>,
    data_model:             DataModel<'a>,
    waveform_editor:        WaveformEditor<'a>,
    load_new_sample_button: TextButton<'a>, // default = { "Load New Sample"  }
    undo_button:            TextButton<'a>, // default = { "Undo"  }
    redo_button:            TextButton<'a>, // default = { "Redo"  }
    centre_frequency:       Slider<'a>,
    loop_kind_none:         TextButton<'a>, // default = { "None"  }
    loop_kind_forward:      TextButton<'a>, // default = { "Forward"  }
    loop_kind_pingpong:     TextButton<'a>, // default = { "Ping Pong"  }
    centre_frequency_label: Label<'a>, // default = { {}, "Sample Centre Freq / Hz"  }
    loop_kind_label:        Label<'a>, // default = { {}, "Looping Mode"  }
    file_chooser:           FileChooser<'a>, //{ "Select a file to load...", File(), dataModel.getAudioFormatManager().getWildcardForAllFormats() };
    undo_manager:           &'a mut UndoManager<'a>,
}

impl<'a> ChangeListener for MainSamplerView<'a> {

    fn change_listener_callback(&mut self, source: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            if (source == &undoManager)
            {
                undoButton.setEnabled (undoManager.canUndo());
                redoButton.setEnabled (undoManager.canRedo());
            }
        */
    }
}

impl<'a> DataModelListener for MainSamplerView<'a> {

}

impl<'a> Drop for MainSamplerView<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            undoManager.removeChangeListener (this);
         */
    }
}

impl<'a> MainSamplerView<'a> {

    pub fn new(
        model:    &DataModel,
        provider: PlaybackPositionOverlayProvider,
        um:       &mut UndoManager) -> Self {
    
        todo!();
        /*
        : data_model(model),
        : waveform_editor(dataModel, move (provider), um),
        : undo_manager(um),

            dataModel.addListener (*this);

            addAndMakeVisible (waveformEditor);
            addAndMakeVisible (loadNewSampleButton);
            addAndMakeVisible (undoButton);
            addAndMakeVisible (redoButton);

            auto setReader = [this] (const FileChooser& fc)
            {
                const auto result = fc.getResult();

                if (result != File())
                {
                    undoManager.beginNewTransaction();
                    auto readerFactory = new FileAudioFormatReaderFactory (result);
                    dataModel.setSampleReader (std::unique_ptr<AudioFormatReaderFactory> (readerFactory),
                                               &undoManager);
                }
            };

            loadNewSampleButton.onClick = [this, setReader]
            {
                fileChooser.launchAsync (FileBrowserComponent::FileChooserFlags::openMode |
                                         FileBrowserComponent::FileChooserFlags::canSelectFiles,
                                         setReader);
            };

            addAndMakeVisible (centreFrequency);
            centreFrequency.onValueChange = [this]
            {
                undoManager.beginNewTransaction();
                dataModel.setCentreFrequencyHz (centreFrequency.getValue(),
                                                centreFrequency.isMouseButtonDown() ? nullptr : &undoManager);
            };

            centreFrequency.setRange (20, 20000, 1);
            centreFrequency.setSliderStyle (Slider::SliderStyle::IncDecButtons);
            centreFrequency.setIncDecButtonsMode (Slider::IncDecButtonMode::incDecButtonsDraggable_Vertical);

            auto radioGroupId = 1;

            for (auto buttonPtr : { &loopKindNone, &loopKindForward, &loopKindPingpong })
            {
                addAndMakeVisible (buttonPtr);
                buttonPtr->setRadioGroupId (radioGroupId, dontSendNotification);
                buttonPtr->setClickingTogglesState (true);
            }

            loopKindNone.onClick = [this]
            {
                if (loopKindNone.getToggleState())
                {
                    undoManager.beginNewTransaction();
                    dataModel.setLoopMode (LoopMode::none, &undoManager);
                }
            };

            loopKindForward.onClick = [this]
            {
                if (loopKindForward.getToggleState())
                {
                    undoManager.beginNewTransaction();
                    dataModel.setLoopMode (LoopMode::forward, &undoManager);
                }
            };

            loopKindPingpong.onClick = [this]
            {
                if (loopKindPingpong.getToggleState())
                {
                    undoManager.beginNewTransaction();
                    dataModel.setLoopMode (LoopMode::pingpong, &undoManager);
                }
            };

            undoButton.onClick = [this] { undoManager.undo(); };
            redoButton.onClick = [this] { undoManager.redo(); };

            addAndMakeVisible (centreFrequencyLabel);
            addAndMakeVisible (loopKindLabel);

            changeListenerCallback (&undoManager);
            undoManager.addChangeListener (this);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();

            auto topBar = bounds.removeFromTop (50);
            auto padding = 4;
            loadNewSampleButton .setBounds (topBar.removeFromRight (100).reduced (padding));
            redoButton          .setBounds (topBar.removeFromRight (100).reduced (padding));
            undoButton          .setBounds (topBar.removeFromRight (100).reduced (padding));
            centreFrequencyLabel.setBounds (topBar.removeFromLeft  (100).reduced (padding));
            centreFrequency     .setBounds (topBar.removeFromLeft  (100).reduced (padding));

            auto bottomBar = bounds.removeFromBottom (50);
            loopKindLabel   .setBounds (bottomBar.removeFromLeft (100).reduced (padding));
            loopKindNone    .setBounds (bottomBar.removeFromLeft (80) .reduced (padding));
            loopKindForward .setBounds (bottomBar.removeFromLeft (80) .reduced (padding));
            loopKindPingpong.setBounds (bottomBar.removeFromLeft (80) .reduced (padding));

            waveformEditor.setBounds (bounds);
        */
    }
    
    pub fn loop_mode_changed(&mut self, value: LoopMode)  {
        
        todo!();
        /*
            switch (value)
            {
                case LoopMode::none:
                    loopKindNone.setToggleState (true, dontSendNotification);
                    break;
                case LoopMode::forward:
                    loopKindForward.setToggleState (true, dontSendNotification);
                    break;
                case LoopMode::pingpong:
                    loopKindPingpong.setToggleState (true, dontSendNotification);
                    break;

                default:
                    break;
            }
        */
    }
    
    pub fn centre_frequency_hz_changed(&mut self, value: f64)  {
        
        todo!();
        /*
            centreFrequency.setValue (value, dontSendNotification);
        */
    }
}
