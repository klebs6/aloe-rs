crate::ix!();

pub struct MpeInstrumentLegacyMode
{
    is_enabled:      bool,
    channel_range:   Range<i32>,
    pitchbend_range: i32,
}

pub struct MpeInstrumentMPEDimension
{
    tracking_mode:                  MpeInstrumentTrackingMode, // default = lastNotePlayedOnChannel
    last_value_received_on_channel: [MPEValue; 16],
    value:                          *mut MPEValue,
}

impl MpeInstrumentMPEDimension {
    
    pub fn get_value(&mut self, note: &mut MPENote) -> &mut MPEValue {
        
        todo!();
        /*
            return note.*(value);
        */
    }
}

/**
  | The MPE note tracking mode. In case there
  | is more than one note playing simultaneously
  | on the same MIDI channel, this determines
  | which of these notes will be modulated
  | by an incoming MPE message on that channel
  | (pressure, pitchbend, or timbre).
  | 
  | The default is lastNotePlayedOnChannel.
  |
  */
pub enum MpeInstrumentTrackingMode
{
    /** 
      The most recent note on the channel that
      is still played (key down and/or
      sustained). 
      */
    lastNotePlayedOnChannel, 

    /** 
      The lowest note (by initialNote) on the
      channel with the note key still down
      */
    lowestNoteOnChannel,     

    /** 
      The highest note (by initialNote) on the
      channel with the note key still down
      */
    highestNoteOnChannel,    

    /** 
      All notes on the channel (key down
      and/or sustained)
      */
    allNotesOnChannel        
}
