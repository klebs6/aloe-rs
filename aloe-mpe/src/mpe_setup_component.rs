crate::ix!();

pub trait MpeSetupComponentListener {

    fn zone_changed(&mut self, 
        is_lower:         bool,
        num_member_chans: i32,
        per_note_pb:      i32,
        master_pb:        i32);

    fn all_zones_cleared(&mut self);

    fn legacy_mode_changed(&mut self, 
        legacy_mode_enabled: bool,
        pitchbend_range:     i32,
        channel_range:       Range<i32>);

    fn voice_stealing_enabled_changed(&mut self, voice_stealing_enabled: bool);

    fn number_of_voices_changed(&mut self, number_of_voices: i32);
}

#[no_copy]
#[leak_detector]
pub struct MPESetupComponent<'a> {
    base:                           Component<'a>,
    base2:                          ChangeBroadcaster<'a>,
    zone_layout:                    MPEZoneLayout,
    member_channels:                ComboBox<'a>,
    master_pitchbend_range:         ComboBox<'a>,
    note_pitchbend_range:           ComboBox<'a>,
    is_lower_zone_button:           ToggleButton<'a>, //{ "Lower zone" };
    member_channels_label:          Label<'a>,        // { {}, "Nr. of member channels:" };
    master_pitchbend_range_label:   Label<'a>,        // { {}, "Master pitchbend range (semitones):" };
    note_pitchbend_range_label:     Label<'a>,        // { {}, "Note pitchbend range (semitones):" };
    set_zone_button:                TextButton<'a>,   // { "Set zone" };
    clear_all_zones_button:         TextButton<'a>,   // { "Clear all zones" };
    legacy_start_channel:           ComboBox<'a>,
    legacy_end_channel:             ComboBox<'a>,
    legacy_pitchbend_range:         ComboBox<'a>,
    legacy_start_channel_label:     Label<'a>,        // { {}, "First channel:" };
    legacy_end_channel_label:       Label<'a>,        // { {}, "Last channel:" };
    legacy_pitchbend_range_label:   Label<'a>,        // { {}, "Pitchbend range (semitones):"};
    legacy_mode_enabled_toggle:     ToggleButton<'a>, // { "Enable Legacy Mode" };
    voice_stealing_enabled_toggle:  ToggleButton<'a>, // { "Enable synth voice stealing" };
    number_of_voices:               ComboBox<'a>,
    number_of_voices_label:         Label<'a>,        // { {}, "Number of synth voices"};
    listeners:                      ListenerList<Box<dyn MpeSetupComponentListener>>,
    default_member_channels:        i32, // default = 15
    default_master_pitchbend_range: i32, // default = 2
    default_note_pitchbend_range:   i32, // default = 48
}

impl<'a> Default for MPESetupComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (isLowerZoneButton);
            isLowerZoneButton.setToggleState (true, NotificationType::dontSendNotification);

            initialiseComboBoxWithConsecutiveIntegers (memberChannels, memberChannelsLabel, 0, 16, defaultMemberChannels);
            initialiseComboBoxWithConsecutiveIntegers (masterPitchbendRange, masterPitchbendRangeLabel, 0, 96, defaultMasterPitchbendRange);
            initialiseComboBoxWithConsecutiveIntegers (notePitchbendRange, notePitchbendRangeLabel, 0, 96, defaultNotePitchbendRange);

            initialiseComboBoxWithConsecutiveIntegers (legacyStartChannel, legacyStartChannelLabel, 1, 16, 1, false);
            initialiseComboBoxWithConsecutiveIntegers (legacyEndChannel, legacyEndChannelLabel, 1, 16, 16, false);
            initialiseComboBoxWithConsecutiveIntegers (legacyPitchbendRange, legacyPitchbendRangeLabel, 0, 96, 2, false);

            addAndMakeVisible (setZoneButton);
            setZoneButton.onClick = [this] { setZoneButtonClicked(); };
            addAndMakeVisible (clearAllZonesButton);
            clearAllZonesButton.onClick = [this] { clearAllZonesButtonClicked(); };
            addAndMakeVisible (legacyModeEnabledToggle);
            legacyModeEnabledToggle.onClick = [this] { legacyModeEnabledToggleClicked(); };
            addAndMakeVisible (voiceStealingEnabledToggle);
            voiceStealingEnabledToggle.onClick = [this] { voiceStealingEnabledToggleClicked(); };

            initialiseComboBoxWithConsecutiveIntegers (numberOfVoices, numberOfVoicesLabel, 1, 20, 15)
        */
    }
}

