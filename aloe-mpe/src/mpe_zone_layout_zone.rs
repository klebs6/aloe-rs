crate::ix!();

/**
  | This struct represents an MPE zone.
  | 
  | It can either be a lower or an upper zone,
  | where:
  | 
  | - A lower zone encompasses master channel
  | 1 and an arbitrary number of ascending
  | MIDI channels, increasing from channel
  | 2.
  | 
  | - An upper zone encompasses master channel
  | 16 and an arbitrary number of descending
  | MIDI channels, decreasing from channel
  | 15.
  | 
  | It also defines a pitchbend range (in
  | semitones) to be applied for per-note
  | pitchbends and master pitchbends,
  | respectively.
  |
  */
#[derive(Clone)]
pub struct MpeZoneLayoutZone {
    num_member_channels:      i32,
    per_note_pitchbend_range: i32,
    master_pitchbend_range:   i32,
    lower_zone:               bool,
}

impl PartialEq<MpeZoneLayoutZone> for MpeZoneLayoutZone {
    
    #[inline] fn eq(&self, other: &MpeZoneLayoutZone) -> bool {
        todo!();
        /*
           return lowerZone == other.lowerZone
           && numMemberChannels == other.numMemberChannels
           && perNotePitchbendRange == other.perNotePitchbendRange
           && masterPitchbendRange == other.masterPitchbendRange;
        */
    }
}

impl Eq for MpeZoneLayoutZone {}

impl MpeZoneLayoutZone {
    
    pub fn is_lower_zone(&self) -> bool {
        
        todo!();
        /*
            return lowerZone;
        */
    }
    
    pub fn is_upper_zone(&self) -> bool {
        
        todo!();
        /*
            return ! lowerZone;
        */
    }
    
    pub fn is_active(&self) -> bool {
        
        todo!();
        /*
            return numMemberChannels > 0;
        */
    }
    
    pub fn get_master_channel(&self) -> i32 {
        
        todo!();
        /*
            return lowerZone ? 1 : 16;
        */
    }
    
    pub fn get_first_member_channel(&self) -> i32 {
        
        todo!();
        /*
            return lowerZone ? 2 : 15;
        */
    }
    
    pub fn get_last_member_channel(&self) -> i32 {
        
        todo!();
        /*
            return lowerZone ? (1 + numMemberChannels)
                                                                             : (16 - numMemberChannels);
        */
    }
    
    pub fn is_using_channel_as_member_channel(&self, channel: i32) -> bool {
        
        todo!();
        /*
            return lowerZone ? (channel > 1 && channel <= 1 + numMemberChannels)
                                 : (channel < 16 && channel >= 16 - numMemberChannels);
        */
    }
    
    pub fn is_using(&self, channel: i32) -> bool {
        
        todo!();
        /*
            return isUsingChannelAsMemberChannel (channel) || channel == getMasterChannel();
        */
    }
    
    pub fn new(
        lower:        bool,
        member_chans: Option<i32>,
        per_note_pb:  Option<i32>,
        master_pb:    Option<i32>

    ) -> Self {

        let member_chans: i32 = member_chans.unwrap_or(0);
        let per_note_pb:  i32 = per_note_pb.unwrap_or(48);
        let master_pb:    i32 = master_pb.unwrap_or(2);

        todo!();
        /*
        : num_member_channels(memberChans),
        : per_note_pitchbend_range(perNotePb),
        : master_pitchbend_range(masterPb),
        : lower_zone(lower),

        
        */
    }
}
