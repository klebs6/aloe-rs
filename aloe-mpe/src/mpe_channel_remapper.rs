crate::ix!();

/**
  | This class handles the logic for remapping
  | MIDI note messages from multiple MPE
  | sources onto a specified MPE zone.
  | 
  | @tags{Audio}
  |
  */
pub struct MPEChannelRemapper {
    zone:               MpeZoneLayoutZone,
    channel_increment:  i32,
    first_channel:      i32,
    last_channel:       i32,
    source_and_channel: [u32; 17],
    last_used:          [u32; 17],
    counter:            u32, // default = 0
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPEUtils.cpp]
impl MPEChannelRemapper {

    /**
      | Used to indicate that a particular source
      | & channel combination is not currently
      | using MPE.
      |
      */
    pub const notMPE: u32 = 0;

    /**
      | Remaps the MIDI channel of the specified
      | MIDI message (if necessary).
      | 
      | Note that the MidiMessage object passed
      | in will have it's channel changed if
      | it needs to be remapped.
      | 
      | -----------
      | @param message
      | 
      | the message to be remapped
      | ----------
      | @param mpeSourceID
      | 
      | the ID of the MPE source of the message.
      | This is up to the user to define and keep
      | constant
      |
      */
    pub fn remap_midi_channel_if_needed(&mut self, 
        message:      &mut MidiMessage,
        mpe_sourceid: u32)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Resets all the source & channel combinations.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Clears a specified channel of this MPE
      | zone.
      |
      */
    pub fn clear_channel(&mut self, channel: i32)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Clears all channels in use by a specified
      | source.
      |
      */
    pub fn clear_source(&mut self, mpe_sourceid: u32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn apply_remap_if_existing(&mut self, 
        channel:              i32,
        source_and_channelid: u32,
        m:                    &mut MidiMessage) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_best_chan_to_reuse(&self) -> i32 {
        
        todo!();
        /*
        
        */
    }
    
    pub fn zero_arrays(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn message_is_note_data(&mut self, m: &MidiMessage) -> bool {
        
        todo!();
        /*
            return (*m.getRawData() & 0xf0) != 0xf0;
        */
    }
}

