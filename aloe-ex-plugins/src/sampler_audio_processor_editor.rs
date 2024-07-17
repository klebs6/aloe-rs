crate::ix!();

pub const MAX_VOICES: usize = 20;

#[no_copy]
#[leak_detector]
pub struct SamplerAudioProcessorEditor<'a> {
    base:                    AudioProcessorEditor<'a>,
    sampler_audio_processor: &'a mut SamplerAudioProcessor<'a>,
    format_manager:          AudioFormatManager,
    data_model:              DataModel<'a>, //{ formatManager };
    undo_manager:            UndoManager<'a>,
    mpe_settings:            MPESettingsDataModel<'a>, //{ dataModel.mpeSettings() };
    tabbed_component:        TabbedComponent<'a>, //{ TabbedButtonBar::Orientation::TabsAtTop };
    settings_component:      MPESettingsComponent<'a>, //{ dataModel.mpeSettings(), undoManager };
    main_sampler_view:       MainSamplerView<'a>,
}

impl<'a> MPESettingsDataModelListener for SamplerAudioProcessorEditor<'a> {

}

impl<'a> DataModelListener for SamplerAudioProcessorEditor<'a> {

}

impl<'a> FileDragAndDropTarget for SamplerAudioProcessorEditor<'a> {

}

impl<'a> IsInterestedInDragSource for SamplerAudioProcessorEditor<'a> {

    fn is_interested_in_drag_source(&mut self, _: &DragAndDropTargetSourceDetails<'_>) -> bool { todo!() }
}

impl<'a> ItemDragEnter for SamplerAudioProcessorEditor<'a> {

}

impl<'a> ItemDragMove for SamplerAudioProcessorEditor<'a> {

}

impl<'a> ItemDragExit for SamplerAudioProcessorEditor<'a> {

}

impl<'a> ItemDropped for SamplerAudioProcessorEditor<'a> {

    fn item_dropped(&mut self, _: &DragAndDropTargetSourceDetails<'_>) { todo!() }
}

impl<'a> ShouldDrawDragImageWhenOver for SamplerAudioProcessorEditor<'a> {

}

impl<'a> SamplerAudioProcessorEditor<'a> {
    
    pub fn new(
        p:     &mut SamplerAudioProcessor,
        state: ProcessorState) -> Self {
    
        todo!();
        /*


            : AudioProcessorEditor (&p),
                  samplerAudioProcessor (p),
                  mainSamplerView (dataModel,
                                   [&p]
                                   {
                                       std::vector<float> ret;
                                       auto voices = p.getNumVoices();
                                       ret.reserve ((size_t) voices);

                                       for (auto i = 0; i != voices; ++i)
                                           ret.emplace_back (p.getPlaybackPosition (i));

                                       return ret;
                                   },
                                   undoManager)

                dataModel.addListener (*this);
                mpeSettings.addListener (*this);

                formatManager.registerBasicFormats();

                addAndMakeVisible (tabbedComponent);

                auto lookFeel = dynamic_cast<LookAndFeel_V4*> (&getLookAndFeel());
                auto bg = lookFeel->getCurrentColourScheme()
                                   .getUIColour (LookAndFeel_V4::ColourScheme::UIColour::widgetBackground);

                tabbedComponent.addTab ("Sample Editor", bg, &mainSamplerView, false);
                tabbedComponent.addTab ("MPE Settings", bg, &settingsComponent, false);

                mpeSettings.setSynthVoices          (state.synthVoices,               nullptr);
                mpeSettings.setLegacyModeEnabled    (state.legacyModeEnabled,         nullptr);
                mpeSettings.setLegacyFirstChannel   (state.legacyChannels.getStart(), nullptr);
                mpeSettings.setLegacyLastChannel    (state.legacyChannels.getEnd(),   nullptr);
                mpeSettings.setLegacyPitchbendRange (state.legacyPitchbendRange,      nullptr);
                mpeSettings.setVoiceStealingEnabled (state.voiceStealingEnabled,      nullptr);
                mpeSettings.setMPEZoneLayout        (state.mpeZoneLayout,             nullptr);

                dataModel.setSampleReader (move (state.readerFactory),    nullptr);
                dataModel.setLoopPointsSeconds  (state.loopPointsSeconds, nullptr);
                dataModel.setCentreFrequencyHz  (state.centreFrequencyHz, nullptr);
                dataModel.setLoopMode           (state.loopMode,          nullptr);

                // Make sure that before the constructor has finished, you've set the
                // editor's size to whatever you need it to be.
                setResizable (true, true);
                setResizeLimits (640, 480, 2560, 1440);
                setSize (640, 480);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            tabbedComponent.setBounds (getLocalBounds());
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (key == KeyPress ('z', ModifierKeys::commandModifier, 0))
                {
                    undoManager.undo();
                    return true;
                }

                if (key == KeyPress ('z', ModifierKeys::commandModifier | ModifierKeys::shiftModifier, 0))
                {
                    undoManager.redo();
                    return true;
                }

                return Component::keyPressed (key);
        */
    }
    
