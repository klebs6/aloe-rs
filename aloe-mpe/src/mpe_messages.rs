crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPEMessages.h]

/**
  | This helper class contains the necessary
  | helper functions to generate MIDI messages
  | that are exclusive to MPE, such as defining
  | the upper and lower MPE zones and setting
  | per-note and master pitchbend ranges.  You can
  | then send them to your MPE device using
  | MidiOutput::sendBlockOfMessagesNow.
  |
  | All other MPE messages like per-note
  | pitchbend, pressure, and third dimension, are
  | ordinary MIDI messages that should be created
  | using the MidiMessage class instead. You just
  | need to take care to send them to the
  | appropriate per-note MIDI channel.
  |
  | Note: If you are working with an MPEZoneLayout
  | object inside your app, you should not use the
  | message sequences provided here. Instead, you
  | should change the zone layout programmatically
  | with the member functions provided in the
  | MPEZoneLayout class itself. You should also
  | make sure that the Expressive MIDI zone layout
  | of your C++ code and of the MPE device are
  | kept in sync.
  |
  | @see MidiMessage, MPEZoneLayout
  |
  | @tags{Audio}
  */
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPEMessages.cpp]
pub struct MPEMessages {

}

impl MPEMessages {

    /**
      | The RPN number used for MPE zone layout
      | messages.
      | 
      | Pitchbend range messages (both per-note
      | and master) are instead sent on RPN 0
      | as in standard MIDI 1.0.
      |
      */
    pub const zoneLayoutMessagesRpnNumber: i32 = 6;

    /**
      | Returns the sequence of MIDI messages
      | that, if sent to an Expressive MIDI device,
      | will set the lower MPE zone.
      |
      */
    pub fn set_lower_zone(
        &mut self, 
        num_member_channels:      Option<i32>,
        per_note_pitchbend_range: Option<i32>,
        master_pitchbend_range:   Option<i32>

    ) -> MidiBuffer {

        let num_member_channels:      i32 = num_member_channels.unwrap_or(0);
        let per_note_pitchbend_range: i32 = per_note_pitchbend_range.unwrap_or(48);
        let master_pitchbend_range:   i32 = master_pitchbend_range.unwrap_or(2);
        
        todo!();
        /*
            auto buffer = MidiRPNGenerator::generate (1, zoneLayoutMessagesRpnNumber, numMemberChannels, false, false);

        buffer.addEvents (setLowerZonePerNotePitchbendRange (perNotePitchbendRange), 0, -1, 0);
        buffer.addEvents (setLowerZoneMasterPitchbendRange (masterPitchbendRange), 0, -1, 0);

        return buffer;
        */
    }
    
    /**
      | Returns the sequence of MIDI messages
      | that, if sent to an Expressive MIDI device,
      | will set the upper MPE zone.
      |
      */
    pub fn set_upper_zone(
        &mut self, 
        num_member_channels:      Option<i32>,
        per_note_pitchbend_range: Option<i32>,
        master_pitchbend_range:   Option<i32>

    ) -> MidiBuffer {

        let num_member_channels:      i32 = num_member_channels.unwrap_or(0);
        let per_note_pitchbend_range: i32 = per_note_pitchbend_range.unwrap_or(48);
        let master_pitchbend_range:   i32 = master_pitchbend_range.unwrap_or(2);
        
        todo!();
        /*
            auto buffer = MidiRPNGenerator::generate (16, zoneLayoutMessagesRpnNumber, numMemberChannels, false, false);

        buffer.addEvents (setUpperZonePerNotePitchbendRange (perNotePitchbendRange), 0, -1, 0);
        buffer.addEvents (setUpperZoneMasterPitchbendRange (masterPitchbendRange), 0, -1, 0);

        return buffer;
        */
    }
    
    /**
      | Returns the sequence of MIDI messages
      | that, if sent to an Expressive MIDI device,
      | will set the per-note pitchbend range
      | of the lower MPE zone.
      |
      */
    pub fn set_lower_zone_per_note_pitchbend_range(&mut self, per_note_pitchbend_range: Option<i32>) -> MidiBuffer {

        let per_note_pitchbend_range: i32 = per_note_pitchbend_range.unwrap_or(48);
        
        todo!();
        /*
            return MidiRPNGenerator::generate (2, 0, perNotePitchbendRange, false, false);
        */
    }
    
