crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Audio/MPEDemo.h]

#[derive(Default)]
#[no_copy]
#[leak_detector]
pub struct ZoneColourPicker {
    zone_layout:         MPEZoneLayout,
    legacy_mode_enabled: bool, // default = false
}

impl ZoneColourPicker {
    
    pub fn get_colour_for_midi_channel(&mut self, midi_channel: i32) -> Colour {
        
        todo!();
        /*
            if (legacyModeEnabled)
                return Colours::white;

            if (zoneLayout.getLowerZone().isUsingChannelAsMemberChannel (midiChannel))
                return getColourForZone (true);

            if (zoneLayout.getUpperZone().isUsingChannelAsMemberChannel (midiChannel))
                return getColourForZone (false);

            return Colours::transparentBlack;
        */
    }
    
    pub fn get_colour_for_zone(&self, is_lower_zone: bool) -> Colour {
        
        todo!();
        /*
            if (legacyModeEnabled)
                return Colours::white;

            if (isLowerZone)
                return Colours::blue;

            return Colours::red;
        */
    }
    
    pub fn set_zone_layout(&mut self, layout: MPEZoneLayout)  {
        
        todo!();
        /*
            zoneLayout = layout;
        */
    }
    
    pub fn set_legacy_mode_enabled(&mut self, should_be_enabled: bool)  {
        
        todo!();
        /*
            legacyModeEnabled = shouldBeEnabled;
        */
    }
}

