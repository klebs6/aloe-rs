crate::ix!();

pub const CONTROL_HEIGHT:     usize = 24;
pub const CONTROL_SEPARATION: usize = 6;

///-------------------
pub struct MPELegacySettingsComponent<'a> {
    base:                         Component<'a>,
    data_model:                   MPESettingsDataModel<'a>,
    legacy_start_channel:         ComboBox<'a>,
    legacy_end_channel:           ComboBox<'a>,
    legacy_pitchbend_range:       ComboBox<'a>,
    legacy_start_channel_label:   Label<'a>, // default = { {}, "First channel"  }
    legacy_end_channel_label:     Label<'a>, // default = { {}, "Last channel"  }
    legacy_pitchbend_range_label: Label<'a>, // default = { {}, "Pitchbend range (semitones)"  }
    undo_manager:                 *mut UndoManager<'a>,
}

impl<'a> MPESettingsDataModelListener for MPELegacySettingsComponent<'a> {

}

impl<'a> MPELegacySettingsComponent<'a> {
    
    pub fn new(
        model: &MPESettingsDataModel,
        um:    &mut UndoManager) -> Self {
    
        todo!();
        /*
        : data_model(model),
        : undo_manager(&um),

            dataModel.addListener (*this);

            initialiseComboBoxWithConsecutiveIntegers (*this, legacyStartChannel, legacyStartChannelLabel, 1, 16, 1);
            initialiseComboBoxWithConsecutiveIntegers (*this, legacyEndChannel, legacyEndChannelLabel, 1, 16, 16);
            initialiseComboBoxWithConsecutiveIntegers (*this, legacyPitchbendRange, legacyPitchbendRangeLabel, 0, 96, 2);

            legacyStartChannel.onChange = [this]
            {
                if (isLegacyModeValid())
                {
                    undoManager->beginNewTransaction();
                    dataModel.setLegacyFirstChannel (getFirstChannel(), undoManager);
                }
            };

            legacyEndChannel.onChange = [this]
            {
                if (isLegacyModeValid())
                {
                    undoManager->beginNewTransaction();
                    dataModel.setLegacyLastChannel (getLastChannel(), undoManager);
                }
            };

            legacyPitchbendRange.onChange = [this]
            {
                if (isLegacyModeValid())
                {
                    undoManager->beginNewTransaction();
                    dataModel.setLegacyPitchbendRange (legacyPitchbendRange.getText().getIntValue(), undoManager);
                }
            };
        */
    }
    
    pub fn get_min_height(&self) -> i32 {
        
        todo!();
        /*
            return (controlHeight * 3) + (controlSeparation * 2);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            Rectangle<int> r (proportionOfWidth (0.65f), 0, proportionOfWidth (0.25f), getHeight());

            for (auto& comboBox : { &legacyStartChannel, &legacyEndChannel, &legacyPitchbendRange })
            {
                comboBox->setBounds (r.removeFromTop (controlHeight));
                r.removeFromTop (controlSeparation);
            }
        */
    }
    
    pub fn is_legacy_mode_valid(&self) -> bool {
        
        todo!();
        /*
            if (! areLegacyModeParametersValid())
            {
                handleInvalidLegacyModeParameters();
                return false;
            }

            return true;
        */
    }
    
    pub fn legacy_first_channel_changed(&mut self, value: i32)  {
        
        todo!();
        /*
            legacyStartChannel.setSelectedId (value, dontSendNotification);
        */
    }
    
    pub fn legacy_last_channel_changed(&mut self, value: i32)  {
        
        todo!();
        /*
            legacyEndChannel.setSelectedId (value, dontSendNotification);
        */
    }
    
    pub fn legacy_pitchbend_range_changed(&mut self, value: i32)  {
        
        todo!();
        /*
            legacyPitchbendRange.setSelectedId (value + 1, dontSendNotification);
        */
    }
    
    pub fn get_first_channel(&self) -> i32 {
        
        todo!();
        /*
            return legacyStartChannel.getText().getIntValue();
        */
    }
    
    pub fn get_last_channel(&self) -> i32 {
        
        todo!();
        /*
            return legacyEndChannel.getText().getIntValue();
        */
    }
    
    pub fn are_legacy_mode_parameters_valid(&self) -> bool {
        
        todo!();
        /*
            return getFirstChannel() <= getLastChannel();
        */
    }
    
    pub fn handle_invalid_legacy_mode_parameters(&self)  {
        
        todo!();
        /*
            AlertWindow::showMessageBoxAsync (AlertWindow::WarningIcon,
                                              "Invalid legacy mode channel layout",
                                              "Cannot set legacy mode start/end channel:\n"
                                              "The end channel must not be less than the start channel!",
                                              "Got it");
        */
    }
}
