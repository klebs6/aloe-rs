crate::ix!();

pub struct MPENewSettingsComponent<'a> {
    base:                         Component<'a>,
    data_model:                   MPESettingsDataModel<'a>,
    zone_layout:                  MPEZoneLayout,
    member_channels:              ComboBox<'a>,
    master_pitchbend_range:       ComboBox<'a>,
    note_pitchbend_range:         ComboBox<'a>,
    is_lower_zone_button:         ToggleButton<'a>, // default = { "Lower zone"  }
    member_channels_label:        Label<'a>, // default = { {}, "Nr. of member channels"  }
    master_pitchbend_range_label: Label<'a>, // default = { {}, "Master pitchbend range (semitones)"  }
    note_pitchbend_range_label:   Label<'a>, // default = { {}, "Note pitchbend range (semitones)"  }
    set_zone_button:              TextButton<'a>, // default = { "Set zone"  }
    clear_all_zones_button:       TextButton<'a>, // default = { "Clear all zones"  }
    undo_manager:                 *mut UndoManager<'a>,
}

impl<'a> MPESettingsDataModelListener for MPENewSettingsComponent<'a> {

}

impl<'a> MPENewSettingsComponent<'a> {

    pub fn new(
        model: &MPESettingsDataModel,
        um:    &mut UndoManager) -> Self {
    
        todo!();
        /*
        : data_model(model),
        : undo_manager(&um),

            dataModel.addListener (*this);

            addAndMakeVisible (isLowerZoneButton);
            isLowerZoneButton.setToggleState (true, NotificationType::dontSendNotification);

            initialiseComboBoxWithConsecutiveIntegers (*this, memberChannels, memberChannelsLabel, 0, 16, 15);
            initialiseComboBoxWithConsecutiveIntegers (*this, masterPitchbendRange, masterPitchbendRangeLabel, 0, 96, 2);
            initialiseComboBoxWithConsecutiveIntegers (*this, notePitchbendRange, notePitchbendRangeLabel, 0, 96, 48);

            for (auto& button : { &setZoneButton, &clearAllZonesButton })
                addAndMakeVisible (button);

            setZoneButton.onClick = [this]
            {
                auto isLowerZone = isLowerZoneButton.getToggleState();
                auto numMemberChannels = memberChannels.getText().getIntValue();
                auto perNotePb = notePitchbendRange.getText().getIntValue();
                auto masterPb = masterPitchbendRange.getText().getIntValue();

                if (isLowerZone)
                    zoneLayout.setLowerZone (numMemberChannels, perNotePb, masterPb);
                else
                    zoneLayout.setUpperZone (numMemberChannels, perNotePb, masterPb);

                undoManager->beginNewTransaction();
                dataModel.setMPEZoneLayout (zoneLayout, undoManager);
            };

            clearAllZonesButton.onClick = [this]
            {
                zoneLayout.clearAllZones();
                undoManager->beginNewTransaction();
                dataModel.setMPEZoneLayout (zoneLayout, undoManager);
            };
        */
    }
    
    pub fn get_min_height(&self) -> i32 {
        
        todo!();
        /*
            return (controlHeight * 6) + (controlSeparation * 6);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            Rectangle<int> r (proportionOfWidth (0.65f), 0, proportionOfWidth (0.25f), getHeight());

            isLowerZoneButton.setBounds (r.removeFromTop (controlHeight));
            r.removeFromTop (controlSeparation);

            for (auto& comboBox : { &memberChannels, &masterPitchbendRange, &notePitchbendRange })
            {
                comboBox->setBounds (r.removeFromTop (controlHeight));
                r.removeFromTop (controlSeparation);
            }

            r.removeFromTop (controlSeparation);

            auto buttonLeft = proportionOfWidth (0.5f);

            setZoneButton.setBounds (r.removeFromTop (controlHeight).withLeft (buttonLeft));
            r.removeFromTop (controlSeparation);
            clearAllZonesButton.setBounds (r.removeFromTop (controlHeight).withLeft (buttonLeft));
        */
    }
    
    pub fn mpe_zone_layout_changed(&mut self, value: &MPEZoneLayout)  {
        
        todo!();
        /*
            zoneLayout = value;
        */
    }
}
