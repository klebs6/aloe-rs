crate::ix!();

pub trait MPESettingsDataModelListener {
    fn synth_voices_changed(&mut self, _0: i32)               { }
    fn voice_stealing_enabled_changed(&mut self, _0: bool)    { }
    fn legacy_mode_enabled_changed(&mut self, _0: bool)       { }
    fn mpe_zone_layout_changed(&mut self, _0: &MPEZoneLayout) { }
    fn legacy_first_channel_changed(&mut self, _0: i32)       { }
    fn legacy_last_channel_changed(&mut self, _0: i32)        { }
    fn legacy_pitchbend_range_changed(&mut self, _0: i32)     { }
}

pub struct MPESettingsDataModel<'a> {
    value_tree:             ValueTree,
    synth_voices:           CachedValue<'a, i32>,
    voice_stealing_enabled: CachedValue<'a, bool>,
    legacy_mode_enabled:    CachedValue<'a, bool>,
    mpe_zone_layout:        CachedValue<'a, MPEZoneLayout>,
    legacy_first_channel:   CachedValue<'a, i32>,
    legacy_last_channel:    CachedValue<'a, i32>,
    legacy_pitchbend_range: CachedValue<'a, i32>,
    listener_list:          ListenerList<Box<dyn MPESettingsDataModelListener>>,
}

impl<'a> ValueTreeListener for MPESettingsDataModel<'a> {

}

impl<'a> Default for MPESettingsDataModel<'a> {
    
    fn default() -> Self {
        todo!();
        /*
        : mpe_settings_data_model(ValueTree (IDs::MPE_SETTINGS)),

        
        */
    }
}

impl<'a> From<&ValueTree> for MPESettingsDataModel<'a> {

    fn from(vt: &ValueTree) -> Self {
    
        todo!();
        /*


            : valueTree (vt),
              synthVoices          (valueTree, IDs::synthVoices,          nullptr, 15),
              voiceStealingEnabled (valueTree, IDs::voiceStealingEnabled, nullptr, false),
              legacyModeEnabled    (valueTree, IDs::legacyModeEnabled,    nullptr, true),
              mpeZoneLayout        (valueTree, IDs::mpeZoneLayout,        nullptr, {}),
              legacyFirstChannel   (valueTree, IDs::legacyFirstChannel,   nullptr, 1),
              legacyLastChannel    (valueTree, IDs::legacyLastChannel,    nullptr, 15),
              legacyPitchbendRange (valueTree, IDs::legacyPitchbendRange, nullptr, 48)

            jassert (valueTree.hasType (IDs::MPE_SETTINGS));
            valueTree.addListener (this);
        */
    }
}

impl<'a> From<&MPESettingsDataModel<'a>> for MPESettingsDataModel<'a> {
    
    fn from(other: &MPESettingsDataModel) -> Self {
    
        todo!();
        /*
        : mpe_settings_data_model(other.valueTree),

        
        */
    }
}

impl<'a> MPESettingsDataModel<'a> {
    
    pub fn assign_from(&mut self, other: &MPESettingsDataModel) -> &mut MPESettingsDataModel {
        
        todo!();
        /*
            auto copy (other);
            swap (copy);
            return *this;
        */
    }
    
    pub fn get_synth_voices(&self) -> i32 {
        
        todo!();
        /*
            return synthVoices;
        */
    }
    
    pub fn set_synth_voices(&mut self, 
        value:        i32,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            synthVoices.setValue (Range<int> (1, 20).clipValue (value), undoManager);
        */
    }
    
    pub fn get_voice_stealing_enabled(&self) -> bool {
        
        todo!();
        /*
            return voiceStealingEnabled;
        */
    }
    
    pub fn set_voice_stealing_enabled(&mut self, 
        value:        bool,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            voiceStealingEnabled.setValue (value, undoManager);
        */
    }
    
    pub fn get_legacy_mode_enabled(&self) -> bool {
        
        todo!();
        /*
            return legacyModeEnabled;
        */
    }
    
    pub fn set_legacy_mode_enabled(&mut self, 
        value:        bool,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            legacyModeEnabled.setValue (value, undoManager);
        */
    }
    
    pub fn get_mpe_zone_layout(&self) -> MPEZoneLayout {
        
        todo!();
        /*
            return mpeZoneLayout;
        */
    }
    
    pub fn set_mpe_zone_layout(&mut self, 
        value:        MPEZoneLayout,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            mpeZoneLayout.setValue (value, undoManager);
        */
    }
    