impl<'a> MPESetupComponent<'a> {

    pub fn add_listener(&mut self, listener_to_add: *mut dyn MpeSetupComponentListener)  {
        
        todo!();
        /*
            listeners.add (listenerToAdd);
        */
    }
    
    pub fn remove_listener(&mut self, listener_to_remove: *mut dyn MpeSetupComponentListener)  {
        
        todo!();
        /*
            listeners.remove (listenerToRemove);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            Rectangle<int> r (proportionOfWidth (0.65f), 15, proportionOfWidth (0.25f), 3000);
            auto h = 24;
            auto hspace = 6;
            auto hbigspace = 18;

            isLowerZoneButton.setBounds (r.removeFromTop (h));
            r.removeFromTop (hspace);
            memberChannels.setBounds (r.removeFromTop (h));
            r.removeFromTop (hspace);
            notePitchbendRange.setBounds (r.removeFromTop (h));
            r.removeFromTop (hspace);
            masterPitchbendRange.setBounds (r.removeFromTop (h));

            legacyStartChannel  .setBounds (isLowerZoneButton .getBounds());
            legacyEndChannel    .setBounds (memberChannels    .getBounds());
            legacyPitchbendRange.setBounds (notePitchbendRange.getBounds());

            r.removeFromTop (hbigspace);

            auto buttonLeft = proportionOfWidth (0.5f);

            setZoneButton.setBounds (r.removeFromTop (h).withLeft (buttonLeft));
            r.removeFromTop (hspace);
            clearAllZonesButton.setBounds (r.removeFromTop (h).withLeft (buttonLeft));

            r.removeFromTop (hbigspace);

            auto toggleLeft = proportionOfWidth (0.25f);

            legacyModeEnabledToggle.setBounds (r.removeFromTop (h).withLeft (toggleLeft));
            r.removeFromTop (hspace);
            voiceStealingEnabledToggle.setBounds (r.removeFromTop (h).withLeft (toggleLeft));
            r.removeFromTop (hspace);
            numberOfVoices.setBounds (r.removeFromTop (h));
        */
    }
    
    pub fn initialise_combo_box_with_consecutive_integers(
        &mut self, 
        combo_box:       &mut ComboBox,
        label_to_attach: &mut Label,
        first_value:     i32,
        num_values:      i32,
        value_to_select: i32,
        make_visible:    Option<bool>

    ) {

        let make_visible: bool = make_visible.unwrap_or(true);

        todo!();
        /*
            for (auto i = 0; i < numValues; ++i)
                comboBox.addItem (String (i + firstValue), i + 1);

            comboBox.setSelectedId (valueToSelect - firstValue + 1);
            labelToAttach.attachToComponent (&comboBox, true);

            if (makeVisible)
                addAndMakeVisible (comboBox);
            else
                addChildComponent (comboBox);

            if (&comboBox == &numberOfVoices)
                comboBox.onChange = [this] { numberOfVoicesChanged(); };
            else if (&comboBox == &legacyPitchbendRange)
                comboBox.onChange = [this] { if (legacyModeEnabledToggle.getToggleState()) legacyModePitchbendRangeChanged(); };
            else if (&comboBox == &legacyStartChannel || &comboBox == &legacyEndChannel)
                comboBox.onChange = [this] { if (legacyModeEnabledToggle.getToggleState()) legacyModeChannelRangeChanged(); };
        */
    }
    
    pub fn set_zone_button_clicked(&mut self)  {
        
        todo!();
        /*
            auto isLowerZone = isLowerZoneButton.getToggleState();
            auto numMemberChannels = memberChannels.getText().getIntValue();
            auto perNotePb = notePitchbendRange.getText().getIntValue();
            auto masterPb = masterPitchbendRange.getText().getIntValue();

            if (isLowerZone)
                zoneLayout.setLowerZone (numMemberChannels, perNotePb, masterPb);
            else
                zoneLayout.setUpperZone (numMemberChannels, perNotePb, masterPb);

            listeners.call ([&] (MpeSetupComponentListener& l) { l.zoneChanged (isLowerZone, numMemberChannels, perNotePb, masterPb); });
        */
    }
    
