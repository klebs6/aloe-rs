crate::ix!();

pub struct MPESettingsComponent<'a> {
    base:                          Component<'a>,
    data_model:                    MPESettingsDataModel<'a>,
    legacy_settings:               MPELegacySettingsComponent<'a>,
    new_settings:                  MPENewSettingsComponent<'a>,
    legacy_mode_enabled_toggle:    ToggleButton<'a>, // default = { "Enable Legacy Mode"  }
    voice_stealing_enabled_toggle: ToggleButton<'a>, // default = { "Enable synth voice stealing"  }
    number_of_voices:              ComboBox<'a>,
    number_of_voices_label:        Label<'a>, // default = { {}, "Number of synth voices"  }
    undo_manager:                  *mut UndoManager<'a>,
}

impl<'a> MPESettingsDataModelListener for MPESettingsComponent<'a> {

}

impl<'a> MPESettingsComponent<'a> {
    
    pub fn new(
        model: &MPESettingsDataModel,
        um:    &mut UndoManager) -> Self {
    
        todo!();
        /*
        : data_model(model),
        : legacy_settings(dataModel, um),
        : new_settings(dataModel, um),
        : undo_manager(&um),

            dataModel.addListener (*this);

            addAndMakeVisible (newSettings);
            addChildComponent (legacySettings);

            initialiseComboBoxWithConsecutiveIntegers (*this, numberOfVoices, numberOfVoicesLabel, 1, 20, 15);
            numberOfVoices.onChange = [this]
            {
                undoManager->beginNewTransaction();
                dataModel.setSynthVoices (numberOfVoices.getText().getIntValue(), undoManager);
            };

            for (auto& button : { &legacyModeEnabledToggle, &voiceStealingEnabledToggle })
            {
                addAndMakeVisible (button);
            }

            legacyModeEnabledToggle.onClick = [this]
            {
                undoManager->beginNewTransaction();
                dataModel.setLegacyModeEnabled (legacyModeEnabledToggle.getToggleState(), undoManager);
            };

            voiceStealingEnabledToggle.onClick = [this]
            {
                undoManager->beginNewTransaction();
                dataModel.setVoiceStealingEnabled (voiceStealingEnabledToggle.getToggleState(), undoManager);
            };
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto topHeight = jmax (legacySettings.getMinHeight(), newSettings.getMinHeight());
            auto r = getLocalBounds();
            r.removeFromTop (15);
            auto top = r.removeFromTop (topHeight);
            legacySettings.setBounds (top);
            newSettings.setBounds (top);

            r.removeFromLeft (proportionOfWidth (0.65f));
            r = r.removeFromLeft (proportionOfWidth (0.25f));

            auto toggleLeft = proportionOfWidth (0.25f);

            legacyModeEnabledToggle.setBounds (r.removeFromTop (controlHeight).withLeft (toggleLeft));
            r.removeFromTop (controlSeparation);
            voiceStealingEnabledToggle.setBounds (r.removeFromTop (controlHeight).withLeft (toggleLeft));
            r.removeFromTop (controlSeparation);
            numberOfVoices.setBounds (r.removeFromTop (controlHeight));
        */
    }
    
    pub fn legacy_mode_enabled_changed(&mut self, value: bool)  {
        
        todo!();
        /*
            legacySettings.setVisible (value);
            newSettings.setVisible (! value);
            legacyModeEnabledToggle.setToggleState (value, dontSendNotification);
        */
    }
    
    pub fn voice_stealing_enabled_changed(&mut self, value: bool)  {
        
        todo!();
        /*
            voiceStealingEnabledToggle.setToggleState (value, dontSendNotification);
        */
    }
    
    pub fn synth_voices_changed(&mut self, value: i32)  {
        
        todo!();
        /*
            numberOfVoices.setSelectedId (value, dontSendNotification);
        */
    }
}
