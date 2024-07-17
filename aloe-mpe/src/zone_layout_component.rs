crate::ix!();

impl PartialEq<MPEZoneLayout> for MPEZoneLayout {
    
    #[inline] fn eq(&self, other: &MPEZoneLayout) -> bool {
        todo!();
        /*
            if (a.getLowerZone() != b.getLowerZone())
            return false;

        if (a.getUpperZone() != b.getUpperZone())
            return false;

        return true;
        */
    }
}

impl Eq for MPEZoneLayout {}

pub struct ZoneLayoutComponent<'a> {
    base:                        Component<'a>,
    zone_layout:                 MPEZoneLayout,
    colour_picker:               &'a ZoneColourPicker,
    legacy_mode_enabled:         bool, // default = false
    legacy_mode_pitchbend_range: i32, // default = 48
    legacy_mode_channel_range:   Range<i32>, // = { 1, 17 };
    num_midi_channels:           i32, // default = 16
}

impl<'a> MpeSetupComponentListener for ZoneLayoutComponent<'a> {

    fn zone_changed(&mut self, 
        is_lower_zone:            bool,
        num_member_channels:      i32,
        per_note_pitchbend_range: i32,
        master_pitchbend_range:   i32)  {
        
        todo!();
        /*
            if (isLowerZone)
                zoneLayout.setLowerZone (numMemberChannels, perNotePitchbendRange, masterPitchbendRange);
            else
                zoneLayout.setUpperZone (numMemberChannels, perNotePitchbendRange, masterPitchbendRange);

            repaint();
        */
    }
    
    fn all_zones_cleared(&mut self)  {
        
        todo!();
        /*
            zoneLayout.clearAllZones();
            repaint();
        */
    }
    
    fn legacy_mode_changed(&mut self, 
        legacy_mode_should_be_enabled: bool,
        pitchbend_range:               i32,
        channel_range:                 Range<i32>)  {
        
        todo!();
        /*
            legacyModeEnabled = legacyModeShouldBeEnabled;
            legacyModePitchbendRange = pitchbendRange;
            legacyModeChannelRange = channelRange;

            repaint();
        */
    }
    
    fn voice_stealing_enabled_changed(&mut self, _0: bool)  {
        
        todo!();
        /*
            /* not interested in this change */
        */
    }
    
    fn number_of_voices_changed(&mut self, _0: i32)  {
        
        todo!();
        /*
            /* not interested in this change */
        */
    }
}

impl<'a> ZoneLayoutComponent<'a> {

    pub fn new(zone_colour_picker: &ZoneColourPicker) -> Self {
    
        todo!();
        /*
        : colour_picker(zoneColourPicker),

        
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            paintBackground (g);

            if (legacyModeEnabled)
                paintLegacyMode (g);
            else
                paintZones (g);
        */
    }
    
    
    pub fn paint_background(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (Colours::black);
            auto channelWidth = getChannelRectangleWidth();

            for (auto i = 0; i < numMidiChannels; ++i)
            {
                auto x = float (i) * channelWidth;
                Rectangle<int> channelArea ((int) x, 0, (int) channelWidth, getHeight());

                g.drawLine ({ x, 0.0f, x, float (getHeight()) });
                g.drawText (String (i + 1), channelArea.reduced (4, 4), Justification::topLeft, false);
            }
        */
    }
    
    pub fn paint_zones(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto channelWidth = getChannelRectangleWidth();

            Vec<MPEZoneLayout::Zone> activeZones;
            if (zoneLayout.getLowerZone().isActive())  activeZones.add (zoneLayout.getLowerZone());
            if (zoneLayout.getUpperZone().isActive())  activeZones.add (zoneLayout.getUpperZone());

            for (auto zone : activeZones)
            {
                auto zoneColour = colourPicker.getColourForZone (zone.isLowerZone());

                auto xPos = zone.isLowerZone() ? 0 : zone.getLastMemberChannel() - 1;

                Rectangle<int> zoneRect { int (channelWidth * (float) xPos), 20,
                                          int (channelWidth * (float) (zone.numMemberChannels + 1)), getHeight() - 20 };

                g.setColour (zoneColour);
                g.drawRect (zoneRect, 3);

                auto masterRect = zone.isLowerZone() ? zoneRect.removeFromLeft ((int) channelWidth) : zoneRect.removeFromRight ((int) channelWidth);

                g.setColour (zoneColour.withAlpha (0.3f));
                g.fillRect (masterRect);

                g.setColour (zoneColour.contrasting());
                g.drawText ("<>" + String (zone.masterPitchbendRange),  masterRect.reduced (4), Justification::top,    false);
                g.drawText ("<>" + String (zone.perNotePitchbendRange), masterRect.reduced (4), Justification::bottom, false);
            }
        */
    }
    
    pub fn paint_legacy_mode(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto startChannel = legacyModeChannelRange.getStart() - 1;
            auto numChannels  = legacyModeChannelRange.getEnd() - startChannel - 1;


            Rectangle<int> zoneRect (int (getChannelRectangleWidth() * (float) startChannel), 0,
                                     int (getChannelRectangleWidth() * (float) numChannels), getHeight());

            zoneRect.removeFromTop (20);

            g.setColour (Colours::white);
            g.drawRect (zoneRect, 3);
            g.drawText ("LGCY", zoneRect.reduced (4, 4), Justification::topLeft, false);
            g.drawText ("<>" + String (legacyModePitchbendRange), zoneRect.reduced (4, 4), Justification::bottomLeft, false);
        */
    }
    
    pub fn get_channel_rectangle_width(&self) -> f32 {
        
        todo!();
        /*
            return (float) getWidth() / (float) numMidiChannels;
        */
    }
}