    pub fn clear_all_zones_button_clicked(&mut self)  {
        
        todo!();
        /*
            zoneLayout.clearAllZones();
            listeners.call ([] (MpeSetupComponentListener& l) { l.allZonesCleared(); });
        */
    }
    
    pub fn legacy_mode_enabled_toggle_clicked(&mut self)  {
        
        todo!();
        /*
            auto legacyModeEnabled = legacyModeEnabledToggle.getToggleState();

            isLowerZoneButton   .setVisible (! legacyModeEnabled);
            memberChannels      .setVisible (! legacyModeEnabled);
            notePitchbendRange  .setVisible (! legacyModeEnabled);
            masterPitchbendRange.setVisible (! legacyModeEnabled);
            setZoneButton       .setVisible (! legacyModeEnabled);
            clearAllZonesButton .setVisible (! legacyModeEnabled);

            legacyStartChannel  .setVisible (legacyModeEnabled);
            legacyEndChannel    .setVisible (legacyModeEnabled);
            legacyPitchbendRange.setVisible (legacyModeEnabled);

            if (areLegacyModeParametersValid())
            {
                listeners.call ([&] (MpeSetupComponentListener& l) { l.legacyModeChanged (legacyModeEnabledToggle.getToggleState(),
                                                                         legacyPitchbendRange.getText().getIntValue(),
                                                                         getLegacyModeChannelRange()); });
            }
            else
            {
                handleInvalidLegacyModeParameters();
            }
        */
    }
    
    pub fn voice_stealing_enabled_toggle_clicked(&mut self)  {
        
        todo!();
        /*
            auto newState = voiceStealingEnabledToggle.getToggleState();
            listeners.call ([=] (MpeSetupComponentListener& l) { l.voiceStealingEnabledChanged (newState); });
        */
    }
    
    pub fn number_of_voices_changed(&mut self)  {
        
        todo!();
        /*
            listeners.call ([this] (MpeSetupComponentListener& l) { l.numberOfVoicesChanged (numberOfVoices.getText().getIntValue()); });
        */
    }
    
    pub fn legacy_mode_pitchbend_range_changed(&mut self)  {
        
        todo!();
        /*
            jassert (legacyModeEnabledToggle.getToggleState() == true);

            listeners.call ([this] (MpeSetupComponentListener& l) { l.legacyModeChanged (true,
                                                                        legacyPitchbendRange.getText().getIntValue(),
                                                                        getLegacyModeChannelRange()); });
        */
    }
    
    pub fn legacy_mode_channel_range_changed(&mut self)  {
        
        todo!();
        /*
            jassert (legacyModeEnabledToggle.getToggleState() == true);

            if (areLegacyModeParametersValid())
            {
                listeners.call ([this] (MpeSetupComponentListener& l) { l.legacyModeChanged (true,
                                                                            legacyPitchbendRange.getText().getIntValue(),
                                                                            getLegacyModeChannelRange()); });
            }
            else
            {
                handleInvalidLegacyModeParameters();
            }
        */
    }
    
    pub fn are_legacy_mode_parameters_valid(&self) -> bool {
        
        todo!();
        /*
            return legacyStartChannel.getText().getIntValue() <= legacyEndChannel.getText().getIntValue();
        */
    }
    
    pub fn handle_invalid_legacy_mode_parameters(&self)  {
        
        todo!();
        /*
            AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                              "Invalid legacy mode channel layout",
                                              "Cannot set legacy mode start/end channel:\n"
                                              "The end channel must not be less than the start channel!",
                                              "Got it");
        */
    }
    
    pub fn get_legacy_mode_channel_range(&self) -> Range<i32> {
        
        todo!();
        /*
            return { legacyStartChannel.getText().getIntValue(),
                     legacyEndChannel.getText().getIntValue() + 1 };
        */
    }
}