    /**
      | Returns the sequence of MIDI messages
      | that, if sent to an Expressive MIDI device,
      | will set the per-note pitchbend range
      | of the upper MPE zone.
      |
      */
    pub fn set_upper_zone_per_note_pitchbend_range(&mut self, per_note_pitchbend_range: Option<i32>) -> MidiBuffer {

        let per_note_pitchbend_range: i32 = per_note_pitchbend_range.unwrap_or(48);
        
        todo!();
        /*
            return MidiRPNGenerator::generate (15, 0, perNotePitchbendRange, false, false);
        */
    }
    
    /**
      | Returns the sequence of MIDI messages
      | that, if sent to an Expressive MIDI device,
      | will set the master pitchbend range
      | of the lower MPE zone.
      |
      */
    pub fn set_lower_zone_master_pitchbend_range(&mut self, master_pitchbend_range: Option<i32>) -> MidiBuffer {

        let master_pitchbend_range: i32 = master_pitchbend_range.unwrap_or(2);
        
        todo!();
        /*
            return MidiRPNGenerator::generate (1, 0, masterPitchbendRange, false, false);
        */
    }
    
    /**
      | Returns the sequence of MIDI messages
      | that, if sent to an Expressive MIDI device,
      | will set the master pitchbend range
      | of the upper MPE zone.
      |
      */
    pub fn set_upper_zone_master_pitchbend_range(&mut self, master_pitchbend_range: Option<i32>) -> MidiBuffer {

        let master_pitchbend_range: i32 = master_pitchbend_range.unwrap_or(2);
        
        todo!();
        /*
            return MidiRPNGenerator::generate (16, 0, masterPitchbendRange, false, false);
        */
    }
    
    /**
      | Returns the sequence of MIDI messages
      | that, if sent to an Expressive MIDI device,
      | will clear the lower zone.
      |
      */
    pub fn clear_lower_zone(&mut self) -> MidiBuffer {
        
        todo!();
        /*
            return MidiRPNGenerator::generate (1, zoneLayoutMessagesRpnNumber, 0, false, false);
        */
    }
    
    /**
      | Returns the sequence of MIDI messages
      | that, if sent to an Expressive MIDI device,
      | will clear the upper zone.
      |
      */
    pub fn clear_upper_zone(&mut self) -> MidiBuffer {
        
        todo!();
        /*
            return MidiRPNGenerator::generate (16, zoneLayoutMessagesRpnNumber, 0, false, false);
        */
    }
    
    /**
      | Returns the sequence of MIDI messages
      | that, if sent to an Expressive MIDI device,
      | will clear the lower and upper zones.
      |
      */
    pub fn clear_all_zones(&mut self) -> MidiBuffer {
        
        todo!();
        /*
            MidiBuffer buffer;

        buffer.addEvents (clearLowerZone(), 0, -1, 0);
        buffer.addEvents (clearUpperZone(), 0, -1, 0);

        return buffer;
        */
    }
    
    /**
      | Returns the sequence of MIDI messages
      | that, if sent to an Expressive MIDI device,
      | will reset the whole MPE zone layout
      | of the device to the layout passed in.
      | This will first clear the current lower
      | and upper zones, then then set the zones
      | contained in the passed-in zone layout,
      | and set their per-note and master pitchbend
      | ranges to their current values.
      |
      */
    pub fn set_zone_layout(&mut self, layout: MPEZoneLayout) -> MidiBuffer {
        
        todo!();
        /*
            MidiBuffer buffer;

        buffer.addEvents (clearAllZones(), 0, -1, 0);

        auto lowerZone = layout.getLowerZone();
        if (lowerZone.isActive())
            buffer.addEvents (setLowerZone (lowerZone.numMemberChannels,
                                            lowerZone.perNotePitchbendRange,
                                            lowerZone.masterPitchbendRange),
                              0, -1, 0);

        auto upperZone = layout.getUpperZone();
        if (upperZone.isActive())
            buffer.addEvents (setUpperZone (upperZone.numMemberChannels,
                                            upperZone.perNotePitchbendRange,
                                            upperZone.masterPitchbendRange),
                              0, -1, 0);

        return buffer;
        */
    }
}