    pub fn is_interested_in_file_drag(&mut self, files: &StringArray) -> bool {
        
        todo!();
        /*
            WildcardFileFilter filter (formatManager.getWildcardForAllFormats(), {}, "Known Audio Formats");
                return files.size() == 1 && filter.isFileSuitable (files[0]);
        */
    }
    
    pub fn files_dropped(&mut self, 
        files: &StringArray,
        _1:    i32,
        _2:    i32)  {
        
        todo!();
        /*
            jassert (files.size() == 1);
                undoManager.beginNewTransaction();
                auto r = new FileAudioFormatReaderFactory (files[0]);
                dataModel.setSampleReader (std::unique_ptr<AudioFormatReaderFactory> (r),
                                           &undoManager);
        */
    }
    
    pub fn sample_reader_changed(&mut self, value: Arc<dyn AudioFormatReaderFactory>)  {
        
        todo!();
        /*
            samplerAudioProcessor.setSample (value == nullptr ? nullptr : value->clone(),
                                                 dataModel.getAudioFormatManager());
        */
    }
    
    pub fn centre_frequency_hz_changed(&mut self, value: f64)  {
        
        todo!();
        /*
            samplerAudioProcessor.setCentreFrequency (value);
        */
    }
    
    pub fn loop_points_seconds_changed(&mut self, value: Range<f64>)  {
        
        todo!();
        /*
            samplerAudioProcessor.setLoopPoints (value);
        */
    }
    
    pub fn loop_mode_changed(&mut self, value: LoopMode)  {
        
        todo!();
        /*
            samplerAudioProcessor.setLoopMode (value);
        */
    }
    
    pub fn synth_voices_changed(&mut self, value: i32)  {
        
        todo!();
        /*
            samplerAudioProcessor.setNumberOfVoices (value);
        */
    }
    
    pub fn voice_stealing_enabled_changed(&mut self, value: bool)  {
        
        todo!();
        /*
            samplerAudioProcessor.setVoiceStealingEnabled (value);
        */
    }
    
    pub fn legacy_mode_enabled_changed(&mut self, value: bool)  {
        
        todo!();
        /*
            if (value)
                    setProcessorLegacyMode();
                else
                    setProcessorMPEMode();
        */
    }
    
    pub fn mpe_zone_layout_changed(&mut self, _0: &MPEZoneLayout)  {
        
        todo!();
        /*
            setProcessorMPEMode();
        */
    }
    
    pub fn legacy_first_channel_changed(&mut self, _0: i32)  {
        
        todo!();
        /*
            setProcessorLegacyMode();
        */
    }
    
    pub fn legacy_last_channel_changed(&mut self, _0: i32)  {
        
        todo!();
        /*
            setProcessorLegacyMode();
        */
    }
    
    pub fn legacy_pitchbend_range_changed(&mut self, _0: i32)  {
        
        todo!();
        /*
            setProcessorLegacyMode();
        */
    }
    
    pub fn set_processor_legacy_mode(&mut self)  {
        
        todo!();
        /*
            samplerAudioProcessor.setLegacyModeEnabled (mpeSettings.getLegacyPitchbendRange(),
                                                            Range<int> (mpeSettings.getLegacyFirstChannel(),
                                                            mpeSettings.getLegacyLastChannel()));
        */
    }
    
    pub fn set_processor_mpe_mode(&mut self)  {
        
        todo!();
        /*
            samplerAudioProcessor.setMPEZoneLayout (mpeSettings.getMPEZoneLayout());
        */
    }
}