    pub fn get_legacy_first_channel(&self) -> i32 {
        
        todo!();
        /*
            return legacyFirstChannel;
        */
    }
    
    pub fn set_legacy_first_channel(&mut self, 
        value:        i32,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            legacyFirstChannel.setValue (Range<int> (1, legacyLastChannel).clipValue (value), undoManager);
        */
    }
    
    pub fn get_legacy_last_channel(&self) -> i32 {
        
        todo!();
        /*
            return legacyLastChannel;
        */
    }
    
    pub fn set_legacy_last_channel(&mut self, 
        value:        i32,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            legacyLastChannel.setValue (Range<int> (legacyFirstChannel, 15).clipValue (value), undoManager);
        */
    }
    
    pub fn get_legacy_pitchbend_range(&self) -> i32 {
        
        todo!();
        /*
            return legacyPitchbendRange;
        */
    }
    
    pub fn set_legacy_pitchbend_range(&mut self, 
        value:        i32,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            legacyPitchbendRange.setValue (Range<int> (0, 95).clipValue (value), undoManager);
        */
    }
    
    pub fn add_listener(&mut self, listener: &mut dyn MPESettingsDataModelListener)  {
        
        todo!();
        /*
            listenerList.add (&listener);
        */
    }
    
    pub fn remove_listener(&mut self, listener: &mut dyn MPESettingsDataModelListener)  {
        
        todo!();
        /*
            listenerList.remove (&listener);
        */
    }
    
    pub fn swap(&mut self, other: &mut MPESettingsDataModel)  {
        
        todo!();
        /*
            using std::swap;
            swap (other.valueTree, valueTree);
        */
    }
    
    pub fn value_tree_property_changed(&mut self, 
        _0:       &mut ValueTree,
        property: &Identifier)  {
        
        todo!();
        /*
            if (property == IDs::synthVoices)
            {
                synthVoices.forceUpdateOfCachedValue();
                listenerList.call ([this] (Listener& l) { l.synthVoicesChanged (synthVoices); });
            }
            else if (property == IDs::voiceStealingEnabled)
            {
                voiceStealingEnabled.forceUpdateOfCachedValue();
                listenerList.call ([this] (Listener& l) { l.voiceStealingEnabledChanged (voiceStealingEnabled); });
            }
            else if (property == IDs::legacyModeEnabled)
            {
                legacyModeEnabled.forceUpdateOfCachedValue();
                listenerList.call ([this] (Listener& l) { l.legacyModeEnabledChanged (legacyModeEnabled); });
            }
            else if (property == IDs::mpeZoneLayout)
            {
                mpeZoneLayout.forceUpdateOfCachedValue();
                listenerList.call ([this] (Listener& l) { l.mpeZoneLayoutChanged (mpeZoneLayout); });
            }
            else if (property == IDs::legacyFirstChannel)
            {
                legacyFirstChannel.forceUpdateOfCachedValue();
                listenerList.call ([this] (Listener& l) { l.legacyFirstChannelChanged (legacyFirstChannel); });
            }
            else if (property == IDs::legacyLastChannel)
            {
                legacyLastChannel.forceUpdateOfCachedValue();
                listenerList.call ([this] (Listener& l) { l.legacyLastChannelChanged (legacyLastChannel); });
            }
            else if (property == IDs::legacyPitchbendRange)
            {
                legacyPitchbendRange.forceUpdateOfCachedValue();
                listenerList.call ([this] (Listener& l) { l.legacyPitchbendRangeChanged (legacyPitchbendRange); });
            }
        */
    }
    
    pub fn value_tree_child_added(&mut self, 
        _0: &mut ValueTree,
        _1: &mut ValueTree)  {
        
        todo!();
        /*
            jassertfalse;
        */
    }
    
    pub fn value_tree_child_removed(&mut self, 
        _0: &mut ValueTree,
        _1: &mut ValueTree,
        _2: i32)  {
        
        todo!();
        /*
            jassertfalse;
        */
    }
    
    pub fn value_tree_child_order_changed(&mut self, 
        _0: &mut ValueTree,
        _1: i32,
        _2: i32)  {
        
        todo!();
        /*
            jassertfalse;
        */
    }
    
    pub fn value_tree_parent_changed(&mut self, _0: &mut ValueTree)  {
        
        todo!();
        /*
            jassertfalse;
        */
    }
}
